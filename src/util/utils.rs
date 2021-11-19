use std::fs::{File, OpenOptions, read_to_string};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

const ERR_NO_FILE: &'static str = "no such file";

const ERR_PARSE: &'static str = "Could not parse line";

pub fn paths_from_file(filename: &Path) -> Vec<PathBuf> {
    let file = File::open(filename).expect(ERR_NO_FILE);
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| PathBuf::from(l.expect(ERR_PARSE)))
        .collect()
}

pub fn lines_from_file(filename: &Path) -> Vec<String> {
    let file = File::open(filename).expect(ERR_NO_FILE);
    let buf = BufReader::new(file);
    buf.lines().map(|l| l.expect(ERR_PARSE)).collect()
}

pub fn string_from_file(filename: &Path) -> String {
    read_to_string(filename).unwrap()
}

pub fn append_file(x: &PathBuf, buffer: &String) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(x)
        .unwrap();
    file.write(buffer.as_bytes());
}

pub fn write_file(x: &PathBuf, buffer: &String) {
    let mut file = OpenOptions::new().create(true).open(x).unwrap();
    file.write(buffer.as_bytes());
}

pub fn get_ms() -> String {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .to_string()
}
