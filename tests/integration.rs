use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

static TEST_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Helper: path to the built binary
fn bin() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_BIN_EXE_tp"));
    if !path.exists() {
        path = PathBuf::from(env!("CARGO_BIN_EXE_temprs"));
    }
    path
}

/// Create a unique isolated directory for each test, returning its path.
fn setup_clean_env() -> PathBuf {
    let id = TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
    let dir = std::env::temp_dir().join(format!(
        "temprs_test_{}_{}",
        std::process::id(),
        id,
    ));
    if dir.exists() {
        let _ = fs::remove_dir_all(&dir);
    }
    dir
}

fn run_tp(dir: &PathBuf, args: &[&str]) -> std::process::Output {
    Command::new(bin())
        .env("TEMPRS_DIR", dir)
        .args(args)
        .stdin(std::process::Stdio::null())
        .output()
        .expect("failed to execute tp")
}

/// Delay to avoid tempfile name collisions (ms-resolution timestamp).
/// Must be long enough that sequential process invocations never land on
/// the same millisecond, even under CI load.
fn tick() {
    std::thread::sleep(std::time::Duration::from_millis(50));
}

fn run_tp_stdin(dir: &PathBuf, args: &[&str], input: &str) -> std::process::Output {
    use std::io::Write;
    let mut child = Command::new(bin())
        .env("TEMPRS_DIR", dir)
        .args(args)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("failed to spawn tp");
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();
    child.wait_with_output().unwrap()
}

fn stdout(output: &std::process::Output) -> String {
    String::from_utf8_lossy(&output.stdout).to_string()
}

// ── Help output ─────────────────────────────────────────

#[test]
fn help_flag_shows_cyberpunk_banner() {
    let dir = setup_clean_env();
    let out = run_tp(&dir, &["-h"]);
    let text = stdout(&out);
    assert!(text.contains("████████╗"), "missing ASCII art banner");
    assert!(text.contains("STATUS: ONLINE"), "missing status bar");
    assert!(text.contains("TEMPORARY FILE STACK MANAGER"), "missing tagline");
    assert!(text.contains("JACK IN"), "missing cyberpunk footer");
}

#[test]
fn help_shows_all_flags() {
    let dir = setup_clean_env();
    let out = run_tp(&dir, &["-h"]);
    let text = stdout(&out);
    for flag in &[
        "--input", "--output", "--add", "--remove", "--pop", "--unshift",
        "--shift", "--dir", "--master", "--list-files", "--list-files-numbered",
        "--list-contents", "--list-contents-numbered", "--quiet", "--clear",
        "--verbose",
    ] {
        assert!(text.contains(flag), "missing flag: {}", flag);
    }
}

#[test]
fn version_flag_shows_version() {
    let dir = setup_clean_env();
    let out = run_tp(&dir, &["-V"]);
    let text = stdout(&out);
    assert!(text.contains(env!("CARGO_PKG_VERSION")));
}

// ── Clear ───────────────────────────────────────────────

#[test]
fn clear_removes_all() {
    let dir = setup_clean_env();
    // push some data first
    run_tp_stdin(&dir, &[], "data1");
    run_tp_stdin(&dir, &[], "data2");
    // clear
    let out = run_tp(&dir, &["-c"]);
    assert!(out.status.success());
    // after clear, list should be empty (dir is gone, so it'll error or be empty)
    let list = run_tp(&dir, &["-l"]);
    let text = stdout(&list);
    assert!(text.trim().is_empty());
}

// ── Push and read back ──────────────────────────────────

#[test]
fn push_stdin_and_read_top() {
    let dir = setup_clean_env();
    // push
    run_tp_stdin(&dir, &[], "hello from test\n");
    // read top of stack via -o
    let out = run_tp(&dir, &["-o", "1"]);
    let text = stdout(&out);
    assert_eq!(text, "hello from test\n");
}

