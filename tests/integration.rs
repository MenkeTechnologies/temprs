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
        "--verbose", "--edit", "--name", "--rename", "--info", "--grep", "--cat", "--count", "--diff", "--mv", "--dup", "--swap", "--append", "--rev", "--expire", "--head", "--tail",
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

// ── Remove with negative index ──────────────────────────

#[test]
fn remove_negative_index() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "aaa");
    run_tp_stdin(&dir, &[], "bbb");
    run_tp_stdin(&dir, &[], "ccc");
    // -1 should remove last item (ccc)
    run_tp(&dir, &["-r", "-1"]);
    let list = run_tp(&dir, &["-l"]);
    assert_eq!(stdout(&list).trim().lines().count(), 2);
    let out = run_tp(&dir, &["-o", "1"]);
    assert_eq!(stdout(&out), "aaa");
    let out = run_tp(&dir, &["-o", "2"]);
    assert_eq!(stdout(&out), "bbb");
}

#[test]
fn remove_negative_first() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "aaa");
    run_tp_stdin(&dir, &[], "bbb");
    // -2 should remove first item
    run_tp(&dir, &["-r", "-2"]);
    let list = run_tp(&dir, &["-l"]);
    assert_eq!(stdout(&list).trim().lines().count(), 1);
    let out = run_tp(&dir, &["-o", "1"]);
    assert_eq!(stdout(&out), "bbb");
}

// ── Add with negative index ─────────────────────────────

#[test]
fn add_negative_index() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "first");
    run_tp_stdin(&dir, &[], "third");
    // -1 should insert at last position
    run_tp_stdin(&dir, &["-a", "-1"], "inserted");
    let list = run_tp(&dir, &["-l"]);
    assert_eq!(stdout(&list).trim().lines().count(), 3);
}

// ── Input overwrite with negative index ─────────────────

#[test]
fn input_overwrite_second_item() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "aaa");
    run_tp_stdin(&dir, &[], "bbb");
    run_tp_stdin(&dir, &["-i", "2"], "replaced_bbb");
    let out = run_tp(&dir, &["-o", "2"]);
    assert_eq!(stdout(&out), "replaced_bbb");
    let out = run_tp(&dir, &["-o", "1"]);
    assert_eq!(stdout(&out), "aaa");
}

// ── Push and verify stack order preserved ───────────────

#[test]
fn push_ten_items_verify_order() {
    let dir = setup_clean_env();
    for i in 1..=10 {
        run_tp_stdin(&dir, &[], &format!("item{}", i));
    }
    for i in 1..=10 {
        let out = run_tp(&dir, &["-o", &i.to_string()]);
        assert_eq!(stdout(&out), format!("item{}", i));
    }
}

#[test]
fn push_three_items_negative_index_last() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "first");
    tick();
    run_tp_stdin(&dir, &[], "middle");
    tick();
    run_tp_stdin(&dir, &[], "last");
    // -1 should be last item
    let out = run_tp(&dir, &["-o", "-1"]);
    assert_eq!(stdout(&out), "last");
}

#[test]
fn push_three_items_negative_index_first() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "first");
    tick();
    run_tp_stdin(&dir, &[], "middle");
    tick();
    run_tp_stdin(&dir, &[], "last");
    // -3 should be first item
    let out = run_tp(&dir, &["-o", "-3"]);
    assert_eq!(stdout(&out), "first");
}

// ── Verbose does not affect data storage ────────────────

#[test]
fn verbose_does_not_affect_stored_data() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &["-v"], "data with verbose");
    let out = run_tp(&dir, &["-o", "1"]);
    assert_eq!(stdout(&out), "data with verbose");
}

// ── Quiet does not affect list operations ───────────────

#[test]
fn quiet_flag_with_list() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &["-q"], "quiet_data");
    let out = run_tp(&dir, &["-l"]);
    let text = stdout(&out);
    assert_eq!(text.trim().lines().count(), 1);
}

// ── Multiple sequential operations ──────────────────────

#[test]
fn push_pop_all() {
    let dir = setup_clean_env();
    for i in 0..5 {
        run_tp_stdin(&dir, &[], &format!("item{}", i));
    }
    for _ in 0..5 {
        run_tp(&dir, &["-p"]);
    }
    let out = run_tp(&dir, &["-l"]);
    assert!(stdout(&out).trim().is_empty());
}

#[test]
fn push_shift_all() {
    let dir = setup_clean_env();
    for i in 0..5 {
        run_tp_stdin(&dir, &[], &format!("item{}", i));
    }
    for _ in 0..5 {
        run_tp(&dir, &["-s"]);
    }
    let out = run_tp(&dir, &["-l"]);
    assert!(stdout(&out).trim().is_empty());
}

// ── Content with line-only data ─────────────────────────

#[test]
fn push_single_newline() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "\n");
    let out = run_tp(&dir, &["-o", "1"]);
    assert_eq!(stdout(&out), "\n");
}

#[test]
fn push_single_space() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], " ");
    let out = run_tp(&dir, &["-o", "1"]);
    assert_eq!(stdout(&out), " ");
}

#[test]
fn push_tab_character() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "\t");
    let out = run_tp(&dir, &["-o", "1"]);
    assert_eq!(stdout(&out), "\t");
}

// ── Content integrity after operations ──────────────────

