use std::fs::{File, OpenOptions, read_to_string};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use crate::util::consts::{ERR_NO_FILE, ERR_PARSE};

pub fn paths_from_file(path: &Path) -> Vec<PathBuf> {
    let file = File::open(path).expect(ERR_NO_FILE);
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| PathBuf::from(l.expect(ERR_PARSE)))
        .collect()
}

pub fn lines_from_file(path: &Path) -> Vec<String> {
    let file = File::open(path).expect(ERR_NO_FILE);
    let buf = BufReader::new(file);
    buf.lines().map(|l| l.expect(ERR_PARSE)).collect()
}

pub fn path_as_string(path: &PathBuf) -> String {
    path
        .clone()
        .into_os_string()
        .into_string()
        .unwrap()
}

pub fn file_contents(filename: &Path) -> String {
    read_to_string(filename).unwrap()
}

pub fn append_file(path: &PathBuf, buffer: &String) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .unwrap();
    file.write(buffer.as_bytes());
}

pub fn write_file(path: &PathBuf, buffer: &String) {
    let mut file = OpenOptions::new().create(true).open(path).unwrap();
    file.write(buffer.as_bytes());
}

pub fn get_ms() -> String {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .to_string()
}