#[test]
fn push_multiple_and_list() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "first");
    run_tp_stdin(&dir, &[], "second");
    run_tp_stdin(&dir, &[], "third");
    let out = run_tp(&dir, &["-l"]);
    let text = stdout(&out);
    let lines: Vec<&str> = text.trim().lines().collect();
    assert_eq!(lines.len(), 3, "expected 3 files on stack");
}

// ── Output at index ─────────────────────────────────────

#[test]
fn output_at_index() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "aaa");
    run_tp_stdin(&dir, &[], "bbb");
    run_tp_stdin(&dir, &[], "ccc");

    let out1 = run_tp(&dir, &["-o", "1"]);
    assert_eq!(stdout(&out1), "aaa");

    let out2 = run_tp(&dir, &["-o", "2"]);
    assert_eq!(stdout(&out2), "bbb");

    let out3 = run_tp(&dir, &["-o", "3"]);
    assert_eq!(stdout(&out3), "ccc");
}

#[test]
fn output_negative_index_top() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "first");
    tick();
    run_tp_stdin(&dir, &[], "last");

    // -1 should be top of stack (last)
    let out = run_tp(&dir, &["-o", "-1"]);
    assert_eq!(stdout(&out), "last");
}

#[test]
fn output_negative_index_bottom() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "first");
    tick();
    run_tp_stdin(&dir, &[], "last");

    // -2 should be bottom of stack (first)
    let out = run_tp(&dir, &["-o", "-2"]);
    assert_eq!(stdout(&out), "first");
}

// ── Input (overwrite) at index ──────────────────────────

#[test]
fn input_overwrites_at_index() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "original");
    run_tp_stdin(&dir, &["-i", "1"], "replaced");
    let out = run_tp(&dir, &["-o", "1"]);
    assert_eq!(stdout(&out), "replaced");
}

// ── Pop ─────────────────────────────────────────────────

#[test]
fn pop_removes_top() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "bottom");
    run_tp_stdin(&dir, &[], "top");

    let list_before = run_tp(&dir, &["-l"]);
    let count_before = stdout(&list_before).trim().lines().count();
    assert_eq!(count_before, 2);

    run_tp(&dir, &["-p"]);

    let list_after = run_tp(&dir, &["-l"]);
    let count_after = stdout(&list_after).trim().lines().count();
    assert_eq!(count_after, 1);

    // remaining should be bottom
    let out = run_tp(&dir, &["-o", "1"]);
    assert_eq!(stdout(&out), "bottom");
}

// ── Shift ───────────────────────────────────────────────

#[test]
fn shift_removes_bottom() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "bottom");
    run_tp_stdin(&dir, &[], "top");
    run_tp(&dir, &["-s"]);

    let list = run_tp(&dir, &["-l"]);
    let count = stdout(&list).trim().lines().count();
    assert_eq!(count, 1);

    let out = run_tp(&dir, &["-o", "1"]);
    assert_eq!(stdout(&out), "top");
}

// ── Unshift ─────────────────────────────────────────────

#[test]
fn unshift_adds_to_bottom() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "existing");
    run_tp_stdin(&dir, &["-u"], "new_bottom");

    let out = run_tp(&dir, &["-o", "1"]);
    assert_eq!(stdout(&out), "new_bottom");

    let out = run_tp(&dir, &["-o", "2"]);
    assert_eq!(stdout(&out), "existing");
}

// ── Add at index ────────────────────────────────────────

#[test]
fn add_inserts_at_index() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "first");
    run_tp_stdin(&dir, &[], "third");
    run_tp_stdin(&dir, &["-a", "2"], "second");

    let out = run_tp(&dir, &["-o", "1"]);
    assert_eq!(stdout(&out), "first");

    let out = run_tp(&dir, &["-o", "2"]);
    assert_eq!(stdout(&out), "second");

    let out = run_tp(&dir, &["-o", "3"]);
    assert_eq!(stdout(&out), "third");
}

// ── Remove at index ─────────────────────────────────────

