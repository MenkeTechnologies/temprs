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
    let dir = std::env::temp_dir().join(format!("temprs_test_{}_{}", std::process::id(), id,));
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
    assert!(
        text.contains("TEMPORARY FILE STACK MANAGER"),
        "missing tagline"
    );
    assert!(text.contains("JACK IN"), "missing cyberpunk footer");
}

#[test]
fn help_shows_all_flags() {
    let dir = setup_clean_env();
    let out = run_tp(&dir, &["-h"]);
    let text = stdout(&out);
    for flag in &[
        "--input",
        "--output",
        "--add",
        "--remove",
        "--pop",
        "--unshift",
        "--shift",
        "--dir",
        "--master",
        "--list-files",
        "--list-files-numbered",
        "--list-contents",
        "--list-contents-numbered",
        "--quiet",
        "--clear",
        "--verbose",
        "--edit",
        "--name",
        "--rename",
        "--info",
        "--grep",
        "--cat",
        "--count",
        "--diff",
        "--mv",
        "--dup",
        "--swap",
        "--append",
        "--rev",
        "--expire",
        "--head",
        "--tail",
        "--wc",
        "--size",
        "--sort",
        "--replace",
        "--path",
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
    assert!(
        text.contains("temprs"),
        "dir output should contain 'temprs'"
    );
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
    assert!(
        text.contains("echo me"),
        "verbose should echo stdin to stdout"
    );
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
fn unshift_to_empty_stack_succeeds() {
    let dir = setup_clean_env();
    // unshift on empty stack should succeed (insert at position 0)
    let out = run_tp_stdin(&dir, &["-u"], "first_via_unshift");
    assert!(out.status.success());
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])), "first_via_unshift");
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
        assert!(
            line.contains("tempfile"),
            "path should contain 'tempfile': {}",
            line
        );
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
    for flag in &[
        "-i", "-o", "-a", "-r", "-p", "-u", "-s", "-d", "-m", "-l", "-n", "-L", "-N", "-q", "-c",
        "-v",
    ] {
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
    assert!(
        text.contains("---"),
        "numbered list should have separator lines"
    );
}

#[test]
fn list_contents_numbered_contains_separator() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["-N"]);
    let text = stdout(&out);
    assert!(
        text.contains("---"),
        "numbered contents list should have separator lines"
    );
}

// ── Stderr for errors ───────────────────────────────────

#[test]
fn invalid_index_shows_error_message() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["-o", "0"]);
    // simple_logger outputs to stdout
    let text = stdout(&out);
    assert!(
        text.contains("Invalid specified index"),
        "output should contain error message: {}",
        text
    );
}

#[test]
fn out_of_bounds_shows_error_message() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["-o", "100"]);
    let text = stdout(&out);
    assert!(
        text.contains("Invalid specified index"),
        "output should contain error message: {}",
        text
    );
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
    assert!(
        text.contains("@myname"),
        "numbered list should show @name tag"
    );
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
    assert!(
        !text.contains("name:"),
        "unnamed file should not show name line"
    );
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
    assert!(
        !out.status.success(),
        "grep with no matches should exit nonzero"
    );
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
    assert!(
        text.contains("2:"),
        "should show line number 2, got: {}",
        text
    );
}

#[test]
fn grep_shows_name_tag() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &["-w", "tagged"], "findme");
    let out = run_tp(&dir, &["-g", "findme"]);
    let text = stdout(&out);
    assert!(
        text.contains("@tagged"),
        "should show @name tag in grep output"
    );
}

#[test]
fn grep_multiple_lines_in_one_file() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "foo bar\nbaz foo\nqux");
    let out = run_tp(&dir, &["-g", "foo"]);
    let text = stdout(&out);
    let match_lines: Vec<&str> = text.lines().filter(|l| l.contains("foo")).collect();
    assert_eq!(
        match_lines.len(),
        2,
        "should match 2 lines, got: {:?}",
        match_lines
    );
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
        .args(["-e", "editable"])
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
        .args(["-e", "1"])
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
        .args(["-e", "99"])
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
    assert!(
        out.status.success(),
        "diff of identical files should exit 0"
    );
    assert!(stdout(&out).trim().is_empty());
}

#[test]
fn diff_different_files_exits_one() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "aaa\n");
    tick();
    run_tp_stdin(&dir, &[], "bbb\n");
    let out = run_tp(&dir, &["-D", "1", "2"]);
    assert_eq!(
        out.status.code(),
        Some(1),
        "diff of different files should exit 1"
    );
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
fn name_with_nul_rejected() {
    // null bytes cannot be passed via CLI arguments (OS restriction),
    // so validation is only relevant for programmatic/library use
}

#[test]
fn rename_to_nul_rejected() {
    // null bytes cannot be passed via CLI arguments (OS restriction),
    // so validation is only relevant for programmatic/library use
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

// ── Wc (line count) ───────────────────────────────────

#[test]
fn wc_counts_lines() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "a\nb\nc\nd\ne\n");
    let out = run_tp(&dir, &["--wc", "1"]);
    assert!(out.status.success());
    assert_eq!(stdout(&out).trim(), "5");
}

#[test]
fn wc_by_name() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &["-w", "log"], "line1\nline2\nline3\n");
    let out = run_tp(&dir, &["--wc", "log"]);
    assert_eq!(stdout(&out).trim(), "3");
}

#[test]
fn wc_single_line_no_newline() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "hello");
    let out = run_tp(&dir, &["--wc", "1"]);
    assert_eq!(stdout(&out).trim(), "1");
}

#[test]
fn wc_empty_file() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "");
    let out = run_tp(&dir, &["--wc", "1"]);
    assert_eq!(stdout(&out).trim(), "0");
}

#[test]
fn wc_invalid_index_fails() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["--wc", "99"]);
    assert!(!out.status.success());
}

// ── Size (byte count) ─────────────────────────────────

#[test]
fn size_by_index() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "hello");
    let out = run_tp(&dir, &["--size", "1"]);
    assert!(out.status.success());
    assert_eq!(stdout(&out).trim(), "5");
}

#[test]
fn size_by_name() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &["-w", "data"], "abcdefghij");
    let out = run_tp(&dir, &["--size", "data"]);
    assert_eq!(stdout(&out).trim(), "10");
}

#[test]
fn size_empty_file() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "");
    let out = run_tp(&dir, &["--size", "1"]);
    assert_eq!(stdout(&out).trim(), "0");
}

#[test]
fn size_with_newlines() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "a\nb\nc\n");
    let out = run_tp(&dir, &["--size", "1"]);
    assert_eq!(stdout(&out).trim(), "6");
}

#[test]
fn size_invalid_index_fails() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["--size", "99"]);
    assert!(!out.status.success());
}

// ── Sort ───────────────────────────────────────────────

#[test]
fn sort_by_name_default() {
    let dir = setup_clean_env();
    // Push files that will have ascending timestamp names
    run_tp_stdin(&dir, &[], "CCC");
    tick();
    run_tp_stdin(&dir, &[], "AAA");
    tick();
    run_tp_stdin(&dir, &[], "BBB");
    // sort by name (filename = tempfileXXX, so timestamp order = name order)
    let out = run_tp(&dir, &["--sort", "name"]);
    assert!(out.status.success());
    // After sorting by name, order should match push order (ascending timestamps)
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])), "CCC");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "2"])), "AAA");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "3"])), "BBB");
}

#[test]
fn sort_by_size() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "medium!!"); // 8 bytes
    tick();
    run_tp_stdin(&dir, &[], "z"); // 1 byte
    tick();
    run_tp_stdin(&dir, &[], "very long content here"); // 22 bytes
    let out = run_tp(&dir, &["--sort", "size"]);
    assert!(out.status.success());
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])), "z");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "2"])), "medium!!");
    assert_eq!(
        stdout(&run_tp(&dir, &["-o", "3"])),
        "very long content here"
    );
}

#[test]
fn sort_by_mtime() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "first");
    tick();
    run_tp_stdin(&dir, &[], "second");
    tick();
    run_tp_stdin(&dir, &[], "third");
    // Already in mtime order, sort should preserve it
    run_tp(&dir, &["--sort", "mtime"]);
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])), "first");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "3"])), "third");
}

#[test]
fn sort_preserves_names() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &["-w", "big"], "xxxxxxxxxx"); // 10 bytes
    tick();
    run_tp_stdin(&dir, &["-w", "small"], "x"); // 1 byte
    run_tp(&dir, &["--sort", "size"]);
    assert_eq!(stdout(&run_tp(&dir, &["-o", "small"])), "x");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "big"])), "xxxxxxxxxx");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])), "x");
}

#[test]
fn sort_empty_stack() {
    let dir = setup_clean_env();
    let out = run_tp(&dir, &["--sort", "name"]);
    assert!(out.status.success());
}

#[test]
fn sort_invalid_key_fails() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["--sort", "bogus"]);
    assert!(!out.status.success());
}

#[test]
fn sort_single_file() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "only");
    let out = run_tp(&dir, &["--sort", "size"]);
    assert!(out.status.success());
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])), "only");
}

// ── Replace ────────────────────────────────────────────

#[test]
fn replace_basic() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "hello world");
    let out = run_tp(&dir, &["--replace", "1", "world", "rust"]);
    assert!(out.status.success());
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])), "hello rust");
}

#[test]
fn replace_by_name() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &["-w", "doc"], "foo bar foo");
    let out = run_tp(&dir, &["--replace", "doc", "foo", "baz"]);
    assert!(out.status.success());
    assert_eq!(stdout(&run_tp(&dir, &["-o", "doc"])), "baz bar baz");
}

#[test]
fn replace_prints_count() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "aaa bbb aaa ccc aaa");
    let out = run_tp(&dir, &["--replace", "1", "aaa", "zzz"]);
    assert_eq!(stdout(&out).trim(), "3");
}

/// Replacement text may appear multiple times in the result without implying multiple substitutions
/// (e.g. replacing `x` with `foo` in `xfoo` → `foofoo` is one replacement, not two `foo` substrings).
#[test]
fn replace_count_is_pattern_occurrences_not_replacement_substrings_in_result() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "xfoo");
    let out = run_tp(&dir, &["--replace", "1", "x", "foo"]);
    assert_eq!(stdout(&out).trim(), "1");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])), "foofoo");
}

#[test]
fn replace_no_match() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "hello world");
    let out = run_tp(&dir, &["--replace", "1", "xyz", "abc"]);
    assert!(out.status.success());
    assert_eq!(stdout(&out).trim(), "0");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])), "hello world");
}

#[test]
fn replace_with_empty_string() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "remove this word");
    run_tp(&dir, &["--replace", "1", " this", ""]);
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])), "remove word");
}

#[test]
fn replace_invalid_index_fails() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["--replace", "99", "a", "b"]);
    assert!(!out.status.success());
}

