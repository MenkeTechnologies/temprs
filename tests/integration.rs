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