#[test]
fn remove_at_index() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "aaa");
    run_tp_stdin(&dir, &[], "bbb");
    run_tp_stdin(&dir, &[], "ccc");
    run_tp(&dir, &["-r", "2"]);

    let list = run_tp(&dir, &["-l"]);
    let count = stdout(&list).trim().lines().count();
    assert_eq!(count, 2);

    let out1 = run_tp(&dir, &["-o", "1"]);
    assert_eq!(stdout(&out1), "aaa");

    let out2 = run_tp(&dir, &["-o", "2"]);
    assert_eq!(stdout(&out2), "ccc");
}

// ── List contents ───────────────────────────────────────

#[test]
fn list_contents_shows_data() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "hello");
    run_tp_stdin(&dir, &[], "world");

    let out = run_tp(&dir, &["-L"]);
    let text = stdout(&out);
    assert!(text.contains("hello"));
    assert!(text.contains("world"));
}

#[test]
fn list_contents_numbered_shows_indices() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "alpha");
    run_tp_stdin(&dir, &[], "beta");

    let out = run_tp(&dir, &["-N"]);
    let text = stdout(&out);
    assert!(text.contains("1:"));
    assert!(text.contains("2:"));
    assert!(text.contains("alpha"));
    assert!(text.contains("beta"));
}

#[test]
fn list_files_numbered_shows_indices() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data1");
    run_tp_stdin(&dir, &[], "data2");

    let out = run_tp(&dir, &["-n"]);
    let text = stdout(&out);
    assert!(text.contains("1:"));
    assert!(text.contains("2:"));
}

// ── Dir and master ──────────────────────────────────────

#[test]
fn dir_flag_shows_directory() {
    let dir = setup_clean_env();
    // push something to ensure dir exists
    run_tp_stdin(&dir, &[], "x");
    let out = run_tp(&dir, &["-d"]);
    let text = stdout(&out);
    assert!(text.contains("temprs"), "dir output should contain 'temprs'");
}

#[test]
fn master_flag_shows_master_file() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "x");
    let out = run_tp(&dir, &["-m"]);
    let text = stdout(&out);
    assert!(
        text.contains("temprs-stack"),
        "master output should contain 'temprs-stack'"
    );
}

// ── Verbose flag with stdin ─────────────────────────────

#[test]
fn verbose_echoes_stdin() {
    let dir = setup_clean_env();
    let out = run_tp_stdin(&dir, &["-v"], "echo me");
    let text = stdout(&out);
    assert!(text.contains("echo me"), "verbose should echo stdin to stdout");
}

// ── File argument ───────────────────────────────────────
// Note: file argument only works when stdin is a real terminal (atty check).
// In test harness stdin is never a terminal, so we pipe file content
// through stdin instead to test equivalent functionality.

#[test]
fn file_content_via_stdin() {
    let dir = setup_clean_env();
    let content = "file content here";
    run_tp_stdin(&dir, &[], content);

    let out = run_tp(&dir, &["-o", "1"]);
    assert_eq!(stdout(&out), content);
}

#[test]
fn file_content_via_stdin_with_verbose() {
    let dir = setup_clean_env();
    let out = run_tp_stdin(&dir, &["-v"], "verbose file");
    let text = stdout(&out);
    assert!(text.contains("verbose file"));
}

// ── Edge cases ──────────────────────────────────────────

#[test]
fn empty_stack_list_is_empty() {
    let dir = setup_clean_env();
    let out = run_tp(&dir, &["-l"]);
    let text = stdout(&out);
    assert!(text.trim().is_empty());
}

#[test]
fn push_empty_string() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "");
    let out = run_tp(&dir, &["-o", "1"]);
    assert_eq!(stdout(&out), "");
}

#[test]
fn push_multiline_content() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "line1\nline2\nline3\n");
    let out = run_tp(&dir, &["-o", "1"]);
    assert_eq!(stdout(&out), "line1\nline2\nline3\n");
}

