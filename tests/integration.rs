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

/// Small delay to avoid tempfile name collisions (ms-resolution timestamp)
fn tick() {
    std::thread::sleep(std::time::Duration::from_millis(10));
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
    let out = run_tp(&["-h"]);
    let text = stdout(&out);
    assert!(text.contains("████████╗"), "missing ASCII art banner");
    assert!(text.contains("STATUS: ONLINE"), "missing status bar");
    assert!(text.contains("TEMPORARY FILE STACK MANAGER"), "missing tagline");
    assert!(text.contains("JACK IN"), "missing cyberpunk footer");
}

#[test]
fn help_shows_all_flags() {
    let out = run_tp(&["-h"]);
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
    let out = run_tp(&["-V"]);
    let text = stdout(&out);
    assert!(text.contains("0.5.2"));
}

// ── Clear ───────────────────────────────────────────────

#[test]
fn clear_removes_all() {
    setup_clean_env();
    // push some data first
    run_tp_stdin(&[], "data1");
    run_tp_stdin(&[], "data2");
    // clear
    let out = run_tp(&["-c"]);
    assert!(out.status.success());
    // after clear, list should be empty (dir is gone, so it'll error or be empty)
    let list = run_tp(&["-l"]);
    let text = stdout(&list);
    assert!(text.trim().is_empty());
}

// ── Push and read back ──────────────────────────────────

#[test]
fn push_stdin_and_read_top() {
    setup_clean_env();
    // push
    run_tp_stdin(&[], "hello from test\n");
    // read top of stack
    // read top of stack via -o
    let out = run_tp(&["-o", "1"]);
    let text = stdout(&out);
    assert_eq!(text, "hello from test\n");
}

#[test]
fn push_multiple_and_list() {
    setup_clean_env();
    run_tp_stdin(&[], "first");
    run_tp_stdin(&[], "second");
    run_tp_stdin(&[], "third");
    let out = run_tp(&["-l"]);
    let text = stdout(&out);
    let lines: Vec<&str> = text.trim().lines().collect();
    assert_eq!(lines.len(), 3, "expected 3 files on stack");
}

// ── Output at index ─────────────────────────────────────

#[test]
fn output_at_index() {
    setup_clean_env();
    run_tp_stdin(&[], "aaa");
    run_tp_stdin(&[], "bbb");
    run_tp_stdin(&[], "ccc");

    let out1 = run_tp(&["-o", "1"]);
    assert_eq!(stdout(&out1), "aaa");

    let out2 = run_tp(&["-o", "2"]);
    assert_eq!(stdout(&out2), "bbb");

    let out3 = run_tp(&["-o", "3"]);
    assert_eq!(stdout(&out3), "ccc");
}

#[test]
fn output_negative_index_top() {
    setup_clean_env();
    run_tp_stdin(&[], "first");
    tick();
    run_tp_stdin(&[], "last");

    // -1 should be top of stack (last)
    let out = run_tp(&["-o", "-1"]);
    assert_eq!(stdout(&out), "last");
}

#[test]
fn output_negative_index_bottom() {
    setup_clean_env();
    run_tp_stdin(&[], "first");
    tick();
    run_tp_stdin(&[], "last");

    // -2 should be bottom of stack (first)
    let out = run_tp(&["-o", "-2"]);
    assert_eq!(stdout(&out), "first");
}

// ── Input (overwrite) at index ──────────────────────────

#[test]
fn input_overwrites_at_index() {
    setup_clean_env();
    run_tp_stdin(&[], "original");
    run_tp_stdin(&["-i", "1"], "replaced");
    let out = run_tp(&["-o", "1"]);
    assert_eq!(stdout(&out), "replaced");
}

// ── Pop ─────────────────────────────────────────────────

#[test]
fn pop_removes_top() {
    setup_clean_env();
    run_tp_stdin(&[], "bottom");
    run_tp_stdin(&[], "top");

    let list_before = run_tp(&["-l"]);
    let count_before = stdout(&list_before).trim().lines().count();
    assert_eq!(count_before, 2);

    run_tp(&["-p"]);

    let list_after = run_tp(&["-l"]);
    let count_after = stdout(&list_after).trim().lines().count();
    assert_eq!(count_after, 1);

    // remaining should be bottom
    let out = run_tp(&["-o", "1"]);
    assert_eq!(stdout(&out), "bottom");
}

// ── Shift ───────────────────────────────────────────────

#[test]
fn shift_removes_bottom() {
    setup_clean_env();
    run_tp_stdin(&[], "bottom");
    run_tp_stdin(&[], "top");
    run_tp(&["-s"]);

    let list = run_tp(&["-l"]);
    let count = stdout(&list).trim().lines().count();
    assert_eq!(count, 1);

    let out = run_tp(&["-o", "1"]);
    assert_eq!(stdout(&out), "top");
}

// ── Unshift ─────────────────────────────────────────────

#[test]
fn unshift_adds_to_bottom() {
    setup_clean_env();
    run_tp_stdin(&[], "existing");
    run_tp_stdin(&["-u"], "new_bottom");

    let out = run_tp(&["-o", "1"]);
    assert_eq!(stdout(&out), "new_bottom");

    let out = run_tp(&["-o", "2"]);
    assert_eq!(stdout(&out), "existing");
}

// ── Add at index ────────────────────────────────────────

