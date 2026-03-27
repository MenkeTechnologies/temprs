use fs::{create_dir, remove_dir_all};
use io::stdin;
use std::env::temp_dir;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::PathBuf;
use std::process::{exit, Command as ProcessCommand};

use std::io::IsTerminal;
use log::{debug, Level};

use crate::model::opts::parse_opts;
use crate::model::state::TempState;
use crate::util::consts::*;
use crate::util::utils::*;

pub struct TempApp {
    state: TempState,
}

impl TempApp {

    pub fn run(&mut self) {
        self.parse_opts();
        self.input();
        self.output()
    }


    fn input(&mut self) {
        if !stdin().is_terminal() {
            self.read_stdin_pipe()
        } else {
            self.read_stdin_terminal();
        }
    }


    pub fn new() -> Self {
        simple_logger::init_with_level(TEMP_LOG_LEVEL).expect(ERR_LOGGER);

        let system_temp_dir = match std::env::var("TEMPRS_DIR") {
            Ok(dir) => PathBuf::from(dir),
            Err(_) => {
                let mut d = temp_dir();
                d.push(TEMP_DIR);
                d
            }
        };

        let mut temprs_dir = PathBuf::new();
        temprs_dir.push(system_temp_dir.as_path());

        let mut out_file = PathBuf::new();
        let mut master_file = PathBuf::new();

        out_file.push(system_temp_dir.as_path());
        master_file.push(system_temp_dir.as_path());

        out_file.push(format!("{}{}", TEMPFILE_PREFIX, util_time_ms()));
        master_file.push(MASTER_RECORD_FILENAME);

        if !temprs_dir.exists() {
            match create_dir(temprs_dir.as_path()) {
                Ok(_success) => {
                    debug!("create temp dir {}", temprs_dir.display());
                }
                Err(error) => {
                    panic!("_____________'e' = '{}'_____________", error);
                }
            }
        }

        if !master_file.exists() {
            match File::create(&master_file) {
                Ok(_success) => {
                    debug!("create master file {}", master_file.display());
                }
                Err(error) => {
                    panic!("_____________'e' = '{}'_____________", error);
                }
            }
        } else {
            let (paths, names) = util_file_to_paths_and_names(master_file.as_path());
            let (exist, exist_names): (Vec<PathBuf>, Vec<Option<String>>) = paths.into_iter()
                .zip(names.into_iter())
                .filter(|(p, _)| p.exists())
                .unzip();
            debug!("exists size {}", exist.len());
            util_paths_and_names_to_file(exist, &exist_names, &master_file);
        }

        debug!("out file {}", out_file.display());
        debug!("file stack {}", master_file.display());

        let (temp_file_stack, temp_file_names) = util_file_to_paths_and_names(&master_file);
        debug!("found '{}' temp files on stack", temp_file_stack.len());

        let state = TempState::new(
            out_file,
            master_file,
            temprs_dir,
            temp_file_stack,
            temp_file_names,
            None,
            String::new(),
        );

        Self { state }
    }


    pub fn state(&mut self) -> &mut TempState {
        &mut self.state
    }


    fn read_stdin_terminal(&mut self) {
        debug!("stdin term");

        match self.state().arg_file() {
            Some(arg_file) => {
                let str = util_file_contents_to_string(arg_file.as_path()).expect(ERR_FILE_READ);
                if self.state.verbose() > 0 {
                    self.state().set_output_buffer(str.clone());
                }
                self.state().set_holding_buffer(str);

                self.overwrite_idx_or_write_new_tempfile();
            }
            None => {
                if let Some(f) = self.state().temp_file_stack().last() {
                    let string = util_file_contents_to_string(f.as_path()).expect(ERR_FILE_READ);
                    self.state().set_output_buffer(string);
                }
            }
        }
    }


    fn output(&mut self) {
        if !io::stdout().is_terminal() {
            self.write_stdout_pipe();
        } else {
            self.write_stdout_terminal();
        }
    }


    fn write_stdout_terminal(&mut self) {
        debug!("stdout term");
        self.cyber_print_buffer_or_stack_file();
    }


    fn write_stdout_pipe(&mut self) {
        debug!("stdout pipe");
        self.print_buffer_or_stack_file();
    }