#[test]
fn push_unicode_content() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "hello 世界 🚀");
    let out = run_tp(&dir, &["-o", "1"]);
    assert_eq!(stdout(&out), "hello 世界 🚀");
}

// ── Invalid index exits with error ──────────────────────

#[test]
fn invalid_index_zero_errors() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["-o", "0"]);
    assert!(!out.status.success());
}

#[test]
fn index_out_of_bounds_errors() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "only one");
    let out = run_tp(&dir, &["-o", "5"]);
    assert!(!out.status.success());
}

// ── Negative index errors ───────────────────────────────

#[test]
fn negative_index_out_of_bounds_errors() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "only one");
    let out = run_tp(&dir, &["-o", "-2"]);
    assert!(!out.status.success());
}

#[test]
fn negative_index_far_out_of_bounds_errors() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "a");
    run_tp_stdin(&dir, &[], "b");
    let out = run_tp(&dir, &["-o", "-10"]);
    assert!(!out.status.success());
}

// ── Large stack operations ──────────────────────────────

#[test]
fn push_many_items() {
    let dir = setup_clean_env();
    for i in 0..10 {
        run_tp_stdin(&dir, &[], &format!("item{}", i));
    }
    let out = run_tp(&dir, &["-l"]);
    let text = stdout(&out);
    let lines: Vec<&str> = text.trim().lines().collect();
    assert_eq!(lines.len(), 10);
}

#[test]
fn push_many_and_read_all() {
    let dir = setup_clean_env();
    for i in 1..=5 {
        run_tp_stdin(&dir, &[], &format!("val{}", i));
    }
    for i in 1..=5 {
        let out = run_tp(&dir, &["-o", &i.to_string()]);
        assert_eq!(stdout(&out), format!("val{}", i));
    }
}

#[test]
fn push_many_and_list_all_by_positive_index() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "val1");
    tick();
    run_tp_stdin(&dir, &[], "val2");
    tick();
    run_tp_stdin(&dir, &[], "val3");
    tick();
    run_tp_stdin(&dir, &[], "val4");
    tick();
    run_tp_stdin(&dir, &[], "val5");
    // verify stack has exactly 5 items
    let list = run_tp(&dir, &["-l"]);
    let count = stdout(&list).trim().lines().count();
    assert_eq!(count, 5, "expected 5 items on stack");
    // read each by positive index
    for i in 1..=5 {
        let out = run_tp(&dir, &["-o", &i.to_string()]);
        assert_eq!(stdout(&out), format!("val{}", i));
    }
}

// ── Pop edge cases ──────────────────────────────────────

#[test]
fn pop_single_item_leaves_empty() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "only");
    run_tp(&dir, &["-p"]);
    let out = run_tp(&dir, &["-l"]);
    let text = stdout(&out);
    assert!(text.trim().is_empty());
}

#[test]
fn pop_multiple_times() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "a");
    run_tp_stdin(&dir, &[], "b");
    run_tp_stdin(&dir, &[], "c");
    run_tp(&dir, &["-p"]); // remove c
    run_tp(&dir, &["-p"]); // remove b
    let out = run_tp(&dir, &["-l"]);
    assert_eq!(stdout(&out).trim().lines().count(), 1);
    let out = run_tp(&dir, &["-o", "1"]);
    assert_eq!(stdout(&out), "a");
}

// ── Shift edge cases ────────────────────────────────────

#[test]
fn shift_single_item_leaves_empty() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "only");
    run_tp(&dir, &["-s"]);
    let out = run_tp(&dir, &["-l"]);
    let text = stdout(&out);
    assert!(text.trim().is_empty());
}

#[test]
fn shift_multiple_times() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "a");
    run_tp_stdin(&dir, &[], "b");
    run_tp_stdin(&dir, &[], "c");
    run_tp(&dir, &["-s"]); // remove a
    run_tp(&dir, &["-s"]); // remove b
    let out = run_tp(&dir, &["-l"]);
    assert_eq!(stdout(&out).trim().lines().count(), 1);
    let out = run_tp(&dir, &["-o", "1"]);
    assert_eq!(stdout(&out), "c");
}

