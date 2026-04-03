#[cfg(test)]
use std::fs::File;
use std::fs::{OpenOptions, read_to_string, remove_file, rename};
use std::io::{self, IsTerminal, Write};
#[cfg(test)]
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::exit;
use std::time::SystemTime;

use log::{debug, error, warn};

use crate::util::consts::*;

// ── Cyberpunk terminal output helpers ────────────────

fn is_tty() -> bool {
    use std::cell::Cell;
    thread_local! {
        static CACHED: Cell<Option<bool>> = const { Cell::new(None) };
    }
    CACHED.with(|c| match c.get() {
        Some(v) => v,
        None => {
            let v = io::stdout().is_terminal();
            c.set(Some(v));
            v
        }
    })
}

pub fn cyber_hr() {
    if is_tty() {
        println!("\x1b[36m ░▒▓{}▓▒░\x1b[0m", "█".repeat(50));
    } else {
        println!("{}", HR_CHAR.repeat(80));
    }
}

pub fn cyber_path(path: &Path) {
    let p = util_path_to_string(path);
    if is_tty() {
        println!("\x1b[32m //\x1b[0m \x1b[35m{}\x1b[0m", p);
    } else {
        println!("{}", p);
    }
}

pub fn cyber_idx_path(i: usize, path: &Path) {
    cyber_idx_path_named(i, path, &None);
}

pub fn cyber_idx_path_named(i: usize, path: &Path, name: &Option<String>) {
    let p = util_path_to_string(path);
    let tag = match name {
        Some(n) => format!(" \x1b[33m@{}\x1b[0m", n),
        None => String::new(),
    };
    let tag_plain = match name {
        Some(n) => format!(" @{}", n),
        None => String::new(),
    };
    if is_tty() {
        println!(
            "\x1b[33m [{:02}]\x1b[0m \x1b[36m>\x1b[0m \x1b[35m{}\x1b[0m{}",
            i, p, tag
        );
    } else {
        println!("{}: {}{}", i, p, tag_plain);
    }
}

pub fn cyber_content(text: &str) {
    if is_tty() {
        println!("\x1b[32m{}\x1b[0m", text.trim_end());
    } else {
        println!("{}", text.trim_end());
    }
}

pub fn cyber_idx_content(i: usize, path: &Path, text: &str) {
    cyber_idx_content_named(i, path, text, &None);
}

pub fn cyber_idx_content_named(i: usize, path: &Path, text: &str, name: &Option<String>) {
    let p = util_path_to_string(path);
    let tag = match name {
        Some(n) => format!(" \x1b[33m@{}\x1b[0m", n),
        None => String::new(),
    };
    let tag_plain = match name {
        Some(n) => format!(" @{}", n),
        None => String::new(),
    };
    if is_tty() {
        println!(
            "\x1b[33m [{:02}]\x1b[0m \x1b[36m>\x1b[0m \x1b[35m{}\x1b[0m{}",
            i, p, tag
        );
        println!("\x1b[32m{}\x1b[0m", text.trim_end());
    } else {
        println!("{}: {}{}", i, p, tag_plain);
        println!("{}", text.trim_end());
    }
}

pub fn cyber_print_content(text: &str) {
    if is_tty() {
        println!("\x1b[36m ┌──────────────────────────────────────────────────────┐\x1b[0m");
        for line in text.trim_end().lines() {
            println!("\x1b[36m │\x1b[0m \x1b[32m{}\x1b[0m", line);
        }
        println!("\x1b[36m └──────────────────────────────────────────────────────┘\x1b[0m");
    } else {
        print!("{}", text);
    }
}

pub fn cyber_single_path(path: &Path) {
    let p = util_path_to_string(path);
    if is_tty() {
        println!("\x1b[36m >\x1b[0m \x1b[35m{}\x1b[0m", p);
    } else {
        println!("{}", p);
    }
}

pub fn util_file_to_paths(path: &Path) -> Vec<PathBuf> {
    let (paths, _names) = util_file_to_paths_and_names(path);
    paths
}

pub fn util_file_to_paths_and_names(path: &Path) -> (Vec<PathBuf>, Vec<Option<String>>) {
    let data = match read_to_string(path) {
        Ok(s) => s,
        Err(_) => {
            util_terminate_error(ERR_NO_FILE);
            unreachable!()
        }
    };
    let mut paths = Vec::new();
    let mut names = Vec::new();
    if data.is_empty() {
        return (paths, names);
    }
    for (rec_num, record) in data.split(MASTER_RECORD_DELIM).enumerate() {
        if record.trim().is_empty() {
            continue;
        }
        if let Some((p, n)) = record.split_once(MASTER_FIELD_DELIM) {
            if p.is_empty() {
                warn!(
                    "skipping record {} with empty path in master record",
                    rec_num + 1
                );
                continue;
            }
            paths.push(PathBuf::from(p));
            names.push(if n.is_empty() {
                None
            } else {
                Some(n.to_string())
            });
        } else {
            paths.push(PathBuf::from(record));
            names.push(None);
        }
    }
    (paths, names)
}

pub fn util_paths_to_file(paths: &[PathBuf], out: &Path) {
    let names: Vec<Option<String>> = vec![None; paths.len()];
    util_paths_and_names_to_file(paths, &names, out);
}

pub fn util_paths_and_names_to_file(paths: &[PathBuf], names: &[Option<String>], out: &Path) {
    let records: Vec<String> = paths
        .iter()
        .zip(names.iter())
        .map(|(p, n)| {
            let ps = util_path_to_string(p);
            match n {
                Some(name) => format!("{}{}{}", ps, MASTER_FIELD_DELIM, name),
                None => ps,
            }
        })
        .collect();
    let buf = if records.is_empty() {
        String::new()
    } else {
        let mut s = records.join(MASTER_RECORD_DELIM);
        s.push_str(MASTER_RECORD_DELIM);
        s
    };
    let tmp = out.with_extension("tmp");
    util_overwrite_file(&tmp, &buf);
    if rename(&tmp, out).is_err() {
        util_terminate_error(ERR_MASTER_WRITE);
    }
    debug!("atomic write master record '{}'", util_path_to_string(out));
}

pub fn util_terminate_error(msg: &str) {
    error!("{}", msg);
    exit(1)
}

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

pub fn util_lines_to_file(out: &Path, lines: Vec<String>) {
    let mut buf: String = lines.join("\n");
    if !buf.is_empty() {
        buf.push('\n');
    }
    debug!("append lines: '{}'", buf);
    util_append_file(out, &buf);
}

#[cfg(test)]
pub fn util_file_to_lines(path: &Path) -> Vec<String> {
    let file = File::open(path).expect(ERR_NO_FILE);
    let buf = BufReader::new(file);
    buf.lines().map(|l| l.expect(ERR_PARSE)).collect()
}

pub fn util_remove_file(f: &Path) {
    match remove_file(f) {
        Ok(_success) => {
            debug!("removed file '{}'", util_path_to_string(f));
        }
        Err(_error) => util_terminate_error(ERR_INVALID_RM),
    }
}

pub fn util_path_to_string(path: &Path) -> String {
    match path.as_os_str().to_str() {
        Some(s) => s.to_owned(),
        None => {
            util_terminate_error(ERR_NO_FILE);
            unreachable!()
        }
    }
}

