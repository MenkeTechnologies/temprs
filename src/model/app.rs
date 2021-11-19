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
use clap::ArgMatches;
use log::{debug, error};

use model::opts::parse_opts;

use crate::model;
use crate::model::state::TempState;
use crate::util::consts::{FILE_LIST_FILE, TEMP_DIR, TEMP_LOG_LEVEL};
use crate::util::consts::TEMPFILE_PREFIX;
use crate::util::utils::{append_file, path_as_string, paths_from_file};
use crate::util::utils::{file_contents, get_ms};

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

        out_file.push(format!("{}{}", TEMPFILE_PREFIX, get_ms()));
        master_file.push(FILE_LIST_FILE);

        let subcommand = String::new();

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
        }

        debug!("out file {}", out_file.display());
        debug!("file stack {}", master_file.display());

        let temp_file_stack = paths_from_file(&master_file);
        debug!("found '{}' temp files on stack", temp_file_stack.len());


        let state = TempState::new(
            out_file,
            master_file,
            temp_file_stack,
            None,
            String::new());

        Self { state }
    }

    pub fn state(&mut self) -> &mut TempState {
        &mut self.state
    }

    fn if_stdin_terminal(&mut self) {
        debug!("stdin term");

        match self.state().arg_file() {
            Some(arg_file) => {
                let str = file_contents(arg_file.as_path());

                self.state().set_buffer(str.clone());

                self.append_temp_file_list();
                append_file(self.state().new_temp_file(), &str);
            }
            None => {
                let _buffer = String::new();
                match self.state().temp_file_stack().last() {
                    Some(f) => {
                        let string = file_contents(f.as_path());

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

    fn stack_file_from_idx(&mut self, f: &String) -> Option<&PathBuf> {
        let idx = f.parse::<usize>().unwrap();
        self.state().temp_file_stack().get(idx)
    }

    fn print_buffer_or_stack_file(&mut self) {
        match self.state().output_temp_file() {
            Some(path) => {
                // self.stack_file_from_idx(path);
                print!("{}", self.state().buffer());
            }
            None => {
                print!("{}", self.state().buffer());
            }
        }
    }

    fn if_stdin_pipe(&mut self) {
        debug!("stdin pipe");
        self.append_temp_file_list();
        let mut buffer = String::new();
        stdin().read_to_string(&mut buffer);
        append_file(self.state().new_temp_file(), &buffer);

        self.state().set_buffer(buffer.clone());
    }

    fn append_temp_file_list(&mut self) {
        debug!(
            "append out file to file list {}",
            self.state().new_temp_file().display()
        );

        let mut buffer = String::new();
        buffer.push_str(self.state().out_file_path_str().as_str());
        buffer.push_str("\n");
        append_file(self.state().master_record_file(), &buffer);
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

        match matches.value_of("FILE") {
            Some(f) => { self.state().set_arg_file(Some(PathBuf::from(f))) }
            None => {}
        }
        match matches.value_of("input") {
            Some(f) => { self.state().set_input_temp_file(Some(String::from(f))) }
            None => {}
        }
        match matches.value_of("output") {
            Some(f) => { self.state().set_output_temp_file(Some(String::from(f))) }
            None => {}
        }
    }
    fn list_contents(&mut self) {
        debug!("list contents");
        for (i, p) in self.state().temp_file_stack().iter().enumerate() {
            println!("{}: {}", i + 1, path_as_string(p));
            println!("{}", file_contents(p.as_path()));
        }
        exit(0)
    }
    fn list_files(&mut self) {
        debug!("list files");
        for (i, p) in self.state().temp_file_stack().iter().enumerate() {
            println!("{}: {}", i + 1, path_as_string(p));
        }
        exit(0)
    }
    fn clear_all(&mut self) {
        let mut to_delete = self.state().temp_file_stack().clone();
        to_delete.push(self.state().master_record_file().clone());
        for (i, p) in to_delete.iter().enumerate() {
            match fs::remove_file(p.as_path()) {
                Ok(success) => {
                    debug!("remove file {} at path {}", i, path_as_string(p))
                }
                Err(error) => {
                    error!("_____________'e' = '{}'_____________", error);
                }
            }
        }
        exit(0)
    }
}