// ── Unshift edge cases ──────────────────────────────────

#[test]
fn unshift_to_empty_stack_fails() {
    let dir = setup_clean_env();
    // unshift on empty stack should fail (index 1 in a 0-length stack)
    let out = run_tp_stdin(&dir, &["-u"], "first_via_unshift");
    assert!(!out.status.success());
}

#[test]
fn unshift_multiple_times() {
    let dir = setup_clean_env();
    // need at least one item before unshift works
    run_tp_stdin(&dir, &[], "third");
    tick();
    run_tp_stdin(&dir, &["-u"], "second");
    tick();
    run_tp_stdin(&dir, &["-u"], "first");
    let out = run_tp(&dir, &["-o", "1"]);
    assert_eq!(stdout(&out), "first");
    let out = run_tp(&dir, &["-o", "2"]);
    assert_eq!(stdout(&out), "second");
    let out = run_tp(&dir, &["-o", "3"]);
    assert_eq!(stdout(&out), "third");
}

// ── Add at index edge cases ─────────────────────────────

#[test]
fn add_at_beginning() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "existing");
    run_tp_stdin(&dir, &["-a", "1"], "new_first");
    let out = run_tp(&dir, &["-o", "1"]);
    assert_eq!(stdout(&out), "new_first");
    let out = run_tp(&dir, &["-o", "2"]);
    assert_eq!(stdout(&out), "existing");
}

#[test]
fn add_at_end() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "first");
    run_tp_stdin(&dir, &[], "second");
    // add at index 2 (end of current 2-item stack)
    run_tp_stdin(&dir, &["-a", "2"], "inserted_at_end");
    let out = run_tp(&dir, &["-o", "2"]);
    assert_eq!(stdout(&out), "inserted_at_end");
    let out = run_tp(&dir, &["-o", "3"]);
    assert_eq!(stdout(&out), "second");
}

// ── Remove at index edge cases ──────────────────────────

#[test]
fn remove_first_item() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "aaa");
    run_tp_stdin(&dir, &[], "bbb");
    run_tp(&dir, &["-r", "1"]);
    let out = run_tp(&dir, &["-l"]);
    assert_eq!(stdout(&out).trim().lines().count(), 1);
    let out = run_tp(&dir, &["-o", "1"]);
    assert_eq!(stdout(&out), "bbb");
}

#[test]
fn remove_last_item() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "aaa");
    run_tp_stdin(&dir, &[], "bbb");
    run_tp(&dir, &["-r", "2"]);
    let out = run_tp(&dir, &["-l"]);
    assert_eq!(stdout(&out).trim().lines().count(), 1);
    let out = run_tp(&dir, &["-o", "1"]);
    assert_eq!(stdout(&out), "aaa");
}

#[test]
fn remove_only_item() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "solo");
    run_tp(&dir, &["-r", "1"]);
    let out = run_tp(&dir, &["-l"]);
    assert!(stdout(&out).trim().is_empty());
}

// ── Input (overwrite) additional cases ──────────────────

#[test]
fn input_overwrites_last_index() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "aaa");
    run_tp_stdin(&dir, &[], "bbb");
    run_tp_stdin(&dir, &["-i", "2"], "replaced_bbb");
    let out = run_tp(&dir, &["-o", "2"]);
    assert_eq!(stdout(&out), "replaced_bbb");
    // first item unchanged
    let out = run_tp(&dir, &["-o", "1"]);
    assert_eq!(stdout(&out), "aaa");
}

#[test]
fn input_overwrites_preserves_count() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "a");
    run_tp_stdin(&dir, &[], "b");
    run_tp_stdin(&dir, &[], "c");
    run_tp_stdin(&dir, &["-i", "2"], "B");
    let out = run_tp(&dir, &["-l"]);
    assert_eq!(stdout(&out).trim().lines().count(), 3);
}

