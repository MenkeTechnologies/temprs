use std::io;
use std::io::{Read, Write};
use atty::Stream;
use std::path::{Path, PathBuf};
use std::fs;
use std::env;
use std::fs::{File, OpenOptions};
use log::{info, debug, warn, trace, error};
use std::env::temp_dir;
use log::log_enabled;
use log::Level;

fn main() -> std::io::Result<()>{

    simple_logger::init().unwrap();

    let mut system_temp_dir = env::temp_dir();
    system_temp_dir.push("temp-rs");

    let our_temp_dir = Path::new(system_temp_dir.as_path());
    let mut current_temp_dir = PathBuf::new();
    let mut master_temp_file = PathBuf::new();

    current_temp_dir.push(system_temp_dir.as_path());
    master_temp_file.push(system_temp_dir.as_path());

    current_temp_dir.push("tempfile");
    master_temp_file.push("temp-rs-master");

    debug!("current temp dir {}", current_temp_dir.display());
    debug!("master temp file {}", master_temp_file.display());


    if !our_temp_dir.exists() {
        debug!("creating {}", our_temp_dir.display());
        fs::create_dir(our_temp_dir);
    }

    if atty::isnt(Stream::Stdin) {
        stdinPipe(current_temp_dir, master_temp_file)
    } else {
        stdinTerminal()
    }

    Ok(())

}

fn stdinTerminal() -> () {

    if atty::isnt(Stream::Stdout) {
        stdoutPipe();
    } else {
        stdoutTerminal();
    }
}

fn stdoutTerminal() {
    debug!("no stdin pipe");
}

fn stdoutPipe() {
    debug!("no stdin pipe");
    debug!("stdout pipe");
}

fn stdinPipe(mut current_temp_dir: PathBuf, mut master_temp_file: PathBuf) -> () {
    debug!("stdin pipe");
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer);
    let temp_file_path = Path::new(current_temp_dir.as_path());
    append_temp_file_list(master_temp_file, temp_file_path);
    fs::write(temp_file_path, &buffer)?;

    //pipe stdout

    if atty::isnt(Stream::Stdout) {
        debug!("writing to stdout {}", buffer);
        stdoutPipe();
    } else {
        stdoutTerminal();
    }
}

fn append_temp_file_list(mut master_temp_file: PathBuf, temp_file_path: &Path) {
    debug!("writing to {}", temp_file_path.display());

//    writeln!(file, "{}", temp_file_path.display());

}