pub fn util_file_contents_to_string(filename: &Path) -> String {
    match read_to_string(filename) {
        Ok(str) => str,
        Err(_error) => {
            util_terminate_error(ERR_INVALID_FILE);
            unreachable!()
        }
    }
}

pub fn util_append_file(path: &Path, buffer: &str) {
    match OpenOptions::new().create(true).append(true).open(path) {
        Ok(mut file) => {
            if let Err(_e) = file.write_all(buffer.as_bytes()) {
                util_terminate_error(ERR_INVALID_FILE);
            }
        }
        Err(_error) => util_terminate_error(ERR_INVALID_FILE),
    }
}

pub fn util_horiz_rule() {
    println!("{}", HR_CHAR.repeat(80))
}

pub fn util_overwrite_file(path: &Path, buffer: &str) {
    match OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)
    {
        Ok(mut file) => {
            if let Err(_e) = file.write_all(buffer.as_bytes()) {
                util_terminate_error(ERR_INVALID_FILE);
            }
        }
        Err(_error) => util_terminate_error(ERR_INVALID_FILE),
    }
}

pub fn util_time_ms() -> String {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(d) => d.as_millis().to_string(),
        Err(_) => {
            util_terminate_error(ERR_CLOCK);
            unreachable!()
        }
    }
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
        let rule: String = HR_CHAR.repeat(80);
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
        assert_eq!(result, "content here");
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
        util_paths_to_file(&paths, &master);
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
        util_paths_to_file(&paths1, &master);
        util_paths_to_file(&paths2, &master);
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

    // ── master file robustness ──────────────────────────

    #[test]
    fn file_to_paths_skips_empty_records() {
        let dir = tmp_dir();
        let master = dir.join("master");
        // extra \0\0 creates empty records between valid ones
        fs::write(&master, "/tmp/a\0\0\0\0/tmp/b\0\0\0\0/tmp/c\0\0").unwrap();
        let paths = util_file_to_paths(master.as_path());
        assert_eq!(
            paths,
            vec![
                PathBuf::from("/tmp/a"),
                PathBuf::from("/tmp/b"),
                PathBuf::from("/tmp/c"),
            ]
        );
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn file_to_paths_skips_whitespace_only_records() {
        let dir = tmp_dir();
        let master = dir.join("master");
        fs::write(&master, "/tmp/a\0\0\0\0/tmp/b\0\0\0\0/tmp/c\0\0").unwrap();
        let paths = util_file_to_paths(master.as_path());
        assert_eq!(
            paths,
            vec![
                PathBuf::from("/tmp/a"),
                PathBuf::from("/tmp/b"),
                PathBuf::from("/tmp/c"),
            ]
        );
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn file_to_paths_skips_empty_path_with_name() {
        let dir = tmp_dir();
        let master = dir.join("master");
        fs::write(&master, "/tmp/a\0foo\0\0\0bar\0\0/tmp/b\0\0").unwrap();
        let (paths, names) = util_file_to_paths_and_names(master.as_path());
        assert_eq!(
            paths,
            vec![PathBuf::from("/tmp/a"), PathBuf::from("/tmp/b")]
        );
        assert_eq!(names, vec![Some("foo".to_string()), None]);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn atomic_write_no_temp_file_left() {
        let dir = tmp_dir();
        let master = dir.join("master");
        let paths = vec![PathBuf::from("/tmp/a"), PathBuf::from("/tmp/b")];
        util_paths_to_file(&paths, &master);
        assert!(master.exists());
        assert!(!master.with_extension("tmp").exists());
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn atomic_write_preserves_content_on_overwrite() {
        let dir = tmp_dir();
        let master = dir.join("master");
        let paths1 = vec![PathBuf::from("/tmp/old")];
        util_paths_to_file(&paths1, &master);
        let paths2 = vec![PathBuf::from("/tmp/new1"), PathBuf::from("/tmp/new2")];
        util_paths_to_file(&paths2, &master);
        let loaded = util_file_to_paths(master.as_path());
        assert_eq!(loaded, paths2);
        assert!(!master.with_extension("tmp").exists());
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn file_to_paths_handles_newlines_in_paths() {
        let dir = tmp_dir();
        let master = dir.join("master");
        fs::write(&master, "/tmp/file\nwith\nnewlines\0\0/tmp/normal\0\0").unwrap();
        let paths = util_file_to_paths(master.as_path());
        assert_eq!(
            paths,
            vec![
                PathBuf::from("/tmp/file\nwith\nnewlines"),
                PathBuf::from("/tmp/normal"),
            ]
        );
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn paths_with_newlines_round_trip() {
        let dir = tmp_dir();
        let master = dir.join("master");
        let paths = vec![
            PathBuf::from("/tmp/line\none"),
            PathBuf::from("/tmp/line\ntwo\nthree"),
            PathBuf::from("/tmp/normal"),
        ];
        util_paths_to_file(&paths, &master);
        let loaded = util_file_to_paths(master.as_path());
        assert_eq!(loaded, paths);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn paths_with_newlines_and_names_round_trip() {
        let dir = tmp_dir();
        let master = dir.join("master");
        let paths = vec![
            PathBuf::from("/tmp/has\nnewline"),
            PathBuf::from("/tmp/normal"),
        ];
        let names = vec![Some("tagged".to_string()), None];
        util_paths_and_names_to_file(&paths, &names, &master);
        let (loaded_paths, loaded_names) = util_file_to_paths_and_names(master.as_path());
        assert_eq!(loaded_paths, paths);
        assert_eq!(loaded_names, names);
        fs::remove_dir_all(&dir).unwrap();
    }

    // ── util_transform_idx additional cases ──────────────

    #[test]
    fn transform_idx_positive_two_elements() {
        assert_eq!(util_transform_idx(1, 2), 0);
        assert_eq!(util_transform_idx(2, 2), 1);
    }

    #[test]
    fn transform_idx_negative_two_elements() {
        assert_eq!(util_transform_idx(-1, 2), 1);
        assert_eq!(util_transform_idx(-2, 2), 0);
    }

    #[test]
    fn transform_idx_large_list() {
        assert_eq!(util_transform_idx(1, 100), 0);
        assert_eq!(util_transform_idx(50, 100), 49);
        assert_eq!(util_transform_idx(100, 100), 99);
        assert_eq!(util_transform_idx(-1, 100), 99);
        assert_eq!(util_transform_idx(-50, 100), 50);
        assert_eq!(util_transform_idx(-100, 100), 0);
    }

    #[test]
    fn transform_idx_positive_and_negative_equivalent() {
        // idx 1 and idx -5 both map to 0 in a 5-element list
        assert_eq!(util_transform_idx(1, 5), util_transform_idx(-5, 5));
        // idx 5 and idx -1 both map to 4
        assert_eq!(util_transform_idx(5, 5), util_transform_idx(-1, 5));
        // idx 3 and idx -3 both map to 2
        assert_eq!(util_transform_idx(3, 5), util_transform_idx(-3, 5));
    }

    // ── util_path_to_string additional cases ─────────────

    #[test]
    fn path_to_string_root() {
        let p = PathBuf::from("/");
        assert_eq!(util_path_to_string(&p), "/");
    }

    #[test]
    fn path_to_string_with_spaces() {
        let p = PathBuf::from("/tmp/my dir/my file.txt");
        assert_eq!(util_path_to_string(&p), "/tmp/my dir/my file.txt");
    }

    #[test]
    fn path_to_string_with_dots() {
        let p = PathBuf::from("/tmp/../tmp/foo");
        assert_eq!(util_path_to_string(&p), "/tmp/../tmp/foo");
    }

    #[test]
    fn path_to_string_single_component() {
        let p = PathBuf::from("filename");
        assert_eq!(util_path_to_string(&p), "filename");
    }

    // ── util_time_ms additional cases ────────────────────

    #[test]
    fn time_ms_increases_monotonically() {
        let t1: u128 = util_time_ms().parse().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(2));
        let t2: u128 = util_time_ms().parse().unwrap();
        assert!(t2 >= t1);
    }

    #[test]
    fn time_ms_not_empty() {
        let t = util_time_ms();
        assert!(!t.is_empty());
        assert!(t.len() >= 13); // ms since epoch is at least 13 digits
    }

    // ── util_overwrite_file additional cases ─────────────

    #[test]
    fn overwrite_file_empty_string() {
        let dir = tmp_dir();
        let file = dir.join("test.txt");
        util_overwrite_file(&file, "");
        assert_eq!(fs::read_to_string(&file).unwrap(), "");
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn overwrite_file_multiline() {
        let dir = tmp_dir();
        let file = dir.join("test.txt");
        let content = String::from("line1\nline2\nline3\n");
        util_overwrite_file(&file, &content);
        assert_eq!(fs::read_to_string(&file).unwrap(), "line1\nline2\nline3\n");
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn overwrite_file_unicode() {
        let dir = tmp_dir();
        let file = dir.join("test.txt");
        let content = String::from("hello 世界 🚀 café");
        util_overwrite_file(&file, &content);
        assert_eq!(fs::read_to_string(&file).unwrap(), "hello 世界 🚀 café");
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn overwrite_file_large_content() {
        let dir = tmp_dir();
        let file = dir.join("test.txt");
        let content: String = "x".repeat(10_000);
        util_overwrite_file(&file, &content);
        assert_eq!(fs::read_to_string(&file).unwrap().len(), 10_000);
        fs::remove_dir_all(&dir).unwrap();
    }

    // ── util_append_file additional cases ────────────────

    #[test]
    fn append_file_creates_if_missing() {
        let dir = tmp_dir();
        let file = dir.join("new.txt");
        assert!(!file.exists());
        util_append_file(&file, &String::from("created"));
        assert_eq!(fs::read_to_string(&file).unwrap(), "created");
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn append_file_empty_string() {
        let dir = tmp_dir();
        let file = dir.join("test.txt");
        util_append_file(&file, &String::from("start"));
        util_append_file(&file, "");
        assert_eq!(fs::read_to_string(&file).unwrap(), "start");
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn append_file_multiple_times() {
        let dir = tmp_dir();
        let file = dir.join("test.txt");
        util_append_file(&file, &String::from("a"));
        util_append_file(&file, &String::from("b"));
        util_append_file(&file, &String::from("c"));
        util_append_file(&file, &String::from("d"));
        assert_eq!(fs::read_to_string(&file).unwrap(), "abcd");
        fs::remove_dir_all(&dir).unwrap();
    }

    // ── util_file_contents_to_string additional cases ────

    #[test]
    fn file_contents_to_string_empty_file() {
        let dir = tmp_dir();
        let file = dir.join("empty.txt");
        fs::write(&file, "").unwrap();
        let result = util_file_contents_to_string(file.as_path());
        assert_eq!(result, "");
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn file_contents_to_string_multiline() {
        let dir = tmp_dir();
        let file = dir.join("multi.txt");
        fs::write(&file, "a\nb\nc").unwrap();
        let result = util_file_contents_to_string(file.as_path());
        assert_eq!(result, "a\nb\nc");
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn file_contents_to_string_unicode() {
        let dir = tmp_dir();
        let file = dir.join("unicode.txt");
        fs::write(&file, "日本語テスト").unwrap();
        let result = util_file_contents_to_string(file.as_path());
        assert_eq!(result, "日本語テスト");
        fs::remove_dir_all(&dir).unwrap();
    }

    // ── util_file_to_lines additional cases ──────────────

    #[test]
    fn file_to_lines_single_line() {
        let dir = tmp_dir();
        let file = dir.join("test.txt");
        fs::write(&file, "only one line").unwrap();
        let lines = util_file_to_lines(file.as_path());
        assert_eq!(lines, vec!["only one line"]);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn file_to_lines_trailing_newline() {
        let dir = tmp_dir();
        let file = dir.join("test.txt");
        fs::write(&file, "a\nb\n").unwrap();
        let lines = util_file_to_lines(file.as_path());
        assert_eq!(lines, vec!["a", "b"]);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn file_to_lines_empty_lines() {
        let dir = tmp_dir();
        let file = dir.join("test.txt");
        fs::write(&file, "a\n\nb\n\nc").unwrap();
        let lines = util_file_to_lines(file.as_path());
        assert_eq!(lines, vec!["a", "", "b", "", "c"]);
        fs::remove_dir_all(&dir).unwrap();
    }

    // ── util_lines_to_file additional cases ──────────────

    #[test]
    fn lines_to_file_single_line() {
        let dir = tmp_dir();
        let file = dir.join("test.txt");
        util_lines_to_file(&file, vec!["single".to_string()]);
        assert_eq!(fs::read_to_string(&file).unwrap(), "single\n");
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn lines_to_file_many_lines() {
        let dir = tmp_dir();
        let file = dir.join("test.txt");
        let lines: Vec<String> = (0..10).map(|i| format!("line{}", i)).collect();
        util_lines_to_file(&file, lines);
        let content = fs::read_to_string(&file).unwrap();
        let read_lines: Vec<&str> = content.trim_end().split('\n').collect();
        assert_eq!(read_lines.len(), 10);
        assert_eq!(read_lines[0], "line0");
        assert_eq!(read_lines[9], "line9");
        fs::remove_dir_all(&dir).unwrap();
    }

    // ── util_file_to_paths additional cases ──────────────

    #[test]
    fn file_to_paths_single_path() {
        let dir = tmp_dir();
        let file = dir.join("paths.txt");
        fs::write(&file, "/tmp/only\0\0").unwrap();
        let paths = util_file_to_paths(file.as_path());
        assert_eq!(paths, vec![PathBuf::from("/tmp/only")]);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn file_to_paths_empty_file() {
        let dir = tmp_dir();
        let file = dir.join("paths.txt");
        fs::write(&file, "").unwrap();
        let paths = util_file_to_paths(file.as_path());
        assert!(paths.is_empty());
        fs::remove_dir_all(&dir).unwrap();
    }

    // ── util_paths_to_file additional cases ──────────────

    #[test]
    fn paths_to_file_empty_vec() {
        let dir = tmp_dir();
        let master = dir.join("master");
        util_paths_to_file(&[], &master);
        let content = fs::read_to_string(&master).unwrap_or_default();
        assert!(content.is_empty());
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn paths_to_file_single_path() {
        let dir = tmp_dir();
        let master = dir.join("master");
        let paths = vec![PathBuf::from("/tmp/single")];
        util_paths_to_file(&paths, &master);
        let loaded = util_file_to_paths(master.as_path());
        assert_eq!(loaded, paths);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn paths_to_file_many_paths() {
        let dir = tmp_dir();
        let master = dir.join("master");
        let paths: Vec<PathBuf> = (0..20)
            .map(|i| PathBuf::from(format!("/tmp/f{}", i)))
            .collect();
        util_paths_to_file(&paths, &master);
        let loaded = util_file_to_paths(master.as_path());
        assert_eq!(loaded, paths);
        fs::remove_dir_all(&dir).unwrap();
    }

    // ── util_overwrite + util_file_contents round trip ───

    #[test]
    fn overwrite_and_read_back() {
        let dir = tmp_dir();
        let file = dir.join("roundtrip.txt");
        let content = String::from("round trip data");
        util_overwrite_file(&file, &content);
        let result = util_file_contents_to_string(file.as_path());
        assert_eq!(result, "round trip data");
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn overwrite_then_append_then_read() {
        let dir = tmp_dir();
        let file = dir.join("combo.txt");
        util_overwrite_file(&file, &String::from("base"));
        util_append_file(&file, &String::from("+extra"));
        let result = util_file_contents_to_string(file.as_path());
        assert_eq!(result, "base+extra");
        fs::remove_dir_all(&dir).unwrap();
    }

    // ── horiz_rule additional ────────────────────────────

    #[test]
    fn horiz_rule_char_is_dash() {
        assert_eq!(HR_CHAR, "-");
    }

    // ── util_transform_idx boundary exhaustive ───────────

    #[test]
    fn transform_idx_three_elements_all_positive() {
        assert_eq!(util_transform_idx(1, 3), 0);
        assert_eq!(util_transform_idx(2, 3), 1);
        assert_eq!(util_transform_idx(3, 3), 2);
    }

    #[test]
    fn transform_idx_three_elements_all_negative() {
        assert_eq!(util_transform_idx(-1, 3), 2);
        assert_eq!(util_transform_idx(-2, 3), 1);
        assert_eq!(util_transform_idx(-3, 3), 0);
    }

    #[test]
    fn transform_idx_ten_elements_boundaries() {
        assert_eq!(util_transform_idx(1, 10), 0);
        assert_eq!(util_transform_idx(10, 10), 9);
        assert_eq!(util_transform_idx(-1, 10), 9);
        assert_eq!(util_transform_idx(-10, 10), 0);
    }

    #[test]
    fn transform_idx_all_equivalences_for_four() {
        // For a 4-element list, positive i and negative -(4-i+1) should match
        for i in 1..=4 {
            let neg = -(4 - i + 1);
            assert_eq!(
                util_transform_idx(i, 4),
                util_transform_idx(neg, 4),
                "idx {} and {} should be equivalent for len 4",
                i,
                neg
            );
        }
    }

    // ── util_path_to_string edge cases ───────────────────

    #[test]
    fn path_to_string_unicode_dir() {
        let p = PathBuf::from("/tmp/日本語/file.txt");
        assert_eq!(util_path_to_string(&p), "/tmp/日本語/file.txt");
    }

    #[test]
    fn path_to_string_emoji() {
        let p = PathBuf::from("/tmp/🚀/data");
        assert_eq!(util_path_to_string(&p), "/tmp/🚀/data");
    }

    #[test]
    fn path_to_string_empty() {
        let p = PathBuf::from("");
        assert_eq!(util_path_to_string(&p), "");
    }

    #[test]
    fn path_to_string_with_hyphens_and_underscores() {
        let p = PathBuf::from("/tmp/my-dir/my_file.txt");
        assert_eq!(util_path_to_string(&p), "/tmp/my-dir/my_file.txt");
    }

    #[test]
    fn path_to_string_deep_nesting() {
        let p = PathBuf::from("/a/b/c/d/e/f/g/h/i/j/k");
        assert_eq!(util_path_to_string(&p), "/a/b/c/d/e/f/g/h/i/j/k");
    }

    // ── util_time_ms concurrency / uniqueness ────────────

    #[test]
    fn time_ms_two_calls_close_together() {
        let t1 = util_time_ms();
        let t2 = util_time_ms();
        // both should parse, t2 >= t1
        let v1: u128 = t1.parse().unwrap();
        let v2: u128 = t2.parse().unwrap();
        assert!(v2 >= v1);
    }

    #[test]
    fn time_ms_no_leading_zeros() {
        let t = util_time_ms();
        assert!(!t.starts_with('0'), "timestamp should not start with 0");
    }

    // ── util_overwrite_file edge cases ───────────────────

    #[test]
    fn overwrite_file_with_newlines() {
        let dir = tmp_dir();
        let file = dir.join("nl.txt");
        let content = String::from("\n\n\n");
        util_overwrite_file(&file, &content);
        assert_eq!(fs::read_to_string(&file).unwrap(), "\n\n\n");
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn overwrite_file_with_tabs() {
        let dir = tmp_dir();
        let file = dir.join("tabs.txt");
        let content = String::from("col1\tcol2\tcol3");
        util_overwrite_file(&file, &content);
        assert_eq!(fs::read_to_string(&file).unwrap(), "col1\tcol2\tcol3");
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn overwrite_file_then_overwrite_shorter() {
        let dir = tmp_dir();
        let file = dir.join("shrink.txt");
        util_overwrite_file(&file, &String::from("long content here"));
        util_overwrite_file(&file, &String::from("short"));
        assert_eq!(fs::read_to_string(&file).unwrap(), "short");
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn overwrite_file_then_overwrite_longer() {
        let dir = tmp_dir();
        let file = dir.join("grow.txt");
        util_overwrite_file(&file, &String::from("short"));
        util_overwrite_file(&file, &String::from("much longer content here now"));
        assert_eq!(
            fs::read_to_string(&file).unwrap(),
            "much longer content here now"
        );
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn overwrite_file_multiple_cycles() {
        let dir = tmp_dir();
        let file = dir.join("cycle.txt");
        for i in 0..10 {
            util_overwrite_file(&file, &format!("iteration_{}", i));
        }
        assert_eq!(fs::read_to_string(&file).unwrap(), "iteration_9");
        fs::remove_dir_all(&dir).unwrap();
    }

    // ── util_append_file edge cases ──────────────────────

    #[test]
    fn append_file_with_newlines() {
        let dir = tmp_dir();
        let file = dir.join("nl.txt");
        util_append_file(&file, &String::from("line1\n"));
        util_append_file(&file, &String::from("line2\n"));
        assert_eq!(fs::read_to_string(&file).unwrap(), "line1\nline2\n");
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn append_file_unicode() {
        let dir = tmp_dir();
        let file = dir.join("uni.txt");
        util_append_file(&file, &String::from("hello "));
        util_append_file(&file, &String::from("世界"));
        assert_eq!(fs::read_to_string(&file).unwrap(), "hello 世界");
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn append_file_large() {
        let dir = tmp_dir();
        let file = dir.join("big.txt");
        let chunk = "x".repeat(1000);
        for _ in 0..10 {
            util_append_file(&file, &chunk);
        }
        assert_eq!(fs::read_to_string(&file).unwrap().len(), 10_000);
        fs::remove_dir_all(&dir).unwrap();
    }

    // ── util_file_contents_to_string edge cases ──────────

    #[test]
    fn file_contents_to_string_with_trailing_newline() {
        let dir = tmp_dir();
        let file = dir.join("trail.txt");
        fs::write(&file, "data\n").unwrap();
        let result = util_file_contents_to_string(file.as_path());
        assert_eq!(result, "data\n");
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn file_contents_to_string_with_special_chars() {
        let dir = tmp_dir();
        let file = dir.join("special.txt");
        let content = "tab\there\nnewline\r\nwindows\0null";
        fs::write(&file, content).unwrap();
        let result = util_file_contents_to_string(file.as_path());
        assert_eq!(result, content);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn file_contents_to_string_large_file() {
        let dir = tmp_dir();
        let file = dir.join("large.txt");
        let content = "y".repeat(50_000);
        fs::write(&file, &content).unwrap();
        let result = util_file_contents_to_string(file.as_path());
        assert_eq!(result.len(), 50_000);
        fs::remove_dir_all(&dir).unwrap();
    }

    // ── util_file_to_lines edge cases ────────────────────

    #[test]
    fn file_to_lines_unicode_lines() {
        let dir = tmp_dir();
        let file = dir.join("uni.txt");
        fs::write(&file, "hello\n世界\n🚀").unwrap();
        let lines = util_file_to_lines(file.as_path());
        assert_eq!(lines, vec!["hello", "世界", "🚀"]);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn file_to_lines_long_lines() {
        let dir = tmp_dir();
        let file = dir.join("long.txt");
        let long_line = "a".repeat(5000);
        fs::write(&file, &long_line).unwrap();
        let lines = util_file_to_lines(file.as_path());
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].len(), 5000);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn file_to_lines_many_lines() {
        let dir = tmp_dir();
        let file = dir.join("many.txt");
        let content: String = (0..100)
            .map(|i| format!("line{}", i))
            .collect::<Vec<_>>()
            .join("\n");
        fs::write(&file, &content).unwrap();
        let lines = util_file_to_lines(file.as_path());
        assert_eq!(lines.len(), 100);
        assert_eq!(lines[0], "line0");
        assert_eq!(lines[99], "line99");
        fs::remove_dir_all(&dir).unwrap();
    }

    // ── util_lines_to_file edge cases ────────────────────

    #[test]
    fn lines_to_file_unicode_lines() {
        let dir = tmp_dir();
        let file = dir.join("uni.txt");
        let lines = vec![
            "日本語".to_string(),
            "中文".to_string(),
            "한국어".to_string(),
        ];
        util_lines_to_file(&file, lines);
        let content = fs::read_to_string(&file).unwrap();
        assert_eq!(content, "日本語\n中文\n한국어\n");
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn lines_to_file_with_spaces() {
        let dir = tmp_dir();
        let file = dir.join("sp.txt");
        let lines = vec!["hello world".to_string(), "foo bar baz".to_string()];
        util_lines_to_file(&file, lines);
        let content = fs::read_to_string(&file).unwrap();
        assert_eq!(content, "hello world\nfoo bar baz\n");
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn lines_to_file_then_read_back() {
        let dir = tmp_dir();
        let file = dir.join("rt.txt");
        let original = vec!["alpha".to_string(), "beta".to_string(), "gamma".to_string()];
        util_lines_to_file(&file, original.clone());
        let read_back = util_file_to_lines(file.as_path());
        assert_eq!(read_back, original);
        fs::remove_dir_all(&dir).unwrap();
    }

    // ── util_file_to_paths edge cases ────────────────────

    #[test]
    fn file_to_paths_with_spaces_in_paths() {
        let dir = tmp_dir();
        let file = dir.join("paths.txt");
        fs::write(&file, "/tmp/my dir/file1\0\0/tmp/another dir/file2\0\0").unwrap();
        let paths = util_file_to_paths(file.as_path());
        assert_eq!(paths.len(), 2);
        assert_eq!(paths[0], PathBuf::from("/tmp/my dir/file1"));
        assert_eq!(paths[1], PathBuf::from("/tmp/another dir/file2"));
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn file_to_paths_many_paths() {
        let dir = tmp_dir();
        let file = dir.join("paths.txt");
        let content: String = (0..50).map(|i| format!("/tmp/f{}\0\0", i)).collect();
        fs::write(&file, &content).unwrap();
        let paths = util_file_to_paths(file.as_path());
        assert_eq!(paths.len(), 50);
        assert_eq!(paths[0], PathBuf::from("/tmp/f0"));
        assert_eq!(paths[49], PathBuf::from("/tmp/f49"));
        fs::remove_dir_all(&dir).unwrap();
    }

    // ── util_paths_to_file edge cases ────────────────────

    #[test]
    fn paths_to_file_with_unicode_paths() {
        let dir = tmp_dir();
        let master = dir.join("master");
        let paths = vec![
            PathBuf::from("/tmp/日本語/file"),
            PathBuf::from("/tmp/café/data"),
        ];
        util_paths_to_file(&paths, &master);
        let loaded = util_file_to_paths(master.as_path());
        assert_eq!(loaded, paths);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn paths_to_file_overwrite_three_times() {
        let dir = tmp_dir();
        let master = dir.join("master");
        util_paths_to_file(&[PathBuf::from("/a")], &master);
        util_paths_to_file(&[PathBuf::from("/b"), PathBuf::from("/c")], &master);
        util_paths_to_file(&[PathBuf::from("/d")], &master);
        let loaded = util_file_to_paths(master.as_path());
        assert_eq!(loaded, vec![PathBuf::from("/d")]);
        fs::remove_dir_all(&dir).unwrap();
    }

    // ── util_remove_file + util_overwrite_file combo ─────

    #[test]
    fn overwrite_then_remove() {
        let dir = tmp_dir();
        let file = dir.join("combo.txt");
        util_overwrite_file(&file, &String::from("data"));
        assert!(file.exists());
        util_remove_file(&file);
        assert!(!file.exists());
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn overwrite_remove_overwrite_cycle() {
        let dir = tmp_dir();
        let file = dir.join("cycle.txt");
        util_overwrite_file(&file, &String::from("first"));
        util_remove_file(&file);
        util_overwrite_file(&file, &String::from("second"));
        assert_eq!(fs::read_to_string(&file).unwrap(), "second");
        fs::remove_dir_all(&dir).unwrap();
    }

    // ── append then overwrite (overwrite should truncate) ─

    #[test]
    fn append_then_overwrite_truncates() {
        let dir = tmp_dir();
        let file = dir.join("trunc.txt");
        util_append_file(&file, &String::from("first "));
        util_append_file(&file, &String::from("second "));
        util_append_file(&file, &String::from("third"));
        util_overwrite_file(&file, &String::from("replaced"));
        assert_eq!(fs::read_to_string(&file).unwrap(), "replaced");
        fs::remove_dir_all(&dir).unwrap();
    }

    // ── util_lines_to_file + util_file_to_paths combo ────

    #[test]
    fn paths_to_file_and_read_round_trip() {
        let dir = tmp_dir();
        let file = dir.join("paths.txt");
        let paths = vec![
            PathBuf::from("/tmp/a"),
            PathBuf::from("/tmp/b"),
            PathBuf::from("/tmp/c"),
        ];
        util_paths_to_file(&paths, &file);
        let loaded = util_file_to_paths(file.as_path());
        assert_eq!(loaded, paths);
        fs::remove_dir_all(&dir).unwrap();
    }

    // ── util_transform_idx exhaustive equivalences and boundaries ──

    #[test]
    fn transform_idx_all_equivalences_for_ten() {
        let len = 10;
        for i in 1..=10i32 {
            let pos = util_transform_idx(i, len);
            let neg = util_transform_idx(-(10 - i + 1), len);
            assert_eq!(
                pos,
                neg,
                "positive {} and negative {} should map to same index",
                i,
                -(10 - i + 1)
            );
        }
    }

    #[test]
    fn transform_idx_positive_second() {
        assert_eq!(util_transform_idx(2, 5), 1);
    }

    #[test]
    fn transform_idx_negative_second_from_end() {
        assert_eq!(util_transform_idx(-2, 5), 3);
    }

    #[test]
    fn transform_idx_large_list_mid() {
        assert_eq!(util_transform_idx(500, 1000), 499);
    }

    #[test]
    fn transform_idx_large_list_negative_mid() {
        assert_eq!(util_transform_idx(-500, 1000), 500);
    }

    #[test]
    fn transform_idx_positive_equals_len() {
        assert_eq!(util_transform_idx(10, 10), 9);
    }

    // ── util_path_to_string additional ──────────────────

    #[test]
    fn path_to_string_relative() {
        assert_eq!(util_path_to_string(Path::new("foo/bar")), "foo/bar");
    }

    #[test]
    fn path_to_string_dot_prefix() {
        assert_eq!(util_path_to_string(Path::new("./foo")), "./foo");
    }

    #[test]
    fn path_to_string_hidden_file() {
        assert_eq!(
            util_path_to_string(Path::new("/tmp/.hidden")),
            "/tmp/.hidden"
        );
    }

    #[test]
    fn path_to_string_extension_chain() {
        assert_eq!(util_path_to_string(Path::new("file.tar.gz")), "file.tar.gz");
    }

    #[test]
    fn path_to_string_very_long() {
        let long_path = "/".to_string() + &"a".repeat(499);
        assert_eq!(util_path_to_string(Path::new(&long_path)), long_path);
    }

    // ── util_time_ms additional ─────────────────────────

    #[test]
    fn time_ms_different_across_sleep() {
        let first = util_time_ms();
        std::thread::sleep(std::time::Duration::from_millis(10));
        let second = util_time_ms();
        let a: u128 = first.parse().unwrap();
        let b: u128 = second.parse().unwrap();
        assert!(
            b > a,
            "second call {} should be strictly greater than first {}",
            b,
            a
        );
    }

    #[test]
    fn time_ms_length_is_13_digits() {
        let ms = util_time_ms();
        assert_eq!(ms.len(), 13, "expected 13-digit epoch ms, got '{}'", ms);
    }

    // ── util_overwrite_file additional ──────────────────

    #[test]
    fn overwrite_file_binary_like_content() {
        let dir = tmp_dir();
        let file = dir.join("binary.txt");
        let content = "hello\0world\0";
        util_overwrite_file(&file, content);
        let read_back = fs::read_to_string(&file).unwrap();
        assert_eq!(read_back, content);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn overwrite_file_single_char() {
        let dir = tmp_dir();
        let file = dir.join("single.txt");
        util_overwrite_file(&file, "x");
        assert_eq!(fs::read_to_string(&file).unwrap(), "x");
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn overwrite_file_only_whitespace() {
        let dir = tmp_dir();
        let file = dir.join("ws.txt");
        let content = "   \t\n  ";
        util_overwrite_file(&file, content);
        assert_eq!(fs::read_to_string(&file).unwrap(), content);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn overwrite_file_repeated_same_content() {
        let dir = tmp_dir();
        let file = dir.join("repeat.txt");
        let content = "same content here";
        for _ in 0..5 {
            util_overwrite_file(&file, content);
        }
        assert_eq!(fs::read_to_string(&file).unwrap(), content);
        fs::remove_dir_all(&dir).unwrap();
    }

    // ── util_append_file additional ─────────────────────

    #[test]
    fn append_file_alternating_content() {
        let dir = tmp_dir();
        let file = dir.join("alt.txt");
        for _ in 0..5 {
            util_append_file(&file, "A\n");
            util_append_file(&file, "B\n");
        }
        let content = fs::read_to_string(&file).unwrap();
        assert_eq!(content, "A\nB\nA\nB\nA\nB\nA\nB\nA\nB\n");
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn append_file_single_bytes() {
        let dir = tmp_dir();
        let file = dir.join("bytes.txt");
        for ch in ["h", "e", "l", "l", "o"] {
            util_append_file(&file, ch);
        }
        assert_eq!(fs::read_to_string(&file).unwrap(), "hello");
        fs::remove_dir_all(&dir).unwrap();
    }

    // ── util_file_contents_to_string additional ─────────

    #[test]
    fn file_contents_to_string_preserves_whitespace() {
        let dir = tmp_dir();
        let file = dir.join("ws.txt");
        let content = "  hello  \n  world  ";
        fs::write(&file, content).unwrap();
        assert_eq!(util_file_contents_to_string(&file), content);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn file_contents_to_string_carriage_return() {
        let dir = tmp_dir();
        let file = dir.join("cr.txt");
        let content = "a\r\nb\r\n";
        fs::write(&file, content).unwrap();
        assert_eq!(util_file_contents_to_string(&file), content);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn file_contents_to_string_only_newlines() {
        let dir = tmp_dir();
        let file = dir.join("nls.txt");
        let content = "\n\n\n";
        fs::write(&file, content).unwrap();
        assert_eq!(util_file_contents_to_string(&file), content);
        fs::remove_dir_all(&dir).unwrap();
    }

    // ── util_file_to_lines additional ───────────────────

    #[test]
    fn file_to_lines_blank_file() {
        let dir = tmp_dir();
        let file = dir.join("blank.txt");
        fs::write(&file, "").unwrap();
        let lines = util_file_to_lines(file.as_path());
        assert!(lines.is_empty());
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn file_to_lines_only_newlines() {
        let dir = tmp_dir();
        let file = dir.join("nls.txt");
        fs::write(&file, "\n\n\n").unwrap();
        let lines = util_file_to_lines(file.as_path());
        assert_eq!(lines, vec!["", "", ""]);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn file_to_lines_mixed_content() {
        let dir = tmp_dir();
        let file = dir.join("mixed.txt");
        fs::write(&file, "data\n\nmore\n").unwrap();
        let lines = util_file_to_lines(file.as_path());
        assert_eq!(lines, vec!["data", "", "more"]);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn file_to_lines_windows_line_endings() {
        let dir = tmp_dir();
        let file = dir.join("win.txt");
        fs::write(&file, "a\r\nb\r\n").unwrap();
        let lines = util_file_to_lines(file.as_path());
        // BufReader::lines() strips \n but \r may be preserved
        for line in &lines {
            // Each line should contain the original content (possibly with \r)
            assert!(line == "a" || line == "a\r" || line == "b" || line == "b\r");
        }
        assert_eq!(lines.len(), 2);
        fs::remove_dir_all(&dir).unwrap();
    }

    // ── util_lines_to_file additional ───────────────────

    #[test]
    fn lines_to_file_empty_strings() {
        let dir = tmp_dir();
        let file = dir.join("empties.txt");
        let lines = vec!["".to_string(), "".to_string(), "".to_string()];
        util_lines_to_file(&file, lines);
        let content = fs::read_to_string(&file).unwrap();
        assert_eq!(content, "\n\n\n");
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn lines_to_file_with_special_chars() {
        let dir = tmp_dir();
        let file = dir.join("special.txt");
        let lines = vec!["col1\tcol2".to_string(), "he said \"hi\"".to_string()];
        util_lines_to_file(&file, lines);
        let content = fs::read_to_string(&file).unwrap();
        assert_eq!(content, "col1\tcol2\nhe said \"hi\"\n");
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn lines_to_file_very_long_lines() {
        let dir = tmp_dir();
        let file = dir.join("longline.txt");
        let long = "x".repeat(10000);
        let lines = vec![long.clone()];
        util_lines_to_file(&file, lines);
        let content = fs::read_to_string(&file).unwrap();
        assert_eq!(content, format!("{}\n", long));
        fs::remove_dir_all(&dir).unwrap();
    }

    // ── util_file_to_paths additional ───────────────────

    #[test]
    fn file_to_paths_preserves_order() {
        let dir = tmp_dir();
        let file = dir.join("ordered.txt");
        let expected: Vec<PathBuf> = (0..10)
            .map(|i| PathBuf::from(format!("/path/{}", i)))
            .collect();
        let content: String = expected
            .iter()
            .map(|p| format!("{}\0\0", p.display()))
            .collect();
        fs::write(&file, &content).unwrap();
        let paths = util_file_to_paths(file.as_path());
        assert_eq!(paths, expected);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn file_to_paths_deep_paths() {
        let dir = tmp_dir();
        let file = dir.join("deep.txt");
        let deep: Vec<PathBuf> = (0..3)
            .map(|i| PathBuf::from(format!("/a/b/c/d/e/f/g/h/i/j/file{}", i)))
            .collect();
        let content: String = deep
            .iter()
            .map(|p| format!("{}\0\0", p.display()))
            .collect();
        fs::write(&file, &content).unwrap();
        let paths = util_file_to_paths(file.as_path());
        assert_eq!(paths, deep);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn file_to_paths_relative_paths() {
        let dir = tmp_dir();
        let file = dir.join("rel.txt");
        fs::write(&file, "foo/bar\0\0baz/qux\0\0").unwrap();
        let paths = util_file_to_paths(file.as_path());
        assert_eq!(
            paths,
            vec![PathBuf::from("foo/bar"), PathBuf::from("baz/qux")]
        );
        fs::remove_dir_all(&dir).unwrap();
    }

    // ── util_paths_to_file additional ───────────────────

    #[test]
    fn paths_to_file_then_modify_and_rewrite() {
        let dir = tmp_dir();
        let master = dir.join("master");
        let original = vec![
            PathBuf::from("/a"),
            PathBuf::from("/b"),
            PathBuf::from("/c"),
        ];
        util_paths_to_file(&original, &master);
        let mut loaded = util_file_to_paths(master.as_path());
        loaded.retain(|p| p != &PathBuf::from("/b"));
        util_paths_to_file(&loaded, &master);
        let final_paths = util_file_to_paths(master.as_path());
        assert_eq!(final_paths, vec![PathBuf::from("/a"), PathBuf::from("/c")]);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn paths_to_file_fifty_paths() {
        let dir = tmp_dir();
        let master = dir.join("master");
        let paths: Vec<PathBuf> = (0..50)
            .map(|i| PathBuf::from(format!("/dir/file{}", i)))
            .collect();
        util_paths_to_file(&paths, &master);
        let loaded = util_file_to_paths(master.as_path());
        assert_eq!(loaded, paths);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn paths_to_file_with_deep_nesting() {
        let dir = tmp_dir();
        let master = dir.join("master");
        let paths = vec![
            PathBuf::from("/a/b/c/d/e/f/g/h"),
            PathBuf::from("/i/j/k/l/m/n/o/p"),
        ];
        util_paths_to_file(&paths, &master);
        let loaded = util_file_to_paths(master.as_path());
        assert_eq!(loaded, paths);
        fs::remove_dir_all(&dir).unwrap();
    }

    // ── Cross-function combos ───────────────────────────

    #[test]
    fn overwrite_append_overwrite_cycle() {
        let dir = tmp_dir();
        let file = dir.join("cycle.txt");
        util_overwrite_file(&file, "A");
        util_append_file(&file, "B");
        util_overwrite_file(&file, "C");
        assert_eq!(fs::read_to_string(&file).unwrap(), "C");
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn paths_to_file_empty_then_nonempty() {
        let dir = tmp_dir();
        let master = dir.join("master");
        util_paths_to_file(&[], &master);
        util_paths_to_file(&[PathBuf::from("/x"), PathBuf::from("/y")], &master);
        let loaded = util_file_to_paths(master.as_path());
        assert_eq!(loaded, vec![PathBuf::from("/x"), PathBuf::from("/y")]);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn multiple_files_in_same_dir() {
        let dir = tmp_dir();
        for i in 0..5 {
            let file = dir.join(format!("file{}.txt", i));
            let content = format!("content{}", i);
            util_overwrite_file(&file, &content);
        }
        for i in 0..5 {
            let file = dir.join(format!("file{}.txt", i));
            let expected = format!("content{}", i);
            assert_eq!(fs::read_to_string(&file).unwrap(), expected);
        }
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn lines_to_file_round_trip_100_lines() {
        let dir = tmp_dir();
        let file = dir.join("100lines.txt");
        let original: Vec<String> = (0..100).map(|i| format!("line number {}", i)).collect();
        util_lines_to_file(&file, original.clone());
        let read_back = util_file_to_lines(file.as_path());
        assert_eq!(read_back, original);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn file_to_paths_round_trip_with_spaces_and_unicode() {
        let dir = tmp_dir();
        let master = dir.join("master");
        let paths = vec![
            PathBuf::from("/my dir/some file"),
            PathBuf::from("/データ/ファイル"),
            PathBuf::from("/café/résumé"),
            PathBuf::from("/path with spaces/and 日本語"),
        ];
        util_paths_to_file(&paths, &master);
        let loaded = util_file_to_paths(master.as_path());
        assert_eq!(loaded, paths);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn paths_and_names_round_trip_unicode_tag() {
        let dir = tmp_dir();
        let master = dir.join("master");
        let paths = vec![PathBuf::from("/tmp/文件")];
        let names = vec![Some("标签".to_string())];
        util_paths_and_names_to_file(&paths, &names, &master);
        let (p, n) = util_file_to_paths_and_names(master.as_path());
        assert_eq!(p, paths);
        assert_eq!(n, names);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn paths_and_names_mixed_none_some_round_trip() {
        let dir = tmp_dir();
        let master = dir.join("master");
        let paths = vec![
            PathBuf::from("/a"),
            PathBuf::from("/b"),
            PathBuf::from("/c"),
        ];
        let names = vec![None, Some("mid".to_string()), None];
        util_paths_and_names_to_file(&paths, &names, &master);
        let (p, n) = util_file_to_paths_and_names(master.as_path());
        assert_eq!(p, paths);
        assert_eq!(n, names);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn transform_idx_boundary_positive_one() {
        assert_eq!(util_transform_idx(1, 1000), 0);
    }

    #[test]
    fn transform_idx_boundary_negative_one() {
        assert_eq!(util_transform_idx(-1, 1000), 999);
    }

    #[test]
    fn paths_and_names_single_record_with_empty_name_tag() {
        let dir = tmp_dir();
        let master = dir.join("m");
        let paths = vec![PathBuf::from("/p")];
        let names = vec![Some(String::new())];
        util_paths_and_names_to_file(&paths, &names, &master);
        let (p, n) = util_file_to_paths_and_names(master.as_path());
        assert_eq!(p, paths);
        assert_eq!(n, vec![None]);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn util_overwrite_file_preserves_binary_null() {
        let dir = tmp_dir();
        let f = dir.join("b.bin");
        util_overwrite_file(&f, "a\0b\0c");
        assert_eq!(fs::read(&f).unwrap(), b"a\0b\0c");
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn util_append_file_preserves_utf8_bom() {
        let dir = tmp_dir();
        let f = dir.join("bom.txt");
        util_overwrite_file(&f, "\u{feff}hi");
        util_append_file(&f, "\u{feff}lo");
        let s = fs::read_to_string(&f).unwrap();
        assert!(s.contains("hi") && s.contains("lo"));
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn util_lines_to_file_single_empty_string_writes_nothing() {
        let dir = tmp_dir();
        let f = dir.join("e.txt");
        util_lines_to_file(&f, vec![String::new()]);
        assert_eq!(fs::read_to_string(&f).unwrap(), "");
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn util_lines_to_file_two_empty_lines_trailing_newline() {
        let dir = tmp_dir();
        let f = dir.join("e2.txt");
        util_lines_to_file(&f, vec![String::new(), String::new()]);
        assert_eq!(fs::read_to_string(&f).unwrap(), "\n\n");
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn util_file_to_paths_single_record_no_trailing_delim() {
        let dir = tmp_dir();
        let file = dir.join("legacy");
        fs::write(&file, "/tmp/only").unwrap();
        let paths = util_file_to_paths(file.as_path());
        assert_eq!(paths, vec![PathBuf::from("/tmp/only")]);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn transform_idx_len_two_both_directions() {
        assert_eq!(util_transform_idx(1, 2), 0);
        assert_eq!(util_transform_idx(2, 2), 1);
        assert_eq!(util_transform_idx(-1, 2), 1);
        assert_eq!(util_transform_idx(-2, 2), 0);
    }

    #[test]
    fn transform_idx_len_three_mid() {
        assert_eq!(util_transform_idx(2, 3), 1);
        assert_eq!(util_transform_idx(-2, 3), 1);
    }

    #[test]
    fn util_paths_to_file_round_trip_ten_paths() {
        let dir = tmp_dir();
        let master = dir.join("m10");
        let paths: Vec<PathBuf> = (0..10)
            .map(|i| PathBuf::from(format!("/p/{}", i)))
            .collect();
        util_paths_to_file(&paths, &master);
        assert_eq!(util_file_to_paths(master.as_path()), paths);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn util_file_contents_round_trip_with_bom() {
        let dir = tmp_dir();
        let f = dir.join("bom_read");
        fs::write(&f, "\u{feff}only").unwrap();
        assert_eq!(util_file_contents_to_string(&f), "\u{feff}only");
        fs::remove_dir_all(&dir).unwrap();
    }

    // ── util_file_to_paths_and_names edge cases ───────────

    #[test]
    fn file_to_paths_and_names_name_with_embedded_null_becomes_suffix() {
        let dir = tmp_dir();
        let master = dir.join("m");
        // One record: path "\0" name-part (first split only; extra NULs stay in "name")
        let p = PathBuf::from("/safe");
        let raw = format!(
            "{}{}{}\0\0",
            p.display(),
            crate::util::consts::MASTER_FIELD_DELIM,
            "tag\0extra"
        );
        fs::write(&master, raw).unwrap();
        let (paths, names) = util_file_to_paths_and_names(master.as_path());
        assert_eq!(paths, vec![p]);
        assert_eq!(names, vec![Some("tag\0extra".to_string())]);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn file_to_paths_and_names_three_records_trailing_only_delim() {
        let dir = tmp_dir();
        let master = dir.join("m");
        fs::write(&master, "/a\0\0/b\0\0/c\0\0").unwrap();
        let (paths, names) = util_file_to_paths_and_names(master.as_path());
        assert_eq!(
            paths,
            vec![
                PathBuf::from("/a"),
                PathBuf::from("/b"),
                PathBuf::from("/c")
            ]
        );
        assert_eq!(names, vec![None, None, None]);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn paths_and_names_round_trip_name_with_newlines() {
        let dir = tmp_dir();
        let master = dir.join("m");
        let paths = vec![PathBuf::from("/p")];
        let names = vec![Some("line1\nline2".to_string())];
        util_paths_and_names_to_file(&paths, &names, &master);
        let (p, n) = util_file_to_paths_and_names(master.as_path());
        assert_eq!(p, paths);
        assert_eq!(n, names);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn util_path_to_string_unc_path_style_double_prefix() {
        assert_eq!(
            util_path_to_string(Path::new("//server/share/dir")),
            "//server/share/dir"
        );
    }

    #[test]
    fn util_time_ms_monotonic_across_many_calls() {
        let mut prev: u128 = 0;
        for _ in 0..25 {
            let t: u128 = util_time_ms().parse().unwrap();
            assert!(t >= prev, "time_ms should not go backwards");
            prev = t;
        }
    }

    #[test]
    fn util_path_to_string_root_only() {
        assert_eq!(util_path_to_string(Path::new("/")), "/");
    }

    #[test]
    fn util_path_to_string_single_component() {
        assert_eq!(util_path_to_string(Path::new("file.txt")), "file.txt");
    }

    #[test]
    fn paths_and_names_round_trip_ten_named_records() {
        let dir = tmp_dir();
        let master = dir.join("m10n");
        let paths: Vec<PathBuf> = (0..10)
            .map(|i| PathBuf::from(format!("/tmp/f{i}")))
            .collect();
        let names: Vec<Option<String>> = (0..10).map(|i| Some(format!("n{i}"))).collect();
        util_paths_and_names_to_file(&paths, &names, &master);
        let (p, n) = util_file_to_paths_and_names(master.as_path());
        assert_eq!(p, paths);
        assert_eq!(n, names);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn util_file_to_paths_skips_empty_path_record_with_warning_behavior() {
        let dir = tmp_dir();
        let f = dir.join("bad");
        fs::write(&f, "\0name\0\0/ok\0\0").unwrap();
        let (paths, names) = util_file_to_paths_and_names(f.as_path());
        assert_eq!(paths, vec![PathBuf::from("/ok")]);
        assert_eq!(names, vec![None]);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn util_paths_to_file_single_path_round_trip() {
        let dir = tmp_dir();
        let m = dir.join("one");
        util_paths_to_file(&[PathBuf::from("/solo")], &m);
        assert_eq!(
            util_file_to_paths(m.as_path()),
            vec![PathBuf::from("/solo")]
        );
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn util_path_to_string_percent_encoded_looking() {
        assert_eq!(
            util_path_to_string(Path::new("/tmp/hello%20world")),
            "/tmp/hello%20world"
        );
    }

    #[test]
    fn util_paths_and_names_empty_paths_empty_names_round_trip() {
        let dir = tmp_dir();
        let master = dir.join("empty");
        util_paths_and_names_to_file(&[], &[], &master);
        let (p, n) = util_file_to_paths_and_names(master.as_path());
        assert!(p.is_empty());
        assert!(n.is_empty());
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn util_overwrite_then_read_round_trip_unicode() {
        let dir = tmp_dir();
        let f = dir.join("u.txt");
        let s = "你好\nこんにちは\n";
        util_overwrite_file(&f, s);
        assert_eq!(util_file_contents_to_string(&f), s);
        fs::remove_dir_all(&dir).unwrap();
    }
}