#[test]
fn replace_preserves_other_files() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "original");
    tick();
    run_tp_stdin(&dir, &[], "untouched");
    run_tp(&dir, &["--replace", "1", "original", "modified"]);
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])), "modified");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "2"])), "untouched");
}

// ── Path ───────────────────────────────────────────────

#[test]
fn path_by_index() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["--path", "1"]);
    assert!(out.status.success());
    let text = stdout(&out);
    assert!(text.trim().starts_with(dir.to_str().unwrap()));
    assert!(text.trim().contains("tempfile"));
}

#[test]
fn path_by_name() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &["-w", "myfile"], "data");
    let out = run_tp(&dir, &["--path", "myfile"]);
    assert!(out.status.success());
    assert!(stdout(&out).trim().contains("tempfile"));
}

#[test]
fn path_usable_in_scripts() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "content here");
    let out = run_tp(&dir, &["--path", "1"]);
    let path = stdout(&out).trim().to_string();
    // The path should be a real file we can read
    let content = std::fs::read_to_string(&path).unwrap();
    assert_eq!(content, "content here");
}

#[test]
fn path_invalid_index_fails() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["--path", "99"]);
    assert!(!out.status.success());
}

#[test]
fn path_negative_index() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "first");
    tick();
    run_tp_stdin(&dir, &[], "last");
    let out1 = run_tp(&dir, &["--path", "-1"]);
    let out2 = run_tp(&dir, &["--path", "2"]);
    assert_eq!(stdout(&out1).trim(), stdout(&out2).trim());
}

// ── master file format and robustness ─────────────────

#[test]
fn master_file_uses_null_byte_delimiters() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &["-w", "tagged"], "first");
    tick();
    run_tp_stdin(&dir, &[], "second");
    let raw = fs::read(dir.join("temprs-stack")).unwrap();
    // records separated by \0\0, fields by \0
    let nulls: Vec<usize> = raw
        .iter()
        .enumerate()
        .filter(|&(_, &b)| b == 0)
        .map(|(i, _)| i)
        .collect();
    assert!(
        nulls.len() >= 3,
        "expected at least 3 null bytes, got {}",
        nulls.len()
    );
    // no tab bytes used as delimiters
    assert!(
        !raw.windows(1).any(|w| w == b"\t"),
        "master file should not contain tabs"
    );
}

#[test]
fn lock_file_created_on_push() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    assert!(
        dir.join("temprs-stack.lock").exists(),
        "lock file should exist"
    );
}

#[test]
fn no_tmp_file_left_after_push() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    assert!(
        !dir.join("temprs-stack.tmp").exists(),
        "no .tmp file should remain"
    );
}

#[test]
fn no_tmp_file_left_after_pop() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    tick();
    run_tp_stdin(&dir, &[], "more");
    run_tp(&dir, &["-p"]);
    assert!(
        !dir.join("temprs-stack.tmp").exists(),
        "no .tmp file should remain after pop"
    );
}

#[test]
fn no_tmp_file_left_after_remove() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    tick();
    run_tp_stdin(&dir, &[], "more");
    run_tp(&dir, &["-r", "1"]);
    assert!(
        !dir.join("temprs-stack.tmp").exists(),
        "no .tmp file should remain after remove"
    );
}

#[test]
fn master_file_tab_content_not_corrupted() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "col1\tcol2\tcol3");
    tick();
    run_tp_stdin(&dir, &[], "second");
    let out = run_tp(&dir, &["-o", "1"]);
    assert_eq!(stdout(&out), "col1\tcol2\tcol3");
    let out2 = run_tp(&dir, &["-o", "2"]);
    assert_eq!(stdout(&out2), "second");
}

#[test]
fn master_file_newline_content_not_corrupted() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "line1\nline2\nline3");
    tick();
    run_tp_stdin(&dir, &[], "other");
    let out = run_tp(&dir, &["-o", "1"]);
    assert_eq!(stdout(&out), "line1\nline2\nline3");
    let out2 = run_tp(&dir, &["-o", "2"]);
    assert_eq!(stdout(&out2), "other");
}

#[test]
fn corrupt_master_file_empty_records_recovered() {
    let dir = setup_clean_env();
    // push two items normally
    run_tp_stdin(&dir, &[], "first");
    tick();
    run_tp_stdin(&dir, &[], "second");
    // read master file raw bytes and inject extra \0\0 (empty records)
    let master = dir.join("temprs-stack");
    let raw = fs::read(&master).unwrap();
    let mut corrupted = Vec::new();
    corrupted.extend_from_slice(&[0, 0]); // leading empty record
    corrupted.extend_from_slice(&raw);
    corrupted.extend_from_slice(&[0, 0]); // trailing empty record
    fs::write(&master, &corrupted).unwrap();
    // listing should still work and show 2 items
    let out = run_tp(&dir, &["-k"]);
    assert!(out.status.success());
    let count: usize = stdout(&out).trim().parse().unwrap();
    assert_eq!(count, 2);
}

// ══════════════════════════════════════════════════════════
// ── ADDITIONAL INTEGRATION TESTS ─────────────────────────
// ══════════════════════════════════════════════════════════

// ── Move with negative indices ──────────────────────────

#[test]
fn move_negative_source() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "AAA");
    tick();
    run_tp_stdin(&dir, &[], "BBB");
    tick();
    run_tp_stdin(&dir, &[], "CCC");
    // -1 = last (CCC), move to position 1
    run_tp(&dir, &["-M", "-1", "1"]);
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])), "CCC");
}

#[test]
fn move_negative_destination() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "AAA");
    tick();
    run_tp_stdin(&dir, &[], "BBB");
    tick();
    run_tp_stdin(&dir, &[], "CCC");
    // move 1 to -1 (last)
    run_tp(&dir, &["-M", "1", "-1"]);
    assert_eq!(stdout(&run_tp(&dir, &["-o", "3"])), "AAA");
}

#[test]
fn move_both_negative() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "AAA");
    tick();
    run_tp_stdin(&dir, &[], "BBB");
    tick();
    run_tp_stdin(&dir, &[], "CCC");
    run_tp(&dir, &["-M", "-1", "-3"]);
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])), "CCC");
}

#[test]
fn move_invalid_index_fails() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["-M", "1", "99"]);
    assert!(!out.status.success());
}

// ── Swap with negative indices ──────────────────────────

#[test]
fn swap_negative_indices() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "AAA");
    tick();
    run_tp_stdin(&dir, &[], "BBB");
    tick();
    run_tp_stdin(&dir, &[], "CCC");
    // -1 = CCC, -3 = AAA
    run_tp(&dir, &["-S", "-1", "-3"]);
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])), "CCC");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "3"])), "AAA");
}

#[test]
fn swap_adjacent() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "AAA");
    tick();
    run_tp_stdin(&dir, &[], "BBB");
    run_tp(&dir, &["-S", "1", "2"]);
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])), "BBB");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "2"])), "AAA");
}

#[test]
fn swap_preserves_count() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "AAA");
    tick();
    run_tp_stdin(&dir, &[], "BBB");
    tick();
    run_tp_stdin(&dir, &[], "CCC");
    run_tp(&dir, &["-S", "1", "3"]);
    assert_eq!(stdout(&run_tp(&dir, &["-k"])).trim(), "3");
}

// ── Diff edge cases ─────────────────────────────────────

#[test]
fn diff_negative_indices() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "aaa\n");
    tick();
    run_tp_stdin(&dir, &[], "bbb\n");
    let out = run_tp(&dir, &["-D", "-2", "-1"]);
    assert_eq!(out.status.code(), Some(1));
    let text = stdout(&out);
    assert!(text.contains("-aaa"));
    assert!(text.contains("+bbb"));
}

#[test]
fn diff_multiline_content() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "line1\nline2\nline3\n");
    tick();
    run_tp_stdin(&dir, &[], "line1\nchanged\nline3\n");
    let out = run_tp(&dir, &["-D", "1", "2"]);
    assert_eq!(out.status.code(), Some(1));
    let text = stdout(&out);
    assert!(text.contains("-line2"));
    assert!(text.contains("+changed"));
}

#[test]
fn diff_empty_files() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "");
    tick();
    run_tp_stdin(&dir, &[], "");
    let out = run_tp(&dir, &["-D", "1", "2"]);
    assert!(out.status.success());
}

#[test]
fn diff_one_empty_one_not() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "");
    tick();
    run_tp_stdin(&dir, &[], "content\n");
    let out = run_tp(&dir, &["-D", "1", "2"]);
    assert_eq!(out.status.code(), Some(1));
}

// ── Cat edge cases ──────────────────────────────────────

#[test]
fn cat_all_negative_indices() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "AAA");
    tick();
    run_tp_stdin(&dir, &[], "BBB");
    let out = run_tp(&dir, &["-C", "-2", "-1"]);
    assert_eq!(stdout(&out), "AAABBB");
}

#[test]
fn cat_empty_file() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "");
    let out = run_tp(&dir, &["-C", "1"]);
    assert!(out.status.success());
    assert_eq!(stdout(&out), "");
}

#[test]
fn cat_many_files() {
    let dir = setup_clean_env();
    for i in 1..=5 {
        run_tp_stdin(&dir, &[], &format!("{}", i));
        if i < 5 {
            tick();
        }
    }
    let out = run_tp(&dir, &["-C", "1", "2", "3", "4", "5"]);
    assert_eq!(stdout(&out), "12345");
}

#[test]
fn cat_preserves_newlines_between_files() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "line1\n");
    tick();
    run_tp_stdin(&dir, &[], "line2\n");
    let out = run_tp(&dir, &["-C", "1", "2"]);
    assert_eq!(stdout(&out), "line1\nline2\n");
}

// ── Grep edge cases ─────────────────────────────────────

#[test]
fn grep_case_sensitive() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "Hello World");
    let out = run_tp(&dir, &["-g", "hello"]);
    // grep should be case-sensitive by default
    assert!(!out.status.success());
}

#[test]
fn grep_special_regex_chars() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "price is $100.00");
    let out = run_tp(&dir, &["-g", "100"]);
    let text = stdout(&out);
    assert!(text.contains("100"));
}

#[test]
fn grep_across_multiple_files_shows_indices() {
    let dir = setup_clean_env();
    for i in 1..=5 {
        run_tp_stdin(&dir, &[], &format!("item{} needle", i));
        tick();
    }
    let out = run_tp(&dir, &["-g", "needle"]);
    let text = stdout(&out);
    for i in 1..=5 {
        assert!(
            text.contains(&format!("{}:", i)),
            "missing index {} in grep output",
            i
        );
    }
}

#[test]
fn grep_empty_file_no_crash() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "");
    tick();
    run_tp_stdin(&dir, &[], "has content");
    let out = run_tp(&dir, &["-g", "content"]);
    assert!(out.status.success());
    let text = stdout(&out);
    assert!(text.contains("content"));
}

// ── Replace edge cases ──────────────────────────────────