#[test]
fn add_inserts_at_index() {
    setup_clean_env();
    run_tp_stdin(&[], "first");
    run_tp_stdin(&[], "third");
    run_tp_stdin(&["-a", "2"], "second");

    let out = run_tp(&["-o", "1"]);
    assert_eq!(stdout(&out), "first");

    let out = run_tp(&["-o", "2"]);
    assert_eq!(stdout(&out), "second");

    let out = run_tp(&["-o", "3"]);
    assert_eq!(stdout(&out), "third");
}

// ── Remove at index ─────────────────────────────────────

#[test]
fn remove_at_index() {
    setup_clean_env();
    run_tp_stdin(&[], "aaa");
    run_tp_stdin(&[], "bbb");
    run_tp_stdin(&[], "ccc");
    run_tp(&["-r", "2"]);

    let list = run_tp(&["-l"]);
    let count = stdout(&list).trim().lines().count();
    assert_eq!(count, 2);

    let out1 = run_tp(&["-o", "1"]);
    assert_eq!(stdout(&out1), "aaa");

    let out2 = run_tp(&["-o", "2"]);
    assert_eq!(stdout(&out2), "ccc");
}

// ── List contents ───────────────────────────────────────

#[test]
fn list_contents_shows_data() {
    setup_clean_env();
    run_tp_stdin(&[], "hello");
    run_tp_stdin(&[], "world");

    let out = run_tp(&["-L"]);
    let text = stdout(&out);
    assert!(text.contains("hello"));
    assert!(text.contains("world"));
}

#[test]
fn list_contents_numbered_shows_indices() {
    setup_clean_env();
    run_tp_stdin(&[], "alpha");
    run_tp_stdin(&[], "beta");

    let out = run_tp(&["-N"]);
    let text = stdout(&out);
    assert!(text.contains("1:"));
    assert!(text.contains("2:"));
    assert!(text.contains("alpha"));
    assert!(text.contains("beta"));
}

#[test]
fn list_files_numbered_shows_indices() {
    setup_clean_env();
    run_tp_stdin(&[], "data1");
    run_tp_stdin(&[], "data2");

    let out = run_tp(&["-n"]);
    let text = stdout(&out);
    assert!(text.contains("1:"));
    assert!(text.contains("2:"));
}

// ── Dir and master ──────────────────────────────────────

#[test]
fn dir_flag_shows_directory() {
    setup_clean_env();
    // push something to ensure dir exists
    run_tp_stdin(&[], "x");
    let out = run_tp(&["-d"]);
    let text = stdout(&out);
    assert!(text.contains("temprs"), "dir output should contain 'temprs'");
}

#[test]
fn master_flag_shows_master_file() {
    setup_clean_env();
    run_tp_stdin(&[], "x");
    let out = run_tp(&["-m"]);
    let text = stdout(&out);
    assert!(
        text.contains("temprs-stack"),
        "master output should contain 'temprs-stack'"
    );
}

// ── Verbose flag with stdin ─────────────────────────────

#[test]
fn verbose_echoes_stdin() {
    setup_clean_env();
    let out = run_tp_stdin(&["-v"], "echo me");
    let text = stdout(&out);
    assert!(text.contains("echo me"), "verbose should echo stdin to stdout");
}

// ── File argument ───────────────────────────────────────
// Note: file argument only works when stdin is a real terminal (atty check).
// In test harness stdin is never a terminal, so we pipe file content
// through stdin instead to test equivalent functionality.

#[test]
fn file_content_via_stdin() {
    setup_clean_env();
    let content = "file content here";
    run_tp_stdin(&[], content);

    let out = run_tp(&["-o", "1"]);
    assert_eq!(stdout(&out), content);
}

#[test]
fn file_content_via_stdin_with_verbose() {
    setup_clean_env();
    let out = run_tp_stdin(&["-v"], "verbose file");
    let text = stdout(&out);
    assert!(text.contains("verbose file"));
}

// ── Edge cases ──────────────────────────────────────────

#[test]
fn empty_stack_list_is_empty() {
    setup_clean_env();
    let out = run_tp(&["-l"]);
    let text = stdout(&out);
    assert!(text.trim().is_empty());
}

#[test]
fn push_empty_string() {
    setup_clean_env();
    run_tp_stdin(&[], "");
    let out = run_tp(&["-o", "1"]);
    assert_eq!(stdout(&out), "");
}

#[test]
fn push_multiline_content() {
    setup_clean_env();
    run_tp_stdin(&[], "line1\nline2\nline3\n");
    let out = run_tp(&["-o", "1"]);
    assert_eq!(stdout(&out), "line1\nline2\nline3\n");
}

#[test]
fn push_unicode_content() {
    setup_clean_env();
    run_tp_stdin(&[], "hello 世界 🚀");
    let out = run_tp(&["-o", "1"]);
    assert_eq!(stdout(&out), "hello 世界 🚀");
}

// ── Invalid index exits with error ──────────────────────

#[test]
fn invalid_index_zero_errors() {
    setup_clean_env();
    run_tp_stdin(&[], "data");
    let out = run_tp(&["-o", "0"]);
    assert!(!out.status.success());
}

#[test]
fn index_out_of_bounds_errors() {
    setup_clean_env();
    run_tp_stdin(&[], "only one");
    let out = run_tp(&["-o", "5"]);
    assert!(!out.status.success());
}
