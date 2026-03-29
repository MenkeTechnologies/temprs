use fs::{create_dir, remove_dir_all};
use io::stdin;
use std::env::temp_dir;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::PathBuf;
use std::process::{Command as ProcessCommand, exit};

use fs2::FileExt;
use log::{Level, debug};
use std::io::IsTerminal;

use crate::model::opts::parse_opts;
use crate::model::state::TempState;
use crate::util::consts::*;
use crate::util::utils::*;

pub struct TempApp {
    state: TempState,
    _lock_file: File,
}

impl Default for TempApp {
    fn default() -> Self {
        Self::new()
    }
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
        if simple_logger::init_with_level(TEMP_LOG_LEVEL).is_err() {
            eprintln!("{}", ERR_LOGGER);
        }

        let temprs_dir = match std::env::var("TEMPRS_DIR") {
            Ok(dir) => PathBuf::from(dir),
            Err(_) => temp_dir().join(TEMP_DIR),
        };

        let out_file = temprs_dir.join(format!("{}{}", TEMPFILE_PREFIX, util_time_ms()));
        let master_file = temprs_dir.join(MASTER_RECORD_FILENAME);

        if !temprs_dir.exists() {
            match create_dir(temprs_dir.as_path()) {
                Ok(_success) => {
                    debug!("create temp dir {}", temprs_dir.display());
                }
                Err(_error) => {
                    util_terminate_error(ERR_NO_FILE);
                }
            }
        }

        let lock_path = temprs_dir.join(format!("{}.lock", MASTER_RECORD_FILENAME));
        let lock_file = match File::create(&lock_path) {
            Ok(f) => f,
            Err(_) => {
                util_terminate_error(ERR_MASTER_LOCK);
                unreachable!()
            }
        };
        if lock_file.lock_exclusive().is_err() {
            util_terminate_error(ERR_MASTER_LOCK);
        }
        debug!("acquired exclusive lock on {}", lock_path.display());

        let (temp_file_stack, temp_file_names) = if !master_file.exists() {
            match File::create(&master_file) {
                Ok(_success) => {
                    debug!("create master file {}", master_file.display());
                }
                Err(_error) => {
                    util_terminate_error(ERR_FILE_READ);
                }
            }
            (Vec::new(), Vec::new())
        } else {
            let (paths, names) = util_file_to_paths_and_names(master_file.as_path());
            let (exist, exist_names): (Vec<PathBuf>, Vec<Option<String>>) = paths
                .into_iter()
                .zip(names)
                .filter(|(p, _)| p.exists())
                .unzip();
            debug!("exists size {}", exist.len());
            util_paths_and_names_to_file(exist.clone(), &exist_names, &master_file);
            (exist, exist_names)
        };

        debug!("out file {}", out_file.display());
        debug!("file stack {}", master_file.display());
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

