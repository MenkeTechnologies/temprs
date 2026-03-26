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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);

    fn tmp_dir() -> PathBuf {
        let id = COUNTER.fetch_add(1, Ordering::SeqCst);
        let dir = std::env::temp_dir().join(format!(
            "temprs_test_{}_{}_{}",
            std::process::id(),
            id,
            util_time_ms()
        ));
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    // ── util_transform_idx ─────────────────────────────

    #[test]
    fn transform_idx_positive_first() {
        assert_eq!(util_transform_idx(1, 5), 0);
    }

    #[test]
    fn transform_idx_positive_last() {
        assert_eq!(util_transform_idx(5, 5), 4);
    }

    #[test]
    fn transform_idx_positive_mid() {
        assert_eq!(util_transform_idx(3, 5), 2);
    }

    #[test]
    fn transform_idx_negative_last() {
        assert_eq!(util_transform_idx(-1, 5), 4);
    }

    #[test]
    fn transform_idx_negative_first() {
        assert_eq!(util_transform_idx(-5, 5), 0);
    }

    #[test]
    fn transform_idx_negative_mid() {
        assert_eq!(util_transform_idx(-3, 5), 2);
    }

    #[test]
    fn transform_idx_single_element_positive() {
        assert_eq!(util_transform_idx(1, 1), 0);
    }

    #[test]
    fn transform_idx_single_element_negative() {
        assert_eq!(util_transform_idx(-1, 1), 0);
    }

    // ── util_path_to_string ────────────────────────────

    #[test]
    fn path_to_string_simple() {
        let p = PathBuf::from("/tmp/foo");
        assert_eq!(util_path_to_string(&p), "/tmp/foo");
    }

    #[test]
    fn path_to_string_nested() {
        let p = PathBuf::from("/a/b/c/d.txt");
        assert_eq!(util_path_to_string(&p), "/a/b/c/d.txt");
    }

    // ── util_time_ms ──────────────────────────────────

    #[test]
    fn time_ms_is_numeric() {
        let t = util_time_ms();
        assert!(t.parse::<u128>().is_ok());
    }

    #[test]
    fn time_ms_is_recent() {
        let t: u128 = util_time_ms().parse().unwrap();
        // should be after 2024-01-01 in ms
        assert!(t > 1_704_067_200_000);
    }

    // ── util_horiz_rule ────────────────────────────────

    #[test]
    fn horiz_rule_length() {
        let rule: String = std::iter::repeat(HR_CHAR).take(80).collect();
        assert_eq!(rule.len(), 80);
        assert!(rule.chars().all(|c| c == '-'));
    }

    // ── file I/O round-trip tests ──────────────────────

    #[test]
    fn overwrite_file_creates_and_writes() {
        let dir = tmp_dir();
        let file = dir.join("test.txt");
        let content = String::from("hello world");
        util_overwrite_file(&file, &content);
        assert_eq!(fs::read_to_string(&file).unwrap(), "hello world");
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn overwrite_file_truncates_existing() {
        let dir = tmp_dir();
        let file = dir.join("test.txt");
        util_overwrite_file(&file, &String::from("first"));
        util_overwrite_file(&file, &String::from("second"));
        assert_eq!(fs::read_to_string(&file).unwrap(), "second");
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn append_file_appends() {
        let dir = tmp_dir();
        let file = dir.join("test.txt");
        util_append_file(&file, &String::from("aaa"));
        util_append_file(&file, &String::from("bbb"));
        assert_eq!(fs::read_to_string(&file).unwrap(), "aaabbb");
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn file_contents_to_string_reads() {
        let dir = tmp_dir();
        let file = dir.join("test.txt");
        fs::write(&file, "content here").unwrap();
        let result = util_file_contents_to_string(file.as_path());
        assert_eq!(result.unwrap(), "content here");
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn file_to_lines_reads_lines() {
        let dir = tmp_dir();
        let file = dir.join("test.txt");
        fs::write(&file, "line1\nline2\nline3").unwrap();
        let lines = util_file_to_lines(file.as_path());
        assert_eq!(lines, vec!["line1", "line2", "line3"]);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn lines_to_file_writes_lines() {
        let dir = tmp_dir();
        let file = dir.join("test.txt");
        let lines = vec!["alpha".to_string(), "beta".to_string()];
        util_lines_to_file(&file, lines);
        assert_eq!(fs::read_to_string(&file).unwrap(), "alpha\nbeta\n");
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn lines_to_file_empty_vec() {
        let dir = tmp_dir();
        let file = dir.join("test.txt");
        util_lines_to_file(&file, vec![]);
        // empty vec writes nothing, file may not exist or be empty
        let content = fs::read_to_string(&file).unwrap_or_default();
        assert!(content.is_empty());
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn file_to_paths_round_trip() {
        let dir = tmp_dir();
        let master = dir.join("master");
        let paths = vec![
            PathBuf::from("/tmp/a"),
            PathBuf::from("/tmp/b"),
            PathBuf::from("/tmp/c"),
        ];
        util_paths_to_file(paths.clone(), &master);
        let loaded = util_file_to_paths(master.as_path());
        assert_eq!(loaded, paths);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn paths_to_file_overwrites_existing() {
        let dir = tmp_dir();
        let master = dir.join("master");
        let paths1 = vec![PathBuf::from("/tmp/old")];
        let paths2 = vec![PathBuf::from("/tmp/new1"), PathBuf::from("/tmp/new2")];
        util_paths_to_file(paths1, &master);
        util_paths_to_file(paths2.clone(), &master);
        let loaded = util_file_to_paths(master.as_path());
        assert_eq!(loaded, paths2);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn remove_file_removes() {
        let dir = tmp_dir();
        let file = dir.join("test.txt");
        fs::write(&file, "data").unwrap();
        assert!(file.exists());
        util_remove_file(&file);
        assert!(!file.exists());
        fs::remove_dir_all(&dir).unwrap();
    }
}
