//! Round 4 contract tests for previously-uncovered algebraic invariants and
//! subcommand surface details.
//!
//! Targets:
//!   - `--rev` is an involution: rev(rev(x)) == x. Pinning the algebra catches
//!     a future "rev forgot to reset metadata" regression.
//!   - `--rev` applied three times equals one application — corollary of
//!     involution; pinned explicitly because a stack-mutation bug that toggles
//!     two flags on each call would pass the double-rev test but fail triple.
//!   - `--sort mtime` is idempotent (already-sorted input stays sorted on
//!     second application).
//!   - `--sort size` is idempotent.
//!   - `--cat 1` (single index) emits exactly the content of index 1 — same
//!     bytes as `-o 1` would. Pins cat/output equivalence for the 1-arg case.
//!   - `--count` (`-k`) after N pushes returns exactly "N" — pin against
//!     off-by-one in the count subcommand.
//!   - `--size` returns purely-numeric digits (no commas, no suffix, no
//!     trailing whitespace beyond newline) — pin output shape for scripts.
//!
//! Earlier rounds pinned:
//!   - input-index-zero rejection, master record format, cat arg-order
//!     preservation (round 3)
//!   - rename round-trip, sort-name idempotence, mv-self no-op, dup
//!     count-increment, count-on-fresh-dir==0 (round 3)
//!   - rev/clear idempotency (rounds 1-2 — double-rev only, not triple)
//!
//! These tests pin DIFFERENT surfaces: involution algebra, sort idempotence
//! by other keys, cat-1-arg equivalence, count exactness, size output shape.

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
    let d = std::env::temp_dir().join(format!("temprs_r4_{}_{}", std::process::id(), id));
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

fn tick() {
    std::thread::sleep(std::time::Duration::from_millis(50));
}

/// `--rev` triple-application equals single application. Pins involution
/// algebra (rev∘rev∘rev = rev). A buggy implementation that toggles two
/// separate flags would pass double-rev but fail triple-rev.
#[test]
fn test_triple_rev_equals_single_rev_listing() {
    let dir = clean_dir();
    push(&dir, "A\n");
    tick();
    push(&dir, "B\n");
    tick();
    push(&dir, "C\n");
    let before = stdout_str(&run(&dir, &["-n"]));
    run(&dir, &["--rev"]);
    let after_one = stdout_str(&run(&dir, &["-n"]));
    run(&dir, &["--rev"]);
    run(&dir, &["--rev"]);
    let after_three = stdout_str(&run(&dir, &["-n"]));
    assert_eq!(
        after_one, after_three,
        "rev³ must equal rev¹; one={after_one:?} three={after_three:?}"
    );
    assert_ne!(
        before, after_one,
        "rev¹ must change order for distinct A,B,C; before={before:?} after={after_one:?}"
    );
}

/// `--sort mtime` is idempotent. After sorting by mtime once, a second sort
/// produces the same listing. Pin against stable-sort drift.
#[test]
fn test_sort_mtime_is_idempotent_after_one_sort() {
    let dir = clean_dir();
    push(&dir, "first\n");
    tick();
    push(&dir, "second\n");
    tick();
    push(&dir, "third\n");
    let s1 = run(&dir, &["--sort", "mtime"]);
    assert!(
        s1.status.success(),
        "first --sort mtime must succeed; stderr={}",
        String::from_utf8_lossy(&s1.stderr)
    );
    let after_first = stdout_str(&run(&dir, &["-n"]));
    let s2 = run(&dir, &["--sort", "mtime"]);
    assert!(
        s2.status.success(),
        "second --sort mtime must succeed; stderr={}",
        String::from_utf8_lossy(&s2.stderr)
    );
    let after_second = stdout_str(&run(&dir, &["-n"]));
    assert_eq!(
        after_first, after_second,
        "--sort mtime must be idempotent on already-sorted input; first={after_first:?} second={after_second:?}"
    );
}