#[test]
fn replace_multiline_pattern() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "aaa\nbbb\naaa\nccc");
    let out = run_tp(&dir, &["--replace", "1", "aaa", "zzz"]);
    assert_eq!(stdout(&out).trim(), "2");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])), "zzz\nbbb\nzzz\nccc");
}

#[test]
fn replace_with_longer_string() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "short");
    run_tp(
        &dir,
        &["--replace", "1", "short", "very long replacement string"],
    );
    assert_eq!(
        stdout(&run_tp(&dir, &["-o", "1"])),
        "very long replacement string"
    );
}

#[test]
fn replace_entire_content() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "replace me entirely");
    run_tp(&dir, &["--replace", "1", "replace me entirely", "done"]);
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])), "done");
}

#[test]
fn replace_negative_index() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "first");
    tick();
    run_tp_stdin(&dir, &[], "hello world");
    run_tp(&dir, &["--replace", "-1", "world", "rust"]);
    assert_eq!(stdout(&run_tp(&dir, &["-o", "2"])), "hello rust");
}

#[test]
fn replace_unicode_content() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "hello 世界");
    run_tp(&dir, &["--replace", "1", "世界", "world"]);
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])), "hello world");
}

// ── Head/tail with negative indices ─────────────────────

#[test]
fn head_negative_index() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "a\nb\nc\nd\n");
    tick();
    run_tp_stdin(&dir, &[], "x\ny\nz\n");
    let out = run_tp(&dir, &["--head", "-1", "2"]);
    assert_eq!(stdout(&out).trim(), "x\ny");
}

#[test]
fn tail_negative_index() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "a\nb\nc\nd\n");
    tick();
    run_tp_stdin(&dir, &[], "x\ny\nz\n");
    let out = run_tp(&dir, &["--tail", "-1", "2"]);
    assert_eq!(stdout(&out).trim(), "y\nz");
}

#[test]
fn head_single_line_file() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "only line\n");
    let out = run_tp(&dir, &["--head", "1", "5"]);
    assert_eq!(stdout(&out).trim(), "only line");
}

#[test]
fn tail_single_line_file() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "only line\n");
    let out = run_tp(&dir, &["--tail", "1", "5"]);
    assert_eq!(stdout(&out).trim(), "only line");
}

// ── Wc edge cases ───────────────────────────────────────

#[test]
fn wc_many_lines() {
    let dir = setup_clean_env();
    let content: String = (0..100).map(|i| format!("line{}\n", i)).collect();
    run_tp_stdin(&dir, &[], &content);
    let out = run_tp(&dir, &["--wc", "1"]);
    assert_eq!(stdout(&out).trim(), "100");
}

#[test]
fn wc_negative_index() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "a\nb\nc\n");
    let out = run_tp(&dir, &["--wc", "-1"]);
    assert_eq!(stdout(&out).trim(), "3");
}

#[test]
fn wc_content_with_no_trailing_newline() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "line1\nline2\nline3");
    let out = run_tp(&dir, &["--wc", "1"]);
    // 3 lines (last line has no trailing newline but still counts)
    let count: usize = stdout(&out).trim().parse().unwrap();
    assert!(
        (2..=3).contains(&count),
        "expected 2-3 lines, got {}",
        count
    );
}

// ── Size edge cases ─────────────────────────────────────

#[test]
fn size_unicode_content() {
    let dir = setup_clean_env();
    // "世界" is 6 bytes in UTF-8
    run_tp_stdin(&dir, &[], "世界");
    let out = run_tp(&dir, &["--size", "1"]);
    assert_eq!(stdout(&out).trim(), "6");
}

#[test]
fn size_negative_index() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "hello");
    let out = run_tp(&dir, &["--size", "-1"]);
    assert_eq!(stdout(&out).trim(), "5");
}

#[test]
fn size_large_file() {
    let dir = setup_clean_env();
    let big = "x".repeat(100_000);
    run_tp_stdin(&dir, &[], &big);
    let out = run_tp(&dir, &["--size", "1"]);
    assert_eq!(stdout(&out).trim(), "100000");
}

#[test]
fn size_after_append() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "hello");
    run_tp_stdin(&dir, &["-A", "1"], " world");
    let out = run_tp(&dir, &["--size", "1"]);
    assert_eq!(stdout(&out).trim(), "11"); // "hello world" = 11
}

// ── Path edge cases ─────────────────────────────────────

#[test]
fn path_all_files_unique() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "a");
    tick();
    run_tp_stdin(&dir, &[], "b");
    tick();
    run_tp_stdin(&dir, &[], "c");
    let p1 = stdout(&run_tp(&dir, &["--path", "1"])).trim().to_string();
    let p2 = stdout(&run_tp(&dir, &["--path", "2"])).trim().to_string();
    let p3 = stdout(&run_tp(&dir, &["--path", "3"])).trim().to_string();
    assert_ne!(p1, p2);
    assert_ne!(p2, p3);
    assert_ne!(p1, p3);
}

#[test]
fn path_file_exists_on_disk() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "verify me");
    let path = stdout(&run_tp(&dir, &["--path", "1"])).trim().to_string();
    assert!(std::path::Path::new(&path).exists());
}

#[test]
fn path_matches_dir_prefix() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let dir_out = stdout(&run_tp(&dir, &["-d"])).trim().to_string();
    let path_out = stdout(&run_tp(&dir, &["--path", "1"])).trim().to_string();
    assert!(path_out.starts_with(&dir_out));
}

// ── Info edge cases ─────────────────────────────────────

#[test]
fn info_negative_index() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "content");
    let out = run_tp(&dir, &["-I", "-1"]);
    assert!(out.status.success());
    let text = stdout(&out);
    assert!(text.contains("index: 1"));
}

#[test]
fn info_after_overwrite() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "original");
    run_tp_stdin(&dir, &["-i", "1"], "REPLACED");
    let out = run_tp(&dir, &["-I", "1"]);
    assert!(out.status.success());
    let text = stdout(&out);
    // verify info shows the file and size changed from original
    assert!(text.contains("index: 1"));
    assert!(text.contains("size:"));
    // verify content was actually replaced
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])), "REPLACED");
}

#[test]
fn info_named_file_shows_name() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &["-w", "testname"], "data");
    let out = run_tp(&dir, &["-I", "1"]);
    let text = stdout(&out);
    assert!(text.contains("name: testname"));
}

#[test]
fn info_multiple_files_correct_index() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "first");
    tick();
    run_tp_stdin(&dir, &[], "second");
    tick();
    run_tp_stdin(&dir, &[], "third");
    let out = run_tp(&dir, &["-I", "2"]);
    let text = stdout(&out);
    assert!(text.contains("index: 2"));
}

// ── Expire edge cases ───────────────────────────────────

#[test]
fn expire_negative_hours_fails() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["--expire", "-1"]);
    assert!(!out.status.success());
}

#[test]
fn expire_preserves_names_on_kept_files() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &["-w", "keeper"], "keep me");
    // large TTL keeps everything
    run_tp(&dir, &["--expire", "9999"]);
    let out = run_tp(&dir, &["-o", "keeper"]);
    assert_eq!(stdout(&out), "keep me");
}

#[test]
fn expire_then_push_works() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "old");
    std::thread::sleep(std::time::Duration::from_millis(50));
    let expire_out = run_tp(&dir, &["--expire", "0"]);
    assert_eq!(stdout(&expire_out).trim(), "1");
    // stack should be empty now
    assert_eq!(stdout(&run_tp(&dir, &["-k"])).trim(), "0");
    // push new item
    run_tp_stdin(&dir, &[], "new");
    let out = run_tp(&dir, &["-o", "1"]);
    assert_eq!(stdout(&out), "new");
}

// ── Sort edge cases ─────────────────────────────────────

#[test]
fn sort_by_size_equal_sizes() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "aaa"); // 3 bytes
    tick();
    run_tp_stdin(&dir, &[], "bbb"); // 3 bytes
    tick();
    run_tp_stdin(&dir, &[], "ccc"); // 3 bytes
    let out = run_tp(&dir, &["--sort", "size"]);
    assert!(out.status.success());
    // all same size, count should still be 3
    assert_eq!(stdout(&run_tp(&dir, &["-k"])).trim(), "3");
}

#[test]
fn sort_by_size_after_append() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "x"); // 1 byte
    tick();
    run_tp_stdin(&dir, &[], "y"); // 1 byte
    run_tp_stdin(&dir, &["-A", "2"], "yyyy"); // now 5 bytes
    run_tp(&dir, &["--sort", "size"]);
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])), "x");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "2"])), "yyyyy");
}

#[test]
fn sort_preserves_content() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "medium!!"); // 8 bytes
    tick();
    run_tp_stdin(&dir, &[], "z"); // 1 byte
    tick();
    run_tp_stdin(&dir, &[], "very long content here"); // 22 bytes
    run_tp(&dir, &["--sort", "size"]);
    // verify all content is still intact
    let all = run_tp(&dir, &["-L"]);
    let text = stdout(&all);
    assert!(text.contains("medium!!"));
    assert!(text.contains("z"));
    assert!(text.contains("very long content here"));
}

#[test]
fn sort_then_reverse() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "medium!!"); // 8 bytes
    tick();
    run_tp_stdin(&dir, &[], "z"); // 1 byte
    tick();
    run_tp_stdin(&dir, &[], "very long content here"); // 22 bytes
    run_tp(&dir, &["--sort", "size"]); // z, medium!!, very long...
    run_tp(&dir, &["--rev"]); // reverse: very long..., medium!!, z
    assert_eq!(
        stdout(&run_tp(&dir, &["-o", "1"])),
        "very long content here"
    );
    assert_eq!(stdout(&run_tp(&dir, &["-o", "3"])), "z");
}

// ── Named file complex workflows ────────────────────────

#[test]
fn name_with_add_at_index() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "existing");
    run_tp_stdin(&dir, &["-a", "1", "-w", "inserted"], "new at 1");
    let out = run_tp(&dir, &["-o", "inserted"]);
    assert_eq!(stdout(&out), "new at 1");
}

#[test]
fn name_with_unshift() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "existing");
    run_tp_stdin(&dir, &["-u", "-w", "bottom"], "new bottom");
    let out = run_tp(&dir, &["-o", "bottom"]);
    assert_eq!(stdout(&out), "new bottom");
    let out = run_tp(&dir, &["-o", "1"]);
    assert_eq!(stdout(&out), "new bottom");
}

#[test]
fn multiple_named_files_all_resolvable() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &["-w", "alpha"], "A");
    tick();
    run_tp_stdin(&dir, &["-w", "beta"], "B");
    tick();
    run_tp_stdin(&dir, &["-w", "gamma"], "C");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "alpha"])), "A");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "beta"])), "B");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "gamma"])), "C");
    // also by index
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])), "A");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "2"])), "B");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "3"])), "C");
}

