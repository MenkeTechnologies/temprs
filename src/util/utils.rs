use std::fs::{read_to_string, remove_file, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use log::{debug, error};

use crate::util::consts::{ERR_NO_FILE, ERR_PARSE};

pub fn util_paths_from_file(path: &Path) -> Vec<PathBuf> {
    let file = File::open(path).expect(ERR_NO_FILE);
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| PathBuf::from(l.expect(ERR_PARSE)))
        .collect()
}

pub fn util_paths_to_file(paths: Vec<PathBuf>, out: &PathBuf) {
    let lines: Vec<String> = paths.iter().map(|p| util_path_as_string(p)).collect();
    if out.as_path().exists() {
        debug!("remove file '{}'", util_path_as_string(out));
        util_remove_file(out);
    }
    util_write_lines_to_file(out, lines)
}

pub fn util_write_lines_to_file(out: &PathBuf, lines: Vec<String>) {
    let mut buf: String = lines.join("\n");
    if !buf.is_empty() {
        buf.push_str("\n");
    }
    debug!("append lines: '{}'", buf);
    util_append_file(out, &buf);
}

pub fn util_lines_from_file(path: &Path) -> Vec<String> {
    let file = File::open(path).expect(ERR_NO_FILE);
    let buf = BufReader::new(file);
    buf.lines().map(|l| l.expect(ERR_PARSE)).collect()
}

pub fn util_remove_file(f: &PathBuf) {
    match remove_file(f.as_path()) {
        Ok(_success) => {
            debug!("removed file '{}'", util_path_as_string(f));
        }
        Err(error) => {
            error!("_____________'e' = '{}'_____________", error);
        }
    }
}

pub fn util_path_as_string(path: &PathBuf) -> String {
    path.clone().into_os_string().into_string().unwrap()
}

pub fn util_file_contents(filename: &Path) -> String {
    read_to_string(filename).unwrap()
}

pub fn util_append_file(path: &PathBuf, buffer: &String) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .unwrap();
    file.write(buffer.as_bytes());
}

pub fn util_overwrite_file(path: &PathBuf, buffer: &String) {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path)
        .unwrap();
    file.write(buffer.as_bytes());
}

pub fn util_write_file(path: &PathBuf, buffer: &String) {
    let mut file = OpenOptions::new().create(true).open(path).unwrap();
    file.write(buffer.as_bytes());
}

pub fn util_get_ms() -> String {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .to_string()
}
