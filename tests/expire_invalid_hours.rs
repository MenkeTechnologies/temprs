//! Pin the failure semantics of `--expire HOURS` against adversarial
//! numeric inputs that the existing test suite does not cover.
//!
//! The CLI exposes `--expire HOURS` with no `allow_hyphen_values(true)`
//! on the EXPIRE flag (src/model/opts.rs:215-218). Space-separated
//! `--expire -1` is rejected by clap before the body runs and that
//! case is pinned by `integration::expire_negative_hours_fails`.
//!
//! Three cases are NOT pinned anywhere and slip past the current suite
//! because clap accepts them: (a) `--expire=-1` via `=` syntax (clap
//! treats the value as a single token regardless of leading hyphen,
//! so it reaches the body), (b) `--expire NaN`, and (c) `--expire inf`.
//! In `expire_tempfiles` (src/model/app.rs:618-653) the parsed `hours`
//! is multiplied by 3600 and cast `as u64`. Negative results saturate
//! to 0, NaN-as-u64 is 0, and infinity-as-u64 is u64::MAX. The first
//! two silently behave like `--expire 0` (purge everything) while
//! infinity behaves like "keep everything". None of those branches are
//! pinned by the existing suite — adversarial input slips through.
//!
//! Each test below pins ONE bug class — see the doc-comment per test
//! for the specific failure path. These are NOT mirror/smoke tests:
//! they fail today against the live binary and would catch a future
//! regression introducing the same silent-purge behavior even after
//! the bug is fixed.

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
    let dir = std::env::temp_dir().join(format!(
        "temprs_expire_invalid_{}_{}",
        std::process::id(),
        id
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

fn count(dir: &PathBuf) -> usize {
    let out = run_tp(dir, &["--count"]);
    String::from_utf8_lossy(&out.stdout)
        .split_whitespace()
        .find_map(|tok| tok.parse().ok())
        .unwrap_or(usize::MAX)
}

fn tick() {
    std::thread::sleep(std::time::Duration::from_millis(50));
}

// ─── --expire=-1 must NOT silently purge ───────────────────────────
//
// integration::expire_negative_hours_fails covers `--expire -1` (space
// form) which clap rejects because the EXPIRE flag does not
// `allow_hyphen_values(true)`. But `--expire=-1` bypasses clap's
// hyphen check (= syntax is treated as a single token) and reaches
// `expire_tempfiles`. There `(-1.0 * 3600.0) as u64` saturates to 0,
// `age.as_secs() >= 0` is true for every file, so every entry is
// purged silently with exit code 0. A negative TTL is nonsensical;
// the correct behavior is to fail (either at parse or with a clear
// non-zero exit), not to silently clear the stack.
#[test]
fn expire_equals_negative_must_not_silently_clear_stack() {
    let dir = setup_clean_env();
    let push = run_tp_stdin(&dir, &[], "keepme");
    assert!(push.status.success(), "seed push must succeed");
    tick();
    assert_eq!(count(&dir), 1, "precondition: one item on stack");

    let out = run_tp(&dir, &["--expire=-1"]);

    // Two acceptable correct behaviors: (a) hard fail (non-zero exit
    // AND stack untouched) or (b) refuse to remove anything (zero exit
    // AND stack untouched). Silent purge is the bug.
    let stack_after = count(&dir);
    assert_eq!(
        stack_after,
        1,
        "--expire=-1 must not silently purge files; \
         stack went from 1 to {} (exit={:?}, stderr={:?})",
        stack_after,
        out.status.code(),
        String::from_utf8_lossy(&out.stderr)
    );
}

// ─── --expire NaN must NOT silently purge ──────────────────────────
//
// `"NaN".parse::<f64>()` succeeds and returns f64::NAN. Then
// `(NAN * 3600.0) as u64` is 0 per Rust's cast semantics (NaN-as-u64
// is 0, not a panic, not a saturating max). All files get age >= 0,
// every file purges, exit 0. The user expected "garbage in, error
// out", got "garbage in, all data gone".
#[test]
fn expire_nan_must_not_silently_clear_stack() {
    let dir = setup_clean_env();
    let push = run_tp_stdin(&dir, &[], "keepme");
    assert!(push.status.success(), "seed push must succeed");
    tick();
    assert_eq!(count(&dir), 1, "precondition: one item on stack");

    let out = run_tp(&dir, &["--expire", "NaN"]);

    let stack_after = count(&dir);
    assert_eq!(
        stack_after,
        1,
        "--expire NaN must reject NaN (or be a no-op), \
         not silently purge; stack went from 1 to {} (exit={:?}, stderr={:?})",
        stack_after,
        out.status.code(),
        String::from_utf8_lossy(&out.stderr)
    );
}

// ─── -c (clear-all) must refuse to wipe a non-temprs TEMPRS_DIR ────
//
// Pre-fix, `tp -c` called `remove_dir_all` on the parent directory of
// the master record file. If a user accidentally set TEMPRS_DIR=$HOME
// or TEMPRS_DIR=/tmp, a single `tp -c` wiped that path. The fix
// requires the parent dir's basename to contain "temprs" (case-
// insensitive) before any rm — both the default `~/.temprs` and any
// sane override satisfy that.
#[test]
fn clear_all_refuses_when_temprs_dir_basename_does_not_contain_temprs() {
    let dir = setup_clean_env();
    // Set TEMPRS_DIR to a path whose basename does NOT contain "temprs"
    // — simulating a foot-gun like TEMPRS_DIR=$HOME.
    let unsafe_dir = std::env::temp_dir().join(format!(
        "not_a_safe_name_{}_{}",
        std::process::id(),
        TEST_COUNTER.fetch_add(1, Ordering::SeqCst)
    ));
    fs::create_dir_all(&unsafe_dir).unwrap();
    // Drop a sentinel file to prove `remove_dir_all` would have wiped it.
    let sentinel = unsafe_dir.join("DO_NOT_DELETE.txt");
    fs::write(&sentinel, b"important user data").unwrap();

    let out = run_tp(&unsafe_dir, &["-c"]);

    assert!(
        sentinel.exists(),
        "tp -c must refuse to remove a TEMPRS_DIR whose basename does not contain `temprs`; \
         sentinel was wiped (exit={:?}, stderr={:?})",
        out.status.code(),
        String::from_utf8_lossy(&out.stderr)
    );

    let _ = fs::remove_dir_all(&unsafe_dir);
    let _ = dir;
}