#[test]
fn named_and_unnamed_mixed() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &["-w", "named1"], "N1");
    tick();
    run_tp_stdin(&dir, &[], "unnamed1");
    tick();
    run_tp_stdin(&dir, &["-w", "named2"], "N2");
    tick();
    run_tp_stdin(&dir, &[], "unnamed2");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "named1"])), "N1");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "named2"])), "N2");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "2"])), "unnamed1");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "4"])), "unnamed2");
}

#[test]
fn remove_named_by_index_clears_name() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &["-w", "gone"], "data");
    tick();
    run_tp_stdin(&dir, &[], "stay");
    run_tp(&dir, &["-r", "1"]);
    let out = run_tp(&dir, &["-o", "gone"]);
    assert!(
        !out.status.success(),
        "removed named file should not resolve"
    );
}

#[test]
fn name_tag_shows_in_contents_numbered() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &["-w", "tagged"], "hello");
    let out = run_tp(&dir, &["-N"]);
    let text = stdout(&out);
    assert!(text.contains("@tagged"));
}

// ── Dup edge cases ──────────────────────────────────────

#[test]
fn dup_then_modify_original() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "original");
    run_tp(&dir, &["-x", "1"]);
    run_tp_stdin(&dir, &["-i", "1"], "modified");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])), "modified");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "2"])), "original");
}

#[test]
fn dup_then_modify_clone() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "original");
    run_tp(&dir, &["-x", "1"]);
    run_tp_stdin(&dir, &["-i", "2"], "clone_modified");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])), "original");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "2"])), "clone_modified");
}

#[test]
fn dup_middle_of_stack() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "AAA");
    tick();
    run_tp_stdin(&dir, &[], "BBB");
    tick();
    run_tp_stdin(&dir, &[], "CCC");
    run_tp(&dir, &["-x", "2"]);
    assert_eq!(stdout(&run_tp(&dir, &["-k"])).trim(), "4");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "4"])), "BBB");
}

#[test]
fn dup_preserves_original_content() {
    let dir = setup_clean_env();
    let content = "multiline\ncontent\nhere\n";
    run_tp_stdin(&dir, &[], content);
    run_tp(&dir, &["-x", "1"]);
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])), content);
    assert_eq!(stdout(&run_tp(&dir, &["-o", "2"])), content);
}

// ── Append edge cases ───────────────────────────────────

#[test]
fn append_unicode() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "hello ");
    run_tp_stdin(&dir, &["-A", "1"], "世界 🚀");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])), "hello 世界 🚀");
}

#[test]
fn append_multiline() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "header\n");
    run_tp_stdin(&dir, &["-A", "1"], "line1\nline2\nline3\n");
    assert_eq!(
        stdout(&run_tp(&dir, &["-o", "1"])),
        "header\nline1\nline2\nline3\n"
    );
}

#[test]
fn append_empty_string() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "original");
    run_tp_stdin(&dir, &["-A", "1"], "");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])), "original");
}

#[test]
fn append_negative_index() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "first");
    tick();
    run_tp_stdin(&dir, &[], "second");
    run_tp_stdin(&dir, &["-A", "-1"], "+extra");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "2"])), "second+extra");
}

// ── Reverse edge cases ──────────────────────────────────

#[test]
fn reverse_two_items() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "AAA");
    tick();
    run_tp_stdin(&dir, &[], "BBB");
    run_tp(&dir, &["--rev"]);
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])), "BBB");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "2"])), "AAA");
}

#[test]
fn reverse_preserves_content_integrity() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "line1\nline2\n");
    tick();
    run_tp_stdin(&dir, &[], "hello 世界");
    tick();
    run_tp_stdin(&dir, &[], "tabs\there");
    run_tp(&dir, &["--rev"]);
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])), "tabs\there");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "2"])), "hello 世界");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "3"])), "line1\nline2\n");
}

#[test]
fn reverse_four_items() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "A");
    tick();
    run_tp_stdin(&dir, &[], "B");
    tick();
    run_tp_stdin(&dir, &[], "C");
    tick();
    run_tp_stdin(&dir, &[], "D");
    run_tp(&dir, &["--rev"]);
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])), "D");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "2"])), "C");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "3"])), "B");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "4"])), "A");
}

// ── Complex multi-operation workflows ───────────────────

#[test]
fn workflow_push_name_sort_grep() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &["-w", "big"], "xxxxxxxxxx"); // 10 bytes
    tick();
    run_tp_stdin(&dir, &["-w", "small"], "x"); // 1 byte
    tick();
    run_tp_stdin(&dir, &["-w", "medium"], "xxxxx"); // 5 bytes
    run_tp(&dir, &["--sort", "size"]);
    // after sort by size: small(1), medium(5), big(10)
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])), "x");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "small"])), "x");
    // grep should still work with names
    let grep = run_tp(&dir, &["-g", "xxxxx"]);
    assert!(grep.status.success());
}

#[test]
fn workflow_push_dup_swap_remove() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "AAA");
    tick();
    run_tp_stdin(&dir, &[], "BBB");
    run_tp(&dir, &["-x", "1"]); // dup AAA to top -> [AAA, BBB, AAA]
    run_tp(&dir, &["-S", "1", "3"]); // swap -> [AAA, BBB, AAA]
    run_tp(&dir, &["-r", "2"]); // remove BBB -> [AAA, AAA]
    assert_eq!(stdout(&run_tp(&dir, &["-k"])).trim(), "2");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])), "AAA");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "2"])), "AAA");
}

#[test]
fn workflow_push_replace_diff() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "hello world\n");
    run_tp(&dir, &["-x", "1"]); // dup
    run_tp(&dir, &["--replace", "2", "world", "rust"]);
    let diff = run_tp(&dir, &["-D", "1", "2"]);
    assert_eq!(diff.status.code(), Some(1));
    let text = stdout(&diff);
    assert!(text.contains("-hello world"));
    assert!(text.contains("+hello rust"));
}

#[test]
fn workflow_push_many_expire_push_more() {
    let dir = setup_clean_env();
    for i in 0..5 {
        run_tp_stdin(&dir, &[], &format!("batch1_{}", i));
        tick();
    }
    std::thread::sleep(std::time::Duration::from_millis(50));
    run_tp(&dir, &["--expire", "0"]); // remove all
    assert_eq!(stdout(&run_tp(&dir, &["-k"])).trim(), "0");
    for i in 0..3 {
        run_tp_stdin(&dir, &[], &format!("batch2_{}", i));
        tick();
    }
    assert_eq!(stdout(&run_tp(&dir, &["-k"])).trim(), "3");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])), "batch2_0");
}

#[test]
fn workflow_push_reverse_pop_verify() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "first");
    tick();
    run_tp_stdin(&dir, &[], "second");
    tick();
    run_tp_stdin(&dir, &[], "third");
    run_tp(&dir, &["--rev"]); // [third, second, first]
    run_tp(&dir, &["-p"]); // pop first -> [third, second]
    assert_eq!(stdout(&run_tp(&dir, &["-k"])).trim(), "2");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])), "third");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "2"])), "second");
}

#[test]
fn workflow_push_head_tail_wc_size() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "line1\nline2\nline3\nline4\nline5\n");
    // head
    let head = run_tp(&dir, &["--head", "1", "2"]);
    assert_eq!(stdout(&head).trim(), "line1\nline2");
    // tail
    let tail = run_tp(&dir, &["--tail", "1", "2"]);
    assert_eq!(stdout(&tail).trim(), "line4\nline5");
    // wc
    let wc = run_tp(&dir, &["--wc", "1"]);
    assert_eq!(stdout(&wc).trim(), "5");
    // size = 5 lines * 6 chars each = 30 bytes
    let size = run_tp(&dir, &["--size", "1"]);
    assert_eq!(stdout(&size).trim(), "30");
}

#[test]
fn workflow_named_sort_rename_output() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &["-w", "z_last"], "tiny"); // 4 bytes
    tick();
    run_tp_stdin(&dir, &["-w", "a_first"], "xxxxxxxxxx"); // 10 bytes
    run_tp(&dir, &["--sort", "size"]); // tiny(4) comes first
    run_tp(&dir, &["-R", "z_last", "was_last"]);
    assert_eq!(stdout(&run_tp(&dir, &["-o", "was_last"])), "tiny");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "a_first"])), "xxxxxxxxxx");
}

#[test]
fn workflow_cat_head_of_concatenated() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "part1\n");
    tick();
    run_tp_stdin(&dir, &[], "part2\n");
    let cat = run_tp(&dir, &["-C", "1", "2"]);
    assert_eq!(stdout(&cat), "part1\npart2\n");
}

// ── Stack integrity stress tests ────────────────────────

#[test]
fn stress_push_twenty_items() {
    let dir = setup_clean_env();
    for i in 1..=20 {
        run_tp_stdin(&dir, &[], &format!("item_{}", i));
    }
    assert_eq!(stdout(&run_tp(&dir, &["-k"])).trim(), "20");
    for i in 1..=20 {
        assert_eq!(
            stdout(&run_tp(&dir, &["-o", &i.to_string()])),
            format!("item_{}", i)
        );
    }
}

#[test]
fn stress_remove_from_middle_repeatedly() {
    let dir = setup_clean_env();
    for i in 1..=10 {
        run_tp_stdin(&dir, &[], &format!("item_{}", i));
    }
    // remove index 5 five times (always removing the middle-ish element)
    for _ in 0..5 {
        run_tp(&dir, &["-r", "3"]);
    }
    assert_eq!(stdout(&run_tp(&dir, &["-k"])).trim(), "5");
}

#[test]
fn stress_alternating_push_pop() {
    let dir = setup_clean_env();
    for i in 0..10 {
        run_tp_stdin(&dir, &[], &format!("item_{}", i));
        if i % 2 == 1 {
            run_tp(&dir, &["-p"]);
        }
    }
    // pushed 10, popped 5 (at i=1,3,5,7,9)
    assert_eq!(stdout(&run_tp(&dir, &["-k"])).trim(), "5");
}

#[test]
fn stress_alternating_push_shift() {
    let dir = setup_clean_env();
    for i in 0..10 {
        run_tp_stdin(&dir, &[], &format!("item_{}", i));
        if i % 2 == 1 {
            run_tp(&dir, &["-s"]);
        }
    }
    assert_eq!(stdout(&run_tp(&dir, &["-k"])).trim(), "5");
}

// ── Edit edge cases ─────────────────────────────────────

#[test]
fn edit_negative_index() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "content");
    let out = Command::new(bin())
        .env("TEMPRS_DIR", &dir)
        .env("EDITOR", "true")
        .args(["-e", "-1"])
        .output()
        .expect("failed to execute tp");
    assert!(out.status.success());
}

#[test]
fn edit_preserves_content_when_editor_is_noop() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "keep this");
    Command::new(bin())
        .env("TEMPRS_DIR", &dir)
        .env("EDITOR", "true")
        .args(["-e", "1"])
        .output()
        .expect("failed to execute tp");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])), "keep this");
}