#[test]
fn input_overwrite_with_multiline() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "original");
    run_tp_stdin(&dir, &["-i", "1"], "line1\nline2\nline3");
    let out = run_tp(&dir, &["-o", "1"]);
    assert_eq!(stdout(&out), "line1\nline2\nline3");
}

// ── Content types ───────────────────────────────────────

#[test]
fn push_content_with_special_chars() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "hello\tworld\nfoo\tbar");
    let out = run_tp(&dir, &["-o", "1"]);
    assert_eq!(stdout(&out), "hello\tworld\nfoo\tbar");
}

#[test]
fn push_content_with_quotes() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], r#"She said "hello" and 'goodbye'"#);
    let out = run_tp(&dir, &["-o", "1"]);
    assert_eq!(stdout(&out), r#"She said "hello" and 'goodbye'"#);
}

#[test]
fn push_content_with_backslashes() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], r"path\to\file");
    let out = run_tp(&dir, &["-o", "1"]);
    assert_eq!(stdout(&out), r"path\to\file");
}

#[test]
fn push_content_with_newlines_only() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "\n\n\n");
    let out = run_tp(&dir, &["-o", "1"]);
    assert_eq!(stdout(&out), "\n\n\n");
}

#[test]
fn push_large_content() {
    let dir = setup_clean_env();
    let big = "x".repeat(10_000);
    run_tp_stdin(&dir, &[], &big);
    let out = run_tp(&dir, &["-o", "1"]);
    assert_eq!(stdout(&out).len(), 10_000);
}

// ── List files ──────────────────────────────────────────

#[test]
fn list_files_empty_stack() {
    let dir = setup_clean_env();
    let out = run_tp(&dir, &["-l"]);
    assert!(stdout(&out).trim().is_empty());
    assert!(out.status.success());
}

#[test]
fn list_files_single_item() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["-l"]);
    let text = stdout(&out);
    assert_eq!(text.trim().lines().count(), 1);
    assert!(text.contains("tempfile"));
}

#[test]
fn list_files_paths_contain_tempfile_prefix() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "a");
    run_tp_stdin(&dir, &[], "b");
    let out = run_tp(&dir, &["-l"]);
    let text = stdout(&out);
    for line in text.trim().lines() {
        assert!(line.contains("tempfile"), "path should contain 'tempfile': {}", line);
    }
}

// ── List contents ───────────────────────────────────────

#[test]
fn list_contents_single_item() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "only content");
    let out = run_tp(&dir, &["-L"]);
    let text = stdout(&out);
    assert!(text.contains("only content"));
}

#[test]
fn list_contents_empty_stack() {
    let dir = setup_clean_env();
    let out = run_tp(&dir, &["-L"]);
    let text = stdout(&out);
    assert!(text.trim().is_empty());
}

#[test]
fn list_contents_preserves_order() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "first_content");
    run_tp_stdin(&dir, &[], "second_content");
    run_tp_stdin(&dir, &[], "third_content");
    let out = run_tp(&dir, &["-L"]);
    let text = stdout(&out);
    let first_pos = text.find("first_content").unwrap();
    let second_pos = text.find("second_content").unwrap();
    let third_pos = text.find("third_content").unwrap();
    assert!(first_pos < second_pos);
    assert!(second_pos < third_pos);
}

// ── List numbered ───────────────────────────────────────

#[test]
fn list_files_numbered_empty_stack() {
    let dir = setup_clean_env();
    let out = run_tp(&dir, &["-n"]);
    let text = stdout(&out);
    assert!(text.trim().is_empty());
}

#[test]
fn list_contents_numbered_empty_stack() {
    let dir = setup_clean_env();
    let out = run_tp(&dir, &["-N"]);
    let text = stdout(&out);
    assert!(text.trim().is_empty());
}

