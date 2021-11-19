use fs::create_dir;
use io::stdin;
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::{Path, PathBuf};

use atty::Stream;
use log::debug;

use util::consts::{FILE_LIST_FILE, TEMP_DIR, TEMP_LOG_LEVEL};
use util::consts::TEMPFILE_PREFIX;
use util::utils::{append_file, paths_from_file};
use util::utils::{get_ms, string_from_file};

mod util;

struct TempState {
    out_file: PathBuf,
    master_file: PathBuf,
    temp_file_stack: Vec<PathBuf>,
    subcommand: String,
    buffer: String,
}

impl TempState {
    pub fn set_out_file(&mut self, out_file: PathBuf) {
        self.out_file = out_file;
    }
    pub fn set_master_file(&mut self, master_file: PathBuf) {
        self.master_file = master_file;
    }
    pub fn set_temp_file_stack(&mut self, temp_file_stack: Vec<PathBuf>) {
        self.temp_file_stack = temp_file_stack;
    }
    pub fn set_subcommand(&mut self, subcommand: String) {
        self.subcommand = subcommand;
    }
    pub fn set_buffer(&mut self, buffer: String) {
        self.buffer = buffer;
    }
}

impl TempState {
    pub fn out_file(&self) -> &PathBuf {
        &self.out_file
    }
    pub fn master_file(&self) -> &PathBuf {
        &self.master_file
    }
    pub fn temp_file_stack(&self) -> &Vec<PathBuf> {
        &self.temp_file_stack
    }
    pub fn subcommand(&self) -> &str {
        &self.subcommand
    }
    pub fn buffer(&self) -> &str {
        &self.buffer
    }
}

impl TempState {
    pub fn out_file_string(&self) -> String {
        self.out_file
            .clone()
            .into_os_string()
            .into_string()
            .unwrap()
    }
    pub fn temp_list_file(&self) -> &PathBuf {
        &self.master_file
    }

    pub fn file_list_string(&self) -> String {
        self.master_file
            .clone()
            .into_os_string()
            .into_string()
            .unwrap()
    }
}

struct TempApp {
    state: TempState,
}

impl TempApp {
    fn run(&mut self) {
        self.parse_opts();
        self.input();
        self.ouput()
    }

    fn input(&mut self) {
        if atty::isnt(Stream::Stdin) {
            self.stdin_pipe()
        } else {
            self.stdin_terminal()
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

        let state = TempState {
            out_file,
            master_file,
            temp_file_stack,
            subcommand,
            buffer: String::new(),
        };

        Self { state }
    }

    pub fn state(&mut self) -> &mut TempState {
        &mut self.state
    }

    pub fn stdin_terminal(&mut self) {
        debug!("stdin term");

        let _buffer = String::new();
        match self.state().temp_file_stack.last() {
            Some(f) => {
                let string = string_from_file(f.as_path());
                self.state().set_buffer(string);
            }
            _ => {}
        }
    }

    fn ouput(&mut self) {
        if atty::isnt(Stream::Stdout) {
            self.stdout_pipe();
        } else {
            self.stdout_terminal();
        }
    }

    pub fn stdout_terminal(&mut self) {
        print!("{}", self.state().buffer());
        debug!("stdout term");
    }

    pub fn stdout_pipe(&mut self) {
        print!("{}", self.state().buffer());
        debug!("stdout pipe");
    }

    pub fn stdin_pipe(&mut self) {
        debug!("stdin pipe");
        self.append_temp_file_list();
        let mut buffer = String::new();
        stdin().read_to_string(&mut buffer);
        append_file(self.state().out_file(), &buffer);
        self.state().set_buffer(buffer.clone());
    }

    pub fn append_temp_file_list(&mut self) {
        debug!(
            "append out file to file list {}",
            self.state().out_file().display()
        );

        let mut buffer = String::new();
        buffer.push_str(self.state().out_file_string().as_str());
        buffer.push_str("\n");
        append_file(self.state().temp_list_file(), &buffer);
    }
    fn parse_opts(&self) {}
}

fn main() {
    let mut app = TempApp::new();

    app.run();
}
