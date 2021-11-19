use fs::write;
use io::stdin;
use std::env;
use std::fs;
use std::io;
use std::io::{Read, Result};
use std::path::{Path, PathBuf};

use atty::Stream;
use log::debug;
use log::Level;

const TEMP_LOG_LEVEL: Level = Level::Debug;

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
    pub fn new() -> Self {
        let mut system_temp_dir = env::temp_dir();
        system_temp_dir.push("temp-rs");

        let our_temp_dir = Path::new(system_temp_dir.as_path());

        let mut out_file = PathBuf::new();
        let mut file_list = PathBuf::new();

        out_file.push(system_temp_dir.as_path());
        file_list.push(system_temp_dir.as_path());

        out_file.push("tempfile");
        file_list.push("temp-rs-master");

        debug!("current temp dir {}", out_file.display());
        debug!("master temp file {}", file_list.display());

        let subcommand = String::new();

        let state = TempState {
            out_file,
            file_list,
            subcommand,
        };

        if !our_temp_dir.exists() {
            debug!("creating {}", our_temp_dir.display());
            fs::create_dir(our_temp_dir);
        }

        Self { state }
    }

    pub fn state(&self) -> &TempState {
        &self.state
    }

    pub fn stdin_terminal(&self) -> () {
        if atty::isnt(Stream::Stdout) {
            self.stdout_pipe();
        } else {
            self.stdout_terminal();
        }
    }

    pub fn stdout_terminal(&self) {
        debug!("no stdin pipe");
    }

    pub fn stdout_pipe(&self) {
        debug!("no stdin pipe");
        debug!("stdout pipe");
    }

    pub fn stdin_pipe(&self) -> () {
        debug!("stdin pipe");
        let mut buffer = String::new();
        stdin().read_to_string(&mut buffer);
        write(self.state().out_file(), &buffer).unwrap();

        self.append_temp_file_list();

        if atty::isnt(Stream::Stdout) {
            debug!("writing to stdout {}", buffer);
            self.stdout_pipe();
        } else {
            self.stdout_terminal();
        }
    }

    pub fn append_temp_file_list(&self) {
        debug!("writing to {}", self.state().out_file().display());

        let mut buffer = String::new();
        buffer.push_str(self.state().out_file_string().as_str());
        write(self.state().temp_list_file(), &buffer).unwrap();
    }
}

fn main() -> Result<()> {
    simple_logger::init_with_level(TEMP_LOG_LEVEL).unwrap();

    let app = TempApp::new();

    if atty::isnt(Stream::Stdin) {
        app.stdin_pipe()
    } else {
        app.stdin_terminal()
    }

    Ok(())
}