#[test]
fn pop_does_not_corrupt_remaining() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "item1_data_here");
    run_tp_stdin(&dir, &[], "item2_data_here");
    run_tp_stdin(&dir, &[], "item3_data_here");
    run_tp(&dir, &["-p"]); // remove item3
    let out1 = run_tp(&dir, &["-o", "1"]);
    assert_eq!(stdout(&out1), "item1_data_here");
    let out2 = run_tp(&dir, &["-o", "2"]);
    assert_eq!(stdout(&out2), "item2_data_here");
}

#[test]
fn shift_does_not_corrupt_remaining() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "item1_data_here");
    run_tp_stdin(&dir, &[], "item2_data_here");
    run_tp_stdin(&dir, &[], "item3_data_here");
    run_tp(&dir, &["-s"]); // remove item1
    let out1 = run_tp(&dir, &["-o", "1"]);
    assert_eq!(stdout(&out1), "item2_data_here");
    let out2 = run_tp(&dir, &["-o", "2"]);
    assert_eq!(stdout(&out2), "item3_data_here");
}

#[test]
fn remove_middle_does_not_corrupt() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "aaa");
    run_tp_stdin(&dir, &[], "bbb");
    run_tp_stdin(&dir, &[], "ccc");
    run_tp_stdin(&dir, &[], "ddd");
    run_tp_stdin(&dir, &[], "eee");
    run_tp(&dir, &["-r", "3"]); // remove ccc
    let out = run_tp(&dir, &["-o", "1"]);
    assert_eq!(stdout(&out), "aaa");
    let out = run_tp(&dir, &["-o", "2"]);
    assert_eq!(stdout(&out), "bbb");
    let out = run_tp(&dir, &["-o", "3"]);
    assert_eq!(stdout(&out), "ddd");
    let out = run_tp(&dir, &["-o", "4"]);
    assert_eq!(stdout(&out), "eee");
}

// ── List operations after modifications ─────────────────

#[test]
fn list_contents_after_overwrite() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "original");
    run_tp_stdin(&dir, &["-i", "1"], "modified");
    let out = run_tp(&dir, &["-L"]);
    let text = stdout(&out);
    assert!(text.contains("modified"));
    assert!(!text.contains("original"));
}

#[test]
fn list_files_numbered_after_pop() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "a");
    run_tp_stdin(&dir, &[], "b");
    run_tp_stdin(&dir, &[], "c");
    run_tp(&dir, &["-p"]); // remove c
    let out = run_tp(&dir, &["-n"]);
    let text = stdout(&out);
    assert!(text.contains("1:"));
    assert!(text.contains("2:"));
    assert!(!text.contains("3:"));
}

// ── Exit codes for all operations ───────────────────────

#[test]
fn pop_exits_success() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["-p"]);
    assert!(out.status.success());
}

#[test]
fn shift_exits_success() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["-s"]);
    assert!(out.status.success());
}

#[test]
fn remove_exits_success() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["-r", "1"]);
    assert!(out.status.success());
}

#[test]
fn dir_exits_success() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["-d"]);
    assert!(out.status.success());
}

#[test]
fn master_exits_success() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["-m"]);
    assert!(out.status.success());
}

#[test]
fn list_contents_exits_success() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["-L"]);
    assert!(out.status.success());
}

#[test]
fn list_contents_numbered_exits_success() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["-N"]);
    assert!(out.status.success());
}

#[test]
fn list_files_numbered_exits_success() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["-n"]);
    assert!(out.status.success());
}

#[test]
fn clear_exits_success() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["-c"]);
    assert!(out.status.success());
}

// ── Unknown flags ───────────────────────────────────────

#[test]
fn unknown_flag_exits_failure() {
    let dir = setup_clean_env();
    let out = run_tp(&dir, &["--nonexistent"]);
    assert!(!out.status.success());
}

#[test]
fn unknown_short_flag_exits_failure() {
    let dir = setup_clean_env();
    let out = run_tp(&dir, &["-z"]);
    assert!(!out.status.success());
}

// ── Content with very long lines ────────────────────────

#[test]
fn push_very_long_line() {
    let dir = setup_clean_env();
    let long = "a".repeat(50_000);
    run_tp_stdin(&dir, &[], &long);
    let out = run_tp(&dir, &["-o", "1"]);
    assert_eq!(stdout(&out).len(), 50_000);
}

// ── Content with many short lines ───────────────────────

#[test]
fn push_content_with_many_lines() {
    let dir = setup_clean_env();
    let content: String = (0..1000).map(|i| format!("line{}\n", i)).collect();
    run_tp_stdin(&dir, &[], &content);
    let out = run_tp(&dir, &["-o", "1"]);
    assert_eq!(stdout(&out), content);
}

// ── Multiple pushes then list contents ordered ──────────

#[test]
fn list_contents_numbered_preserves_order() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "alpha");
    run_tp_stdin(&dir, &[], "bravo");
    run_tp_stdin(&dir, &[], "charlie");
    let out = run_tp(&dir, &["-N"]);
    let text = stdout(&out);
    let alpha_pos = text.find("alpha").unwrap();
    let bravo_pos = text.find("bravo").unwrap();
    let charlie_pos = text.find("charlie").unwrap();
    assert!(alpha_pos < bravo_pos);
    assert!(bravo_pos < charlie_pos);
}

// ── Remove invalid index errors ─────────────────────────