    fn add_idx_in_stack(&mut self, f: String) {
        match f.parse::<i32>() {
            Ok(idx) => {
                let mut cur_files = self.state().temp_file_stack().clone();
                let mut cur_names = self.state().temp_file_names().clone();
                let insert_pos = util_transform_idx(idx, cur_files.len());
                cur_files.insert(insert_pos, self.state().new_temp_file().clone());
                cur_names.insert(insert_pos, self.state().name().clone());
                util_paths_and_names_to_file(cur_files, &cur_names, self.state().master_record_file());
            }
            Err(_error) => {
                util_terminate_error(ERR_INVALID_IDX);
            }
        }
    }

    fn resolve_idx(&self, f: &str) -> Option<usize> {
        match f.parse::<i32>() {
            Ok(idx) => {
                let stk = self.state.temp_file_stack();
                Some(util_transform_idx(idx, stk.len()))
            }
            Err(_) => {
                self.state.temp_file_names().iter().position(|n| {
                    n.as_deref() == Some(f)
                })
            }
        }
    }

    fn idx_in_stack_tempfile(&mut self, f: String) -> Option<&PathBuf> {
        match self.resolve_idx(&f) {
            Some(idx) => self.state().temp_file_stack().get(idx),
            None => {
                util_terminate_error(ERR_INVALID_IDX);
                None
            }
        }
    }


    fn print_buffer_or_stack_file(&mut self) {
        match self.state().output_temp_file().clone() {
            Some(stk_idx) => {
                if let Some(f) = self.idx_in_stack_tempfile(stk_idx) {
                    print!(
                        "{}",
                        util_file_contents_to_string(f.as_path()).expect(ERR_FILE_READ)
                    );
                }
            }
            None => {
                if !self.state().output_buffer().is_empty() {
                    print!("{}", self.state().output_buffer());
                }
            }
        }
    }

    fn cyber_print_buffer_or_stack_file(&mut self) {
        match self.state().output_temp_file().clone() {
            Some(stk_idx) => {
                if let Some(f) = self.idx_in_stack_tempfile(stk_idx) {
                    let content = util_file_contents_to_string(f.as_path()).expect(ERR_FILE_READ);
                    cyber_print_content(&content);
                }
            }
            None => {
                if !self.state().output_buffer().is_empty() {
                    cyber_print_content(self.state().output_buffer());
                }
            }
        }
    }


    fn read_stdin_pipe(&mut self) {
        debug!("stdin pipe");
        let mut str = String::new();
        if let Err(_e) = stdin().read_to_string(&mut str) {
            util_terminate_error(ERR_FILE_READ);
        }

        if self.state.verbose() > 0 {
            self.state().set_output_buffer(str.clone());
        }
        self.state().set_holding_buffer(str);

        self.overwrite_idx_or_write_new_tempfile()
    }


    fn overwrite_idx_or_write_new_tempfile(&mut self) {
        let file_contents = String::from(self.state().holding_buffer());
        match self.state().input_temp_file().clone() {
            Some(stk_idx) => {
                if let Some(f) = self.idx_in_stack_tempfile(stk_idx) {
                    util_overwrite_file(f, &file_contents);
                }
            }
            None => {
                let insert_idx = self.state().insert_idx().clone();
                match insert_idx {
                    Some(idx) => {
                        self.add_idx_in_stack(idx);
                        util_overwrite_file(self.state().new_temp_file(), &file_contents);
                    }
                    None => {
                        self.append_to_master_list();
                        util_overwrite_file(self.state().new_temp_file(), &file_contents);
                    }
                }
            }
        }
    }


    fn append_to_master_list(&mut self) {
        debug!(
            "append file {} to master",
            self.state().new_temp_file().display()
        );

        let mut buffer = String::new();
        buffer.push_str(self.state().out_file_path_str().as_str());
        if let Some(ref name) = self.state().name().clone() {
            buffer.push('\t');
            buffer.push_str(name);
        }
        buffer.push('\n');
        util_append_file(self.state().master_record_file(), &buffer);
    }

