#![allow(dead_code)]
#![allow(unused_must_use)]

use std::fs::{read_to_string, remove_file, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::iter::repeat;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::time::SystemTime;

use log::{debug, error};

use crate::util::consts::*;

#[inline(always)]
pub fn util_file_to_paths(path: &Path) -> Vec<PathBuf> {
    let file = File::open(path).expect(ERR_NO_FILE);
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| PathBuf::from(l.expect(ERR_PARSE)))
        .collect()
}

#[inline(always)]
pub fn util_paths_to_file(paths: Vec<PathBuf>, out: &PathBuf) {
    let lines: Vec<String> = paths.iter().map(|p| util_path_to_string(p)).collect();
    if out.as_path().exists() {
        debug!("remove file '{}'", util_path_to_string(out));
        util_remove_file(out);
    }
    util_lines_to_file(out, lines)
}

#[inline(always)]
pub fn util_terminate_error(msg: &str) {
    error!("{}", msg);
    exit(1)
}

#[inline(always)]
pub fn util_transform_idx(idx: i32, len: usize) -> usize {
    if idx < 0 {
        let bnd = idx + len as i32;
        if bnd < 0 {
            util_terminate_error(ERR_INVALID_IDX)
        }
        bnd as usize
    } else {
        let bnd = idx - 1;
        if bnd < 0 {
            util_terminate_error(ERR_INVALID_IDX)
        }
        if bnd >= len as i32 {
            util_terminate_error(ERR_INVALID_IDX)
        }
        bnd as usize
    }
}

#[inline(always)]
pub fn util_lines_to_file(out: &PathBuf, lines: Vec<String>) {
    let mut buf: String = lines.join("\n");
    if !buf.is_empty() {
        buf.push_str("\n");
    }
    debug!("append lines: '{}'", buf);
    util_append_file(out, &buf);
}

#[inline(always)]
pub fn util_file_to_lines(path: &Path) -> Vec<String> {
    let file = File::open(path).expect(ERR_NO_FILE);
    let buf = BufReader::new(file);
    buf.lines().map(|l| l.expect(ERR_PARSE)).collect()
}

#[inline(always)]
pub fn util_remove_file(f: &PathBuf) {
    match remove_file(f.as_path()) {
        Ok(_success) => {
            debug!("removed file '{}'", util_path_to_string(f));
        }
        Err(_error) => util_terminate_error(ERR_INVALID_RM),
    }
}

#[inline(always)]
pub fn util_path_to_string(path: &PathBuf) -> String {
    path.clone()
        .into_os_string()
        .into_string()
        .expect(ERR_NO_FILE)
}

#[inline(always)]
pub fn util_file_contents_to_string(filename: &Path) -> Option<String> {
    match read_to_string(filename) {
        Ok(str) => Some(str),
        Err(_error) => {
            util_terminate_error(ERR_INVALID_FILE);
            None
        }
    }
}

#[inline(always)]
pub fn util_append_file(path: &PathBuf, buffer: &String) {
    match OpenOptions::new().create(true).append(true).open(path) {
        Ok(mut file) => {
            file.write(buffer.as_bytes());
        }
        Err(_error) => util_terminate_error(ERR_INVALID_FILE),
    }
}

#[inline(always)]
pub fn util_horiz_rule() {
    println!("{}", repeat(HR_CHAR).take(80).collect::<String>())
}

#[inline(always)]
pub fn util_overwrite_file(path: &PathBuf, buffer: &String) {
    match OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)
    {
        Ok(mut file) => {
            file.write(buffer.as_bytes());
        }
        Err(_error) => util_terminate_error(ERR_INVALID_FILE),
    }
}

#[inline(always)]
pub fn util_write_file(path: &PathBuf, buffer: &String) {
    match OpenOptions::new().create(true).open(path) {
        Ok(mut file) => {
            file.write(buffer.as_bytes());
        }
        Err(_error) => util_terminate_error(ERR_INVALID_FILE),
    }
}

#[inline(always)]
pub fn util_time_ms() -> String {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect(ERR_CLOCK)
        .as_millis()
        .to_string()
}