        Self {
            state,
            _lock_file: lock_file,
        }
    }

    pub fn state(&mut self) -> &mut TempState {
        &mut self.state
    }

    fn read_stdin_terminal(&mut self) {
        debug!("stdin term");

        let arg_path = self.state.arg_file().clone();
        match arg_path {
            Some(arg_file) => {
                let content = util_file_contents_to_string(arg_file.as_path());
                if self.state.verbose() > 0 {
                    self.state.set_output_buffer(content.clone());
                }
                self.state.set_holding_buffer(content);
                self.overwrite_idx_or_write_new_tempfile();
            }
            None => {
                if self.state.insert_idx().is_some() {
                    self.read_stdin_pipe();
                    return;
                }
                let last_path = self.state.temp_file_stack().last().cloned();
                if let Some(f) = last_path {
                    let content = util_file_contents_to_string(f.as_path());
                    self.state.set_output_buffer(content);
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
                let insert_pos = if cur_files.is_empty() {
                    0
                } else {
                    util_transform_idx(idx, cur_files.len())
                };
                cur_files.insert(insert_pos, self.state().new_temp_file().clone());
                cur_names.insert(insert_pos, self.state().name().clone());
                util_paths_and_names_to_file(
                    cur_files,
                    &cur_names,
                    self.state().master_record_file(),
                );
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
            Err(_) => self
                .state
                .temp_file_names()
                .iter()
                .position(|n| n.as_deref() == Some(f)),
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

    fn resolve_output_path(&self) -> Option<PathBuf> {
        self.state
            .output_temp_file()
            .as_ref()
            .map(|stk_idx| match self.resolve_idx(stk_idx) {
                Some(idx) => self.state.temp_file_stack()[idx].clone(),
                None => {
                    util_terminate_error(ERR_INVALID_IDX);
                    unreachable!()
                }
            })
    }

    fn print_buffer_or_stack_file(&self) {
        if let Some(path) = self.resolve_output_path() {
            print!("{}", util_file_contents_to_string(path.as_path()));
        } else if !self.state.output_buffer().is_empty() {
            print!("{}", self.state.output_buffer());
        }
    }

    fn cyber_print_buffer_or_stack_file(&self) {
        if let Some(path) = self.resolve_output_path() {
            cyber_print_content(&util_file_contents_to_string(path.as_path()));
        } else if !self.state.output_buffer().is_empty() {
            cyber_print_content(self.state.output_buffer());
        }
    }

    fn read_stdin_pipe(&mut self) {
        debug!("stdin pipe");
        let mut input = String::new();
        if let Err(_e) = stdin().read_to_string(&mut input) {
            util_terminate_error(ERR_FILE_READ);
        }

        if self.state.verbose() > 0 {
            self.state.set_output_buffer(input.clone());
        }
        self.state.set_holding_buffer(input);

        self.overwrite_idx_or_write_new_tempfile()
    }

    fn overwrite_idx_or_write_new_tempfile(&mut self) {
        let file_contents = String::from(self.state.holding_buffer());
        let append_idx = self.state.append_temp_file().clone();
        let input_idx = self.state.input_temp_file().clone();
        let insert_idx = self.state.insert_idx().clone();

        if let Some(stk_idx) = append_idx {
            if let Some(f) = self.idx_in_stack_tempfile(stk_idx) {
                util_append_file(f, &file_contents);
            }
            return;
        }
        match input_idx {
            Some(stk_idx) => {
                if let Some(f) = self.idx_in_stack_tempfile(stk_idx) {
                    util_overwrite_file(f, &file_contents);
                }
            }
            None => match insert_idx {
                Some(idx) => {
                    self.add_idx_in_stack(idx);
                    util_overwrite_file(self.state().new_temp_file(), &file_contents);
                }
                None => {
                    self.append_to_master_list();
                    util_overwrite_file(self.state().new_temp_file(), &file_contents);
                }
            },
        }
    }

    fn append_to_master_list(&mut self) {
        debug!(
            "append file {} to master",
            self.state().new_temp_file().display()
        );

        let mut buffer = String::new();
        buffer.push_str(self.state().out_file_path_str().as_str());
        if let Some(name) = self.state().name() {
            buffer.push(MASTER_FIELD_DELIM);
            buffer.push_str(name);
        }
        buffer.push_str(MASTER_RECORD_DELIM);
        util_append_file(self.state().master_record_file(), &buffer);
    }

    fn parse_opts(&mut self) {
        let matches = parse_opts().get_matches();

        // ── state-setting flags (order independent) ──────────
        if matches.get_count(VERBOSE) > 0 {
            let _ = simple_logger::init_with_level(Level::Debug);
            self.state().set_verbose(1);
        }
        if matches.get_flag(SILENT) {
            self.state().set_silent(true);
        }
        if matches.get_flag(UNSHIFT) {
            self.state().set_insert_idx(Some(String::from("1")));
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
        if let Some(i) = matches.get_one::<String>(APPEND) {
            self.state().set_append_temp_file(Some(i.clone()));
        }
        if let Some(n) = matches.get_one::<String>(TAG) {
            if n.contains(MASTER_FIELD_DELIM) {
                util_terminate_error(ERR_NAME_NUL);
            }
            let name = n.clone();
            if self
                .state()
                .temp_file_names()
                .iter()
                .any(|existing| existing.as_deref() == Some(&name))
            {
                util_terminate_error(ERR_INVALID_NAME);
            }
            self.state().set_name(Some(name));
        }

        // ── immediate-exit commands ──────────────────────────
        if matches.get_flag(LIST_FILES) {
            self.list_tempfiles();
        }
        if matches.get_flag(LIST_FILES_NUMBERED) {
            self.list_tempfiles_numbered();
        }
        if matches.get_flag(COUNT) {
            self.count_tempfiles();
        }
        if matches.get_flag(DIRECTORY) {
            self.list_home();
        }
        if matches.get_flag(MASTER) {
            self.list_master();
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
        if matches.get_flag(REVERSE) {
            self.reverse_stack();
        }
        if let Some(h) = matches.get_one::<String>(EXPIRE) {
            self.expire_tempfiles(h.clone());
        }
        if let Some(vals) = matches.get_many::<String>(HEAD) {
            let v: Vec<String> = vals.cloned().collect();
            self.head_tempfile(v[0].clone(), v[1].clone());
        }
        if let Some(vals) = matches.get_many::<String>(TAIL) {
            let v: Vec<String> = vals.cloned().collect();
            self.tail_tempfile(v[0].clone(), v[1].clone());
        }
        if let Some(f) = matches.get_one::<String>(WC) {
            self.wc_tempfile(f.clone());
        }
        if let Some(f) = matches.get_one::<String>(SIZE) {
            self.size_tempfile(f.clone());
        }
        if let Some(key) = matches.get_one::<String>(SORT) {
            self.sort_stack(key.clone());
        }
        if let Some(vals) = matches.get_many::<String>(REPLACE) {
            let v: Vec<String> = vals.cloned().collect();
            self.replace_in_tempfile(v[0].clone(), v[1].clone(), v[2].clone());
        }
        if let Some(f) = matches.get_one::<String>(PATH) {
            self.path_tempfile(f.clone());
        }
        if matches.get_flag(SHIFT) {
            self.remove_at_idx(1.to_string())
        }
        if matches.get_flag(POP) {
            let top = self.state().temp_file_stack().len();
            self.remove_at_idx(top.to_string())
        }
        if let Some(f) = matches.get_one::<String>(EDIT) {
            self.edit_tempfile(f.clone());
        }
        if let Some(f) = matches.get_one::<String>(INFO) {
            self.info_tempfile(f.clone());
        }
        if let Some(p) = matches.get_one::<String>(GREP) {
            self.grep_tempfiles(p.clone());
        }
        if let Some(vals) = matches.get_many::<String>(CAT) {
            let v: Vec<String> = vals.cloned().collect();
            self.cat_tempfiles(v);
        }
        if let Some(vals) = matches.get_many::<String>(DIFF) {
            let v: Vec<String> = vals.cloned().collect();
            self.diff_tempfiles(v[0].clone(), v[1].clone());
        }
        if let Some(vals) = matches.get_many::<String>(MOVE) {
            let v: Vec<String> = vals.cloned().collect();
            self.move_tempfile(v[0].clone(), v[1].clone());
        }
        if let Some(f) = matches.get_one::<String>(DUP) {
            self.dup_tempfile(f.clone());
        }
        if let Some(vals) = matches.get_many::<String>(SWAP) {
            let v: Vec<String> = vals.cloned().collect();
            self.swap_tempfiles(v[0].clone(), v[1].clone());
        }
        if let Some(vals) = matches.get_many::<String>(RENAME) {
            let v: Vec<String> = vals.cloned().collect();
            self.rename_tag(v[0].clone(), v[1].clone());
        }
        if let Some(f) = matches.get_one::<String>(REMOVE) {
            self.remove_at_idx(f.clone());
        }
    }

    fn list_tempfiles_contents(&self) {
        debug!("list contents");
        for p in self.state.temp_file_stack() {
            let string = util_file_contents_to_string(p.as_path());
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
            let string = util_file_contents_to_string(p.as_path());
            cyber_idx_content_named(i + 1, p, &string, n);
            cyber_hr();
        }
        exit(0)
    }

    fn count_tempfiles(&self) {
        println!("{}", self.state.temp_file_stack().len());
        exit(0)
    }

    fn list_home(&self) {
        cyber_single_path(self.state.temprs_dir());
        exit(0)
    }

    fn list_master(&self) {
        cyber_single_path(self.state.master_record_file());
        exit(0)
    }

    fn list_tempfiles(&self) {
        debug!("list files");
        for p in self.state.temp_file_stack() {
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

    fn size_tempfile(&mut self, stk_idx: String) {
        match self.resolve_idx(&stk_idx) {
            Some(idx) => {
                let path = &self.state.temp_file_stack()[idx];
                let meta = match fs::metadata(path) {
                    Ok(m) => m,
                    Err(_) => {
                        util_terminate_error(ERR_FILE_READ);
                        unreachable!()
                    }
                };
                println!("{}", meta.len());
                exit(0)
            }
            None => util_terminate_error(ERR_INVALID_IDX),
        }
    }

    fn wc_tempfile(&mut self, stk_idx: String) {
        match self.resolve_idx(&stk_idx) {
            Some(idx) => {
                let content =
                    util_file_contents_to_string(self.state.temp_file_stack()[idx].as_path());
                println!("{}", content.lines().count());
                exit(0)
            }
            None => util_terminate_error(ERR_INVALID_IDX),
        }
    }

    fn head_tempfile(&mut self, stk_idx: String, n_str: String) {
        let n: usize = match n_str.parse() {
            Ok(v) => v,
            Err(_) => {
                util_terminate_error(ERR_INVALID_IDX);
                unreachable!()
            }
        };
        match self.resolve_idx(&stk_idx) {
            Some(idx) => {
                let content =
                    util_file_contents_to_string(self.state.temp_file_stack()[idx].as_path());
                let lines: String = content.lines().take(n).collect::<Vec<&str>>().join("\n");
                if !lines.is_empty() {
                    if io::stdout().is_terminal() {
                        cyber_print_content(&lines);
                    } else {
                        println!("{}", lines);
                    }
                }
                exit(0)
            }
            None => util_terminate_error(ERR_INVALID_IDX),
        }
    }

    fn tail_tempfile(&mut self, stk_idx: String, n_str: String) {
        let n: usize = match n_str.parse() {
            Ok(v) => v,
            Err(_) => {
                util_terminate_error(ERR_INVALID_IDX);
                unreachable!()
            }
        };
        match self.resolve_idx(&stk_idx) {
            Some(idx) => {
                let content =
                    util_file_contents_to_string(self.state.temp_file_stack()[idx].as_path());
                let all_lines: Vec<&str> = content.lines().collect();
                let start = all_lines.len().saturating_sub(n);
                let lines: String = all_lines[start..].join("\n");
                if !lines.is_empty() {
                    if io::stdout().is_terminal() {
                        cyber_print_content(&lines);
                    } else {
                        println!("{}", lines);
                    }
                }
                exit(0)
            }
            None => util_terminate_error(ERR_INVALID_IDX),
        }
    }

    fn expire_tempfiles(&mut self, hours_str: String) {
        let hours: f64 = match hours_str.parse() {
            Ok(h) => h,
            Err(_) => {
                util_terminate_error(ERR_INVALID_IDX);
                unreachable!()
            }
        };
        let cutoff_secs = (hours * 3600.0) as u64;
        let now = std::time::SystemTime::now();
        let paths = self.state.temp_file_stack().clone();
        let names = self.state.temp_file_names().clone();
        let mut kept_paths = Vec::new();
        let mut kept_names = Vec::new();
        let mut removed = 0usize;

        for (p, n) in paths.into_iter().zip(names.into_iter()) {
            let dominated = fs::metadata(&p)
                .ok()
                .and_then(|m| m.modified().ok())
                .and_then(|mtime| now.duration_since(mtime).ok())
                .map(|age| age.as_secs() >= cutoff_secs)
                .unwrap_or(true);

            if dominated {
                util_remove_file(&p);
                removed += 1;
            } else {
                kept_paths.push(p);
                kept_names.push(n);
            }
        }

        util_paths_and_names_to_file(kept_paths, &kept_names, self.state.master_record_file());
        println!("{}", removed);
        exit(0)
    }

    fn sort_stack(&mut self, key: String) {
        let paths = self.state.temp_file_stack().clone();
        let names = self.state.temp_file_names().clone();
        let mut indices: Vec<usize> = (0..paths.len()).collect();

        match key.as_str() {
            "name" => {
                indices.sort_by(|&a, &b| paths[a].file_name().cmp(&paths[b].file_name()));
            }
            "size" => {
                let sizes: Vec<u64> = paths
                    .iter()
                    .map(|p| fs::metadata(p).map(|m| m.len()).unwrap_or(0))
                    .collect();
                indices.sort_by(|&a, &b| sizes[a].cmp(&sizes[b]));
            }
            "mtime" => {
                let mtimes: Vec<Option<std::time::SystemTime>> = paths
                    .iter()
                    .map(|p| fs::metadata(p).ok().and_then(|m| m.modified().ok()))
                    .collect();
                indices.sort_by(|&a, &b| mtimes[a].cmp(&mtimes[b]));
            }
            _ => {
                util_terminate_error(ERR_INVALID_IDX);
            }
        }

        let sorted_paths: Vec<PathBuf> = indices.iter().map(|&i| paths[i].clone()).collect();
        let sorted_names: Vec<Option<String>> = indices.iter().map(|&i| names[i].clone()).collect();
        util_paths_and_names_to_file(sorted_paths, &sorted_names, self.state.master_record_file());
        exit(0)
    }

    fn reverse_stack(&mut self) {
        let mut paths = self.state.temp_file_stack().clone();
        let mut names = self.state.temp_file_names().clone();
        paths.reverse();
        names.reverse();
        util_paths_and_names_to_file(paths, &names, self.state.master_record_file());
        exit(0)
    }

    fn clear_all(&mut self) {
        let parent = match self.state().master_record_file().as_path().parent() {
            Some(p) => p.to_path_buf(),
            None => {
                util_terminate_error(ERR_NO_FILE);
                unreachable!()
            }
        };
        if let Err(_e) = remove_dir_all(&parent) {
            util_terminate_error(ERR_NO_FILE);
        }
        exit(0)
    }

    fn dup_tempfile(&mut self, stk_idx: String) {
        match self.resolve_idx(&stk_idx) {
            Some(idx) => {
                let content =
                    util_file_contents_to_string(self.state.temp_file_stack()[idx].as_path());
                self.append_to_master_list();
                util_overwrite_file(self.state.new_temp_file(), &content);
                exit(0)
            }
            None => util_terminate_error(ERR_INVALID_IDX),
        }
    }

    fn swap_tempfiles(&mut self, a: String, b: String) {
        let idx_a = match self.resolve_idx(&a) {
            Some(idx) => idx,
            None => {
                util_terminate_error(ERR_INVALID_IDX);
                unreachable!()
            }
        };
        let idx_b = match self.resolve_idx(&b) {
            Some(idx) => idx,
            None => {
                util_terminate_error(ERR_INVALID_IDX);
                unreachable!()
            }
        };
        let mut paths = self.state().temp_file_stack().clone();
        let mut names = self.state().temp_file_names().clone();
        paths.swap(idx_a, idx_b);
        names.swap(idx_a, idx_b);
        util_paths_and_names_to_file(paths, &names, self.state().master_record_file());
        exit(0)
    }

    fn move_tempfile(&mut self, from: String, to: String) {
        let from_idx = match self.resolve_idx(&from) {
            Some(idx) => idx,
            None => {
                util_terminate_error(ERR_INVALID_IDX);
                unreachable!()
            }
        };
        let to_idx = match to.parse::<i32>() {
            Ok(idx) => util_transform_idx(idx, self.state().temp_file_stack().len()),
            Err(_) => {
                util_terminate_error(ERR_INVALID_IDX);
                unreachable!()
            }
        };
        let mut paths = self.state().temp_file_stack().clone();
        let mut names = self.state().temp_file_names().clone();
        let path = paths.remove(from_idx);
        let name = names.remove(from_idx);
        let insert_at = if to_idx > paths.len() {
            paths.len()
        } else {
            to_idx
        };
        paths.insert(insert_at, path);
        names.insert(insert_at, name);
        util_paths_and_names_to_file(paths, &names, self.state().master_record_file());
        exit(0)
    }

    fn diff_tempfiles(&mut self, a: String, b: String) {
        let path_a = match self.resolve_idx(&a) {
            Some(idx) => self.state().temp_file_stack()[idx].clone(),
            None => {
                util_terminate_error(ERR_INVALID_IDX);
                unreachable!()
            }
        };
        let path_b = match self.resolve_idx(&b) {
            Some(idx) => self.state().temp_file_stack()[idx].clone(),
            None => {
                util_terminate_error(ERR_INVALID_IDX);
                unreachable!()
            }
        };
        let status = ProcessCommand::new("diff")
            .arg("-u")
            .arg(&path_a)
            .arg(&path_b)
            .status();
        match status {
            Ok(s) => exit(s.code().unwrap_or(2)),
            Err(_) => {
                util_terminate_error(ERR_FILE_READ);
            }
        }
    }

    fn cat_tempfiles(&mut self, indices: Vec<String>) {
        let mut combined = String::new();
        for idx_str in &indices {
            match self.resolve_idx(idx_str) {
                Some(idx) => {
                    let path = &self.state().temp_file_stack()[idx];
                    let content = util_file_contents_to_string(path.as_path());
                    combined.push_str(&content);
                }
                None => util_terminate_error(ERR_INVALID_IDX),
            }
        }
        if io::stdout().is_terminal() {
            cyber_print_content(&combined);
        } else {
            print!("{}", combined);
        }
        exit(0)
    }

    fn path_tempfile(&mut self, stk_idx: String) {
        match self.resolve_idx(&stk_idx) {
            Some(idx) => {
                println!("{}", self.state.temp_file_stack()[idx].display());
                exit(0)
            }
            None => util_terminate_error(ERR_INVALID_IDX),
        }
    }

    fn replace_in_tempfile(&mut self, stk_idx: String, pattern: String, replacement: String) {
        match self.resolve_idx(&stk_idx) {
            Some(idx) => {
                let path = self.state.temp_file_stack()[idx].clone();
                let content = util_file_contents_to_string(path.as_path());
                let replaced = content.replace(&pattern, &replacement);
                util_overwrite_file(&path, &replaced);
                println!("{}", replaced.matches(&replacement).count());
                exit(0)
            }
            None => util_terminate_error(ERR_INVALID_IDX),
        }
    }

    fn grep_tempfiles(&mut self, pattern: String) {
        let stk = self.state().temp_file_stack().clone();
        let names = self.state().temp_file_names().clone();
        let is_tty = io::stdout().is_terminal();
        let mut found = false;

        for (i, (p, n)) in stk.iter().zip(names.iter()).enumerate() {
            let content = util_file_contents_to_string(p.as_path());
            let matching_lines: Vec<(usize, &str)> = content
                .lines()
                .enumerate()
                .filter(|(_, line)| line.contains(&pattern))
                .collect();

            if matching_lines.is_empty() {
                continue;
            }
            found = true;

            if is_tty {
                cyber_hr();
                let tag = match n {
                    Some(name) => format!(" \x1b[33m@{}\x1b[0m", name),
                    None => String::new(),
                };
                println!(
                    "\x1b[33m [{:02}]\x1b[0m \x1b[36m>\x1b[0m \x1b[35m{}\x1b[0m{}",
                    i + 1,
                    p.display(),
                    tag
                );
                for (ln, line) in &matching_lines {
                    let highlighted =
                        line.replace(&pattern, &format!("\x1b[31;1m{}\x1b[0;32m", pattern));
                    println!(
                        "\x1b[33m  {}:\x1b[0m \x1b[32m{}\x1b[0m",
                        ln + 1,
                        highlighted
                    );
                }
            } else {
                let tag = match n {
                    Some(name) => format!(" @{}", name),
                    None => String::new(),
                };
                println!("{}:{}{}", i + 1, p.display(), tag);
                for (ln, line) in &matching_lines {
                    println!("  {}:{}", ln + 1, line);
                }
            }
        }

        if found && is_tty {
            cyber_hr();
        }
        exit(if found { 0 } else { 1 })
    }

    fn info_tempfile(&mut self, stk_idx: String) {
        match self.resolve_idx(&stk_idx) {
            Some(idx) => {
                let path = self.state().temp_file_stack()[idx].clone();
                let name = self.state().temp_file_names()[idx].clone();
                let meta = match fs::metadata(&path) {
                    Ok(m) => m,
                    Err(_) => {
                        util_terminate_error(ERR_FILE_READ);
                        unreachable!()
                    }
                };
                let size = meta.len();
                let modified = meta
                    .modified()
                    .ok()
                    .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                    .map(|d| d.as_secs())
                    .unwrap_or(0);

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
        if new.contains(MASTER_FIELD_DELIM) {
            util_terminate_error(ERR_NAME_NUL);
        }
        if self
            .state()
            .temp_file_names()
            .iter()
            .any(|n| n.as_deref() == Some(&new))
        {
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
                match ProcessCommand::new(&editor).arg(&path).status() {
                    Ok(s) => exit(s.code().unwrap_or(1)),
                    Err(_) => util_terminate_error(ERR_EDITOR),
                }
            }
            None => util_terminate_error(ERR_INVALID_IDX),
        }
    }

    fn remove_at_idx(&mut self, stk_idx: String) {
        match self.resolve_idx(&stk_idx) {
            Some(idx) => {
                util_remove_file(&self.state.temp_file_stack()[idx]);
                let mut paths = self.state.temp_file_stack().clone();
                let mut names = self.state.temp_file_names().clone();
                paths.remove(idx);
                names.remove(idx);
                util_paths_and_names_to_file(paths, &names, self.state.master_record_file());
                exit(0)
            }
            None => util_terminate_error(ERR_INVALID_IDX),
        }
    }
}