    fn parse_opts(&mut self) {
        let matches = parse_opts().get_matches();

        if matches.get_flag(LIST_FILES) {
            self.list_tempfiles();
        }
        if matches.get_flag(LIST_FILES_NUMBERED) {
            self.list_tempfiles_numbered();
        }

        if matches.get_flag(DIRECTORY) {
            self.list_home();
        }
        if matches.get_flag(MASTER) {
            self.list_master();
        }
        if matches.get_count(VERBOSE) > 0 {
            let _ = simple_logger::init_with_level(Level::Debug);
            self.state().set_verbose(1);
        }

        if matches.get_flag(LIST_CONTENTS) {
            self.list_tempfiles_contents();
        }
        if matches.get_flag(LIST_CONTENTS_NUMBERED) {
            self.list_tempfiles_contents_numbered();
        }
        if matches.get_flag(CLEAR) {
            self.clear_all();
        }
        if matches.get_flag(SHIFT) {
            self.remove_at_idx(format!("{}", 1))
        }

        if matches.get_flag(UNSHIFT) {
            self.state().set_insert_idx(Some(String::from("1")));
        }

        if matches.get_flag(POP) {
            let top = self.state().temp_file_stack().len();
            self.remove_at_idx(format!("{}", top))
        }

        if matches.get_flag(SILENT) {
            self.state().set_silent(true);
        }
        if let Some(f) = matches.get_one::<String>(EDIT) {
            self.edit_tempfile(f.clone());
        }
        if let Some(f) = matches.get_one::<String>(INFO) {
            self.info_tempfile(f.clone());
        }
        if let Some(vals) = matches.get_many::<String>(RENAME) {
            let v: Vec<String> = vals.cloned().collect();
            self.rename_tag(v[0].clone(), v[1].clone());
        }
        if let Some(f) = matches.get_one::<String>(REMOVE) {
            self.remove_at_idx(f.clone());
        }
        if let Some(i) = matches.get_one::<String>(ADD) {
            self.state().set_insert_idx(Some(i.clone()));
        }
        if let Some(f) = matches.get_one::<String>(ARGFILE) {
            self.state().set_arg_file(Some(PathBuf::from(f)));
        }
        if let Some(i) = matches.get_one::<String>(INPUT) {
            self.state().set_input_temp_file(Some(i.clone()));
        }
        if let Some(i) = matches.get_one::<String>(OUTPUT) {
            self.state().set_output_temp_file(Some(i.clone()));
        }
        if let Some(n) = matches.get_one::<String>(TAG) {
            let name = n.clone();
            if self.state().temp_file_names().iter().any(|existing| existing.as_deref() == Some(&name)) {
                util_terminate_error(ERR_INVALID_NAME);
            }
            self.state().set_name(Some(name));
        }
    }

    fn list_tempfiles_contents(&mut self) {
        debug!("list contents");
        let stk = self.state().temp_file_stack();
        for p in stk.iter() {
            let string = util_file_contents_to_string(p.as_path()).expect(ERR_FILE_READ);
            cyber_content(&string);
        }
        exit(0)
    }

    fn list_tempfiles_contents_numbered(&mut self) {
        debug!("list contents");
        let stk = self.state().temp_file_stack().clone();
        let names = self.state().temp_file_names().clone();
        if !stk.is_empty() {
            cyber_hr();
        }
        for (i, (p, n)) in stk.iter().zip(names.iter()).enumerate() {
            let string = util_file_contents_to_string(p.as_path()).expect(ERR_FILE_READ);
            cyber_idx_content_named(i + 1, p, &string, n);
            cyber_hr();
        }
        exit(0)
    }

    fn list_home(&mut self) {
        let dir = self.state().temprs_dir();
        cyber_single_path(dir);
        exit(0)
    }

    fn list_master(&mut self) {
        let master = self.state().master_record_file();
        cyber_single_path(master);
        exit(0)
    }

    fn list_tempfiles(&mut self) {
        debug!("list files");
        let stk = self.state().temp_file_stack();
        for p in stk.iter() {
            cyber_path(p);
        }
        exit(0)
    }

    fn list_tempfiles_numbered(&mut self) {
        debug!("list files");
        let stk = self.state().temp_file_stack().clone();
        let names = self.state().temp_file_names().clone();
        if !stk.is_empty() {
            cyber_hr();
        }
        for (i, (p, n)) in stk.iter().zip(names.iter()).enumerate() {
            cyber_idx_path_named(i + 1, p, n);
            cyber_hr();
        }
        exit(0)
    }