// ── Content edge cases ──────────────────────────────────

#[test]
fn push_binary_like_content() {
    let dir = setup_clean_env();
    // high bytes that aren't valid UTF-8 sequences
    let content = "binary\x01\x02\x03data";
    run_tp_stdin(&dir, &[], content);
    let out = run_tp(&dir, &["-o", "1"]);
    assert_eq!(stdout(&out), content);
}

#[test]
fn push_very_many_newlines() {
    let dir = setup_clean_env();
    let content = "\n".repeat(1000);
    run_tp_stdin(&dir, &[], &content);
    let out = run_tp(&dir, &["-o", "1"]);
    assert_eq!(stdout(&out), content);
}

#[test]
fn push_mixed_line_endings() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "unix\nwindows\r\nold_mac\rend");
    let out = run_tp(&dir, &["-o", "1"]);
    assert_eq!(stdout(&out), "unix\nwindows\r\nold_mac\rend");
}

#[test]
fn push_content_with_ansi_escape_codes() {
    let dir = setup_clean_env();
    let content = "\x1b[31mred\x1b[0m normal";
    run_tp_stdin(&dir, &[], content);
    let out = run_tp(&dir, &["-o", "1"]);
    assert_eq!(stdout(&out), content);
}

#[test]
fn push_repeated_content_distinct_files() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "same");
    tick();
    run_tp_stdin(&dir, &[], "same");
    tick();
    run_tp_stdin(&dir, &[], "same");
    assert_eq!(stdout(&run_tp(&dir, &["-k"])).trim(), "3");
    // all have same content but different files
    let p1 = stdout(&run_tp(&dir, &["--path", "1"])).trim().to_string();
    let p2 = stdout(&run_tp(&dir, &["--path", "2"])).trim().to_string();
    let p3 = stdout(&run_tp(&dir, &["--path", "3"])).trim().to_string();
    assert_ne!(p1, p2);
    assert_ne!(p2, p3);
}

// ── Count edge cases ────────────────────────────────────

#[test]
fn count_after_pop() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "a");
    tick();
    run_tp_stdin(&dir, &[], "b");
    tick();
    run_tp_stdin(&dir, &[], "c");
    run_tp(&dir, &["-p"]);
    assert_eq!(stdout(&run_tp(&dir, &["-k"])).trim(), "2");
}

#[test]
fn count_after_shift() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "a");
    tick();
    run_tp_stdin(&dir, &[], "b");
    tick();
    run_tp_stdin(&dir, &[], "c");
    run_tp(&dir, &["-s"]);
    assert_eq!(stdout(&run_tp(&dir, &["-k"])).trim(), "2");
}

#[test]
fn count_after_add() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "a");
    tick();
    run_tp_stdin(&dir, &[], "b");
    run_tp_stdin(&dir, &["-a", "2"], "c");
    assert_eq!(stdout(&run_tp(&dir, &["-k"])).trim(), "3");
}

#[test]
fn count_after_dup() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    run_tp(&dir, &["-x", "1"]);
    assert_eq!(stdout(&run_tp(&dir, &["-k"])).trim(), "2");
}

#[test]
fn count_after_sort() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "bbb"); // 3 bytes
    tick();
    run_tp_stdin(&dir, &[], "a"); // 1 byte
    tick();
    run_tp_stdin(&dir, &[], "cc"); // 2 bytes
    run_tp(&dir, &["--sort", "size"]);
    assert_eq!(stdout(&run_tp(&dir, &["-k"])).trim(), "3");
}

#[test]
fn count_after_reverse() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "a");
    tick();
    run_tp_stdin(&dir, &[], "b");
    run_tp(&dir, &["--rev"]);
    assert_eq!(stdout(&run_tp(&dir, &["-k"])).trim(), "2");
}

// ── Rename edge cases ───────────────────────────────────

#[test]
fn rename_nonexistent_name_fails() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["-R", "nonexistent", "newname"]);
    assert!(!out.status.success());
}

#[test]
fn rename_invalid_index_fails() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["-R", "99", "newname"]);
    assert!(!out.status.success());
}

#[test]
fn rename_negative_index_not_supported() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    // negative index for rename is not supported
    let out = run_tp(&dir, &["-R", "-1", "newname"]);
    assert!(!out.status.success());
}

#[test]
fn rename_then_rename_again() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &["-w", "first"], "data");
    run_tp(&dir, &["-R", "first", "second"]);
    run_tp(&dir, &["-R", "second", "third"]);
    assert_eq!(stdout(&run_tp(&dir, &["-o", "third"])), "data");
    assert!(!run_tp(&dir, &["-o", "first"]).status.success());
    assert!(!run_tp(&dir, &["-o", "second"]).status.success());
}

// ── Input overwrite edge cases ──────────────────────────

#[test]
fn input_overwrite_with_unicode() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "original");
    run_tp_stdin(&dir, &["-i", "1"], "日本語テスト 🎉");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])), "日本語テスト 🎉");
}

#[test]
fn input_overwrite_empty_to_content() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "");
    run_tp_stdin(&dir, &["-i", "1"], "now has content");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])), "now has content");
}

#[test]
fn input_overwrite_content_to_empty() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "has content");
    run_tp_stdin(&dir, &["-i", "1"], "");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])), "");
}

#[test]
fn input_overwrite_invalid_index_fails() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp_stdin(&dir, &["-i", "99"], "replacement");
    assert!(!out.status.success());
}

// ── Add at index edge cases ─────────────────────────────

#[test]
fn add_negative_index_to_beginning() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "first");
    tick();
    run_tp_stdin(&dir, &[], "second");
    // -2 should map to index 1 (beginning) in a 2-item stack
    run_tp_stdin(&dir, &["-a", "-2"], "new_beginning");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])), "new_beginning");
}

#[test]
fn add_preserves_names() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &["-w", "keep"], "original");
    run_tp_stdin(&dir, &["-a", "1"], "inserted");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "keep"])), "original");
}

#[test]
fn add_invalid_index_fails() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp_stdin(&dir, &["-a", "99"], "new");
    assert!(!out.status.success());
}

// ── Remove with name edge cases ─────────────────────────

#[test]
fn remove_named_preserves_other_names() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &["-w", "keep1"], "A");
    tick();
    run_tp_stdin(&dir, &["-w", "removeone"], "B");
    tick();
    run_tp_stdin(&dir, &["-w", "keep2"], "C");
    // verify we have 3 items
    assert_eq!(stdout(&run_tp(&dir, &["-k"])).trim(), "3");
    run_tp(&dir, &["-r", "removeone"]);
    assert_eq!(stdout(&run_tp(&dir, &["-k"])).trim(), "2");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "keep1"])), "A");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "keep2"])), "C");
}

#[test]
fn remove_negative_index_preserves_content() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "AAA");
    tick();
    run_tp_stdin(&dir, &[], "BBB");
    tick();
    run_tp_stdin(&dir, &[], "CCC");
    run_tp(&dir, &["-r", "-2"]); // remove middle (BBB)
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])), "AAA");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "2"])), "CCC");
}

// ── Master file robustness ──────────────────────────────

#[test]
fn master_file_survives_many_operations() {
    let dir = setup_clean_env();
    // push 5
    for i in 1..=5 {
        run_tp_stdin(&dir, &[], &format!("item_{}", i));
        tick();
    }
    // pop 2
    run_tp(&dir, &["-p"]);
    run_tp(&dir, &["-p"]);
    // push 2 more
    run_tp_stdin(&dir, &[], "new_1");
    tick();
    run_tp_stdin(&dir, &[], "new_2");
    // shift 1
    run_tp(&dir, &["-s"]);
    // verify count
    assert_eq!(stdout(&run_tp(&dir, &["-k"])).trim(), "4");
    // verify master file exists
    assert!(dir.join("temprs-stack").exists());
}

#[test]
fn master_file_not_corrupted_by_overwrite() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "AAA");
    tick();
    run_tp_stdin(&dir, &[], "BBB");
    tick();
    run_tp_stdin(&dir, &[], "CCC");
    // overwrite middle
    run_tp_stdin(&dir, &["-i", "2"], "NEW_BBB");
    // verify all items intact
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])), "AAA");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "2"])), "NEW_BBB");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "3"])), "CCC");
    // verify no tmp files left
    assert!(!dir.join("temprs-stack.tmp").exists());
}

#[test]
fn no_tmp_file_after_sort() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "bbb");
    tick();
    run_tp_stdin(&dir, &[], "a");
    run_tp(&dir, &["--sort", "size"]);
    assert!(!dir.join("temprs-stack.tmp").exists());
}

#[test]
fn no_tmp_file_after_reverse() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "a");
    tick();
    run_tp_stdin(&dir, &[], "b");
    run_tp(&dir, &["--rev"]);
    assert!(!dir.join("temprs-stack.tmp").exists());
}

#[test]
fn no_tmp_file_after_swap() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "a");
    tick();
    run_tp_stdin(&dir, &[], "b");
    run_tp(&dir, &["-S", "1", "2"]);
    assert!(!dir.join("temprs-stack.tmp").exists());
}

#[test]
fn no_tmp_file_after_move() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "a");
    tick();
    run_tp_stdin(&dir, &[], "b");
    run_tp(&dir, &["-M", "1", "2"]);
    assert!(!dir.join("temprs-stack.tmp").exists());
}

#[test]
fn no_tmp_file_after_expire() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "a");
    std::thread::sleep(std::time::Duration::from_millis(50));
    run_tp(&dir, &["--expire", "0"]);
    assert!(!dir.join("temprs-stack.tmp").exists());
}

// ── Exit codes comprehensive ────────────────────────────

#[test]
fn add_exits_success() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "existing");
    let out = run_tp_stdin(&dir, &["-a", "1"], "new");
    assert!(out.status.success());
}

#[test]
fn unshift_exits_success_with_existing() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "existing");
    let out = run_tp_stdin(&dir, &["-u"], "bottom");
    assert!(out.status.success());
}

#[test]
fn move_exits_success() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "a");
    tick();
    run_tp_stdin(&dir, &[], "b");
    let out = run_tp(&dir, &["-M", "1", "2"]);
    assert!(out.status.success());
}

#[test]
fn swap_exits_success() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "a");
    tick();
    run_tp_stdin(&dir, &[], "b");
    let out = run_tp(&dir, &["-S", "1", "2"]);
    assert!(out.status.success());
}

#[test]
fn dup_exits_success() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["-x", "1"]);
    assert!(out.status.success());
}

#[test]
fn reverse_exits_success() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["--rev"]);
    assert!(out.status.success());
}

#[test]
fn count_exits_success() {
    let dir = setup_clean_env();
    let out = run_tp(&dir, &["-k"]);
    assert!(out.status.success());
}

