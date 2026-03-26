use std::fs::{read_to_string, remove_file, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::exit;
use std::time::SystemTime;

use log::{debug, error};

use crate::util::consts::*;


pub fn util_file_to_paths(path: &Path) -> Vec<PathBuf> {
    let file = File::open(path).expect(ERR_NO_FILE);
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| PathBuf::from(l.expect(ERR_PARSE)))
        .collect()
}


pub fn util_paths_to_file(paths: Vec<PathBuf>, out: &Path) {
    let lines: Vec<String> = paths.iter().map(|p| util_path_to_string(p)).collect();
    if out.exists() {
        debug!("remove file '{}'", util_path_to_string(out));
        util_remove_file(out);
    }
    util_lines_to_file(out, lines)
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
    path.to_path_buf()
        .into_os_string()
        .into_string()
        .expect(ERR_NO_FILE)
}


pub fn util_file_contents_to_string(filename: &Path) -> Option<String> {
    match read_to_string(filename) {
        Ok(str) => Some(str),
        Err(_error) => {
            util_terminate_error(ERR_INVALID_FILE);
            None
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
        util_overwrite_file(&file, &String::from(""));
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
        util_append_file(&file, &String::from(""));
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
        assert_eq!(result.unwrap(), "");
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn file_contents_to_string_multiline() {
        let dir = tmp_dir();
        let file = dir.join("multi.txt");
        fs::write(&file, "a\nb\nc").unwrap();
        let result = util_file_contents_to_string(file.as_path());
        assert_eq!(result.unwrap(), "a\nb\nc");
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn file_contents_to_string_unicode() {
        let dir = tmp_dir();
        let file = dir.join("unicode.txt");
        fs::write(&file, "日本語テスト").unwrap();
        let result = util_file_contents_to_string(file.as_path());
        assert_eq!(result.unwrap(), "日本語テスト");
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
        fs::write(&file, "/tmp/only\n").unwrap();
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
        util_paths_to_file(vec![], &master);
        let content = fs::read_to_string(&master).unwrap_or_default();
        assert!(content.is_empty());
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn paths_to_file_single_path() {
        let dir = tmp_dir();
        let master = dir.join("master");
        let paths = vec![PathBuf::from("/tmp/single")];
        util_paths_to_file(paths.clone(), &master);
        let loaded = util_file_to_paths(master.as_path());
        assert_eq!(loaded, paths);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn paths_to_file_many_paths() {
        let dir = tmp_dir();
        let master = dir.join("master");
        let paths: Vec<PathBuf> = (0..20).map(|i| PathBuf::from(format!("/tmp/f{}", i))).collect();
        util_paths_to_file(paths.clone(), &master);
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
        let result = util_file_contents_to_string(file.as_path()).unwrap();
        assert_eq!(result, "round trip data");
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn overwrite_then_append_then_read() {
        let dir = tmp_dir();
        let file = dir.join("combo.txt");
        util_overwrite_file(&file, &String::from("base"));
        util_append_file(&file, &String::from("+extra"));
        let result = util_file_contents_to_string(file.as_path()).unwrap();
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
                i, neg
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
        assert_eq!(fs::read_to_string(&file).unwrap(), "much longer content here now");
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
        let result = util_file_contents_to_string(file.as_path()).unwrap();
        assert_eq!(result, "data\n");
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn file_contents_to_string_with_special_chars() {
        let dir = tmp_dir();
        let file = dir.join("special.txt");
        let content = "tab\there\nnewline\r\nwindows\0null";
        fs::write(&file, content).unwrap();
        let result = util_file_contents_to_string(file.as_path()).unwrap();
        assert_eq!(result, content);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn file_contents_to_string_large_file() {
        let dir = tmp_dir();
        let file = dir.join("large.txt");
        let content = "y".repeat(50_000);
        fs::write(&file, &content).unwrap();
        let result = util_file_contents_to_string(file.as_path()).unwrap();
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
        let content: String = (0..100).map(|i| format!("line{}", i)).collect::<Vec<_>>().join("\n");
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
        let lines = vec!["日本語".to_string(), "中文".to_string(), "한국어".to_string()];
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
        fs::write(&file, "/tmp/my dir/file1\n/tmp/another dir/file2\n").unwrap();
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
        let content: String = (0..50).map(|i| format!("/tmp/f{}\n", i)).collect();
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
        util_paths_to_file(paths.clone(), &master);
        let loaded = util_file_to_paths(master.as_path());
        assert_eq!(loaded, paths);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn paths_to_file_overwrite_three_times() {
        let dir = tmp_dir();
        let master = dir.join("master");
        util_paths_to_file(vec![PathBuf::from("/a")], &master);
        util_paths_to_file(vec![PathBuf::from("/b"), PathBuf::from("/c")], &master);
        util_paths_to_file(vec![PathBuf::from("/d")], &master);
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
    fn lines_to_file_as_paths_round_trip() {
        let dir = tmp_dir();
        let file = dir.join("paths.txt");
        let paths = vec![
            PathBuf::from("/tmp/a"),
            PathBuf::from("/tmp/b"),
            PathBuf::from("/tmp/c"),
        ];
        // Write paths as lines manually
        let lines: Vec<String> = paths.iter().map(|p| util_path_to_string(p)).collect();
        util_lines_to_file(&file, lines);
        // Read them back as paths
        let loaded = util_file_to_paths(file.as_path());
        assert_eq!(loaded, paths);
        fs::remove_dir_all(&dir).unwrap();
    }
}