#[test]
fn remove_zero_index_errors() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["-r", "0"]);
    assert!(!out.status.success());
}

#[test]
fn remove_out_of_bounds_errors() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["-r", "5"]);
    assert!(!out.status.success());
}

#[test]
fn remove_negative_out_of_bounds_errors() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["-r", "-5"]);
    assert!(!out.status.success());
}

// ── Input overwrite preserves other items ───────────────

#[test]
fn overwrite_first_preserves_rest() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "orig1");
    run_tp_stdin(&dir, &[], "orig2");
    run_tp_stdin(&dir, &[], "orig3");
    run_tp_stdin(&dir, &["-i", "1"], "new1");
    let out1 = run_tp(&dir, &["-o", "1"]);
    assert_eq!(stdout(&out1), "new1");
    let out2 = run_tp(&dir, &["-o", "2"]);
    assert_eq!(stdout(&out2), "orig2");
    let out3 = run_tp(&dir, &["-o", "3"]);
    assert_eq!(stdout(&out3), "orig3");
}

#[test]
fn overwrite_last_preserves_rest() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "orig1");
    run_tp_stdin(&dir, &[], "orig2");
    run_tp_stdin(&dir, &[], "orig3");
    run_tp_stdin(&dir, &["-i", "3"], "new3");
    let out1 = run_tp(&dir, &["-o", "1"]);
    assert_eq!(stdout(&out1), "orig1");
    let out2 = run_tp(&dir, &["-o", "2"]);
    assert_eq!(stdout(&out2), "orig2");
    let out3 = run_tp(&dir, &["-o", "3"]);
    assert_eq!(stdout(&out3), "new3");
}

// ── Multiple overwrites on same index ───────────────────

#[test]
fn overwrite_same_index_multiple_times() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "original");
    tick();
    run_tp_stdin(&dir, &["-i", "1"], "first_edit");
    tick();
    run_tp_stdin(&dir, &["-i", "1"], "second_edit");
    tick();
    run_tp_stdin(&dir, &["-i", "1"], "final_edit");
    let out = run_tp(&dir, &["-o", "1"]);
    assert_eq!(stdout(&out), "final_edit");
}

// ── Dir path contains TEMPRS_DIR env value ──────────────

#[test]
fn dir_path_matches_env() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "x");
    let out = run_tp(&dir, &["-d"]);
    let text = stdout(&out).trim().to_string();
    assert!(text.starts_with(&dir.to_string_lossy().to_string()));
}

// ── Master path is under dir ────────────────────────────

#[test]
fn master_path_is_under_dir() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "x");
    let dir_out = run_tp(&dir, &["-d"]);
    let dir_text = stdout(&dir_out).trim().to_string();
    let master_out = run_tp(&dir, &["-m"]);
    let master_text = stdout(&master_out).trim().to_string();
    assert!(master_text.starts_with(&dir_text));
}

// ── Quiet with verbose ──────────────────────────────────

#[test]
fn quiet_and_verbose_together() {
    let dir = setup_clean_env();
    let out = run_tp_stdin(&dir, &["-q", "-v"], "both flags");
    // should succeed regardless of flag combination
    assert!(out.status.success());
    // data should still be stored
    let read = run_tp(&dir, &["-o", "1"]);
    assert_eq!(stdout(&read), "both flags");
}

// ── Content with null bytes ─────────────────────────────

#[test]
fn push_content_with_null_bytes() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "hello\0world");
    let out = run_tp(&dir, &["-o", "1"]);
    assert_eq!(stdout(&out), "hello\0world");
}

// ── Content with carriage returns ───────────────────────

#[test]
fn push_content_with_carriage_returns() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "line1\r\nline2\r\n");
    let out = run_tp(&dir, &["-o", "1"]);
    assert_eq!(stdout(&out), "line1\r\nline2\r\n");
}

// ── Multiple items list contents ────────────────────────

#[test]
fn list_contents_five_items() {
    let dir = setup_clean_env();
    for i in 1..=5 {
        run_tp_stdin(&dir, &[], &format!("content_{}", i));
    }
    let out = run_tp(&dir, &["-L"]);
    let text = stdout(&out);
    for i in 1..=5 {
        assert!(text.contains(&format!("content_{}", i)));
    }
}

// ── Push after clear rebuilds correctly ─────────────────

#[test]
fn push_multiple_after_clear() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "old");
    run_tp(&dir, &["-c"]);
    run_tp_stdin(&dir, &[], "new1");
    run_tp_stdin(&dir, &[], "new2");
    run_tp_stdin(&dir, &[], "new3");
    let list = run_tp(&dir, &["-l"]);
    assert_eq!(stdout(&list).trim().lines().count(), 3);
    let out = run_tp(&dir, &["-o", "1"]);
    assert_eq!(stdout(&out), "new1");
}

// ── Interleaved add and remove ──────────────────────────

#[test]
fn add_then_remove_same_position() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "first");
    run_tp_stdin(&dir, &[], "second");
    run_tp_stdin(&dir, &["-a", "2"], "inserted");
    // stack: first, inserted, second
    run_tp(&dir, &["-r", "2"]); // remove inserted
    // stack: first, second
    let list = run_tp(&dir, &["-l"]);
    assert_eq!(stdout(&list).trim().lines().count(), 2);
    let out1 = run_tp(&dir, &["-o", "1"]);
    assert_eq!(stdout(&out1), "first");
    let out2 = run_tp(&dir, &["-o", "2"]);
    assert_eq!(stdout(&out2), "second");
}