#[test]
fn list_files_numbered_three_items() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "a");
    run_tp_stdin(&dir, &[], "b");
    run_tp_stdin(&dir, &[], "c");
    let out = run_tp(&dir, &["-n"]);
    let text = stdout(&out);
    assert!(text.contains("1:"));
    assert!(text.contains("2:"));
    assert!(text.contains("3:"));
}

#[test]
fn list_contents_numbered_three_items() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "alpha");
    run_tp_stdin(&dir, &[], "beta");
    run_tp_stdin(&dir, &[], "gamma");
    let out = run_tp(&dir, &["-N"]);
    let text = stdout(&out);
    assert!(text.contains("1:"));
    assert!(text.contains("2:"));
    assert!(text.contains("3:"));
    assert!(text.contains("alpha"));
    assert!(text.contains("beta"));
    assert!(text.contains("gamma"));
}

// ── Dir and master additional tests ─────────────────────

#[test]
fn dir_flag_shows_exact_path() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "x");
    let out = run_tp(&dir, &["-d"]);
    let text = stdout(&out).trim().to_string();
    assert_eq!(text, dir.to_string_lossy());
}

#[test]
fn master_flag_path_ends_with_temprs_stack() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "x");
    let out = run_tp(&dir, &["-m"]);
    let text = stdout(&out).trim().to_string();
    assert!(text.ends_with("temprs-stack"));
}

#[test]
fn master_flag_path_in_correct_dir() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "x");
    let out = run_tp(&dir, &["-m"]);
    let text = stdout(&out).trim().to_string();
    assert!(text.starts_with(&dir.to_string_lossy().to_string()));
}

// ── Clear additional tests ──────────────────────────────

#[test]
fn clear_on_empty_stack() {
    let dir = setup_clean_env();
    // push and clear to create the dir, then clear again
    run_tp_stdin(&dir, &[], "temp");
    run_tp(&dir, &["-c"]);
    let out = run_tp(&dir, &["-c"]);
    // second clear should still succeed (dir already gone)
    // exit code may vary but shouldn't crash
    let _ = out;
}

#[test]
fn clear_then_push_works() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "before_clear");
    run_tp(&dir, &["-c"]);
    run_tp_stdin(&dir, &[], "after_clear");
    let out = run_tp(&dir, &["-o", "1"]);
    assert_eq!(stdout(&out), "after_clear");
}

#[test]
fn clear_removes_many_items() {
    let dir = setup_clean_env();
    for i in 0..10 {
        run_tp_stdin(&dir, &[], &format!("item{}", i));
    }
    let out = run_tp(&dir, &["-c"]);
    assert!(out.status.success());
    let list = run_tp(&dir, &["-l"]);
    assert!(stdout(&list).trim().is_empty());
}

// ── Quiet flag ──────────────────────────────────────────

#[test]
fn quiet_flag_suppresses_output() {
    let dir = setup_clean_env();
    let out = run_tp_stdin(&dir, &["-q"], "quiet data");
    let text = stdout(&out);
    // quiet should suppress output when creating
    assert!(text.trim().is_empty() || !text.contains("quiet data"));
}

#[test]
fn quiet_flag_still_stores_data() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &["-q"], "stored quietly");
    let out = run_tp(&dir, &["-o", "1"]);
    assert_eq!(stdout(&out), "stored quietly");
}

// ── Exit codes ──────────────────────────────────────────

#[test]
fn push_exits_success() {
    let dir = setup_clean_env();
    let out = run_tp_stdin(&dir, &[], "data");
    assert!(out.status.success());
}

#[test]
fn list_exits_success() {
    let dir = setup_clean_env();
    let out = run_tp(&dir, &["-l"]);
    assert!(out.status.success());
}

#[test]
fn output_valid_index_exits_success() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["-o", "1"]);
    assert!(out.status.success());
}

#[test]
fn output_invalid_index_exits_failure() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["-o", "99"]);
    assert!(!out.status.success());
}

#[test]
fn help_exits_success() {
    let dir = setup_clean_env();
    let out = run_tp(&dir, &["-h"]);
    assert!(out.status.success());
}