    fn clear_all(&mut self) {
        if let Err(_e) = remove_dir_all(
            self.state()
                .master_record_file()
                .as_path()
                .parent()
                .expect(ERR_NO_FILE),
        ) {
            util_terminate_error(ERR_NO_FILE);
        }
        exit(0)
    }

    fn info_tempfile(&mut self, stk_idx: String) {
        match self.resolve_idx(&stk_idx) {
            Some(idx) => {
                let path = self.state().temp_file_stack()[idx].clone();
                let name = self.state().temp_file_names()[idx].clone();
                let meta = fs::metadata(&path).expect(ERR_FILE_READ);
                let size = meta.len();
                let modified = meta.modified().ok().and_then(|t| {
                    t.duration_since(std::time::UNIX_EPOCH).ok()
                }).map(|d| d.as_secs()).unwrap_or(0);

                let size_str = if size < 1024 {
                    format!("{} B", size)
                } else if size < 1024 * 1024 {
                    format!("{:.1} KiB", size as f64 / 1024.0)
                } else {
                    format!("{:.1} MiB", size as f64 / (1024.0 * 1024.0))
                };

                if io::stdout().is_terminal() {
                    cyber_hr();
                    println!("\x1b[33m  index:\x1b[0m  {}", idx + 1);
                    if let Some(ref n) = name {
                        println!("\x1b[33m  name:\x1b[0m   \x1b[35m@{}\x1b[0m", n);
                    }
                    println!("\x1b[33m  path:\x1b[0m   \x1b[35m{}\x1b[0m", path.display());
                    println!("\x1b[33m  size:\x1b[0m   {}", size_str);
                    println!("\x1b[33m  mtime:\x1b[0m  {}", modified);
                    cyber_hr();
                } else {
                    println!("index: {}", idx + 1);
                    if let Some(ref n) = name {
                        println!("name: {}", n);
                    }
                    println!("path: {}", path.display());
                    println!("size: {}", size_str);
                    println!("mtime: {}", modified);
                }
                exit(0)
            }
            None => util_terminate_error(ERR_INVALID_IDX),
        }
    }

    fn rename_tag(&mut self, old: String, new: String) {
        if self.state().temp_file_names().iter().any(|n| n.as_deref() == Some(&new)) {
            util_terminate_error(ERR_INVALID_NAME);
        }
        match self.resolve_idx(&old) {
            Some(idx) => {
                let mut names = self.state().temp_file_names().clone();
                names[idx] = Some(new);
                let paths = self.state().temp_file_stack().clone();
                util_paths_and_names_to_file(paths, &names, self.state().master_record_file());
                exit(0)
            }
            None => util_terminate_error(ERR_INVALID_IDX),
        }
    }

    fn edit_tempfile(&mut self, stk_idx: String) {
        let editor = std::env::var("EDITOR").unwrap_or_else(|_| String::from("vi"));
        match self.idx_in_stack_tempfile(stk_idx) {
            Some(f) => {
                let path = f.clone();
                let status = ProcessCommand::new(&editor)
                    .arg(&path)
                    .status()
                    .expect(ERR_EDITOR);
                exit(status.code().unwrap_or(1))
            }
            None => util_terminate_error(ERR_INVALID_IDX),
        }
    }

    fn remove_at_idx(&mut self, stk_idx: String) {
        match self.resolve_idx(&stk_idx) {
            Some(idx) => {
                let cur = self.state().temp_file_stack().clone();
                let cur_names = self.state().temp_file_names().clone();
                if let Some(f) = cur.get(idx) {
                    util_remove_file(f);
                }
                let col: Vec<PathBuf> = cur.into_iter().enumerate().filter(|(i, _)| *i != idx).map(|(_, p)| p).collect();
                let col_names: Vec<Option<String>> = cur_names.into_iter().enumerate().filter(|(i, _)| *i != idx).map(|(_, n)| n).collect();
                util_paths_and_names_to_file(col, &col_names, self.state().master_record_file());
                exit(0)
            }
            None => util_terminate_error(ERR_INVALID_IDX),
        }
    }
}
