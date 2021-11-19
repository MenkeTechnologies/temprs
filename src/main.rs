use fs::write;
use io::stdin;
use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::{Read, Result, Write};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use atty::Stream;
use log::debug;

use util::consts::{FILE_LIST_FILE, TEMP_DIR, TEMP_LOG_LEVEL};
use util::consts::TEMPFILE_PREFIX;

mod util;

struct TempState {
    out_file: PathBuf,
    file_list: PathBuf,
    subcommand: String,
}

impl TempState {
    pub fn out_file(&self) -> &PathBuf {
        &self.out_file
    }
    pub fn out_file_string(&self) -> String {
        self.out_file
            .clone()
            .into_os_string()
            .into_string()
            .unwrap()
    }
    pub fn temp_list_file(&self) -> &PathBuf {
        &self.file_list
    }

    pub fn file_list_string(&self) -> String {
        self.file_list
            .clone()
            .into_os_string()
            .into_string()
            .unwrap()
    }
    pub fn subcommand(&self) -> &str {
        &self.subcommand
    }
}

struct TempApp {
    state: TempState,
}


impl TempApp {
    fn run(&self) {
        if atty::isnt(Stream::Stdin) {
            self.stdin_pipe()
        } else {
            self.stdin_terminal()
        }
        self.append_temp_file_list();
    }

    pub fn new() -> Self {
        simple_logger::init_with_level(TEMP_LOG_LEVEL).unwrap();

        let mut system_temp_dir = env::temp_dir();
        system_temp_dir.push(TEMP_DIR);

        let our_temp_dir = Path::new(system_temp_dir.as_path());

        let mut out_file = PathBuf::new();
        let mut file_list = PathBuf::new();

        out_file.push(system_temp_dir.as_path());
        file_list.push(system_temp_dir.as_path());

        let ms = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .to_string();
        out_file.push(format!("{}{}", TEMPFILE_PREFIX, ms));
        file_list.push(FILE_LIST_FILE);

        debug!("out file {}", out_file.display());
        debug!("file stack {}", file_list.display());

        let subcommand = String::new();

        let state = TempState {
            out_file,
            file_list,
            subcommand,
        };

        if !our_temp_dir.exists() {
            debug!("create temp dir {}", our_temp_dir.display());
            fs::create_dir(our_temp_dir);
        }

        Self { state }
    }

    pub fn state(&self) -> &TempState {
        &self.state
    }

    pub fn stdin_terminal(&self) {
        debug!("stdin term");
        if atty::isnt(Stream::Stdout) {
            self.stdout_pipe();
        } else {
            self.stdout_terminal();
        }
    }

    pub fn stdout_terminal(&self) {
        debug!("stdout term");
    }

    pub fn stdout_pipe(&self) {
        debug!("stdout pipe");
    }

    pub fn stdin_pipe(&self) {
        debug!("stdin pipe");
        let mut buffer = String::new();
        stdin().read_to_string(&mut buffer);
        write(self.state().out_file(), &buffer).unwrap();

        if atty::isnt(Stream::Stdout) {
            debug!("writing to stdout {}", buffer);
            self.stdout_pipe();
        } else {
            self.stdout_terminal();
        }
    }

    pub fn append_temp_file_list(&self) {
        debug!("append out file to file list {}", self.state().out_file().display());

        let mut buffer = String::new();
        buffer.push_str(self.state().out_file_string().as_str());
        buffer.push_str("\n");
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.state().temp_list_file())
            .unwrap();
        file.write(buffer.as_bytes());
    }
}

fn main() {

    let app = TempApp::new();

    app.run();
}