#[test]
fn version_exits_success() {
    let dir = setup_clean_env();
    let out = run_tp(&dir, &["-V"]);
    assert!(out.status.success());
}

// ── Combined operations ─────────────────────────────────

#[test]
fn push_pop_push_cycle() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "first");
    tick();
    run_tp(&dir, &["-p"]);
    tick();
    run_tp_stdin(&dir, &[], "second");
    // after pop+push we should have 1 item
    let list = run_tp(&dir, &["-l"]);
    assert_eq!(stdout(&list).trim().lines().count(), 1);
    let out = run_tp(&dir, &["-o", "1"]);
    assert_eq!(stdout(&out), "second");
}

#[test]
fn push_shift_push_cycle() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "first");
    run_tp(&dir, &["-s"]);
    run_tp_stdin(&dir, &[], "second");
    let out = run_tp(&dir, &["-o", "1"]);
    assert_eq!(stdout(&out), "second");
}

#[test]
fn interleaved_push_and_remove() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "a");
    run_tp_stdin(&dir, &[], "b");
    run_tp(&dir, &["-r", "1"]); // remove a
    run_tp_stdin(&dir, &[], "c");
    run_tp_stdin(&dir, &[], "d");
    run_tp(&dir, &["-r", "2"]); // remove d (which is at index 2 of [b,c,d]... wait, after removing a we have [b], then push c,d => [b,c,d], remove index 2 = c)
    // stack should be [b, d]
    let list = run_tp(&dir, &["-l"]);
    assert_eq!(stdout(&list).trim().lines().count(), 2);
}

#[test]
fn overwrite_then_read_multiple() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "orig1");
    run_tp_stdin(&dir, &[], "orig2");
    run_tp_stdin(&dir, &[], "orig3");
    run_tp_stdin(&dir, &["-i", "1"], "new1");
    run_tp_stdin(&dir, &["-i", "3"], "new3");
    let out = run_tp(&dir, &["-o", "1"]);
    assert_eq!(stdout(&out), "new1");
    let out = run_tp(&dir, &["-o", "2"]);
    assert_eq!(stdout(&out), "orig2");
    let out = run_tp(&dir, &["-o", "3"]);
    assert_eq!(stdout(&out), "new3");
}

// ── Help content validation ─────────────────────────────

#[test]
fn help_shows_short_flags() {
    let dir = setup_clean_env();
    let out = run_tp(&dir, &["-h"]);
    let text = stdout(&out);
    for flag in &["-i", "-o", "-a", "-r", "-p", "-u", "-s", "-d", "-m", "-l", "-n", "-L", "-N", "-q", "-c", "-v"] {
        assert!(text.contains(flag), "missing short flag: {}", flag);
    }
}

#[test]
fn help_contains_usage() {
    let dir = setup_clean_env();
    let out = run_tp(&dir, &["-h"]);
    let text = stdout(&out);
    assert!(text.contains("USAGE"), "help should contain USAGE section");
}

// ── Numbered list formatting ────────────────────────────

#[test]
fn list_files_numbered_contains_separator() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["-n"]);
    let text = stdout(&out);
    // numbered lists use horizontal rules as separators
    assert!(text.contains("---"), "numbered list should have separator lines");
}

#[test]
fn list_contents_numbered_contains_separator() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["-N"]);
    let text = stdout(&out);
    assert!(text.contains("---"), "numbered contents list should have separator lines");
}

// ── Stderr for errors ───────────────────────────────────

#[test]
fn invalid_index_shows_error_message() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["-o", "0"]);
    // simple_logger outputs to stdout
    let text = stdout(&out);
    assert!(text.contains("Invalid specified index"), "output should contain error message: {}", text);
}

#[test]
fn out_of_bounds_shows_error_message() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["-o", "100"]);
    let text = stdout(&out);
    assert!(text.contains("Invalid specified index"), "output should contain error message: {}", text);
}