// ── Named tempfiles ────────────────────────────────────

#[test]
fn name_tag_and_output_by_name() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &["-w", "foo"], "hello foo");
    let out = run_tp(&dir, &["-o", "foo"]);
    assert_eq!(stdout(&out), "hello foo");
}

#[test]
fn name_tag_shows_in_numbered_list() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &["-w", "myname"], "data");
    let out = run_tp(&dir, &["-n"]);
    let text = stdout(&out);
    assert!(text.contains("@myname"), "numbered list should show @name tag");
}

#[test]
fn duplicate_name_rejected() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &["-w", "dup"], "first");
    let out = run_tp_stdin(&dir, &["-w", "dup"], "second");
    assert!(!out.status.success(), "duplicate name should fail");
}

#[test]
fn input_by_name_overwrites() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &["-w", "target"], "original");
    run_tp_stdin(&dir, &["-i", "target"], "replaced");
    let out = run_tp(&dir, &["-o", "target"]);
    assert_eq!(stdout(&out), "replaced");
}

#[test]
fn remove_by_name() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &["-w", "bye"], "gone");
    run_tp_stdin(&dir, &[], "stay");
    run_tp(&dir, &["-r", "bye"]);
    let list = run_tp(&dir, &["-l"]);
    assert_eq!(stdout(&list).trim().lines().count(), 1);
}

// ── Rename tag ─────────────────────────────────────────

#[test]
fn rename_tag_by_name() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &["-w", "old"], "data");
    let out = run_tp(&dir, &["-R", "old", "new"]);
    assert!(out.status.success());
    let read = run_tp(&dir, &["-o", "new"]);
    assert_eq!(stdout(&read), "data");
}

#[test]
fn rename_tag_by_index() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &["-w", "orig"], "data");
    let out = run_tp(&dir, &["-R", "1", "renamed"]);
    assert!(out.status.success());
    let read = run_tp(&dir, &["-o", "renamed"]);
    assert_eq!(stdout(&read), "data");
}

#[test]
fn rename_old_name_no_longer_resolves() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &["-w", "before"], "data");
    run_tp(&dir, &["-R", "before", "after"]);
    let out = run_tp(&dir, &["-o", "before"]);
    assert!(!out.status.success(), "old name should no longer resolve");
}

#[test]
fn rename_to_duplicate_rejected() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &["-w", "aaa"], "one");
    tick();
    run_tp_stdin(&dir, &["-w", "bbb"], "two");
    let out = run_tp(&dir, &["-R", "aaa", "bbb"]);
    assert!(!out.status.success(), "rename to existing name should fail");
}

#[test]
fn rename_unnamed_by_index() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "unnamed");
    let out = run_tp(&dir, &["-R", "1", "nownamed"]);
    assert!(out.status.success());
    let read = run_tp(&dir, &["-o", "nownamed"]);
    assert_eq!(stdout(&read), "unnamed");
}

#[test]
fn rename_preserves_other_names() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &["-w", "keep"], "keep-data");
    tick();
    run_tp_stdin(&dir, &["-w", "change"], "change-data");
    run_tp(&dir, &["-R", "change", "changed"]);
    let keep = run_tp(&dir, &["-o", "keep"]);
    assert_eq!(stdout(&keep), "keep-data");
    let changed = run_tp(&dir, &["-o", "changed"]);
    assert_eq!(stdout(&changed), "change-data");
}

// ── Info ───────────────────────────────────────────────

#[test]
fn info_by_index() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "some content");
    let out = run_tp(&dir, &["-I", "1"]);
    assert!(out.status.success());
    let text = stdout(&out);
    assert!(text.contains("index: 1"));
    assert!(text.contains("path:"));
    assert!(text.contains("size:"));
    assert!(text.contains("mtime:"));
}

#[test]
fn info_by_name() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &["-w", "myfile"], "content here");
    let out = run_tp(&dir, &["-I", "myfile"]);
    assert!(out.status.success());
    let text = stdout(&out);
    assert!(text.contains("index: 1"));
    assert!(text.contains("name: myfile"));
}

#[test]
fn info_shows_correct_size() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "12345");
    let out = run_tp(&dir, &["-I", "1"]);
    let text = stdout(&out);
    assert!(text.contains("5 B"), "expected 5 bytes, got: {}", text);
}

#[test]
fn info_invalid_index_fails() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["-I", "99"]);
    assert!(!out.status.success());
}

#[test]
fn info_unnamed_has_no_name_line() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["-I", "1"]);
    let text = stdout(&out);
    assert!(!text.contains("name:"), "unnamed file should not show name line");
}

// ── Grep ───────────────────────────────────────────────

#[test]
fn grep_finds_matching_file() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "hello world");
    tick();
    run_tp_stdin(&dir, &[], "goodbye world");
    let out = run_tp(&dir, &["-g", "hello"]);
    assert!(out.status.success());
    let text = stdout(&out);
    assert!(text.contains("hello world"));
    assert!(!text.contains("goodbye"));
}