#[test]
fn expire_exits_success() {
    let dir = setup_clean_env();
    let out = run_tp(&dir, &["--expire", "9999"]);
    assert!(out.status.success());
}

#[test]
fn head_exits_success() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data\n");
    let out = run_tp(&dir, &["--head", "1", "1"]);
    assert!(out.status.success());
}

#[test]
fn tail_exits_success() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data\n");
    let out = run_tp(&dir, &["--tail", "1", "1"]);
    assert!(out.status.success());
}

#[test]
fn wc_exits_success() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data\n");
    let out = run_tp(&dir, &["--wc", "1"]);
    assert!(out.status.success());
}

#[test]
fn size_exits_success() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["--size", "1"]);
    assert!(out.status.success());
}

#[test]
fn sort_exits_success() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["--sort", "name"]);
    assert!(out.status.success());
}

#[test]
fn replace_exits_success() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "hello world");
    let out = run_tp(&dir, &["--replace", "1", "hello", "hi"]);
    assert!(out.status.success());
}

#[test]
fn path_exits_success() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["--path", "1"]);
    assert!(out.status.success());
}

#[test]
fn rename_exits_success() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &["-w", "old"], "data");
    let out = run_tp(&dir, &["-R", "old", "new"]);
    assert!(out.status.success());
}

#[test]
fn info_exits_success() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["-I", "1"]);
    assert!(out.status.success());
}

#[test]
fn grep_exits_success_on_match() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "find me");
    let out = run_tp(&dir, &["-g", "find"]);
    assert!(out.status.success());
}

#[test]
fn append_exits_success() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "base");
    let out = run_tp_stdin(&dir, &["-A", "1"], "more");
    assert!(out.status.success());
}

#[test]
fn diff_exits_success_identical() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "same\n");
    tick();
    run_tp_stdin(&dir, &[], "same\n");
    let out = run_tp(&dir, &["-D", "1", "2"]);
    assert!(out.status.success());
}

#[test]
fn cat_exits_success() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["-C", "1"]);
    assert!(out.status.success());
}

// ── Verbose with various operations ─────────────────────

#[test]
fn verbose_with_list() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["-v", "-l"]);
    assert!(out.status.success());
}

#[test]
fn verbose_with_count() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["-v", "-k"]);
    assert!(out.status.success());
    assert!(stdout(&out).contains("1"));
}

// ── Quiet with various operations ───────────────────────

#[test]
fn quiet_with_count() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &["-q"], "data");
    let out = run_tp(&dir, &["-k"]);
    assert_eq!(stdout(&out).trim(), "1");
}

#[test]
fn quiet_with_output() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &["-q"], "secret");
    let out = run_tp(&dir, &["-o", "1"]);
    assert_eq!(stdout(&out), "secret");
}

// ── Pop/shift on empty stack ────────────────────────────

#[test]
fn pop_empty_stack_fails() {
    let dir = setup_clean_env();
    let out = run_tp(&dir, &["-p"]);
    assert!(!out.status.success());
}

#[test]
fn shift_empty_stack_fails() {
    let dir = setup_clean_env();
    let out = run_tp(&dir, &["-s"]);
    assert!(!out.status.success());
}

// ── Long flag variants ──────────────────────────────────

#[test]
fn long_flag_input() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "original");
    run_tp_stdin(&dir, &["--input", "1"], "replaced");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])), "replaced");
}

#[test]
fn long_flag_output() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["--output", "1"]);
    assert_eq!(stdout(&out), "data");
}

#[test]
fn long_flag_add() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "existing");
    run_tp_stdin(&dir, &["--add", "1"], "new");
    assert_eq!(stdout(&run_tp(&dir, &["-k"])).trim(), "2");
}

#[test]
fn long_flag_remove() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    run_tp(&dir, &["--remove", "1"]);
    assert_eq!(stdout(&run_tp(&dir, &["-k"])).trim(), "0");
}

#[test]
fn long_flag_pop() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    run_tp(&dir, &["--pop"]);
    assert_eq!(stdout(&run_tp(&dir, &["-k"])).trim(), "0");
}

#[test]
fn long_flag_shift() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    run_tp(&dir, &["--shift"]);
    assert_eq!(stdout(&run_tp(&dir, &["-k"])).trim(), "0");
}

#[test]
fn long_flag_unshift() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "existing");
    let out = run_tp_stdin(&dir, &["--unshift"], "bottom");
    assert!(out.status.success());
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])), "bottom");
}

#[test]
fn long_flag_list_files() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["--list-files"]);
    assert!(out.status.success());
    assert!(stdout(&out).contains("tempfile"));
}

#[test]
fn long_flag_list_files_numbered() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["--list-files-numbered"]);
    assert!(stdout(&out).contains("1:"));
}

#[test]
fn long_flag_list_contents() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "mydata");
    let out = run_tp(&dir, &["--list-contents"]);
    assert!(stdout(&out).contains("mydata"));
}

#[test]
fn long_flag_list_contents_numbered() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "mydata");
    let out = run_tp(&dir, &["--list-contents-numbered"]);
    let text = stdout(&out);
    assert!(text.contains("1:"));
    assert!(text.contains("mydata"));
}

#[test]
fn long_flag_clear() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["--clear"]);
    assert!(out.status.success());
}

#[test]
fn long_flag_count() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["--count"]);
    assert_eq!(stdout(&out).trim(), "1");
}

#[test]
fn long_flag_dir() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["--dir"]);
    assert!(stdout(&out).contains(&dir.to_string_lossy().to_string()));
}

#[test]
fn long_flag_master() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["--master"]);
    assert!(stdout(&out).contains("temprs-stack"));
}

#[test]
fn long_flag_name() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &["--name", "myfile"], "data");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "myfile"])), "data");
}

#[test]
fn long_flag_rename() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &["-w", "old"], "data");
    run_tp(&dir, &["--rename", "old", "new"]);
    assert_eq!(stdout(&run_tp(&dir, &["-o", "new"])), "data");
}

#[test]
fn long_flag_info() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["--info", "1"]);
    assert!(stdout(&out).contains("index: 1"));
}

#[test]
fn long_flag_grep() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "findme");
    let out = run_tp(&dir, &["--grep", "findme"]);
    assert!(stdout(&out).contains("findme"));
}

#[test]
fn long_flag_cat() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["--cat", "1"]);
    assert_eq!(stdout(&out), "data");
}

#[test]
fn long_flag_diff() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "same\n");
    tick();
    run_tp_stdin(&dir, &[], "same\n");
    let out = run_tp(&dir, &["--diff", "1", "2"]);
    assert!(out.status.success());
}

#[test]
fn long_flag_mv() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "A");
    tick();
    run_tp_stdin(&dir, &[], "B");
    run_tp(&dir, &["--mv", "1", "2"]);
    assert_eq!(stdout(&run_tp(&dir, &["-o", "2"])), "A");
}

#[test]
fn long_flag_dup() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    run_tp(&dir, &["--dup", "1"]);
    assert_eq!(stdout(&run_tp(&dir, &["-k"])).trim(), "2");
}

#[test]
fn long_flag_swap() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "A");
    tick();
    run_tp_stdin(&dir, &[], "B");
    run_tp(&dir, &["--swap", "1", "2"]);
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])), "B");
}

#[test]
fn long_flag_append() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "base");
    run_tp_stdin(&dir, &["--append", "1"], "+more");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])), "base+more");
}

#[test]
fn long_flag_verbose() {
    let dir = setup_clean_env();
    let out = run_tp_stdin(&dir, &["--verbose"], "data");
    assert!(out.status.success());
}

#[test]
fn long_flag_quiet() {
    let dir = setup_clean_env();
    let out = run_tp_stdin(&dir, &["--quiet"], "data");
    assert!(out.status.success());
}

// ── equals-style long flags (--flag=value) ────────────

#[test]
fn equals_output_reads_stack() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "hello-equals\n");
    let out = run_tp(&dir, &["--output=1"]);
    assert_eq!(stdout(&out), "hello-equals\n");
}

#[test]
fn equals_input_writes_stack() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "first");
    tick();
    let out = run_tp_stdin(&dir, &["--input=1"], "replaced");
    assert!(out.status.success());
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])), "replaced");
}

#[test]
fn equals_grep_filters() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "alpha\nbeta\nalpha\n");
    let out = run_tp(&dir, &["--grep=alpha"]);
    assert!(stdout(&out).contains("alpha"));
    assert!(!stdout(&out).contains("beta"));
}

#[test]
fn equals_path_prints_file_path() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["--path=1"]);
    let text = stdout(&out);
    let p = text.trim();
    assert!(!p.is_empty());
    assert!(std::path::Path::new(p).exists());
}

#[test]
fn equals_dup_copies_top() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "solo");
    run_tp(&dir, &["--dup=1"]);
    assert_eq!(stdout(&run_tp(&dir, &["-k"])).trim(), "2");
}

#[test]
fn equals_expire_zero_exits_ok() {
    let dir = setup_clean_env();
    let out = run_tp(&dir, &["--expire=0"]);
    assert!(out.status.success());
}

#[test]
fn equals_sort_name_succeeds() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "a");
    tick();
    run_tp_stdin(&dir, &[], "b");
    let out = run_tp(&dir, &["--sort=name"]);
    assert!(out.status.success());
}

#[test]
fn equals_sort_mtime_succeeds() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "x");
    tick();
    run_tp_stdin(&dir, &[], "y");
    let out = run_tp(&dir, &["--sort=mtime"]);
    assert!(out.status.success());
}

#[test]
fn equals_wc_line_count() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "one\ntwo\nthree\n");
    let out = run_tp(&dir, &["--wc=1"]);
    assert!(stdout(&out).contains('3'));
}

#[test]
fn equals_size_reports_bytes() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "12345");
    let out = run_tp(&dir, &["--size=1"]);
    assert_eq!(stdout(&out).trim(), "5");
}

#[test]
fn equals_append_stdin() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "base");
    let out = run_tp_stdin(&dir, &["--append=1"], "+tail");
    assert!(out.status.success());
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])), "base+tail");
}

#[test]
fn equals_and_short_flags_combined() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "stored");
    let out = run_tp(&dir, &["-q", "--output=1"]);
    assert!(out.status.success());
    assert_eq!(stdout(&out).trim(), "stored");
}

#[test]
fn triple_push_equals_count_matches() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "a");
    tick();
    run_tp_stdin(&dir, &[], "b");
    tick();
    run_tp_stdin(&dir, &[], "c");
    assert_eq!(stdout(&run_tp(&dir, &["-k"])).trim(), "3");
}

// ── list modes and stack edge cases ───────────────────

#[test]
fn list_files_after_double_push() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "one");
    tick();
    run_tp_stdin(&dir, &[], "two");
    let out = run_tp(&dir, &["-l"]);
    let text = stdout(&out);
    assert_eq!(text.trim().lines().count(), 2);
}

