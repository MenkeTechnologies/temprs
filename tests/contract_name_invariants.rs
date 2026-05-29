//! Contract tests for name-handling and stack-invariant edge cases not covered
//! by `tests/integration.rs` or `tests/edge_cases.rs`.
//!
//! Targets:
//! - `-w` accepts names containing dashes; lookup by dash-name round-trips
//! - `-w` accepts names containing spaces; lookup by spaced-name round-trips
//! - Purely-numeric names (`-w 1234`) collide with index resolution — pinning
//!   the current behavior so any future shift is intentional.
//! - `--rev --rev` is the identity transformation on stack content
//! - `--clear --clear` is idempotent (clear twice = clear once)
//! - `--count` on empty stack reports 0 (not error)
//! - `-D` diff of an entry against itself by index exits 0
//! - `-D` diff with a non-existent index exits non-zero (clean error path)

use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};

static COUNTER: AtomicU64 = AtomicU64::new(0);

fn bin() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_BIN_EXE_tp"));
    if !p.exists() {
        p = PathBuf::from(env!("CARGO_BIN_EXE_temprs"));
    }
    p
}

fn clean_dir() -> PathBuf {
    let id = COUNTER.fetch_add(1, Ordering::SeqCst);
    let d = std::env::temp_dir().join(format!("temprs_contract_{}_{}", std::process::id(), id));
    let _ = fs::remove_dir_all(&d);
    d
}

fn run(dir: &PathBuf, args: &[&str]) -> std::process::Output {
    Command::new(bin())
        .env("TEMPRS_DIR", dir)
        .args(args)
        .stdin(Stdio::null())
        .output()
        .expect("spawn tp")
}

fn run_stdin(dir: &PathBuf, args: &[&str], input: &[u8]) -> std::process::Output {
    use std::io::Write;
    let mut child = Command::new(bin())
        .env("TEMPRS_DIR", dir)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn tp");
    child.stdin.as_mut().unwrap().write_all(input).unwrap();
    child.wait_with_output().unwrap()
}

fn stdout_str(o: &std::process::Output) -> String {
    String::from_utf8_lossy(&o.stdout).to_string()
}

#[test]
fn test_name_with_dash_roundtrips_via_output() {
    let dir = clean_dir();
    run_stdin(&dir, &["-w", "with-dash"], b"dash-content");
    let out = run(&dir, &["-o", "with-dash"]);
    assert!(
        out.status.success(),
        "output by dash-name must succeed; stderr={:?}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert_eq!(
        stdout_str(&out),
        "dash-content",
        "name with dash must resolve to the tagged tempfile"
    );
}

#[test]
fn test_name_with_space_roundtrips_via_output() {
    let dir = clean_dir();
    run_stdin(&dir, &["-w", "with space"], b"space-content");
    let out = run(&dir, &["-o", "with space"]);
    assert!(
        out.status.success(),
        "output by name-with-space must succeed; stderr={:?}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert_eq!(
        stdout_str(&out),
        "space-content",
        "name with space must resolve to the tagged tempfile"
    );
}

#[test]
fn test_purely_numeric_name_collides_with_index_lookup() {
    // `-w 1234` is accepted (push succeeds), but `-o 1234` parses 1234 as an
    // index, not a name. Stack has only one entry, so index 1234 is invalid.
    // Pinning current behavior so any future change is deliberate.
    let dir = clean_dir();
    let push = run_stdin(&dir, &["-w", "1234"], b"num-name");
    assert!(
        push.status.success(),
        "push with numeric-string name succeeds"
    );
    let out = run(&dir, &["-o", "1234"]);
    assert!(
        !out.status.success(),
        "`-o 1234` resolves 1234 as index, not as name; expected failure"
    );
}

#[test]
fn test_rev_is_involutive_on_three_entries() {
    let dir = clean_dir();
    run_stdin(&dir, &[], b"first");
    run_stdin(&dir, &[], b"second");
    run_stdin(&dir, &[], b"third");
    // Capture order before
    let before = stdout_str(&run(&dir, &["-l"]));
    run(&dir, &["--rev"]);
    run(&dir, &["--rev"]);
    let after = stdout_str(&run(&dir, &["-l"]));
    assert_eq!(
        before, after,
        "two reverses should restore the original stack order"
    );
}

#[test]
fn test_clear_is_idempotent() {
    let dir = clean_dir();
    run_stdin(&dir, &[], b"a");
    run_stdin(&dir, &[], b"b");
    let r1 = run(&dir, &["-c"]);
    let r2 = run(&dir, &["-c"]);
    assert_eq!(
        r1.status.code(),
        r2.status.code(),
        "second --clear must produce the same exit code as the first"
    );
    let count = stdout_str(&run(&dir, &["-k"]));
    assert_eq!(
        count.trim(),
        "0",
        "after double-clear the stack must be empty"
    );
}

#[test]
fn test_count_on_empty_stack_is_zero() {
    let dir = clean_dir();
    let out = run(&dir, &["-k"]);
    assert!(
        out.status.success(),
        "count on empty stack must succeed (not error); stderr={:?}",
        String::from_utf8_lossy(&out.stderr)
    );
    let n = stdout_str(&out);
    assert_eq!(n.trim(), "0", "empty stack count must be 0, got {n:?}");
}

#[test]
fn test_diff_entry_against_itself_exits_zero() {
    let dir = clean_dir();
    run_stdin(&dir, &[], b"identical");
    let out = run(&dir, &["-D", "1", "1"]);
    assert_eq!(
        out.status.code(),
        Some(0),
        "diff of entry-vs-itself must exit 0; stderr={:?}",
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn test_diff_with_nonexistent_index_fails_cleanly() {
    let dir = clean_dir();
    run_stdin(&dir, &[], b"only-one");
    let out = run(&dir, &["-D", "1", "99"]);
    assert!(
        !out.status.success(),
        "diff against index 99 on a 1-element stack must fail"
    );
    assert!(
        out.status.code().is_some(),
        "process must exit with a code (no panic/signal); got {:?}",
        out.status
    );
}