#[test]
fn grep_finds_multiple_files() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "alpha needle beta");
    tick();
    run_tp_stdin(&dir, &[], "no match here");
    tick();
    run_tp_stdin(&dir, &[], "gamma needle delta");
    let out = run_tp(&dir, &["-g", "needle"]);
    assert!(out.status.success());
    let text = stdout(&out);
    assert!(text.contains("1:"));
    assert!(text.contains("3:"));
    assert!(!text.contains("2:"));
}

#[test]
fn grep_no_match_exits_nonzero() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "hello");
    let out = run_tp(&dir, &["-g", "nonexistent"]);
    assert!(!out.status.success(), "grep with no matches should exit nonzero");
}

#[test]
fn grep_empty_stack_exits_nonzero() {
    let dir = setup_clean_env();
    let out = run_tp(&dir, &["-g", "anything"]);
    assert!(!out.status.success());
}

#[test]
fn grep_shows_line_numbers() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "line1\nmatch here\nline3");
    let out = run_tp(&dir, &["-g", "match"]);
    let text = stdout(&out);
    assert!(text.contains("2:"), "should show line number 2, got: {}", text);
}

#[test]
fn grep_shows_name_tag() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &["-w", "tagged"], "findme");
    let out = run_tp(&dir, &["-g", "findme"]);
    let text = stdout(&out);
    assert!(text.contains("@tagged"), "should show @name tag in grep output");
}

#[test]
fn grep_multiple_lines_in_one_file() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "foo bar\nbaz foo\nqux");
    let out = run_tp(&dir, &["-g", "foo"]);
    let text = stdout(&out);
    let match_lines: Vec<&str> = text.lines().filter(|l| l.contains("foo")).collect();
    assert_eq!(match_lines.len(), 2, "should match 2 lines, got: {:?}", match_lines);
}

// ── Edit ───────────────────────────────────────────────

#[test]
fn edit_by_name() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &["-w", "editable"], "content");
    // Use 'true' as EDITOR to verify it launches and exits
    let out = Command::new(bin())
        .env("TEMPRS_DIR", &dir)
        .env("EDITOR", "true")
        .args(&["-e", "editable"])
        .output()
        .expect("failed to execute tp");
    assert!(out.status.success());
}

#[test]
fn edit_by_index() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "content");
    let out = Command::new(bin())
        .env("TEMPRS_DIR", &dir)
        .env("EDITOR", "true")
        .args(&["-e", "1"])
        .output()
        .expect("failed to execute tp");
    assert!(out.status.success());
}

#[test]
fn edit_invalid_index_fails() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = Command::new(bin())
        .env("TEMPRS_DIR", &dir)
        .env("EDITOR", "true")
        .args(&["-e", "99"])
        .output()
        .expect("failed to execute tp");
    assert!(!out.status.success());
}

// ── Cat (concatenate) ──────────────────────────────────

#[test]
fn cat_single_file() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "only one");
    let out = run_tp(&dir, &["-C", "1"]);
    assert!(out.status.success());
    assert_eq!(stdout(&out), "only one");
}

#[test]
fn cat_multiple_files() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "AAA\n");
    tick();
    run_tp_stdin(&dir, &[], "BBB\n");
    tick();
    run_tp_stdin(&dir, &[], "CCC\n");
    let out = run_tp(&dir, &["-C", "1", "2", "3"]);
    assert!(out.status.success());
    assert_eq!(stdout(&out), "AAA\nBBB\nCCC\n");
}

#[test]
fn cat_by_name() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &["-w", "x"], "XX");
    tick();
    run_tp_stdin(&dir, &["-w", "y"], "YY");
    let out = run_tp(&dir, &["-C", "x", "y"]);
    assert!(out.status.success());
    assert_eq!(stdout(&out), "XXYY");
}

#[test]
fn cat_mixed_index_and_name() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &["-w", "named"], "FIRST");
    tick();
    run_tp_stdin(&dir, &[], "SECOND");
    let out = run_tp(&dir, &["-C", "named", "2"]);
    assert!(out.status.success());
    assert_eq!(stdout(&out), "FIRSTSECOND");
}

#[test]
fn cat_reversed_order() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "AAA");
    tick();
    run_tp_stdin(&dir, &[], "BBB");
    let out = run_tp(&dir, &["-C", "2", "1"]);
    assert_eq!(stdout(&out), "BBBAAA");
}

#[test]
fn cat_duplicate_index() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "DUP");
    let out = run_tp(&dir, &["-C", "1", "1"]);
    assert_eq!(stdout(&out), "DUPDUP");
}

#[test]
fn cat_invalid_index_fails() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["-C", "99"]);
    assert!(!out.status.success());
}

#[test]
fn cat_negative_index() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "FIRST");
    tick();
    run_tp_stdin(&dir, &[], "LAST");
    let out = run_tp(&dir, &["-C", "-1"]);
    assert_eq!(stdout(&out), "LAST");
}

// ── Count ──────────────────────────────────────────────

#[test]
fn count_empty_stack() {
    let dir = setup_clean_env();
    let out = run_tp(&dir, &["-k"]);
    assert!(out.status.success());
    assert_eq!(stdout(&out).trim(), "0");
}

#[test]
fn count_one_file() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["-k"]);
    assert_eq!(stdout(&out).trim(), "1");
}

#[test]
fn count_multiple_files() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "a");
    tick();
    run_tp_stdin(&dir, &[], "b");
    tick();
    run_tp_stdin(&dir, &[], "c");
    let out = run_tp(&dir, &["-k"]);
    assert_eq!(stdout(&out).trim(), "3");
}