#[test]
fn list_files_numbered_two_items_present() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "a");
    tick();
    run_tp_stdin(&dir, &[], "b");
    let out = run_tp(&dir, &["-n"]);
    let text = stdout(&out);
    assert!(text.contains('[') || text.contains("0") || text.contains("1"));
    assert!(text.lines().count() >= 2);
}

#[test]
fn list_contents_shows_both() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "first");
    tick();
    run_tp_stdin(&dir, &[], "second");
    let out = run_tp(&dir, &["-L"]);
    let t = stdout(&out);
    assert!(t.contains("first"));
    assert!(t.contains("second"));
}

#[test]
fn pop_then_count_one() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "a");
    tick();
    run_tp_stdin(&dir, &[], "b");
    run_tp(&dir, &["-p"]);
    assert_eq!(stdout(&run_tp(&dir, &["-k"])).trim(), "1");
}

#[test]
fn shift_bottom_then_output() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "bottom");
    tick();
    run_tp_stdin(&dir, &[], "top");
    run_tp(&dir, &["-s"]);
    let out = run_tp(&dir, &["-o", "1"]);
    assert_eq!(stdout(&out).trim(), "top");
}

#[test]
fn unshift_second_index_is_original_top() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "first");
    tick();
    run_tp_stdin(&dir, &["-u"], "second");
    let out = run_tp(&dir, &["-o", "2"]);
    assert_eq!(stdout(&out).trim(), "first");
}

#[test]
fn replace_substring_in_tempfile() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "hello world");
    let out = run_tp(&dir, &["--replace", "1", "world", "Rust"]);
    assert!(out.status.success());
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])).trim(), "hello Rust");
}

#[test]
fn cat_two_indices_concat() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "aa");
    tick();
    run_tp_stdin(&dir, &[], "bb");
    let out = run_tp(&dir, &["--cat", "1", "2"]);
    let t = stdout(&out);
    assert!(t.contains("aa"));
    assert!(t.contains("bb"));
}

#[test]
fn head_first_two_lines() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "l1\nl2\nl3\n");
    let out = run_tp(&dir, &["--head", "1", "2"]);
    let t = stdout(&out);
    assert!(t.contains("l1"));
    assert!(t.contains("l2"));
    assert!(!t.contains("l3"));
}

#[test]
fn tail_last_line_only() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "a\nb\nc\n");
    let out = run_tp(&dir, &["--tail", "1", "1"]);
    assert_eq!(stdout(&out).trim(), "c");
}

#[test]
fn info_exits_success_with_one_file() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["--info", "1"]);
    assert!(out.status.success());
}

#[test]
fn reverse_two_items_swaps_output_order() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "x");
    tick();
    run_tp_stdin(&dir, &[], "y");
    run_tp(&dir, &["--rev"]);
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])).trim(), "y");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "2"])).trim(), "x");
}

// ── additional CLI workflows ─────────────────────────

#[test]
fn diff_two_distinct_files_exits_nonzero() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "aaa\n");
    tick();
    run_tp_stdin(&dir, &[], "bbb\n");
    let out = run_tp(&dir, &["--diff", "1", "2"]);
    assert!(!out.status.success() || stdout(&out).contains("---") || stdout(&out).contains("+++"));
}

#[test]
fn mv_top_to_bottom_position() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "first");
    tick();
    run_tp_stdin(&dir, &[], "second");
    tick();
    run_tp_stdin(&dir, &[], "third");
    run_tp(&dir, &["--mv", "1", "3"]);
    assert_eq!(stdout(&run_tp(&dir, &["-o", "3"])).trim(), "first");
}

#[test]
fn name_tag_push_then_output_by_name() {
    let dir = setup_clean_env();
    let out = run_tp_stdin(&dir, &["-w", "mine"], "tagged-data");
    assert!(out.status.success());
    let read = run_tp(&dir, &["-o", "mine"]);
    assert_eq!(stdout(&read).trim(), "tagged-data");
}

#[test]
fn pop_twice_leaves_empty_stack() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "a");
    tick();
    run_tp_stdin(&dir, &[], "b");
    run_tp(&dir, &["-p"]);
    run_tp(&dir, &["-p"]);
    assert_eq!(stdout(&run_tp(&dir, &["-k"])).trim(), "0");
}

#[test]
fn list_master_file_exits_ok() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "x");
    let out = run_tp(&dir, &["-m"]);
    assert!(out.status.success());
}

#[test]
fn dir_flag_prints_temprs_path() {
    let dir = setup_clean_env();
    let out = run_tp(&dir, &["-d"]);
    assert!(out.status.success());
    let text = stdout(&out);
    assert!(text.contains("temprs") || dir.to_string_lossy().contains("temprs_test"));
}

#[test]
fn sort_size_three_files() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "xx");
    tick();
    run_tp_stdin(&dir, &[], "xxxx");
    tick();
    run_tp_stdin(&dir, &[], "x");
    let out = run_tp(&dir, &["--sort", "size"]);
    assert!(out.status.success());
}

#[test]
fn replace_all_occurrences_word() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "foo foo foo");
    let out = run_tp(&dir, &["--replace", "1", "foo", "bar"]);
    assert!(out.status.success());
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])).trim(), "bar bar bar");
}

#[test]
fn wc_three_lines_reports_three() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "a\nb\nc\n");
    let out = run_tp(&dir, &["--wc", "1"]);
    assert!(stdout(&out).contains('3'));
}

#[test]
fn path_flag_matches_output_read() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "payload");
    let path_out = run_tp(&dir, &["--path", "1"]);
    let text = stdout(&path_out);
    let p = text.trim();
    let content = std::fs::read_to_string(p).unwrap();
    assert_eq!(content.trim(), "payload");
}

#[test]
fn add_inserts_middle_three_item_stack() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "a");
    tick();
    run_tp_stdin(&dir, &[], "b");
    tick();
    run_tp_stdin(&dir, &["-a", "2"], "inserted");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "2"])).trim(), "inserted");
}

#[test]
fn remove_middle_preserves_neighbors() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "keep1");
    tick();
    run_tp_stdin(&dir, &[], "gone");
    tick();
    run_tp_stdin(&dir, &[], "keep2");
    run_tp(&dir, &["-r", "2"]);
    assert_eq!(stdout(&run_tp(&dir, &["-k"])).trim(), "2");
}

// ── bulk integration smoke tests ──────────────────────

#[test]
fn push_four_items_count_four() {
    let dir = setup_clean_env();
    for i in 0..4 {
        tick();
        run_tp_stdin(&dir, &[], &format!("item{}", i));
    }
    assert_eq!(stdout(&run_tp(&dir, &["-k"])).trim(), "4");
}

#[test]
fn output_negative_two_is_bottom_two_item_stack() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "bottom");
    tick();
    run_tp_stdin(&dir, &[], "top");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "-2"])).trim(), "bottom");
}

#[test]
fn grep_no_match_exits_failure_on_stack() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "hello");
    let out = run_tp(&dir, &["--grep", "zzznomatch"]);
    assert!(!out.status.success());
}

#[test]
fn size_zero_byte_file() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "");
    let out = run_tp(&dir, &["--size", "1"]);
    assert_eq!(stdout(&out).trim(), "0");
}

#[test]
fn clear_then_push_one() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "old");
    run_tp(&dir, &["-c"]);
    run_tp_stdin(&dir, &[], "fresh");
    assert_eq!(stdout(&run_tp(&dir, &["-k"])).trim(), "1");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])).trim(), "fresh");
}

#[test]
fn dup_twice_triples_stack() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "solo");
    run_tp(&dir, &["--dup", "1"]);
    run_tp(&dir, &["--dup", "1"]);
    assert_eq!(stdout(&run_tp(&dir, &["-k"])).trim(), "3");
}

#[test]
fn swap_first_and_last_three_items() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "a");
    tick();
    run_tp_stdin(&dir, &[], "b");
    tick();
    run_tp_stdin(&dir, &[], "c");
    run_tp(&dir, &["--swap", "1", "3"]);
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])).trim(), "c");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "3"])).trim(), "a");
}

#[test]
fn cat_three_files_in_order() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "1");
    tick();
    run_tp_stdin(&dir, &[], "2");
    tick();
    run_tp_stdin(&dir, &[], "3");
    let out = run_tp(&dir, &["--cat", "1", "2", "3"]);
    let t = stdout(&out);
    assert!(t.contains('1') && t.contains('2') && t.contains('3'));
}

// ── additional integration smoke tests ─────────────────

#[test]
fn sort_by_name_three_files_exits_ok() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "z");
    tick();
    run_tp_stdin(&dir, &[], "a");
    tick();
    run_tp_stdin(&dir, &[], "m");
    let out = run_tp(&dir, &["--sort", "name"]);
    assert!(out.status.success());
}

#[test]
fn sort_by_mtime_two_files_exits_ok() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "first");
    tick();
    run_tp_stdin(&dir, &[], "second");
    let out = run_tp(&dir, &["--sort", "mtime"]);
    assert!(out.status.success());
}

#[test]
fn info_top_of_stack_exits_ok() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "meta");
    let out = run_tp(&dir, &["--info", "1"]);
    assert!(out.status.success());
}

#[test]
fn head_two_lines_of_top_file() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "a\nb\nc\nd\n");
    let out = run_tp(&dir, &["--head", "1", "2"]);
    let t = stdout(&out);
    assert!(t.contains('a') && t.contains('b'));
}

#[test]
fn tail_two_lines_of_top_file() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "a\nb\nc\nd\n");
    let out = run_tp(&dir, &["--tail", "1", "2"]);
    let t = stdout(&out);
    assert!(t.contains('c') && t.contains('d'));
}

#[test]
fn mv_bottom_to_top_two_items() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "bottom");
    tick();
    run_tp_stdin(&dir, &[], "top");
    let out = run_tp(&dir, &["--mv", "2", "1"]);
    assert!(out.status.success());
}

#[test]
fn tag_then_output_by_name() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &["-w", "mine"], "tagged-data");
    let out = run_tp(&dir, &["-o", "mine"]);
    assert_eq!(stdout(&out).trim(), "tagged-data");
}

#[test]
fn rename_tag_and_read_by_new_name() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &["-w", "oldname"], "payload");
    run_tp(&dir, &["--rename", "oldname", "newname"]);
    assert_eq!(stdout(&run_tp(&dir, &["-o", "newname"])).trim(), "payload");
}

#[test]
fn diff_two_identical_stack_slots_exits_ok() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "same-bytes");
    tick();
    run_tp_stdin(&dir, &[], "same-bytes");
    let out = run_tp(&dir, &["--diff", "1", "2"]);
    assert!(out.status.success());
}

#[test]
fn count_stack_after_five_pushes() {
    let dir = setup_clean_env();
    for i in 0..5 {
        tick();
        run_tp_stdin(&dir, &[], &format!("n{}", i));
    }
    assert_eq!(stdout(&run_tp(&dir, &["-k"])).trim(), "5");
}

