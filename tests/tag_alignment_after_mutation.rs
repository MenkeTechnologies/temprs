//! Tag-to-path alignment invariants after stack mutation operations.
//!
//! `temp_file_stack` and `temp_file_names` are two parallel `Vec`s that must
//! stay index-aligned across every stack-mutating op (`--shift`, `--pop`,
//! `--remove`, `--rev`). A misalignment bug (e.g. an op that updates one vec
//! but not the other) is silent at write time and only surfaces when the user
//! later looks up a file by its tag — and by then the master record has been
//! overwritten with the wrong pairing. These tests stress the *interleaving*
//! of named pushes with mutations and verify the master record's name/path
//! pairings stay synchronized.
//!
//! Verification reads the master record file directly rather than going
//! through `tp -o <tag>`, because `-o` (OUTPUT) is a state-setting flag (not an
//! immediate-exit command) and so triggers an *additional* push of stdin's
//! empty content when this test harness wires `Stdio::null()` — a side effect
//! that would mask the very invariant we're trying to pin.
//!
//! Helpers are duplicated locally — each `tests/*.rs` is its own integration
//! crate per cargo's convention.

use std::fs;
use std::path::{Path, PathBuf};
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
    let dir = std::env::temp_dir().join(format!("temprs_tagalign_{}_{}", std::process::id(), id,));
    if dir.exists() {
        let _ = fs::remove_dir_all(&dir);
    }
    dir
}

fn run_tp(dir: &Path, args: &[&str]) -> std::process::Output {
    Command::new(bin())
        .env("TEMPRS_DIR", dir)
        .args(args)
        .stdin(std::process::Stdio::null())
        .output()
        .expect("failed to execute tp")
}

fn run_tp_stdin(dir: &Path, args: &[&str], input: &[u8]) -> std::process::Output {
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

/// 50 ms is enough to guarantee a distinct millisecond timestamp on the
/// new-temp-file filename across `simple_logger`/IO jitter on busy CI.
fn tick() {
    std::thread::sleep(std::time::Duration::from_millis(50));
}

/// Parse the master record file's `path\0name\0\0` records into
/// `Vec<(content, Option<name>)>` so tests can assert the path/name pairings
/// without going through `tp` itself (which pushes a stray empty file on
/// `Stdio::null()` for state-setting flags like `-o`).
fn read_master_pairs(dir: &Path) -> Vec<(String, Option<String>)> {
    let master = dir.join("temprs-stack");
    let raw = fs::read_to_string(&master).expect("master record file should exist");
    let mut out = Vec::new();
    for rec in raw.split("\0\0") {
        if rec.is_empty() {
            continue;
        }
        let (path, name) = match rec.split_once('\0') {
            Some((p, n)) => (p.to_string(), Some(n.to_string())),
            None => (rec.to_string(), None),
        };
        let content = fs::read_to_string(&path).unwrap_or_default();
        out.push((content, name));
    }
    out
}

/// After shift+pop on a 4-element tagged stack, the two surviving (name, path)
/// pairings must still match the original ones — i.e. `--shift` removed
/// name[0] in lock-step with path[0], and `--pop` removed name[last] in
/// lock-step with path[last]. A regression in either call site (e.g.
/// forgetting `temp_file_names_mut().remove(...)` next to the path remove)
/// would surface as the wrong (content, tag) pair surviving.
#[test]
fn shift_then_pop_keeps_middle_tag_to_content_pairings_aligned() {
    let dir = setup_clean_env();
    for (tag, body) in [
        ("alpha", b"AAA" as &[u8]),
        ("bravo", b"BBB"),
        ("charlie", b"CCC"),
        ("delta", b"DDD"),
    ] {
        let out = run_tp_stdin(&dir, &["-w", tag], body);
        assert!(out.status.success(), "push tagged {} failed", tag);
        tick();
    }

    assert!(run_tp(&dir, &["-s"]).status.success(), "shift");
    assert!(run_tp(&dir, &["-p"]).status.success(), "pop");

    let pairs = read_master_pairs(&dir);
    assert_eq!(pairs.len(), 2, "expected 2 survivors after shift+pop");
    assert_eq!(
        pairs[0],
        ("BBB".to_string(), Some("bravo".to_string())),
        "position-0 (bravo, BBB) pairing broken after shift+pop",
    );
    assert_eq!(
        pairs[1],
        ("CCC".to_string(), Some("charlie".to_string())),
        "position-1 (charlie, CCC) pairing broken after shift+pop",
    );
}

/// `--rev` calls `.reverse()` on `temp_file_stack` AND `temp_file_names`. A
/// regression that reversed only the path vec (the more obvious one) would
/// leave each tag attached to the wrong file. Mixing tagged and untagged
/// entries makes a "name vec wasn't reversed" bug visible because the `None`
/// gap would land at the wrong index relative to the now-reversed paths.
#[test]
fn rev_reverses_name_vec_in_lockstep_with_path_vec() {
    let dir = setup_clean_env();
    let pushes: &[(Option<&str>, &[u8])] = &[
        (Some("first"), b"ONE"),
        (None, b"TWO"),
        (Some("third"), b"THREE"),
        (None, b"FOUR"),
    ];
    for (tag, body) in pushes {
        let args: Vec<&str> = match tag {
            Some(t) => vec!["-w", *t],
            None => vec![],
        };
        let out = run_tp_stdin(&dir, &args, body);
        assert!(out.status.success());
        tick();
    }

    assert!(run_tp(&dir, &["--rev"]).status.success(), "--rev");

    let pairs = read_master_pairs(&dir);
    assert_eq!(pairs.len(), 4);
    assert_eq!(
        pairs[0],
        ("FOUR".to_string(), None),
        "post-rev pos 0 should be (FOUR, None) — name vec reverse missed?",
    );
    assert_eq!(
        pairs[1],
        ("THREE".to_string(), Some("third".to_string())),
        "post-rev pos 1 should be (THREE, third) — tag drifted off its content",
    );
    assert_eq!(
        pairs[2],
        ("TWO".to_string(), None),
        "post-rev pos 2 should be (TWO, None)",
    );
    assert_eq!(
        pairs[3],
        ("ONE".to_string(), Some("first".to_string())),
        "post-rev pos 3 should be (ONE, first) — tag drifted off its content",
    );
}

/// `--remove` by tag must drop both vec entries at the resolved index. A bug
/// that resolved the index via the names vec but called `temp_file_stack.remove`
/// without the matching `temp_file_names.remove` would leave a "ghost" name
/// attached to whatever shifted into the freed slot. We push three named
/// entries and remove the middle one by tag; the remaining two must keep their
/// own tag/content pairings unchanged.
#[test]
fn remove_by_tag_drops_only_that_entry_from_both_vecs() {
    let dir = setup_clean_env();
    let pushes: &[(&str, &[u8])] = &[("aaa", b"111"), ("bbb", b"222"), ("ccc", b"333")];
    for (tag, body) in pushes {
        let out = run_tp_stdin(&dir, &["-w", tag], body);
        assert!(out.status.success());
        tick();
    }

    let rm = run_tp(&dir, &["-r", "bbb"]);
    assert!(rm.status.success(), "remove by tag bbb");

    let pairs = read_master_pairs(&dir);
    assert_eq!(pairs.len(), 2, "expected 2 survivors after remove bbb");
    assert_eq!(
        pairs[0],
        ("111".to_string(), Some("aaa".to_string())),
        "aaa drifted off 111 after remove bbb",
    );
    assert_eq!(
        pairs[1],
        ("333".to_string(), Some("ccc".to_string())),
        "ccc drifted off 333 after remove bbb",
    );
}
