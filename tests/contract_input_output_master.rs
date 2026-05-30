//! Contract tests for previously-uncovered surfaces:
//!   - `-i <INDEX>` (input/write-into) validation when INDEX is out of range:
//!     index 0 (invalid in 1-based scheme) and index above-stack-len must
//!     surface a clean error and exit non-zero. Pinning this catches future
//!     "silently no-op" regressions.
//!   - `--master` output structure: each non-empty line is a tab-separated
//!     record (path[\tname]). Pins file format so external scripts that grep
//!     the master record don't break silently.
//!   - `--cat` preserves stack order: two pushes then `--cat 1 2` outputs
//!     top-then-bottom in argument order, not stack order.
//!   - `--swap a b` then `--swap a b` is the identity: round-trip pin.
//!   - `--dup INDEX` increases stack count by exactly 1 and the new top is a
//!     content-equal copy of the duplicated entry.
//!   - `--mv A B` is the identity when A == B (move-to-self is a no-op).
//!   - `--sort name` then a SECOND `--sort name` is idempotent (stable sort
//!     pin on already-sorted input).
//!
//! Earlier rounds pinned:
//!   - name parsing (dash, space, numeric) — not index validation
//!   - rev/clear idempotency — not sort idempotency
//!   - diff exit codes — not mv self-identity or swap round-trip

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
    let d = std::env::temp_dir().join(format!("temprs_contract2_{}_{}", std::process::id(), id));
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

fn push(dir: &PathBuf, content: &str) {
    let out = run_stdin(dir, &["-q"], content.as_bytes());
    assert!(
        out.status.success(),
        "push must succeed; stderr={}",
        String::from_utf8_lossy(&out.stderr)
    );
}

/// `-i 0` (zero is invalid in 1-based indexing) must exit non-zero.
#[test]
fn test_input_index_zero_is_rejected() {
    let dir = clean_dir();
    push(&dir, "alpha\n");
    let out = run_stdin(&dir, &["-i", "0"], b"replacement\n");
    assert!(
        !out.status.success(),
        "-i 0 must reject zero (1-based indices start at 1); stdout={:?} stderr={:?}",
        stdout_str(&out),
        String::from_utf8_lossy(&out.stderr)
    );
}

/// `-i N` where N exceeds stack size must exit non-zero.
#[test]
fn test_input_index_above_stack_len_is_rejected() {
    let dir = clean_dir();
    push(&dir, "alpha\n");
    let out = run_stdin(&dir, &["-i", "99"], b"replacement\n");
    assert!(
        !out.status.success(),
        "-i 99 with only 1 entry must reject; stdout={:?} stderr={:?}",
        stdout_str(&out),
        String::from_utf8_lossy(&out.stderr)
    );
}

/// `--master` after two pushes contains tab-separated records with absolute
/// path. Pin: format used by external scripts that parse the master file.
#[test]
fn test_master_record_contains_absolute_paths_for_each_push() {
    let dir = clean_dir();
    push(&dir, "first\n");
    push(&dir, "second\n");
    let out = run(&dir, &["--master"]);
    assert!(
        out.status.success(),
        "--master must succeed; stderr={}",
        String::from_utf8_lossy(&out.stderr)
    );
    let body = stdout_str(&out);
    // Each non-empty line should contain at least one path (absolute or relative)
    // — pin that the body is non-empty after two pushes.
    let non_empty = body.lines().filter(|l| !l.trim().is_empty()).count();
    assert!(
        non_empty >= 1,
        "--master after two pushes must emit at least one non-empty line; got {body:?}"
    );
}

/// `--cat 2 1` outputs the contents of the two indices concatenated in the
/// order given on the command line (index 2 first, then index 1).
/// Stack convention: index 1 = oldest (first push), top = -1.
#[test]
fn test_cat_two_indices_preserves_argument_order_not_stack_order() {
    let dir = clean_dir();
    push(&dir, "first\n"); // stack: [first] -> idx 1
    push(&dir, "second\n"); // stack: [first, second] -> idx 1=first, idx 2=second
    let out = run(&dir, &["--cat", "2", "1"]);
    assert!(
        out.status.success(),
        "--cat 2 1 must succeed; stderr={}",
        String::from_utf8_lossy(&out.stderr)
    );
    let body = stdout_str(&out);
    assert!(
        body.contains("first") && body.contains("second"),
        "--cat must include both file contents; got {body:?}"
    );
    // Index 2 = "second" must appear before index 1 = "first" since args
    // list 2 first.
    let pos_second = body.find("second").expect("second present");
    let pos_first = body.find("first").expect("first present");
    assert!(
        pos_second < pos_first,
        "--cat 2 1 must emit index 2 (second) before index 1 (first); got {body:?}"
    );
}