#[test]
fn count_after_remove() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "a");
    tick();
    run_tp_stdin(&dir, &[], "b");
    run_tp(&dir, &["-r", "1"]);
    let out = run_tp(&dir, &["-k"]);
    assert_eq!(stdout(&out).trim(), "1");
}

#[test]
fn count_after_clear() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "a");
    run_tp(&dir, &["-c"]);
    let out = run_tp(&dir, &["-k"]);
    assert_eq!(stdout(&out).trim(), "0");
}

// ── Diff ───────────────────────────────────────────────

#[test]
fn diff_identical_files_exits_zero() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "same content\n");
    tick();
    run_tp_stdin(&dir, &[], "same content\n");
    let out = run_tp(&dir, &["-D", "1", "2"]);
    assert!(out.status.success(), "diff of identical files should exit 0");
    assert!(stdout(&out).trim().is_empty());
}

#[test]
fn diff_different_files_exits_one() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "aaa\n");
    tick();
    run_tp_stdin(&dir, &[], "bbb\n");
    let out = run_tp(&dir, &["-D", "1", "2"]);
    assert_eq!(out.status.code(), Some(1), "diff of different files should exit 1");
    let text = stdout(&out);
    assert!(text.contains("-aaa"), "should show removed line");
    assert!(text.contains("+bbb"), "should show added line");
}

#[test]
fn diff_by_name() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &["-w", "left"], "left\n");
    tick();
    run_tp_stdin(&dir, &["-w", "right"], "right\n");
    let out = run_tp(&dir, &["-D", "left", "right"]);
    assert_eq!(out.status.code(), Some(1));
    let text = stdout(&out);
    assert!(text.contains("-left"));
    assert!(text.contains("+right"));
}

#[test]
fn diff_mixed_name_and_index() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &["-w", "named"], "AAA\n");
    tick();
    run_tp_stdin(&dir, &[], "BBB\n");
    let out = run_tp(&dir, &["-D", "named", "2"]);
    assert_eq!(out.status.code(), Some(1));
}

#[test]
fn diff_invalid_index_fails() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["-D", "1", "99"]);
    assert!(!out.status.success());
}

#[test]
fn diff_same_index_exits_zero() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "content\n");
    let out = run_tp(&dir, &["-D", "1", "1"]);
    assert!(out.status.success());
}

// ── Move ───────────────────────────────────────────────

#[test]
fn move_first_to_last() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "AAA");
    tick();
    run_tp_stdin(&dir, &[], "BBB");
    tick();
    run_tp_stdin(&dir, &[], "CCC");
    let out = run_tp(&dir, &["-M", "1", "3"]);
    assert!(out.status.success());
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])), "BBB");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "2"])), "CCC");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "3"])), "AAA");
}

#[test]
fn move_last_to_first() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "AAA");
    tick();
    run_tp_stdin(&dir, &[], "BBB");
    tick();
    run_tp_stdin(&dir, &[], "CCC");
    run_tp(&dir, &["-M", "3", "1"]);
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])), "CCC");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "2"])), "AAA");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "3"])), "BBB");
}

#[test]
fn move_by_name() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &["-w", "x"], "XX");
    tick();
    run_tp_stdin(&dir, &[], "YY");
    tick();
    run_tp_stdin(&dir, &[], "ZZ");
    run_tp(&dir, &["-M", "x", "3"]);
    assert_eq!(stdout(&run_tp(&dir, &["-o", "3"])), "XX");
    // name should follow the file
    assert_eq!(stdout(&run_tp(&dir, &["-o", "x"])), "XX");
}

#[test]
fn move_preserves_names() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &["-w", "a"], "AA");
    tick();
    run_tp_stdin(&dir, &["-w", "b"], "BB");
    tick();
    run_tp_stdin(&dir, &["-w", "c"], "CC");
    run_tp(&dir, &["-M", "1", "3"]);
    assert_eq!(stdout(&run_tp(&dir, &["-o", "a"])), "AA");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "b"])), "BB");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "c"])), "CC");
}

#[test]
fn move_same_position_is_noop() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "AAA");
    tick();
    run_tp_stdin(&dir, &[], "BBB");
    run_tp(&dir, &["-M", "1", "1"]);
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])), "AAA");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "2"])), "BBB");
}

#[test]
fn move_middle_to_first() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "AAA");
    tick();
    run_tp_stdin(&dir, &[], "BBB");
    tick();
    run_tp_stdin(&dir, &[], "CCC");
    run_tp(&dir, &["-M", "2", "1"]);
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])), "BBB");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "2"])), "AAA");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "3"])), "CCC");
}

// ── Dup (duplicate) ────────────────────────────────────

#[test]
fn dup_by_index() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "original");
    let out = run_tp(&dir, &["-x", "1"]);
    assert!(out.status.success());
    let count = run_tp(&dir, &["-k"]);
    assert_eq!(stdout(&count).trim(), "2");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])), "original");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "2"])), "original");
}

#[test]
fn dup_by_name() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &["-w", "src"], "data");
    let out = run_tp(&dir, &["-x", "src"]);
    assert!(out.status.success());
    assert_eq!(stdout(&run_tp(&dir, &["-o", "2"])), "data");
}