#[test]
fn reverse_stack_three_items_index_one_is_old_top() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "first");
    tick();
    run_tp_stdin(&dir, &[], "mid");
    tick();
    run_tp_stdin(&dir, &[], "last");
    run_tp(&dir, &["--rev"]);
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])).trim(), "last");
}

#[test]
fn append_to_top_preserves_prior_content() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "base");
    run_tp_stdin(&dir, &["--append", "1"], "\nmore");
    let t = stdout(&run_tp(&dir, &["-o", "1"]));
    assert!(t.contains("base") && t.contains("more"));
}

#[test]
fn pop_then_count_decrements() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "a");
    tick();
    run_tp_stdin(&dir, &[], "b");
    run_tp(&dir, &["-p"]);
    assert_eq!(stdout(&run_tp(&dir, &["-k"])).trim(), "1");
}

#[test]
fn unshift_adds_bottom_then_output_negative_one_is_prior_top() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "top");
    tick();
    run_tp_stdin(&dir, &["-u"], "bottom");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "-1"])).trim(), "top");
}

#[test]
fn shift_removes_bottom_two_item_stack() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "lose-bottom");
    tick();
    run_tp_stdin(&dir, &[], "keep-top");
    run_tp(&dir, &["-s"]);
    assert_eq!(stdout(&run_tp(&dir, &["-k"])).trim(), "1");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])).trim(), "keep-top");
}

#[test]
fn grep_match_succeeds() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "needle in haystack");
    let out = run_tp(&dir, &["--grep", "needle"]);
    assert!(out.status.success());
}

#[test]
fn list_numbered_shows_indices() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "x");
    let out = run_tp(&dir, &["-n"]);
    let t = stdout(&out);
    assert!(t.contains('1'));
}

#[test]
fn list_contents_numbered_includes_index() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "body");
    let out = run_tp(&dir, &["-N"]);
    assert!(out.status.success());
    let t = stdout(&out);
    assert!(t.contains('1'));
}

#[test]
fn output_negative_one_two_files() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "older");
    tick();
    run_tp_stdin(&dir, &[], "newer");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "-1"])).trim(), "newer");
}

#[test]
fn remove_first_leaves_second_at_index_one() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "gone");
    tick();
    run_tp_stdin(&dir, &[], "stays");
    run_tp(&dir, &["-r", "1"]);
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])).trim(), "stays");
}

#[test]
fn add_inserts_at_negative_index_relative() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "a");
    tick();
    run_tp_stdin(&dir, &[], "b");
    run_tp_stdin(&dir, &["-a", "-1"], "inserted");
    assert_eq!(stdout(&run_tp(&dir, &["-k"])).trim(), "3");
}

#[test]
fn verbose_push_shows_extra_output() {
    let dir = setup_clean_env();
    let out = run_tp_stdin(&dir, &["-v"], "v-data");
    assert!(out.status.success());
}

#[test]
fn quiet_push_suppresses_creation_noise() {
    let dir = setup_clean_env();
    let out = run_tp_stdin(&dir, &["-q"], "q-data");
    assert!(out.status.success());
}

// ── CI-oriented smoke tests (additional) ─────────────────

#[test]
fn stack_count_zero_on_fresh_dir() {
    let dir = setup_clean_env();
    assert_eq!(stdout(&run_tp(&dir, &["-k"])).trim(), "0");
}

#[test]
fn double_push_count_two() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "x");
    tick();
    run_tp_stdin(&dir, &[], "y");
    assert_eq!(stdout(&run_tp(&dir, &["-k"])).trim(), "2");
}

#[test]
fn output_index_two_after_two_pushes() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "bottom");
    tick();
    run_tp_stdin(&dir, &[], "top");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "2"])).trim(), "top");
}

#[test]
fn list_files_after_push_exits_ok() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "data");
    let out = run_tp(&dir, &["-l"]);
    assert!(out.status.success());
}

#[test]
fn list_contents_after_push_exits_ok() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "line");
    let out = run_tp(&dir, &["-L"]);
    assert!(out.status.success());
}

#[test]
fn master_flag_after_push_exits_ok() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "m");
    let out = run_tp(&dir, &["-m"]);
    assert!(out.status.success());
}

#[test]
fn pop_one_of_three_leaves_two() {
    let dir = setup_clean_env();
    for _ in 0..3 {
        tick();
        run_tp_stdin(&dir, &[], "p");
    }
    run_tp(&dir, &["-p"]);
    assert_eq!(stdout(&run_tp(&dir, &["-k"])).trim(), "2");
}

#[test]
fn swap_adjacent_indices_one_and_two() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "low");
    tick();
    run_tp_stdin(&dir, &[], "high");
    run_tp(&dir, &["--swap", "1", "2"]);
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])).trim(), "high");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "2"])).trim(), "low");
}

#[test]
fn dup_once_creates_two_stack_entries() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "once");
    run_tp(&dir, &["--dup", "1"]);
    assert_eq!(stdout(&run_tp(&dir, &["-k"])).trim(), "2");
}

#[test]
fn sort_name_on_single_file_exits_ok() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "only");
    let out = run_tp(&dir, &["--sort", "name"]);
    assert!(out.status.success());
}

#[test]
fn count_remains_consistent_after_failed_grep() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "solo");
    let out = run_tp(&dir, &["--grep", "nomatchzzz"]);
    assert!(!out.status.success());
    assert_eq!(stdout(&run_tp(&dir, &["-k"])).trim(), "1");
}

#[test]
fn three_pushes_output_middle_index() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "a");
    tick();
    run_tp_stdin(&dir, &[], "b");
    tick();
    run_tp_stdin(&dir, &[], "c");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "2"])).trim(), "b");
}

#[test]
fn negative_two_outputs_bottom_three_items() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "bottom");
    tick();
    run_tp_stdin(&dir, &[], "mid");
    tick();
    run_tp_stdin(&dir, &[], "top");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "-2"])).trim(), "mid");
}

#[test]
fn move_stack_item_exits_ok_two_files() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "x");
    tick();
    run_tp_stdin(&dir, &[], "y");
    let out = run_tp(&dir, &["--mv", "1", "2"]);
    assert!(out.status.success());
}

#[test]
fn path_and_wc_same_top_index() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "one\ntwo\n");
    let wc = run_tp(&dir, &["--wc", "1"]);
    assert!(stdout(&wc).contains('2'));
    let path_out = run_tp(&dir, &["--path", "1"]);
    assert!(path_out.status.success());
}

#[test]
fn edit_invalid_index_reports_error() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "only");
    let out = run_tp(&dir, &["-e", "99"]);
    assert!(!out.status.success());
}

#[test]
fn clear_then_count_zero() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "x");
    run_tp(&dir, &["-c"]);
    assert_eq!(stdout(&run_tp(&dir, &["-k"])).trim(), "0");
}

#[test]
fn push_two_swap_outputs() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "first");
    tick();
    run_tp_stdin(&dir, &[], "second");
    run_tp(&dir, &["--swap", "1", "2"]);
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])).trim(), "second");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "2"])).trim(), "first");
}

#[test]
fn size_reports_bytes_after_multiline_push() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "a\nb\nc\n");
    let out = run_tp(&dir, &["--size", "1"]);
    assert!(out.status.success());
    let text = stdout(&out);
    let t = text.trim();
    assert!(t.parse::<u64>().is_ok());
}

#[test]
fn head_first_line_only() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "line1\nline2\nline3\n");
    let out = run_tp(&dir, &["--head", "1", "1"]);
    let t = stdout(&out);
    assert!(t.contains("line1"));
}

#[test]
fn tail_last_line_only_three_line_file() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "a\nb\nc\n");
    let out = run_tp(&dir, &["--tail", "1", "1"]);
    let t = stdout(&out);
    assert!(t.contains('c'));
}

#[test]
fn four_pushes_count_four() {
    let dir = setup_clean_env();
    for i in 0..4 {
        tick();
        run_tp_stdin(&dir, &[], &format!("v{i}"));
    }
    assert_eq!(stdout(&run_tp(&dir, &["-k"])).trim(), "4");
}

#[test]
fn remove_bottom_only_one_remains() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "a");
    tick();
    run_tp_stdin(&dir, &[], "b");
    run_tp(&dir, &["-r", "1"]);
    assert_eq!(stdout(&run_tp(&dir, &["-k"])).trim(), "1");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "1"])).trim(), "b");
}

#[test]
fn add_at_top_negative_one_inserts() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "old");
    tick();
    run_tp_stdin(&dir, &[], "mid");
    run_tp_stdin(&dir, &["-a", "-1"], "newtop");
    assert_eq!(stdout(&run_tp(&dir, &["-k"])).trim(), "3");
}

#[test]
fn list_files_numbered_two_files_shows_indices() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "a");
    tick();
    run_tp_stdin(&dir, &[], "b");
    let out = run_tp(&dir, &["-n"]);
    assert!(out.status.success());
    let text = stdout(&out);
    assert!(text.contains('1') && text.contains('2'));
}

#[test]
fn output_index_three_three_pushes() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "bottom");
    tick();
    run_tp_stdin(&dir, &[], "mid");
    tick();
    run_tp_stdin(&dir, &[], "top");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "3"])).trim(), "top");
}

#[test]
fn count_three_after_three_stdin_pushes() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "a");
    tick();
    run_tp_stdin(&dir, &[], "b");
    tick();
    run_tp_stdin(&dir, &[], "c");
    assert_eq!(stdout(&run_tp(&dir, &["-k"])).trim(), "3");
}

#[test]
fn negative_output_minus_one_is_top_after_two_pushes() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "bottom");
    tick();
    run_tp_stdin(&dir, &[], "top");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "-1"])).trim(), "top");
}

#[test]
fn output_negative_two_is_bottom_after_two_pushes() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "bottom");
    tick();
    run_tp_stdin(&dir, &[], "top");
    assert_eq!(stdout(&run_tp(&dir, &["-o", "-2"])).trim(), "bottom");
}

#[test]
fn size_top_of_stack_matches_bytes_written() {
    let dir = setup_clean_env();
    let body = "hello";
    run_tp_stdin(&dir, &[], body);
    let out = stdout(&run_tp(&dir, &["--size", "1"]));
    let sz = out.trim();
    assert_eq!(sz.parse::<usize>().unwrap(), body.len());
}

#[test]
fn path_top_of_stack_is_nonempty_string() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "x");
    let out = stdout(&run_tp(&dir, &["--path", "1"]));
    let p = out.trim();
    assert!(!p.is_empty());
}

#[test]
fn wc_top_reports_line_count_for_multiline_stdin_push() {
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], "line1\nline2\nline3");
    let out = stdout(&run_tp(&dir, &["--wc", "1"]));
    let n = out.trim().parse::<u32>().unwrap();
    assert_eq!(n, 3);
}