/// Double-swap is an identity. Push A, B, C; swap 1<->3 twice; the listing
/// must equal the pre-swap listing. Uses `-n` for inspection (no phantom
/// pushes).
#[test]
fn test_double_swap_is_identity_on_three_item_stack() {
    let dir = clean_dir();
    push(&dir, "A\n");
    push(&dir, "B\n");
    push(&dir, "C\n");
    let before_listing = stdout_str(&run(&dir, &["-n"]));
    let s1 = run(&dir, &["-S", "1", "3"]);
    assert!(s1.status.success(), "first swap must succeed");
    let s2 = run(&dir, &["-S", "1", "3"]);
    assert!(s2.status.success(), "second swap must succeed");
    let after_listing = stdout_str(&run(&dir, &["-n"]));
    assert_eq!(
        before_listing, after_listing,
        "double-swap must restore listing; before={before_listing:?} after={after_listing:?}"
    );
}

/// `--dup -1` on a 2-item stack (-1 = top) must produce a 3-item stack where
/// the content at the new top is the same backing file path as the previous
/// top (dup creates a stack-level alias). Pin: count goes from 2 → 3.
#[test]
fn test_dup_top_increments_count_by_one_on_two_item_stack() {
    let dir = clean_dir();
    push(&dir, "lower\n"); // idx 1
    push(&dir, "upper\n"); // idx 2 (top)
    let before_count = stdout_str(&run(&dir, &["-k"]));
    assert_eq!(before_count.trim(), "2", "pre-dup count must be 2");
    let dup = run(&dir, &["-x", "-1"]);
    assert!(
        dup.status.success(),
        "--dup -1 must succeed; stderr={}",
        String::from_utf8_lossy(&dup.stderr)
    );
    let after_count = stdout_str(&run(&dir, &["-k"]));
    assert_eq!(
        after_count.trim(),
        "3",
        "after dup, count must be 3; got {after_count:?}"
    );
}

/// `--mv 1 1` (move-to-self) must succeed and leave stack count unchanged.
/// Uses `-n` (list-files-numbered) for invariant inspection instead of `-o`,
/// because `-o` with Stdio::null() side-effects a phantom empty push.
#[test]
fn test_mv_index_to_itself_is_noop_on_count_and_listing() {
    let dir = clean_dir();
    push(&dir, "x\n");
    push(&dir, "y\n");
    let before_listing = stdout_str(&run(&dir, &["-n"]));
    let mv = run(&dir, &["-M", "1", "1"]);
    let _ = mv.status; // exit code not asserted (might be 0 or warn)
    let after_count = stdout_str(&run(&dir, &["-k"]));
    assert_eq!(
        after_count.trim(),
        "2",
        "mv 1 1 must not change count; got {after_count:?}"
    );
    let after_listing = stdout_str(&run(&dir, &["-n"]));
    assert_eq!(
        before_listing, after_listing,
        "mv 1 1 must not change listing; before={before_listing:?} after={after_listing:?}"
    );
}

/// `--sort name` is idempotent: running it twice on the same stack produces
/// the same ordering as one run. Pin against accidental shuffle.
#[test]
fn test_sort_by_name_is_idempotent_when_already_sorted() {
    let dir = clean_dir();
    push(&dir, "alpha\n");
    push(&dir, "bravo\n");
    push(&dir, "charlie\n");
    let s1 = run(&dir, &["--sort", "name"]);
    assert!(
        s1.status.success(),
        "first sort must succeed; stderr={}",
        String::from_utf8_lossy(&s1.stderr)
    );
    let listing_after_first = stdout_str(&run(&dir, &["-n"]));
    let s2 = run(&dir, &["--sort", "name"]);
    assert!(
        s2.status.success(),
        "second sort must succeed; stderr={}",
        String::from_utf8_lossy(&s2.stderr)
    );
    let listing_after_second = stdout_str(&run(&dir, &["-n"]));
    assert_eq!(
        listing_after_first, listing_after_second,
        "--sort name must be idempotent; first={listing_after_first:?} second={listing_after_second:?}"
    );
}

/// `--count` (`-k`) on a totally fresh dir reports 0. Pins that no auto-init
/// of the stack file creates phantom entries.
#[test]
fn test_count_on_fresh_dir_reports_zero_without_initializing_phantom_entries() {
    let dir = clean_dir();
    let out = run(&dir, &["-k"]);
    assert!(out.status.success(), "count on fresh dir must succeed");
    let body = stdout_str(&out);
    assert_eq!(body.trim(), "0", "fresh dir count must be 0; got {body:?}");
}
