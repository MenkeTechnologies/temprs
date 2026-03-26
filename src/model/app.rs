use fs::{create_dir, remove_dir_all};
use io::stdin;
use std::env::temp_dir;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::PathBuf;
use std::process::exit;

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
            let paths = util_file_to_paths(master_file.as_path());
            let exist: Vec<PathBuf> = paths.into_iter().filter(|p| p.exists()).collect();
            debug!("exists size {}", exist.len());
            util_paths_to_file(exist, &master_file);
        }

        debug!("out file {}", out_file.display());
        debug!("file stack {}", master_file.display());

        let temp_file_stack = util_file_to_paths(&master_file);
        debug!("found '{}' temp files on stack", temp_file_stack.len());

        let state = TempState::new(
            out_file,
            master_file,
            temprs_dir,
            temp_file_stack,
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
        self.print_buffer_or_stack_file();
    }


    fn write_stdout_pipe(&mut self) {
        debug!("stdout pipe");
        self.print_buffer_or_stack_file();
    }


    fn add_idx_in_stack(&mut self, f: String) {
        match f.parse::<i32>() {
            Ok(idx) => {
                let mut cur_files = self.state().temp_file_stack().clone();
                cur_files.insert(
                    util_transform_idx(idx, cur_files.len()),
                    self.state().new_temp_file().clone(),
                );
                util_paths_to_file(cur_files, self.state().master_record_file());
            }
            Err(_error) => {
                util_terminate_error(ERR_INVALID_IDX);
            }
        }
    }

    fn idx_in_stack_tempfile(&mut self, f: String) -> Option<&PathBuf> {
        match f.parse::<i32>() {
            Ok(idx) => {
                let stk = self.state().temp_file_stack();
                stk.get(util_transform_idx(idx, stk.len()))
            }
            Err(_error) => {
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
        buffer.push('\n');
        util_append_file(self.state().master_record_file(), &buffer);
    }

    fn parse_opts(&mut self) {
        let matches = parse_opts().get_matches();

        if matches.is_present(LIST_FILES) {
            self.list_tempfiles();
        }
        if matches.is_present(LIST_FILES_NUMBERED) {
            self.list_tempfiles_numbered();
        }

        if matches.is_present(DIRECTORY) {
            self.list_home();
        }
        if matches.is_present(MASTER) {
            self.list_master();
        }
        if matches.is_present(VERBOSE) {
            let _ = simple_logger::init_with_level(Level::Debug);
            self.state().set_verbose(1);
        }

        if matches.is_present(LIST_CONTENTS) {
            self.list_tempfiles_contents();
        }
        if matches.is_present(LIST_CONTENTS_NUMBERED) {
            self.list_tempfiles_contents_numbered();
        }
        if matches.is_present(CLEAR) {
            self.clear_all();
        }
        if matches.is_present(SHIFT) {
            self.remove_at_idx(format!("{}", 1))
        }

        if matches.is_present(UNSHIFT) {
            self.state().set_insert_idx(Some(String::from("1")));
        }

        if matches.is_present(POP) {
            let top = self.state().temp_file_stack().len();
            self.remove_at_idx(format!("{}", top))
        }

        if matches.is_present(SILENT) {
            self.state().set_silent(true);
        }
        if let Some(f) = matches.value_of(REMOVE) {
            self.remove_at_idx(String::from(f));
        }
        if let Some(i) = matches.value_of(ADD) {
            self.state().set_insert_idx(Some(String::from(i)));
        }
        if let Some(f) = matches.value_of(ARGFILE) {
            self.state().set_arg_file(Some(PathBuf::from(f)));
        }
        if let Some(i) = matches.value_of(INPUT) {
            self.state().set_input_temp_file(Some(String::from(i)));
        }
        if let Some(i) = matches.value_of(OUTPUT) {
            self.state().set_output_temp_file(Some(String::from(i)));
        }
    }

    fn list_tempfiles_contents(&mut self) {
        debug!("list contents");
        let stk = self.state().temp_file_stack();
        for p in stk.iter() {
            let string = util_file_contents_to_string(p.as_path()).expect(ERR_FILE_READ);
            println!("{}", string.trim_end());
        }
        exit(0)
    }

    fn list_tempfiles_contents_numbered(&mut self) {
        debug!("list contents");
        let stk = self.state().temp_file_stack();
        if !stk.is_empty() {
            util_horiz_rule();
        }
        for (i, p) in stk.iter().enumerate() {
            println!("{}: {}", i + 1, util_path_to_string(p));
            let string = util_file_contents_to_string(p.as_path()).expect(ERR_FILE_READ);
            println!("{}", string.trim_end());
            util_horiz_rule();
        }
        exit(0)
    }

    fn list_home(&mut self) {
        let dir = self.state().temprs_dir();

        println!("{}", util_path_to_string(dir));
        exit(0)
    }

    fn list_master(&mut self) {
        let master = self.state().master_record_file();

        println!("{}", util_path_to_string(master));
        exit(0)
    }

    fn list_tempfiles(&mut self) {
        debug!("list files");
        let stk = self.state().temp_file_stack();
        for p in stk.iter() {
            println!("{}", util_path_to_string(p));
        }
        exit(0)
    }

    fn list_tempfiles_numbered(&mut self) {
        debug!("list files");
        let stk = self.state().temp_file_stack();
        if !stk.is_empty() {
            util_horiz_rule();
        }
        for (i, p) in stk.iter().enumerate() {
            println!("{}: {}", i + 1, util_path_to_string(p));
            util_horiz_rule();
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

    fn remove_at_idx(&mut self, stk_idx: String) {
        let cur = self.state().temp_file_stack().clone();
        match self.idx_in_stack_tempfile(stk_idx) {
            Some(f) => {
                util_remove_file(f);
                let col: Vec<PathBuf> = cur.into_iter().filter(|p| p != f).collect();

                util_paths_to_file(col, self.state().master_record_file());
                exit(0)
            }
            None => util_terminate_error(ERR_INVALID_IDX),
        }
    }
}
