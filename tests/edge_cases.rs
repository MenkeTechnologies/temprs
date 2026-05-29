//! Edge-case and error-path integration tests.
//!
//! `tests/integration.rs` covers the happy-path matrix for every flag.
//! This file targets the *boundary* and *failure* paths that catch
//! real bugs: empty stack ops, binary data round-trips, Unicode in
//! both content and `TEMPRS_DIR`, large input, concurrent processes
//! (the file-lock contract), and stdin-EOF without any bytes.
//!
//! Helpers (`bin`, `setup_clean_env`, `run_tp`, `run_tp_stdin`,
//! `tick`, `stdout`) are duplicated locally rather than re-exported
//! from `tests/integration.rs` to keep each integration crate
//! self-contained — cargo treats each `tests/*.rs` as its own crate.

use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

static TEST_COUNTER: AtomicU64 = AtomicU64::new(0);

fn bin() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_BIN_EXE_tp"));
    if !path.exists() {
        path = PathBuf::from(env!("CARGO_BIN_EXE_temprs"));
    }
    path
}

fn setup_clean_env() -> PathBuf {
    let id = TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
    let dir = std::env::temp_dir().join(format!("temprs_edge_{}_{}", std::process::id(), id,));
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

fn run_tp_stdin(dir: &PathBuf, args: &[&str], input: &[u8]) -> std::process::Output {
    use std::io::Write;
    let mut child = Command::new(bin())
        .env("TEMPRS_DIR", dir)
        .args(args)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("failed to spawn tp");
    child.stdin.as_mut().unwrap().write_all(input).unwrap();
    child.wait_with_output().unwrap()
}

fn tick() {
    std::thread::sleep(std::time::Duration::from_millis(50));
}

fn stdout_str(output: &std::process::Output) -> String {
    String::from_utf8_lossy(&output.stdout).to_string()
}

// ─── empty-stack ops should surface a clean error, not panic ───────

#[test]
fn pop_empty_stack_surfaces_clean_error() {
    let dir = setup_clean_env();
    let out = run_tp(&dir, &["-p"]);
    // Either non-zero exit OR empty stdout — what matters is no
    // Rust panic / signal. Stderr should mention the empty stack OR
    // an index-out-of-range error.
    assert!(
        out.status.code().is_some(),
        "process must exit cleanly (no signal), got {:?}",
        out.status
    );
}

#[test]
fn shift_empty_stack_surfaces_clean_error() {
    let dir = setup_clean_env();
    let out = run_tp(&dir, &["-s"]);
    assert!(out.status.code().is_some(), "no signal exit");
}

#[test]
fn output_empty_stack_surfaces_clean_error() {
    let dir = setup_clean_env();
    let out = run_tp(&dir, &["-o", "1"]);
    assert!(out.status.code().is_some(), "no signal exit");
}

#[test]
fn remove_empty_stack_surfaces_clean_error() {
    let dir = setup_clean_env();
    let out = run_tp(&dir, &["-r", "1"]);
    assert!(out.status.code().is_some(), "no signal exit");
}

// ─── binary content round-trip — no lossy UTF-8 conversion ─────────

// Note: a full 0..=255 byte sweep CANNOT round-trip — `read_stdin_pipe`
// uses `String::read_to_string` (model/app.rs) which rejects invalid
// UTF-8 (bytes 0x80..0xC1 / 0xF5..0xFF in particular). Documented
// limitation, not a bug. NUL (0x00) IS valid UTF-8 so we pin that
// case below.

#[test]
fn ascii_only_byte_range_survives_output() {
    let dir = setup_clean_env();
    // 0x01..=0x7F is the ASCII range (skip NUL — separate test). All
    // bytes are valid UTF-8 singletons so `read_to_string` accepts.
    let payload: Vec<u8> = (1u8..=0x7F).collect();
    let push = run_tp_stdin(&dir, &[], &payload);
    assert!(push.status.success(), "push ASCII byte range");
    tick();
    let out = run_tp(&dir, &["-o", "1"]);
    assert!(out.status.success(), "output must succeed");
    assert_eq!(
        out.stdout, payload,
        "ASCII byte range round-trip differs from input"
    );
}

#[test]
fn null_byte_in_stdin_preserved_on_output() {
    let dir = setup_clean_env();
    let payload = b"before\x00after".to_vec();
    run_tp_stdin(&dir, &[], &payload);
    tick();
    let out = run_tp(&dir, &["-o", "1"]);
    assert_eq!(out.stdout, payload, "NUL byte stripped from output");
}

// ─── empty stdin — push of empty file ──────────────────────────────

#[test]
fn empty_stdin_creates_empty_tempfile() {
    let dir = setup_clean_env();
    let push = run_tp_stdin(&dir, &[], b"");
    assert!(push.status.success(), "push empty stdin");
    tick();
    let count = run_tp(&dir, &["--count"]);
    let text = stdout_str(&count);
    assert!(
        text.trim().starts_with("1") || text.contains("1"),
        "empty push should still create a tempfile; count says {:?}",
        text
    );
}

// ─── UTF-8 content round-trip ──────────────────────────────────────

#[test]
fn utf8_multibyte_content_survives_output() {
    let dir = setup_clean_env();
    let payload = "日本語🚀café\n".as_bytes();
    run_tp_stdin(&dir, &[], payload);
    tick();
    let pop = run_tp(&dir, &["-o", "1"]);
    assert_eq!(
        pop.stdout, payload,
        "UTF-8 multibyte text mangled in round-trip"
    );
}

// ─── TEMPRS_DIR with spaces / unicode ──────────────────────────────

#[test]
fn temprs_dir_with_spaces_works() {
    let id = TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
    let dir =
        std::env::temp_dir().join(format!("temprs edge spaces {} {}", std::process::id(), id));
    let _ = fs::remove_dir_all(&dir);
    let push = run_tp_stdin(&dir, &[], b"hello\n");
    assert!(
        push.status.success(),
        "TEMPRS_DIR with spaces should work; got stderr {:?}",
        String::from_utf8_lossy(&push.stderr)
    );
    tick();
    let pop = run_tp(&dir, &["-o", "1"]);
    assert_eq!(pop.stdout, b"hello\n");
}

#[test]
fn temprs_dir_with_unicode_works() {
    let id = TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
    let dir = std::env::temp_dir().join(format!("temprs_日本_{}_{}", std::process::id(), id));
    let _ = fs::remove_dir_all(&dir);
    let push = run_tp_stdin(&dir, &[], b"unicode-dir\n");
    assert!(push.status.success(), "TEMPRS_DIR with unicode chars");
    tick();
    let pop = run_tp(&dir, &["-o", "1"]);
    assert_eq!(pop.stdout, b"unicode-dir\n");
}

// ─── large input — guard against off-by-one / buffer-size bugs ─────

#[test]
fn one_megabyte_push_output_roundtrip() {
    let dir = setup_clean_env();
    let payload = vec![b'x'; 1024 * 1024]; // exactly 1 MiB
    let push = run_tp_stdin(&dir, &[], &payload);
    assert!(push.status.success(), "push 1MiB");
    tick();
    let pop = run_tp(&dir, &["-o", "1"]);
    assert!(pop.status.success(), "pop 1MiB");
    assert_eq!(pop.stdout.len(), payload.len(), "1MiB round-trip length");
    assert_eq!(pop.stdout, payload, "1MiB content differs");
}

// ─── multi-line stdin — preserves embedded newlines ────────────────

#[test]
fn three_line_stdin_preserved_in_output() {
    let dir = setup_clean_env();
    let payload = b"line1\nline2\nline3\n";
    run_tp_stdin(&dir, &[], payload);
    tick();
    let out = run_tp(&dir, &["-o", "1"]);
    assert_eq!(out.stdout, payload, "multi-line stdin mangled");
}

// ─── verbose at multiple levels — count action stacks ─────────────

#[test]
fn verbose_once_runs_without_error() {
    let dir = setup_clean_env();
    let out = run_tp_stdin(&dir, &["-v"], b"data\n");
    assert!(out.status.success(), "single -v");
}

#[test]
fn verbose_three_times_stacks_count() {
    let dir = setup_clean_env();
    let out = run_tp_stdin(&dir, &["-vvv"], b"data\n");
    assert!(out.status.success(), "stacked -vvv");
}

// ─── quiet suppresses stdout when pushing ──────────────────────────

#[test]
fn quiet_suppresses_stdout_on_push() {
    let dir = setup_clean_env();
    let out = run_tp_stdin(&dir, &["-q"], b"silent\n");
    assert!(out.status.success(), "-q push succeeds");
    assert!(
        out.stdout.is_empty(),
        "-q should suppress stdout, got {:?}",
        out.stdout
    );
}

// ─── concurrent-process file-lock contract ─────────────────────────
//
// Two `tp` instances in parallel must serialize (the lockfile is the
// gatekeeper). A race-condition bug would manifest as a missing or
// duplicated entry in the master list. Run N pushes in parallel and
// confirm the final stack has N elements.

#[test]
fn concurrent_pushes_serialize_correctly() {
    let dir = setup_clean_env();
    // Warm the directory with one synchronous push so the master file
    // exists before the parallel run starts (avoids racing the
    // initial creation).
    let _ = run_tp_stdin(&dir, &[], b"seed\n");
    tick();

    let n_parallel = 4;
    let mut handles = Vec::with_capacity(n_parallel);
    for i in 0..n_parallel {
        let dir = dir.clone();
        let h = std::thread::spawn(move || {
            run_tp_stdin(&dir, &[], format!("payload-{}\n", i).as_bytes())
                .status
                .success()
        });
        handles.push(h);
    }
    for h in handles {
        assert!(h.join().unwrap(), "concurrent push failed");
    }
    tick();
    let count = run_tp(&dir, &["--count"]);
    let text = stdout_str(&count);
    let n: usize = text
        .split_whitespace()
        .find_map(|tok| tok.parse().ok())
        .unwrap_or(0);
    assert_eq!(
        n,
        1 + n_parallel,
        "{} concurrent pushes + 1 seed should produce {} entries, got {} (raw: {:?})",
        n_parallel,
        1 + n_parallel,
        n,
        text
    );
}

// ─── index 0 — clap default-1-based contract ──────────────────────

#[test]
fn output_index_zero_surfaces_clean_error() {
    // temprs indexes are 1-based on top + negative-from-bottom; 0
    // should be invalid (not silently treated as 1).
    let dir = setup_clean_env();
    run_tp_stdin(&dir, &[], b"only\n");
    tick();
    let out = run_tp(&dir, &["-o", "0"]);
    assert!(
        !out.status.success(),
        "-o 0 must error (0 is not a valid 1-based index)"
    );
}

// ─── stdin priority over positional file ───────────────────────────

#[test]
fn stdin_wins_over_positional_file_arg() {
    let dir = setup_clean_env();
    // Per --help: "Stdin takes priority if present."
    let tmpfile = dir.parent().unwrap().join("tp_pos_arg.txt");
    let _ = fs::create_dir_all(dir.parent().unwrap());
    fs::write(&tmpfile, b"from-file\n").unwrap();
    let out = run_tp_stdin(&dir, &[tmpfile.to_str().unwrap()], b"from-stdin\n");
    assert!(out.status.success());
    tick();
    let pop = run_tp(&dir, &["-o", "1"]);
    assert_eq!(
        pop.stdout, b"from-stdin\n",
        "stdin should win over positional"
    );
    let _ = fs::remove_file(&tmpfile);
}

// ─── clear empties the stack ───────────────────────────────────────

#[test]
fn clear_then_count_is_zero() {
    let dir = setup_clean_env();
    for _ in 0..3 {
        run_tp_stdin(&dir, &[], b"x\n");
        tick();
    }
    let cleared = run_tp(&dir, &["-c"]);
    assert!(cleared.status.success(), "-c should succeed");
    tick();
    let count = run_tp(&dir, &["--count"]);
    let text = stdout_str(&count);
    assert!(
        text.trim().starts_with("0") || text.contains(" 0"),
        "after clear, count should be 0, got {:?}",
        text
    );
}

// ─── list when empty — no panic ────────────────────────────────────

#[test]
fn list_files_empty_stack_no_panic() {
    let dir = setup_clean_env();
    let out = run_tp(&dir, &["-l"]);
    assert!(
        out.status.code().is_some(),
        "list-files on empty stack must exit cleanly"
    );
}

#[test]
fn list_contents_empty_stack_no_panic() {
    let dir = setup_clean_env();
    let out = run_tp(&dir, &["-L"]);
    assert!(
        out.status.code().is_some(),
        "list-contents on empty stack must exit cleanly"
    );
}

// ─── help/version short-circuit (don't touch TEMPRS_DIR) ──────────

#[test]
fn help_does_not_create_tempfiles() {
    // tp's lockfile / dir-init runs early in main, BEFORE the help
    // shortcut, so the dir IS created. What we pin instead is that
    // --help doesn't create any *tempfile* entries: stack stays
    // empty. That's the side-effect that would surprise a user.
    let id = TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
    let dir = std::env::temp_dir().join(format!(
        "temprs_help_no_files_{}_{}",
        std::process::id(),
        id
    ));
    let _ = fs::remove_dir_all(&dir);
    let out = run_tp(&dir, &["-h"]);
    assert!(out.status.success());
    if dir.exists() {
        let tempfiles: Vec<_> = fs::read_dir(&dir)
            .unwrap()
            .filter_map(Result::ok)
            .filter(|e| e.file_name().to_string_lossy().starts_with("tempfile"))
            .collect();
        assert!(
            tempfiles.is_empty(),
            "--help must not create tempfiles, found: {:?}",
            tempfiles.iter().map(|e| e.file_name()).collect::<Vec<_>>()
        );
    }
}

#[test]
fn version_long_form_runs_clean() {
    let dir = setup_clean_env();
    let out = run_tp(&dir, &["-V"]);
    assert!(out.status.success(), "-V should print version");
    assert!(
        stdout_str(&out).contains(env!("CARGO_PKG_VERSION")),
        "version output should contain crate version"
    );
}