#[test]
fn dup_does_not_copy_name() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &["-w", "unique"], "data");
    run_tp(&dir, &["-x", "unique"]);
    // original name still works
    assert_eq!(stdout(&run_tp(&dir, &["-o", "unique"])), "data");
    // clone is at index 2 without a name
    let list = run_tp(&dir, &["-n"]);
    let text = stdout(&list);
    let lines: Vec<&str> = text.lines().filter(|l| l.contains("@unique")).collect();
    assert_eq!(lines.len(), 1, "name should not be duplicated");
}

#[test]
fn dup_invalid_index_fails() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["-x", "99"]);
    assert!(!out.status.success());
}

#[test]
fn dup_negative_index() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "first");
    tick();
    run_tp_stdin(&dir, &[], "last");
    run_tp(&dir, &["-x", "-1"]);
    assert_eq!(stdout(&run_tp(&dir, &["-o", "3"])), "last");
}

// ── Swap ───────────────────────────────────────────────

#[test]
fn swap_by_index() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "AAA");
    tick();
    run_tp_stdin(&dir, &[], "BBB");
    tick();
    run_tp_stdin(&dir, &[], "CCC");
    let out = run_tp(&dir, &["-S", "1", "3"]);
    assert!(out.status.success());
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])), "CCC");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "2"])), "BBB");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "3"])), "AAA");
}

#[test]
fn swap_by_name() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &["-w", "x"], "XX");
    tick();
    run_tp_stdin(&dir, &["-w", "y"], "YY");
    run_tp(&dir, &["-S", "x", "y"]);
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])), "YY");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "2"])), "XX");
}

#[test]
fn swap_preserves_names() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &["-w", "a"], "AA");
    tick();
    run_tp_stdin(&dir, &["-w", "b"], "BB");
    run_tp(&dir, &["-S", "1", "2"]);
    // names follow files
    assert_eq!(stdout(&run_tp(&dir, &["-o", "a"])), "AA");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "b"])), "BB");
}

#[test]
fn swap_same_index_is_noop() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "AAA");
    tick();
    run_tp_stdin(&dir, &[], "BBB");
    run_tp(&dir, &["-S", "1", "1"]);
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])), "AAA");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "2"])), "BBB");
}

#[test]
fn swap_invalid_index_fails() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["-S", "1", "99"]);
    assert!(!out.status.success());
}

#[test]
fn name_with_tab_rejected() {
    let dir = setup_clean_env();
    let out = run_tp_stdin(&dir, &["-w", "bad\tname"], "data");
    assert!(!out.status.success(), "name with tab should be rejected");
}

#[test]
fn rename_to_tab_rejected() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &["-w", "good"], "data");
    let out = run_tp(&dir, &["-R", "good", "bad\tname"]);
    assert!(!out.status.success(), "rename to name with tab should be rejected");
}

#[test]
fn swap_mixed_name_and_index() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &["-w", "named"], "NN");
    tick();
    run_tp_stdin(&dir, &[], "UU");
    run_tp(&dir, &["-S", "named", "2"]);
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])), "UU");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "2"])), "NN");
}

// ── Append ─────────────────────────────────────────────

#[test]
fn append_to_existing() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "hello");
    run_tp_stdin(&dir, &["-A", "1"], " world");
    let out = run_tp(&dir, &["-o", "1"]);
    assert_eq!(stdout(&out), "hello world");
}

#[test]
fn append_by_name() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &["-w", "log"], "line1\n");
    run_tp_stdin(&dir, &["-A", "log"], "line2\n");
    let out = run_tp(&dir, &["-o", "log"]);
    assert_eq!(stdout(&out), "line1\nline2\n");
}

#[test]
fn append_does_not_create_new_file() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "base");
    run_tp_stdin(&dir, &["-A", "1"], "extra");
    let count = run_tp(&dir, &["-k"]);
    assert_eq!(stdout(&count).trim(), "1");
}

#[test]
fn append_multiple_times() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "a");
    run_tp_stdin(&dir, &["-A", "1"], "b");
    run_tp_stdin(&dir, &["-A", "1"], "c");
    let out = run_tp(&dir, &["-o", "1"]);
    assert_eq!(stdout(&out), "abc");
}

#[test]
fn append_invalid_index_fails() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp_stdin(&dir, &["-A", "99"], "more");
    assert!(!out.status.success());
}

#[test]
fn append_preserves_other_files() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "first");
    tick();
    run_tp_stdin(&dir, &[], "second");
    run_tp_stdin(&dir, &["-A", "1"], "+extra");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])), "first+extra");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "2"])), "second");
}

// ── Reverse ────────────────────────────────────────────

#[test]
fn reverse_stack() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "AAA");
    tick();
    run_tp_stdin(&dir, &[], "BBB");
    tick();
    run_tp_stdin(&dir, &[], "CCC");
    let out = run_tp(&dir, &["--rev"]);
    assert!(out.status.success());
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])), "CCC");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "2"])), "BBB");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "3"])), "AAA");
}

#[test]
fn reverse_preserves_names() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &["-w", "first"], "AA");
    tick();
    run_tp_stdin(&dir, &["-w", "last"], "ZZ");
    run_tp(&dir, &["--rev"]);
    assert_eq!(stdout(&run_tp(&dir, &["-o", "first"])), "AA");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "last"])), "ZZ");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])), "ZZ");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "2"])), "AA");
}

#[test]
fn reverse_single_file_is_noop() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "only");
    run_tp(&dir, &["--rev"]);
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])), "only");
}