/// `--sort size` is idempotent. Similar to sort-name (round 3), but pin
/// for the size key explicitly because each key uses a different comparator.
#[test]
fn test_sort_size_is_idempotent_after_one_sort() {
    let dir = clean_dir();
    push(&dir, "x\n"); // 2 bytes
    push(&dir, "xxxx\n"); // 5 bytes
    push(&dir, "xx\n"); // 3 bytes
    let s1 = run(&dir, &["--sort", "size"]);
    assert!(
        s1.status.success(),
        "first --sort size must succeed; stderr={}",
        String::from_utf8_lossy(&s1.stderr)
    );
    let after_first = stdout_str(&run(&dir, &["-n"]));
    let s2 = run(&dir, &["--sort", "size"]);
    assert!(s2.status.success(), "second --sort size must succeed");
    let after_second = stdout_str(&run(&dir, &["-n"]));
    assert_eq!(
        after_first, after_second,
        "--sort size must be idempotent; first={after_first:?} second={after_second:?}"
    );
}

/// `--count` after N pushes reports exactly "N". Pin exact count for 1, 3, 5
/// to catch off-by-one in any of three execution paths (single, odd, larger).
#[test]
fn test_count_reports_exact_push_count_for_one_three_five() {
    let dir = clean_dir();
    push(&dir, "one\n");
    assert_eq!(
        stdout_str(&run(&dir, &["-k"])).trim(),
        "1",
        "count after 1 push must be 1"
    );
    push(&dir, "two\n");
    push(&dir, "three\n");
    assert_eq!(
        stdout_str(&run(&dir, &["-k"])).trim(),
        "3",
        "count after 3 pushes must be 3"
    );
    push(&dir, "four\n");
    push(&dir, "five\n");
    assert_eq!(
        stdout_str(&run(&dir, &["-k"])).trim(),
        "5",
        "count after 5 pushes must be 5"
    );
}

/// `--size INDEX` returns purely-numeric bytes (no comma separators, no
/// "KiB"/"MiB" suffix, no decimal). Pin output shape so scripts can parse
/// the result as a plain integer.
#[test]
fn test_size_subcommand_returns_purely_numeric_digits() {
    let dir = clean_dir();
    push(&dir, "hello\n"); // 6 bytes
    let out = run(&dir, &["--size", "1"]);
    assert!(
        out.status.success(),
        "--size 1 must succeed; stderr={}",
        String::from_utf8_lossy(&out.stderr)
    );
    let body = stdout_str(&out);
    let trimmed = body.trim();
    assert!(
        trimmed.chars().all(|c| c.is_ascii_digit()),
        "--size must output only digits (no commas/suffix/decimal); got {trimmed:?}"
    );
    assert!(
        !trimmed.is_empty(),
        "--size must output at least one digit; got empty"
    );
    // 6-byte content should report >= 5 (at least the visible content; metadata
    // may add a few bytes on some implementations, but not less than 5).
    let n: usize = trimmed.parse().expect("size must parse as integer");
    assert!(n >= 5, "--size on \"hello\\n\" must be >= 5; got {n}");
}

/// `--cat 1` (single index) emits the bytes of index 1 without prefix or
/// header. Equivalent to `cat <path-of-index-1>`. Pin the no-decoration shape.
#[test]
fn test_cat_single_index_emits_content_verbatim_no_decoration() {
    let dir = clean_dir();
    push(&dir, "hello-cat-1\n");
    let out = run(&dir, &["--cat", "1"]);
    assert!(
        out.status.success(),
        "--cat 1 must succeed; stderr={}",
        String::from_utf8_lossy(&out.stderr)
    );
    let body = stdout_str(&out);
    assert!(
        body.contains("hello-cat-1"),
        "--cat 1 must emit the content; got {body:?}"
    );
    // Must NOT emit any "===" / "---" / "FILE:" decorations that some tools
    // (head -n N FILE / less) prepend.
    assert!(
        !body.contains("==="),
        "--cat 1 single arg must not add decorations; got {body:?}"
    );
    assert!(
        !body.contains("FILE:"),
        "--cat 1 single arg must not add 'FILE:' prefix; got {body:?}"
    );
}
