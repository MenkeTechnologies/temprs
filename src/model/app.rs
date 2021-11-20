use fs::create_dir;
use io::stdin;
use std::env;

use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::exit;

use atty::Stream;
use log::{debug, error};

use model::opts::parse_opts;

use crate::model;
use crate::model::state::TempState;
use crate::util::consts::TEMPFILE_PREFIX;
use crate::util::consts::{
    ERR_INVALID_INFILE, ERR_INVALID_OUTFILE, ERR_INVALID_RM, FILE_LIST_FILE, TEMP_DIR,
    TEMP_LOG_LEVEL,
};
use crate::util::utils::{
    util_append_file, util_overwrite_file, util_path_as_string, util_paths_from_file,
    util_paths_to_file, util_remove_file,
};
use crate::util::utils::{util_file_contents, util_get_ms};

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
        if atty::isnt(Stream::Stdin) {
            self.if_stdin_pipe()
        } else {
            self.if_stdin_terminal();
        }
    }

    pub fn new() -> Self {
        simple_logger::init_with_level(TEMP_LOG_LEVEL).unwrap();

        let mut system_temp_dir = env::temp_dir();
        system_temp_dir.push(TEMP_DIR);

        let our_temp_dir = Path::new(system_temp_dir.as_path());

        let mut out_file = PathBuf::new();
        let mut master_file = PathBuf::new();

        out_file.push(system_temp_dir.as_path());
        master_file.push(system_temp_dir.as_path());

        out_file.push(format!("{}{}", TEMPFILE_PREFIX, util_get_ms()));
        master_file.push(FILE_LIST_FILE);

        let _subcommand = String::new();

        if !our_temp_dir.exists() {
            match create_dir(our_temp_dir) {
                Ok(_success) => {
                    debug!("create temp dir {}", our_temp_dir.display());
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
            let paths = util_paths_from_file(master_file.as_path());
            let exist: Vec<PathBuf> = paths.into_iter().filter(|p| p.exists()).collect();
            debug!("exists size {}", exist.len());
            util_paths_to_file(exist, &master_file);
        }

        debug!("out file {}", out_file.display());
        debug!("file stack {}", master_file.display());

        let temp_file_stack = util_paths_from_file(&master_file);
        debug!("found '{}' temp files on stack", temp_file_stack.len());

        let state = TempState::new(out_file, master_file, temp_file_stack, None, String::new());

        Self { state }
    }

    pub fn state(&mut self) -> &mut TempState {
        &mut self.state
    }

    fn if_stdin_terminal(&mut self) {
        debug!("stdin term");

        match self.state().arg_file() {
            Some(arg_file) => {
                let str = util_file_contents(arg_file.as_path());
                self.state().set_buffer(str.clone());

                match self.state().input_temp_file().clone() {
                    Some(stk_idx) => match self.stack_file_from_idx(stk_idx.clone()) {
                        Some(f) => {
                            util_overwrite_file(f, &str);
                        }
                        None => {
                            error!("{} at idx: {}", ERR_INVALID_INFILE, stk_idx);
                            exit(1)
                        }
                    },
                    None => {
                        self.append_temp_file_list();
                        util_append_file(self.state().new_temp_file(), &str);
                    }
                }
            }
            None => {
                let _buffer = String::new();
                match self.state().temp_file_stack().last() {
                    Some(f) => {
                        let string = util_file_contents(f.as_path());

                        self.state().set_buffer(string);
                    }
                    _ => {}
                }
            }
        }
    }

    fn output(&mut self) {
        if atty::isnt(Stream::Stdout) {
            self.if_stdout_pipe();
        } else {
            self.if_stdout_terminal();
        }
    }

    fn if_stdout_terminal(&mut self) {
        debug!("stdout term");
        self.print_buffer_or_stack_file();
    }

    fn if_stdout_pipe(&mut self) {
        debug!("stdout pipe");
        self.print_buffer_or_stack_file();
    }

    fn stack_file_from_idx(&mut self, f: String) -> Option<&PathBuf> {
        match f.parse::<usize>() {
            Ok(idx) => {
                if idx < 1 {
                    return None;
                }
                self.state().temp_file_stack().get(idx - 1)
            }
            Err(error) => {
                error!("{}", error);
                None
            }
        }
    }

    fn print_buffer_or_stack_file(&mut self) {
        match self.state().output_temp_file().clone() {
            Some(stk_idx) => match self.stack_file_from_idx(stk_idx.clone()) {
                Some(f) => {
                    print!("{}", util_file_contents(f.as_path()));
                }
                None => {
                    error!("{} at idx: {}", ERR_INVALID_OUTFILE, stk_idx);
                    exit(1)
                }
            },
            None => {
                if !self.state().silent() {
                    print!("{}", self.state().buffer());
                }
            }
        }
    }

    fn if_stdin_pipe(&mut self) {
        debug!("stdin pipe");
        let mut str = String::new();
        stdin().read_to_string(&mut str);

        self.state().set_buffer(str.clone());

        match self.state().input_temp_file().clone() {
            Some(stk_idx) => match self.stack_file_from_idx(stk_idx.clone()) {
                Some(f) => {
                    util_overwrite_file(f, &str);
                }
                None => {
                    error!("{} at idx: {}", ERR_INVALID_INFILE, stk_idx);
                    exit(1)
                }
            },
            None => {
                self.append_temp_file_list();
                util_append_file(self.state().new_temp_file(), &str);
            }
        }
    }

    fn append_temp_file_list(&mut self) {
        debug!(
            "append file {} to master",
            self.state().new_temp_file().display()
        );

        let mut buffer = String::new();
        buffer.push_str(self.state().out_file_path_str().as_str());
        buffer.push_str("\n");
        util_append_file(self.state().master_record_file(), &buffer);
    }
    fn parse_opts(&mut self) {
        let matches = parse_opts().get_matches();

        if matches.is_present("list_files") {
            self.list_files();
        }

        if matches.is_present("list_contents") {
            self.list_contents();
        }
        if matches.is_present("clear") {
            self.clear_all();
        }
        if matches.is_present("shift") {
            self.remove_at_idx(format!("{}", 1))
        }
        if matches.is_present("pop") {
            let top = self.state().temp_file_stack().len();
            self.remove_at_idx(format!("{}", top))
        }

        if matches.is_present("silent") {
            self.state().set_silent(true);
        }
        match matches.value_of("remove") {
            Some(f) => self.remove_at_idx(String::from(f)),
            None => {}
        }

        match matches.value_of("FILE") {
            Some(f) => self.state().set_arg_file(Some(PathBuf::from(f))),
            None => {}
        }
        match matches.value_of("input") {
            Some(f) => self.state().set_input_temp_file(Some(String::from(f))),
            None => {}
        }
        match matches.value_of("output") {
            Some(f) => self.state().set_output_temp_file(Some(String::from(f))),
            None => {}
        }
    }
    fn list_contents(&mut self) {
        debug!("list contents");
        for (i, p) in self.state().temp_file_stack().iter().enumerate() {
            println!("{}: {}", i + 1, util_path_as_string(p));
            println!("{}", util_file_contents(p.as_path()));
        }
        exit(0)
    }
    fn list_files(&mut self) {
        debug!("list files");
        for (i, p) in self.state().temp_file_stack().iter().enumerate() {
            println!("{}: {}", i + 1, util_path_as_string(p));
        }
        exit(0)
    }
    fn clear_all(&mut self) {
        fs::remove_dir_all(
            self.state()
                .master_record_file()
                .as_path()
                .parent()
                .unwrap(),
        );
        exit(0)
    }
    fn remove_at_idx(&mut self, stk_idx: String) {
        let cur = self.state().temp_file_stack().clone();
        match self.stack_file_from_idx(stk_idx.clone()) {
            Some(f) => {
                util_remove_file(f);
                let col: Vec<PathBuf> = cur.into_iter().filter(|p| p != f).collect();

                util_paths_to_file(col, self.state().master_record_file());
                exit(0)
            }
            None => {
                error!("{} at idx: {}", ERR_INVALID_RM, stk_idx);
                exit(1)
            }
        }
    }
}