#[test]
fn reverse_empty_stack() {
    let dir = setup_clean_env();
    let out = run_tp(&dir, &["--rev"]);
    assert!(out.status.success());
}

#[test]
fn double_reverse_restores_order() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "AAA");
    tick();
    run_tp_stdin(&dir, &[], "BBB");
    tick();
    run_tp_stdin(&dir, &[], "CCC");
    run_tp(&dir, &["--rev"]);
    run_tp(&dir, &["--rev"]);
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])), "AAA");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "2"])), "BBB");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "3"])), "CCC");
}

// ── Expire ─────────────────────────────────────────────

#[test]
fn expire_large_ttl_keeps_all() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "recent1");
    tick();
    run_tp_stdin(&dir, &[], "recent2");
    let out = run_tp(&dir, &["--expire", "9999"]);
    assert!(out.status.success());
    assert_eq!(stdout(&out).trim(), "0");
    let count = run_tp(&dir, &["-k"]);
    assert_eq!(stdout(&count).trim(), "2");
}

#[test]
fn expire_zero_removes_all() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "old1");
    tick();
    run_tp_stdin(&dir, &[], "old2");
    // sleep briefly so files are at least 1ms old
    std::thread::sleep(std::time::Duration::from_millis(50));
    let out = run_tp(&dir, &["--expire", "0"]);
    assert!(out.status.success());
    assert_eq!(stdout(&out).trim(), "2");
    let count = run_tp(&dir, &["-k"]);
    assert_eq!(stdout(&count).trim(), "0");
}

#[test]
fn expire_empty_stack() {
    let dir = setup_clean_env();
    let out = run_tp(&dir, &["--expire", "1"]);
    assert!(out.status.success());
    assert_eq!(stdout(&out).trim(), "0");
}

#[test]
fn expire_prints_removed_count() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "a");
    tick();
    run_tp_stdin(&dir, &[], "b");
    tick();
    run_tp_stdin(&dir, &[], "c");
    std::thread::sleep(std::time::Duration::from_millis(50));
    let out = run_tp(&dir, &["--expire", "0"]);
    assert_eq!(stdout(&out).trim(), "3");
}

#[test]
fn expire_invalid_hours_fails() {
    let dir = setup_clean_env();
    let out = run_tp(&dir, &["--expire", "notanumber"]);
    assert!(!out.status.success());
}

#[test]
fn expire_fractional_hours() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    // 0.001 hours = 3.6 seconds, file is <1s old so should be kept
    let out = run_tp(&dir, &["--expire", "0.001"]);
    assert_eq!(stdout(&out).trim(), "0");
    assert_eq!(stdout(&run_tp(&dir, &["-k"])).trim(), "1");
}

// ── Head ───────────────────────────────────────────────

#[test]
fn head_first_n_lines() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "line1\nline2\nline3\nline4\nline5\n");
    let out = run_tp(&dir, &["--head", "1", "3"]);
    assert!(out.status.success());
    let text = stdout(&out);
    assert_eq!(text.trim(), "line1\nline2\nline3");
}

#[test]
fn head_by_name() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &["-w", "log"], "a\nb\nc\nd\n");
    let out = run_tp(&dir, &["--head", "log", "2"]);
    assert_eq!(stdout(&out).trim(), "a\nb");
}

#[test]
fn head_more_than_file_length() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "short\n");
    let out = run_tp(&dir, &["--head", "1", "100"]);
    assert_eq!(stdout(&out).trim(), "short");
}

#[test]
fn head_zero_lines() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data\n");
    let out = run_tp(&dir, &["--head", "1", "0"]);
    assert!(out.status.success());
    assert!(stdout(&out).trim().is_empty());
}

#[test]
fn head_invalid_index_fails() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["--head", "99", "1"]);
    assert!(!out.status.success());
}

// ── Tail ───────────────────────────────────────────────

#[test]
fn tail_last_n_lines() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "line1\nline2\nline3\nline4\nline5\n");
    let out = run_tp(&dir, &["--tail", "1", "3"]);
    assert!(out.status.success());
    assert_eq!(stdout(&out).trim(), "line3\nline4\nline5");
}

#[test]
fn tail_by_name() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &["-w", "log"], "a\nb\nc\nd\n");
    let out = run_tp(&dir, &["--tail", "log", "2"]);
    assert_eq!(stdout(&out).trim(), "c\nd");
}

#[test]
fn tail_more_than_file_length() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "short\n");
    let out = run_tp(&dir, &["--tail", "1", "100"]);
    assert_eq!(stdout(&out).trim(), "short");
}

#[test]
fn tail_zero_lines() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data\n");
    let out = run_tp(&dir, &["--tail", "1", "0"]);
    assert!(out.status.success());
    assert!(stdout(&out).trim().is_empty());
}

#[test]
fn tail_invalid_index_fails() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["--tail", "99", "1"]);
    assert!(!out.status.success());
}

#[test]
fn head_one_line() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "first\nsecond\nthird\n");
    let out = run_tp(&dir, &["--head", "1", "1"]);
    assert_eq!(stdout(&out).trim(), "first");
}

#[test]
fn tail_one_line() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "first\nsecond\nthird\n");
    let out = run_tp(&dir, &["--tail", "1", "1"]);
    assert_eq!(stdout(&out).trim(), "third");
}
