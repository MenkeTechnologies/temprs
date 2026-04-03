use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::util::utils::{util_file_to_paths_and_names, util_time_ms};

use super::*;

static STATE_TEST_TMP_COUNTER: AtomicU64 = AtomicU64::new(0);

fn state_test_tmp_dir() -> PathBuf {
    let id = STATE_TEST_TMP_COUNTER.fetch_add(1, Ordering::SeqCst);
    let dir = std::env::temp_dir().join(format!(
        "temprs_state_test_{}_{}_{}",
        std::process::id(),
        id,
        util_time_ms()
    ));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn make_state() -> TempState {
    TempState::new(
        PathBuf::from("/tmp/temprs/tempfile123"),
        PathBuf::from("/tmp/temprs/temprs-stack"),
        PathBuf::from("/tmp/temprs"),
        vec![
            PathBuf::from("/tmp/temprs/f1"),
            PathBuf::from("/tmp/temprs/f2"),
        ],
        vec![None, None],
        None,
        String::new(),
    )
}

// ── constructor defaults ───────────────────────────

#[test]
fn new_state_defaults() {
    let s = make_state();
    assert_eq!(s.verbose(), 0);
    assert!(!s.silent());
    assert!(s.holding_buffer().is_empty());
    assert!(s.output_buffer().is_empty());
    assert!(s.input_temp_file().is_none());
    assert!(s.output_temp_file().is_none());
    assert!(s.append_temp_file().is_none());
    assert!(s.name().is_none());
    assert_eq!(s.temp_file_names().len(), 2);
    assert_eq!(s.temp_file_names(), &vec![None, None]);
    assert!(s.insert_idx().is_none());
    assert!(s.arg_file().is_none());
}

// ── getters ────────────────────────────────────────

#[test]
fn new_temp_file_getter() {
    let s = make_state();
    assert_eq!(s.new_temp_file(), &PathBuf::from("/tmp/temprs/tempfile123"));
}

#[test]
fn master_record_file_getter() {
    let s = make_state();
    assert_eq!(
        s.master_record_file(),
        &PathBuf::from("/tmp/temprs/temprs-stack")
    );
}

#[test]
fn temprs_dir_getter() {
    let s = make_state();
    assert_eq!(s.temprs_dir(), &PathBuf::from("/tmp/temprs"));
}

#[test]
fn temp_file_stack_getter() {
    let s = make_state();
    assert_eq!(s.temp_file_stack().len(), 2);
    assert_eq!(s.temp_file_stack()[0], PathBuf::from("/tmp/temprs/f1"));
    assert_eq!(s.temp_file_stack()[1], PathBuf::from("/tmp/temprs/f2"));
}

// ── setters ────────────────────────────────────────

#[test]
fn set_verbose() {
    let mut s = make_state();
    s.set_verbose(3);
    assert_eq!(s.verbose(), 3);
}

#[test]
fn set_silent() {
    let mut s = make_state();
    s.set_silent(true);
    assert!(s.silent());
}

#[test]
fn set_holding_buffer() {
    let mut s = make_state();
    s.set_holding_buffer("data".to_string());
    assert_eq!(s.holding_buffer(), "data");
}

#[test]
fn set_output_buffer() {
    let mut s = make_state();
    s.set_output_buffer("out".to_string());
    assert_eq!(s.output_buffer(), "out");
}

#[test]
fn set_input_temp_file() {
    let mut s = make_state();
    s.set_input_temp_file(Some("1".to_string()));
    assert_eq!(s.input_temp_file(), &Some("1".to_string()));
}

#[test]
fn set_output_temp_file() {
    let mut s = make_state();
    s.set_output_temp_file(Some("2".to_string()));
    assert_eq!(s.output_temp_file(), &Some("2".to_string()));
}

#[test]
fn set_insert_idx() {
    let mut s = make_state();
    s.set_insert_idx(Some("3".to_string()));
    assert_eq!(s.insert_idx(), &Some("3".to_string()));
}

#[test]
fn set_arg_file() {
    let mut s = make_state();
    s.set_arg_file(Some(PathBuf::from("/tmp/input.txt")));
    assert_eq!(s.arg_file(), &Some(PathBuf::from("/tmp/input.txt")));
}

#[test]
fn set_new_temp_file() {
    let mut s = make_state();
    s.set_new_temp_file(PathBuf::from("/tmp/new"));
    assert_eq!(s.new_temp_file(), &PathBuf::from("/tmp/new"));
}

#[test]
fn set_master_record_file() {
    let mut s = make_state();
    s.set_master_record_file(PathBuf::from("/tmp/master2"));
    assert_eq!(s.master_record_file(), &PathBuf::from("/tmp/master2"));
}

#[test]
fn set_temprs_dir() {
    let mut s = make_state();
    s.set_temprs_dir(PathBuf::from("/tmp/other"));
    assert_eq!(s.temprs_dir(), &PathBuf::from("/tmp/other"));
}

#[test]
fn set_temp_file_stack() {
    let mut s = make_state();
    s.set_temp_file_stack(vec![PathBuf::from("/tmp/x")]);
    assert_eq!(s.temp_file_stack().len(), 1);
}

// ── path string helpers ────────────────────────────

#[test]
fn out_file_path_str_matches() {
    let s = make_state();
    assert_eq!(s.out_file_path_str(), "/tmp/temprs/tempfile123");
}

#[test]
fn master_file_path_str_matches() {
    let s = make_state();
    assert_eq!(s.master_file_path_str(), "/tmp/temprs/temprs-stack");
}

// ── constructor with non-default args ────────────────

#[test]
fn new_with_output_buffer() {
    let s = TempState::new(
        PathBuf::from("/tmp/out"),
        PathBuf::from("/tmp/master"),
        PathBuf::from("/tmp"),
        vec![],
        vec![],
        None,
        String::from("preset output"),
    );
    assert_eq!(s.output_buffer(), "preset output");
}

#[test]
fn new_with_arg_file() {
    let s = TempState::new(
        PathBuf::from("/tmp/out"),
        PathBuf::from("/tmp/master"),
        PathBuf::from("/tmp"),
        vec![],
        vec![],
        Some(PathBuf::from("/tmp/input.txt")),
        String::new(),
    );
    assert_eq!(s.arg_file(), &Some(PathBuf::from("/tmp/input.txt")));
}

#[test]
fn new_with_empty_stack() {
    let s = TempState::new(
        PathBuf::from("/tmp/out"),
        PathBuf::from("/tmp/master"),
        PathBuf::from("/tmp"),
        vec![],
        vec![],
        None,
        String::new(),
    );
    assert!(s.temp_file_stack().is_empty());
}

#[test]
fn new_with_large_stack() {
    let stack: Vec<PathBuf> = (0..50)
        .map(|i| PathBuf::from(format!("/tmp/temprs/f{}", i)))
        .collect();
    let s = TempState::new(
        PathBuf::from("/tmp/out"),
        PathBuf::from("/tmp/master"),
        PathBuf::from("/tmp"),
        stack.clone(),
        vec![],
        None,
        String::new(),
    );
    assert_eq!(s.temp_file_stack().len(), 50);
    assert_eq!(s.temp_file_stack()[0], PathBuf::from("/tmp/temprs/f0"));
    assert_eq!(s.temp_file_stack()[49], PathBuf::from("/tmp/temprs/f49"));
}

// ── setter overwrites ────────────────────────────────

#[test]
fn set_verbose_multiple_times() {
    let mut s = make_state();
    s.set_verbose(1);
    assert_eq!(s.verbose(), 1);
    s.set_verbose(5);
    assert_eq!(s.verbose(), 5);
    s.set_verbose(0);
    assert_eq!(s.verbose(), 0);
}

#[test]
fn set_silent_toggle() {
    let mut s = make_state();
    s.set_silent(true);
    assert!(s.silent());
    s.set_silent(false);
    assert!(!s.silent());
    s.set_silent(true);
    assert!(s.silent());
}

#[test]
fn set_holding_buffer_overwrite() {
    let mut s = make_state();
    s.set_holding_buffer("first".to_string());
    assert_eq!(s.holding_buffer(), "first");
    s.set_holding_buffer("second".to_string());
    assert_eq!(s.holding_buffer(), "second");
}

#[test]
fn set_output_buffer_overwrite() {
    let mut s = make_state();
    s.set_output_buffer("out1".to_string());
    s.set_output_buffer("out2".to_string());
    assert_eq!(s.output_buffer(), "out2");
}

#[test]
fn set_insert_idx_to_none() {
    let mut s = make_state();
    s.set_insert_idx(Some("5".to_string()));
    assert!(s.insert_idx().is_some());
    s.set_insert_idx(None);
    assert!(s.insert_idx().is_none());
}

#[test]
fn set_input_temp_file_to_none() {
    let mut s = make_state();
    s.set_input_temp_file(Some("1".to_string()));
    assert!(s.input_temp_file().is_some());
    s.set_input_temp_file(None);
    assert!(s.input_temp_file().is_none());
}

#[test]
fn set_output_temp_file_to_none() {
    let mut s = make_state();
    s.set_output_temp_file(Some("2".to_string()));
    assert!(s.output_temp_file().is_some());
    s.set_output_temp_file(None);
    assert!(s.output_temp_file().is_none());
}

#[test]
fn set_arg_file_to_none() {
    let mut s = make_state();
    s.set_arg_file(Some(PathBuf::from("/tmp/f")));
    assert!(s.arg_file().is_some());
    s.set_arg_file(None);
    assert!(s.arg_file().is_none());
}

#[test]
fn set_temp_file_stack_empty() {
    let mut s = make_state();
    assert_eq!(s.temp_file_stack().len(), 2);
    s.set_temp_file_stack(vec![]);
    assert!(s.temp_file_stack().is_empty());
}

#[test]
fn set_temp_file_stack_larger() {
    let mut s = make_state();
    let big: Vec<PathBuf> = (0..10).map(|i| PathBuf::from(format!("/f{}", i))).collect();
    s.set_temp_file_stack(big);
    assert_eq!(s.temp_file_stack().len(), 10);
}

// ── path string helpers with different paths ─────────

#[test]
fn out_file_path_str_with_spaces() {
    let s = TempState::new(
        PathBuf::from("/tmp/my dir/file"),
        PathBuf::from("/tmp/master"),
        PathBuf::from("/tmp"),
        vec![],
        vec![],
        None,
        String::new(),
    );
    assert_eq!(s.out_file_path_str(), "/tmp/my dir/file");
}

#[test]
fn master_file_path_str_deep_path() {
    let s = TempState::new(
        PathBuf::from("/tmp/out"),
        PathBuf::from("/a/b/c/d/e/master"),
        PathBuf::from("/tmp"),
        vec![],
        vec![],
        None,
        String::new(),
    );
    assert_eq!(s.master_file_path_str(), "/a/b/c/d/e/master");
}

// ── holding_buffer and output_buffer with special content ─

#[test]
fn holding_buffer_unicode() {
    let mut s = make_state();
    s.set_holding_buffer("日本語 🎉".to_string());
    assert_eq!(s.holding_buffer(), "日本語 🎉");
}

#[test]
fn output_buffer_multiline() {
    let mut s = make_state();
    s.set_output_buffer("line1\nline2\nline3".to_string());
    assert_eq!(s.output_buffer(), "line1\nline2\nline3");
}

#[test]
fn holding_buffer_empty() {
    let mut s = make_state();
    s.set_holding_buffer(String::new());
    assert!(s.holding_buffer().is_empty());
}

// ── verbose boundary values ──────────────────────────

#[test]
fn verbose_max_value() {
    let mut s = make_state();
    s.set_verbose(u32::MAX);
    assert_eq!(s.verbose(), u32::MAX);
}

#[test]
fn verbose_zero() {
    let mut s = make_state();
    s.set_verbose(0);
    assert_eq!(s.verbose(), 0);
}

#[test]
fn verbose_one() {
    let mut s = make_state();
    s.set_verbose(1);
    assert_eq!(s.verbose(), 1);
}

// ── buffers with large content ───────────────────────

#[test]
fn holding_buffer_large() {
    let mut s = make_state();
    let large = "x".repeat(10_000);
    s.set_holding_buffer(large.clone());
    assert_eq!(s.holding_buffer(), large);
    assert_eq!(s.holding_buffer().len(), 10_000);
}

#[test]
fn output_buffer_large() {
    let mut s = make_state();
    let large = "y".repeat(10_000);
    s.set_output_buffer(large.clone());
    assert_eq!(s.output_buffer(), large);
    assert_eq!(s.output_buffer().len(), 10_000);
}

// ── buffers with special characters ──────────────────

#[test]
fn holding_buffer_tabs_and_newlines() {
    let mut s = make_state();
    s.set_holding_buffer("col1\tcol2\nrow2col1\trow2col2".to_string());
    assert_eq!(s.holding_buffer(), "col1\tcol2\nrow2col1\trow2col2");
}

#[test]
fn output_buffer_special_chars() {
    let mut s = make_state();
    s.set_output_buffer("quotes: \"hello\" 'world'\nbackslash: \\".to_string());
    assert_eq!(
        s.output_buffer(),
        "quotes: \"hello\" 'world'\nbackslash: \\"
    );
}

#[test]
fn output_buffer_empty_string() {
    let mut s = make_state();
    s.set_output_buffer("data".to_string());
    s.set_output_buffer(String::new());
    assert!(s.output_buffer().is_empty());
}

// ── input/output_temp_file with various values ───────

#[test]
fn input_temp_file_negative_index() {
    let mut s = make_state();
    s.set_input_temp_file(Some("-1".to_string()));
    assert_eq!(s.input_temp_file(), &Some("-1".to_string()));
}

#[test]
fn output_temp_file_negative_index() {
    let mut s = make_state();
    s.set_output_temp_file(Some("-3".to_string()));
    assert_eq!(s.output_temp_file(), &Some("-3".to_string()));
}

#[test]
fn input_temp_file_large_index() {
    let mut s = make_state();
    s.set_input_temp_file(Some("999".to_string()));
    assert_eq!(s.input_temp_file(), &Some("999".to_string()));
}

#[test]
fn output_temp_file_large_index() {
    let mut s = make_state();
    s.set_output_temp_file(Some("1000".to_string()));
    assert_eq!(s.output_temp_file(), &Some("1000".to_string()));
}

// ── insert_idx values ────────────────────────────────

#[test]
fn insert_idx_negative() {
    let mut s = make_state();
    s.set_insert_idx(Some("-2".to_string()));
    assert_eq!(s.insert_idx(), &Some("-2".to_string()));
}

#[test]
fn insert_idx_large() {
    let mut s = make_state();
    s.set_insert_idx(Some("100".to_string()));
    assert_eq!(s.insert_idx(), &Some("100".to_string()));
}

#[test]
fn insert_idx_overwrite() {
    let mut s = make_state();
    s.set_insert_idx(Some("1".to_string()));
    s.set_insert_idx(Some("5".to_string()));
    assert_eq!(s.insert_idx(), &Some("5".to_string()));
}

// ── arg_file with various paths ──────────────────────

#[test]
fn arg_file_with_spaces() {
    let mut s = make_state();
    s.set_arg_file(Some(PathBuf::from("/tmp/my dir/file.txt")));
    assert_eq!(s.arg_file(), &Some(PathBuf::from("/tmp/my dir/file.txt")));
}

#[test]
fn arg_file_deep_path() {
    let mut s = make_state();
    s.set_arg_file(Some(PathBuf::from("/a/b/c/d/e/f/g.txt")));
    assert_eq!(s.arg_file(), &Some(PathBuf::from("/a/b/c/d/e/f/g.txt")));
}

#[test]
fn arg_file_overwrite() {
    let mut s = make_state();
    s.set_arg_file(Some(PathBuf::from("/first")));
    s.set_arg_file(Some(PathBuf::from("/second")));
    assert_eq!(s.arg_file(), &Some(PathBuf::from("/second")));
}

// ── temp_file_stack manipulation sequences ───────────

#[test]
fn stack_replace_then_check_elements() {
    let mut s = make_state();
    let stack = vec![
        PathBuf::from("/a"),
        PathBuf::from("/b"),
        PathBuf::from("/c"),
    ];
    s.set_temp_file_stack(stack);
    assert_eq!(s.temp_file_stack().len(), 3);
    assert_eq!(s.temp_file_stack()[0], PathBuf::from("/a"));
    assert_eq!(s.temp_file_stack()[1], PathBuf::from("/b"));
    assert_eq!(s.temp_file_stack()[2], PathBuf::from("/c"));
}

#[test]
fn stack_replace_multiple_times() {
    let mut s = make_state();
    s.set_temp_file_stack(vec![PathBuf::from("/x")]);
    assert_eq!(s.temp_file_stack().len(), 1);
    s.set_temp_file_stack(vec![PathBuf::from("/y"), PathBuf::from("/z")]);
    assert_eq!(s.temp_file_stack().len(), 2);
    s.set_temp_file_stack(vec![]);
    assert!(s.temp_file_stack().is_empty());
}

// ── new_temp_file setter edge cases ──────────────────

#[test]
fn new_temp_file_with_unicode() {
    let mut s = make_state();
    s.set_new_temp_file(PathBuf::from("/tmp/日本語/tempfile"));
    assert_eq!(s.new_temp_file(), &PathBuf::from("/tmp/日本語/tempfile"));
}

#[test]
fn new_temp_file_overwrite() {
    let mut s = make_state();
    s.set_new_temp_file(PathBuf::from("/tmp/first"));
    s.set_new_temp_file(PathBuf::from("/tmp/second"));
    assert_eq!(s.new_temp_file(), &PathBuf::from("/tmp/second"));
}

// ── master_record_file setter edge cases ─────────────

#[test]
fn master_record_file_overwrite() {
    let mut s = make_state();
    s.set_master_record_file(PathBuf::from("/first"));
    s.set_master_record_file(PathBuf::from("/second"));
    assert_eq!(s.master_record_file(), &PathBuf::from("/second"));
}

// ── temprs_dir setter edge cases ─────────────────────

#[test]
fn temprs_dir_overwrite() {
    let mut s = make_state();
    s.set_temprs_dir(PathBuf::from("/dir1"));
    s.set_temprs_dir(PathBuf::from("/dir2"));
    assert_eq!(s.temprs_dir(), &PathBuf::from("/dir2"));
}

#[test]
fn temprs_dir_with_spaces() {
    let mut s = make_state();
    s.set_temprs_dir(PathBuf::from("/tmp/my dir"));
    assert_eq!(s.temprs_dir(), &PathBuf::from("/tmp/my dir"));
}

// ── path str helpers after mutation ──────────────────

#[test]
fn out_file_path_str_after_set() {
    let mut s = make_state();
    s.set_new_temp_file(PathBuf::from("/new/path/file"));
    assert_eq!(s.out_file_path_str(), "/new/path/file");
}

#[test]
fn master_file_path_str_after_set() {
    let mut s = make_state();
    s.set_master_record_file(PathBuf::from("/new/master"));
    assert_eq!(s.master_file_path_str(), "/new/master");
}

// ── full state construction with all non-default args ─

#[test]
fn new_all_fields_non_default() {
    let s = TempState::new(
        PathBuf::from("/tmp/out"),
        PathBuf::from("/tmp/master"),
        PathBuf::from("/tmp/dir"),
        vec![
            PathBuf::from("/tmp/f1"),
            PathBuf::from("/tmp/f2"),
            PathBuf::from("/tmp/f3"),
        ],
        vec![],
        Some(PathBuf::from("/tmp/argfile")),
        String::from("initial output"),
    );
    assert_eq!(s.new_temp_file(), &PathBuf::from("/tmp/out"));
    assert_eq!(s.master_record_file(), &PathBuf::from("/tmp/master"));
    assert_eq!(s.temprs_dir(), &PathBuf::from("/tmp/dir"));
    assert_eq!(s.temp_file_stack().len(), 3);
    assert_eq!(s.arg_file(), &Some(PathBuf::from("/tmp/argfile")));
    assert_eq!(s.output_buffer(), "initial output");
    // defaults still hold
    assert!(s.holding_buffer().is_empty());
    assert!(s.insert_idx().is_none());
    assert!(s.input_temp_file().is_none());
    assert!(s.output_temp_file().is_none());
    assert!(!s.silent());
    assert_eq!(s.verbose(), 0);
}

// ── multiple fields set in sequence ──────────────────

#[test]
fn set_all_fields_then_verify() {
    let mut s = make_state();
    s.set_verbose(2);
    s.set_silent(true);
    s.set_holding_buffer("hold".to_string());
    s.set_output_buffer("out".to_string());
    s.set_input_temp_file(Some("3".to_string()));
    s.set_output_temp_file(Some("4".to_string()));
    s.set_insert_idx(Some("2".to_string()));
    s.set_arg_file(Some(PathBuf::from("/tmp/arg")));
    s.set_new_temp_file(PathBuf::from("/tmp/new"));
    s.set_master_record_file(PathBuf::from("/tmp/master2"));
    s.set_temprs_dir(PathBuf::from("/tmp/dir2"));
    s.set_temp_file_stack(vec![PathBuf::from("/tmp/only")]);

    assert_eq!(s.verbose(), 2);
    assert!(s.silent());
    assert_eq!(s.holding_buffer(), "hold");
    assert_eq!(s.output_buffer(), "out");
    assert_eq!(s.input_temp_file(), &Some("3".to_string()));
    assert_eq!(s.output_temp_file(), &Some("4".to_string()));
    assert_eq!(s.insert_idx(), &Some("2".to_string()));
    assert_eq!(s.arg_file(), &Some(PathBuf::from("/tmp/arg")));
    assert_eq!(s.new_temp_file(), &PathBuf::from("/tmp/new"));
    assert_eq!(s.master_record_file(), &PathBuf::from("/tmp/master2"));
    assert_eq!(s.temprs_dir(), &PathBuf::from("/tmp/dir2"));
    assert_eq!(s.temp_file_stack().len(), 1);
}

// ── setters don't affect other fields ────────────────

#[test]
fn set_verbose_doesnt_change_silent() {
    let mut s = make_state();
    s.set_silent(true);
    s.set_verbose(5);
    assert!(s.silent());
}

#[test]
fn set_holding_doesnt_change_output() {
    let mut s = make_state();
    s.set_output_buffer("original".to_string());
    s.set_holding_buffer("changed".to_string());
    assert_eq!(s.output_buffer(), "original");
}

#[test]
fn set_input_doesnt_change_output_temp() {
    let mut s = make_state();
    s.set_output_temp_file(Some("5".to_string()));
    s.set_input_temp_file(Some("3".to_string()));
    assert_eq!(s.output_temp_file(), &Some("5".to_string()));
}

#[test]
fn set_stack_doesnt_change_dir() {
    let mut s = make_state();
    let orig_dir = s.temprs_dir().clone();
    s.set_temp_file_stack(vec![]);
    assert_eq!(s.temprs_dir(), &orig_dir);
}

#[test]
fn set_arg_file_doesnt_change_new_temp_file() {
    let mut s = make_state();
    let orig = s.new_temp_file().clone();
    s.set_arg_file(Some(PathBuf::from("/tmp/other")));
    assert_eq!(s.new_temp_file(), &orig);
}

// ── constructor edge cases ────────────────────────────

#[test]
fn new_with_single_stack_item() {
    let s = TempState::new(
        PathBuf::from("/tmp/out"),
        PathBuf::from("/tmp/master"),
        PathBuf::from("/tmp"),
        vec![PathBuf::from("/tmp/only_one")],
        vec![],
        None,
        String::new(),
    );
    assert_eq!(s.temp_file_stack().len(), 1);
    assert_eq!(s.temp_file_stack()[0], PathBuf::from("/tmp/only_one"));
}

#[test]
fn new_with_very_large_stack() {
    let stack: Vec<PathBuf> = (0..200)
        .map(|i| PathBuf::from(format!("/tmp/temprs/f{}", i)))
        .collect();
    let s = TempState::new(
        PathBuf::from("/tmp/out"),
        PathBuf::from("/tmp/master"),
        PathBuf::from("/tmp"),
        stack,
        vec![],
        None,
        String::new(),
    );
    assert_eq!(s.temp_file_stack().len(), 200);
    assert_eq!(s.temp_file_stack()[0], PathBuf::from("/tmp/temprs/f0"));
    assert_eq!(s.temp_file_stack()[199], PathBuf::from("/tmp/temprs/f199"));
}

#[test]
fn new_preserves_output_buffer_with_newlines() {
    let s = TempState::new(
        PathBuf::from("/tmp/out"),
        PathBuf::from("/tmp/master"),
        PathBuf::from("/tmp"),
        vec![],
        vec![],
        None,
        String::from("a\nb\nc"),
    );
    assert_eq!(s.output_buffer(), "a\nb\nc");
}

#[test]
fn new_preserves_output_buffer_with_unicode() {
    let s = TempState::new(
        PathBuf::from("/tmp/out"),
        PathBuf::from("/tmp/master"),
        PathBuf::from("/tmp"),
        vec![],
        vec![],
        None,
        String::from("日本語 🚀"),
    );
    assert_eq!(s.output_buffer(), "日本語 🚀");
}

#[test]
fn new_with_deeply_nested_paths() {
    let deep = PathBuf::from("/a/b/c/d/e/f/g/h");
    let s = TempState::new(
        deep.clone(),
        deep.clone(),
        deep.clone(),
        vec![deep.clone()],
        vec![],
        Some(deep.clone()),
        String::new(),
    );
    assert_eq!(s.new_temp_file(), &deep);
    assert_eq!(s.master_record_file(), &deep);
    assert_eq!(s.temprs_dir(), &deep);
    assert_eq!(s.temp_file_stack()[0], deep);
    assert_eq!(s.arg_file(), &Some(deep));
}

// ── getter/setter interaction sequences ───────────────

#[test]
fn set_all_then_reset_all_to_defaults() {
    let mut s = make_state();
    // set everything non-default
    s.set_verbose(10);
    s.set_silent(true);
    s.set_holding_buffer("hold".to_string());
    s.set_output_buffer("out".to_string());
    s.set_input_temp_file(Some("1".to_string()));
    s.set_output_temp_file(Some("2".to_string()));
    s.set_insert_idx(Some("3".to_string()));
    s.set_arg_file(Some(PathBuf::from("/tmp/arg")));
    // reset to defaults
    s.set_verbose(0);
    s.set_silent(false);
    s.set_holding_buffer(String::new());
    s.set_output_buffer(String::new());
    s.set_input_temp_file(None);
    s.set_output_temp_file(None);
    s.set_insert_idx(None);
    s.set_arg_file(None);
    // verify defaults
    assert_eq!(s.verbose(), 0);
    assert!(!s.silent());
    assert!(s.holding_buffer().is_empty());
    assert!(s.output_buffer().is_empty());
    assert!(s.input_temp_file().is_none());
    assert!(s.output_temp_file().is_none());
    assert!(s.insert_idx().is_none());
    assert!(s.arg_file().is_none());
}

#[test]
fn set_holding_then_output_independent() {
    let mut s = make_state();
    s.set_holding_buffer("A".to_string());
    s.set_output_buffer("B".to_string());
    assert_eq!(s.holding_buffer(), "A");
    assert_eq!(s.output_buffer(), "B");
}

#[test]
fn set_verbose_doesnt_affect_output_buffer() {
    let mut s = make_state();
    s.set_output_buffer("original".to_string());
    s.set_verbose(42);
    assert_eq!(s.output_buffer(), "original");
}

#[test]
fn set_silent_doesnt_affect_holding_buffer() {
    let mut s = make_state();
    s.set_holding_buffer("keep_me".to_string());
    s.set_silent(true);
    assert_eq!(s.holding_buffer(), "keep_me");
}

#[test]
fn set_insert_idx_doesnt_affect_input_temp_file() {
    let mut s = make_state();
    s.set_input_temp_file(Some("99".to_string()));
    s.set_insert_idx(Some("5".to_string()));
    assert_eq!(s.input_temp_file(), &Some("99".to_string()));
}

#[test]
fn set_output_temp_file_doesnt_affect_insert_idx() {
    let mut s = make_state();
    s.set_insert_idx(Some("7".to_string()));
    s.set_output_temp_file(Some("88".to_string()));
    assert_eq!(s.insert_idx(), &Some("7".to_string()));
}

// ── path mutation sequences ───────────────────────────

#[test]
fn set_new_temp_file_multiple_times() {
    let mut s = make_state();
    for i in 0..5 {
        s.set_new_temp_file(PathBuf::from(format!("/tmp/path{}", i)));
    }
    assert_eq!(s.new_temp_file(), &PathBuf::from("/tmp/path4"));
}

#[test]
fn set_master_record_file_with_unicode() {
    let mut s = make_state();
    s.set_master_record_file(PathBuf::from("/tmp/日本語/マスター"));
    assert_eq!(
        s.master_record_file(),
        &PathBuf::from("/tmp/日本語/マスター")
    );
}

#[test]
fn set_temprs_dir_deeply_nested() {
    let mut s = make_state();
    s.set_temprs_dir(PathBuf::from("/a/b/c/d/e/f"));
    assert_eq!(s.temprs_dir(), &PathBuf::from("/a/b/c/d/e/f"));
}

#[test]
fn set_temp_file_stack_grow_and_shrink() {
    let mut s = make_state();
    // 3 items
    s.set_temp_file_stack(vec![
        PathBuf::from("/a"),
        PathBuf::from("/b"),
        PathBuf::from("/c"),
    ]);
    assert_eq!(s.temp_file_stack().len(), 3);
    // grow to 10
    let big: Vec<PathBuf> = (0..10).map(|i| PathBuf::from(format!("/f{}", i))).collect();
    s.set_temp_file_stack(big);
    assert_eq!(s.temp_file_stack().len(), 10);
    // shrink to 1
    s.set_temp_file_stack(vec![PathBuf::from("/only")]);
    assert_eq!(s.temp_file_stack().len(), 1);
    assert_eq!(s.temp_file_stack()[0], PathBuf::from("/only"));
}

// ── buffer content edge cases ─────────────────────────

#[test]
fn holding_buffer_with_null_bytes() {
    let mut s = make_state();
    s.set_holding_buffer("data\0more".to_string());
    assert_eq!(s.holding_buffer(), "data\0more");
}

#[test]
fn output_buffer_with_carriage_returns() {
    let mut s = make_state();
    s.set_output_buffer("line1\r\nline2\r\n".to_string());
    assert_eq!(s.output_buffer(), "line1\r\nline2\r\n");
}

#[test]
fn holding_buffer_only_whitespace() {
    let mut s = make_state();
    s.set_holding_buffer("   \t\n  ".to_string());
    assert_eq!(s.holding_buffer(), "   \t\n  ");
    assert!(!s.holding_buffer().is_empty());
}

#[test]
fn output_buffer_very_long() {
    let mut s = make_state();
    let long = "x".repeat(100_000);
    s.set_output_buffer(long.clone());
    assert_eq!(s.output_buffer(), long);
    assert_eq!(s.output_buffer().len(), 100_000);
}

#[test]
fn holding_buffer_repeated_set_and_check() {
    let mut s = make_state();
    for i in 0..20 {
        s.set_holding_buffer(format!("val_{}", i));
    }
    assert_eq!(s.holding_buffer(), "val_19");
}

// ── index fields edge cases ───────────────────────────

#[test]
fn insert_idx_zero_string() {
    let mut s = make_state();
    s.set_insert_idx(Some("0".to_string()));
    assert_eq!(s.insert_idx(), &Some("0".to_string()));
}

#[test]
fn input_temp_file_zero_string() {
    let mut s = make_state();
    s.set_input_temp_file(Some("0".to_string()));
    assert_eq!(s.input_temp_file(), &Some("0".to_string()));
}

#[test]
fn output_temp_file_zero_string() {
    let mut s = make_state();
    s.set_output_temp_file(Some("0".to_string()));
    assert_eq!(s.output_temp_file(), &Some("0".to_string()));
}

#[test]
fn insert_idx_non_numeric() {
    let mut s = make_state();
    s.set_insert_idx(Some("abc".to_string()));
    assert_eq!(s.insert_idx(), &Some("abc".to_string()));
}

#[test]
fn input_temp_file_non_numeric() {
    let mut s = make_state();
    s.set_input_temp_file(Some("xyz".to_string()));
    assert_eq!(s.input_temp_file(), &Some("xyz".to_string()));
}

#[test]
fn output_temp_file_non_numeric() {
    let mut s = make_state();
    s.set_output_temp_file(Some("not_a_number".to_string()));
    assert_eq!(s.output_temp_file(), &Some("not_a_number".to_string()));
}

// ── out_file_path_str / master_file_path_str edge cases

#[test]
fn out_file_path_str_with_unicode() {
    let mut s = make_state();
    s.set_new_temp_file(PathBuf::from("/tmp/日本語/ファイル"));
    assert_eq!(s.out_file_path_str(), "/tmp/日本語/ファイル");
}

#[test]
fn out_file_path_str_relative_path() {
    let mut s = make_state();
    s.set_new_temp_file(PathBuf::from("foo/bar"));
    assert_eq!(s.out_file_path_str(), "foo/bar");
}

#[test]
fn master_file_path_str_with_spaces() {
    let mut s = make_state();
    s.set_master_record_file(PathBuf::from("/tmp/my dir/master file"));
    assert_eq!(s.master_file_path_str(), "/tmp/my dir/master file");
}

#[test]
fn out_file_path_str_very_long_path() {
    let mut s = make_state();
    let long_path = format!("/tmp/{}", "a".repeat(500));
    s.set_new_temp_file(PathBuf::from(&long_path));
    assert_eq!(s.out_file_path_str(), long_path);
}

// ── arg file edge cases ───────────────────────────────

#[test]
fn arg_file_relative_path() {
    let mut s = make_state();
    s.set_arg_file(Some(PathBuf::from("relative/path.txt")));
    assert_eq!(s.arg_file(), &Some(PathBuf::from("relative/path.txt")));
}

#[test]
fn arg_file_root_path() {
    let mut s = make_state();
    s.set_arg_file(Some(PathBuf::from("/")));
    assert_eq!(s.arg_file(), &Some(PathBuf::from("/")));
}

#[test]
fn arg_file_with_unicode() {
    let mut s = make_state();
    s.set_arg_file(Some(PathBuf::from("/tmp/日本語/file")));
    assert_eq!(s.arg_file(), &Some(PathBuf::from("/tmp/日本語/file")));
}

#[test]
fn arg_file_set_check_set_none_check() {
    let mut s = make_state();
    s.set_arg_file(Some(PathBuf::from("/tmp/present")));
    assert_eq!(s.arg_file(), &Some(PathBuf::from("/tmp/present")));
    s.set_arg_file(None);
    assert!(s.arg_file().is_none());
}

// ── stack content verification ────────────────────────

#[test]
fn temp_file_stack_paths_preserved_exactly() {
    let mut s = make_state();
    let paths: Vec<PathBuf> = vec![
        PathBuf::from("/alpha"),
        PathBuf::from("/beta"),
        PathBuf::from("/gamma"),
        PathBuf::from("/delta"),
        PathBuf::from("/epsilon"),
    ];
    s.set_temp_file_stack(paths.clone());
    for (i, p) in paths.iter().enumerate() {
        assert_eq!(&s.temp_file_stack()[i], p);
    }
}

#[test]
fn temp_file_stack_with_unicode_paths() {
    let mut s = make_state();
    s.set_temp_file_stack(vec![
        PathBuf::from("/tmp/日本語"),
        PathBuf::from("/tmp/中文"),
        PathBuf::from("/tmp/한국어"),
    ]);
    assert_eq!(s.temp_file_stack().len(), 3);
    assert_eq!(s.temp_file_stack()[0], PathBuf::from("/tmp/日本語"));
    assert_eq!(s.temp_file_stack()[1], PathBuf::from("/tmp/中文"));
    assert_eq!(s.temp_file_stack()[2], PathBuf::from("/tmp/한국어"));
}

#[test]
fn temp_file_stack_with_duplicate_paths() {
    let mut s = make_state();
    let dup = PathBuf::from("/tmp/same");
    s.set_temp_file_stack(vec![dup.clone(), dup.clone()]);
    assert_eq!(s.temp_file_stack().len(), 2);
    assert_eq!(s.temp_file_stack()[0], dup);
    assert_eq!(s.temp_file_stack()[1], dup);
}

#[test]
fn temp_file_stack_with_relative_paths() {
    let mut s = make_state();
    s.set_temp_file_stack(vec![
        PathBuf::from("relative/one"),
        PathBuf::from("relative/two"),
    ]);
    assert_eq!(s.temp_file_stack()[0], PathBuf::from("relative/one"));
    assert_eq!(s.temp_file_stack()[1], PathBuf::from("relative/two"));
}

// ── multiple field mutation stress test ────────────────

#[test]
fn rapid_mutation_cycle() {
    let mut s = make_state();
    for i in 0..50u32 {
        s.set_verbose(i);
        s.set_holding_buffer(format!("h{}", i));
        s.set_output_buffer(format!("o{}", i));
    }
    assert_eq!(s.verbose(), 49);
    assert_eq!(s.holding_buffer(), "h49");
    assert_eq!(s.output_buffer(), "o49");
}

#[test]
fn all_optional_fields_some_then_none() {
    let mut s = make_state();
    // set all Option fields to Some
    s.set_arg_file(Some(PathBuf::from("/tmp/arg")));
    s.set_insert_idx(Some("5".to_string()));
    s.set_input_temp_file(Some("10".to_string()));
    s.set_output_temp_file(Some("20".to_string()));
    // verify all Some
    assert!(s.arg_file().is_some());
    assert!(s.insert_idx().is_some());
    assert!(s.input_temp_file().is_some());
    assert!(s.output_temp_file().is_some());
    // set all to None
    s.set_arg_file(None);
    s.set_insert_idx(None);
    s.set_input_temp_file(None);
    s.set_output_temp_file(None);
    // verify all None
    assert!(s.arg_file().is_none());
    assert!(s.insert_idx().is_none());
    assert!(s.input_temp_file().is_none());
    assert!(s.output_temp_file().is_none());
}

// ── constructor with various stack sizes ────────────

#[test]
fn new_with_single_item_stack() {
    let s = TempState::new(
        PathBuf::from("/tmp/out"),
        PathBuf::from("/tmp/master"),
        PathBuf::from("/tmp"),
        vec![PathBuf::from("/tmp/f1")],
        vec![],
        None,
        String::new(),
    );
    assert_eq!(s.temp_file_stack().len(), 1);
    assert_eq!(s.temp_file_stack()[0], PathBuf::from("/tmp/f1"));
}

#[test]
fn new_with_hundred_item_stack() {
    let stack: Vec<PathBuf> = (0..100)
        .map(|i| PathBuf::from(format!("/tmp/f{}", i)))
        .collect();
    let s = TempState::new(
        PathBuf::from("/tmp/out"),
        PathBuf::from("/tmp/master"),
        PathBuf::from("/tmp"),
        stack.clone(),
        vec![],
        None,
        String::new(),
    );
    assert_eq!(s.temp_file_stack().len(), 100);
    assert_eq!(s.temp_file_stack()[99], PathBuf::from("/tmp/f99"));
}

// ── holding_buffer and output_buffer interactions ────

#[test]
fn holding_and_output_buffer_independent() {
    let mut s = make_state();
    s.set_holding_buffer("hold".to_string());
    s.set_output_buffer("out".to_string());
    assert_eq!(s.holding_buffer(), "hold");
    assert_eq!(s.output_buffer(), "out");
}

#[test]
fn buffers_can_hold_same_content() {
    let mut s = make_state();
    s.set_holding_buffer("same".to_string());
    s.set_output_buffer("same".to_string());
    assert_eq!(s.holding_buffer(), s.output_buffer());
}

#[test]
fn holding_buffer_whitespace_only() {
    let mut s = make_state();
    s.set_holding_buffer("   \t\n  ".to_string());
    assert_eq!(s.holding_buffer(), "   \t\n  ");
    assert!(!s.holding_buffer().is_empty());
}

#[test]
fn output_buffer_null_bytes() {
    let mut s = make_state();
    s.set_output_buffer("hello\0world".to_string());
    assert_eq!(s.output_buffer(), "hello\0world");
}

// ── input/output_temp_file independence ──────────────

#[test]
fn input_and_output_temp_file_independent() {
    let mut s = make_state();
    s.set_input_temp_file(Some("1".to_string()));
    s.set_output_temp_file(Some("2".to_string()));
    assert_eq!(s.input_temp_file(), &Some("1".to_string()));
    assert_eq!(s.output_temp_file(), &Some("2".to_string()));
}

#[test]
fn input_and_output_temp_file_same_value() {
    let mut s = make_state();
    s.set_input_temp_file(Some("5".to_string()));
    s.set_output_temp_file(Some("5".to_string()));
    assert_eq!(s.input_temp_file(), s.output_temp_file());
}

// ── setter idempotency ──────────────────────────────

#[test]
fn set_verbose_same_value_twice() {
    let mut s = make_state();
    s.set_verbose(3);
    s.set_verbose(3);
    assert_eq!(s.verbose(), 3);
}

#[test]
fn set_silent_same_value_twice() {
    let mut s = make_state();
    s.set_silent(true);
    s.set_silent(true);
    assert!(s.silent());
}

#[test]
fn set_holding_buffer_same_value_twice() {
    let mut s = make_state();
    s.set_holding_buffer("data".to_string());
    s.set_holding_buffer("data".to_string());
    assert_eq!(s.holding_buffer(), "data");
}

// ── new_temp_file and master_record_file with unicode paths ──

#[test]
fn new_temp_file_unicode_path() {
    let mut s = make_state();
    s.set_new_temp_file(PathBuf::from("/tmp/日本語/tempfile"));
    assert_eq!(s.new_temp_file(), &PathBuf::from("/tmp/日本語/tempfile"));
}

#[test]
fn master_record_file_unicode_path() {
    let mut s = make_state();
    s.set_master_record_file(PathBuf::from("/tmp/日本語/master"));
    assert_eq!(s.master_record_file(), &PathBuf::from("/tmp/日本語/master"));
}

#[test]
fn temprs_dir_unicode_path() {
    let mut s = make_state();
    s.set_temprs_dir(PathBuf::from("/tmp/日本語"));
    assert_eq!(s.temprs_dir(), &PathBuf::from("/tmp/日本語"));
}

// ── path str helpers with unicode ───────────────────

#[test]
fn out_file_path_str_unicode() {
    let s = TempState::new(
        PathBuf::from("/tmp/日本語/tempfile"),
        PathBuf::from("/tmp/master"),
        PathBuf::from("/tmp"),
        vec![],
        vec![],
        None,
        String::new(),
    );
    assert_eq!(s.out_file_path_str(), "/tmp/日本語/tempfile");
}

#[test]
fn master_file_path_str_unicode() {
    let s = TempState::new(
        PathBuf::from("/tmp/out"),
        PathBuf::from("/tmp/日本語/master"),
        PathBuf::from("/tmp"),
        vec![],
        vec![],
        None,
        String::new(),
    );
    assert_eq!(s.master_file_path_str(), "/tmp/日本語/master");
}

// ── temp_file_stack manipulations ───────────────────

#[test]
fn set_temp_file_stack_preserves_order() {
    let mut s = make_state();
    let stack: Vec<PathBuf> = (0..5).map(|i| PathBuf::from(format!("/f{}", i))).collect();
    s.set_temp_file_stack(stack.clone());
    for (i, p) in s.temp_file_stack().iter().enumerate() {
        assert_eq!(p, &stack[i]);
    }
}

#[test]
fn set_temp_file_stack_unicode_paths() {
    let mut s = make_state();
    let stack = vec![
        PathBuf::from("/tmp/日本語"),
        PathBuf::from("/tmp/café"),
        PathBuf::from("/tmp/🚀"),
    ];
    s.set_temp_file_stack(stack.clone());
    assert_eq!(s.temp_file_stack(), &stack);
}

#[test]
fn set_temp_file_stack_replace_multiple_times() {
    let mut s = make_state();
    for size in [0usize, 5, 1, 10, 3] {
        let stack: Vec<PathBuf> = (0..size)
            .map(|i| PathBuf::from(format!("/f{}", i)))
            .collect();
        s.set_temp_file_stack(stack.clone());
        assert_eq!(s.temp_file_stack().len(), size);
    }
}

// ── insert_idx with special values ──────────────────

#[test]
fn insert_idx_zero() {
    let mut s = make_state();
    s.set_insert_idx(Some("0".to_string()));
    assert_eq!(s.insert_idx(), &Some("0".to_string()));
}

#[test]
fn insert_idx_empty_string() {
    let mut s = make_state();
    s.set_insert_idx(Some(String::new()));
    assert_eq!(s.insert_idx(), &Some(String::new()));
}

// ── arg_file with unicode paths ─────────────────────

#[test]
fn arg_file_unicode_path() {
    let mut s = make_state();
    s.set_arg_file(Some(PathBuf::from("/tmp/日本語/data.txt")));
    assert_eq!(s.arg_file(), &Some(PathBuf::from("/tmp/日本語/data.txt")));
}

#[test]
fn arg_file_with_emoji_path() {
    let mut s = make_state();
    s.set_arg_file(Some(PathBuf::from("/tmp/🚀/launch.txt")));
    assert_eq!(s.arg_file(), &Some(PathBuf::from("/tmp/🚀/launch.txt")));
}

// ── verbose boundary values additional ───────────────

#[test]
fn verbose_two() {
    let mut s = make_state();
    s.set_verbose(2);
    assert_eq!(s.verbose(), 2);
}

#[test]
fn verbose_ten() {
    let mut s = make_state();
    s.set_verbose(10);
    assert_eq!(s.verbose(), 10);
}

#[test]
fn verbose_hundred() {
    let mut s = make_state();
    s.set_verbose(100);
    assert_eq!(s.verbose(), 100);
}

// ── buffers with very large content ─────────────────

#[test]
fn holding_buffer_100k() {
    let mut s = make_state();
    let large = "z".repeat(100_000);
    s.set_holding_buffer(large.clone());
    assert_eq!(s.holding_buffer().len(), 100_000);
}

#[test]
fn output_buffer_100k() {
    let mut s = make_state();
    let large = "w".repeat(100_000);
    s.set_output_buffer(large.clone());
    assert_eq!(s.output_buffer().len(), 100_000);
}

// ── multiple field modifications in sequence ────────

#[test]
fn full_state_modification_cycle() {
    let mut s = make_state();
    s.set_verbose(2);
    s.set_silent(true);
    s.set_holding_buffer("hold".to_string());
    s.set_output_buffer("out".to_string());
    s.set_input_temp_file(Some("1".to_string()));
    s.set_output_temp_file(Some("2".to_string()));
    s.set_insert_idx(Some("3".to_string()));
    s.set_arg_file(Some(PathBuf::from("/tmp/f")));
    s.set_new_temp_file(PathBuf::from("/tmp/new"));
    s.set_master_record_file(PathBuf::from("/tmp/master2"));
    s.set_temprs_dir(PathBuf::from("/tmp/dir2"));
    s.set_temp_file_stack(vec![PathBuf::from("/x")]);

    assert_eq!(s.verbose(), 2);
    assert!(s.silent());
    assert_eq!(s.holding_buffer(), "hold");
    assert_eq!(s.output_buffer(), "out");
    assert_eq!(s.input_temp_file(), &Some("1".to_string()));
    assert_eq!(s.output_temp_file(), &Some("2".to_string()));
    assert_eq!(s.insert_idx(), &Some("3".to_string()));
    assert_eq!(s.arg_file(), &Some(PathBuf::from("/tmp/f")));
    assert_eq!(s.new_temp_file(), &PathBuf::from("/tmp/new"));
    assert_eq!(s.master_record_file(), &PathBuf::from("/tmp/master2"));
    assert_eq!(s.temprs_dir(), &PathBuf::from("/tmp/dir2"));
    assert_eq!(s.temp_file_stack().len(), 1);
}

#[test]
fn reset_all_optionals_to_none() {
    let mut s = make_state();
    s.set_input_temp_file(Some("1".to_string()));
    s.set_output_temp_file(Some("2".to_string()));
    s.set_insert_idx(Some("3".to_string()));
    s.set_arg_file(Some(PathBuf::from("/tmp/f")));

    s.set_input_temp_file(None);
    s.set_output_temp_file(None);
    s.set_insert_idx(None);
    s.set_arg_file(None);

    assert!(s.input_temp_file().is_none());
    assert!(s.output_temp_file().is_none());
    assert!(s.insert_idx().is_none());
    assert!(s.arg_file().is_none());
}

// ── constructor preserves all passed values ─────────

#[test]
fn constructor_preserves_all_fields() {
    let out = PathBuf::from("/custom/out");
    let master = PathBuf::from("/custom/master");
    let dir = PathBuf::from("/custom");
    let stack = vec![PathBuf::from("/custom/f1"), PathBuf::from("/custom/f2")];
    let arg = Some(PathBuf::from("/custom/arg"));
    let buf = String::from("initial buffer");

    let s = TempState::new(
        out.clone(),
        master.clone(),
        dir.clone(),
        stack.clone(),
        vec![],
        arg.clone(),
        buf.clone(),
    );

    assert_eq!(s.new_temp_file(), &out);
    assert_eq!(s.master_record_file(), &master);
    assert_eq!(s.temprs_dir(), &dir);
    assert_eq!(s.temp_file_stack(), &stack);
    assert_eq!(s.arg_file(), &arg);
    assert_eq!(s.output_buffer(), buf);
}

// ── path str for paths with special characters ──────

#[test]
fn out_file_path_str_with_dashes() {
    let s = TempState::new(
        PathBuf::from("/tmp/temp-file-123"),
        PathBuf::from("/tmp/m"),
        PathBuf::from("/tmp"),
        vec![],
        vec![],
        None,
        String::new(),
    );
    assert_eq!(s.out_file_path_str(), "/tmp/temp-file-123");
}

#[test]
fn out_file_path_str_with_underscores() {
    let s = TempState::new(
        PathBuf::from("/tmp/temp_file_456"),
        PathBuf::from("/tmp/m"),
        PathBuf::from("/tmp"),
        vec![],
        vec![],
        None,
        String::new(),
    );
    assert_eq!(s.out_file_path_str(), "/tmp/temp_file_456");
}

#[test]
fn out_file_path_str_with_dots() {
    let s = TempState::new(
        PathBuf::from("/tmp/file.txt.bak"),
        PathBuf::from("/tmp/m"),
        PathBuf::from("/tmp"),
        vec![],
        vec![],
        None,
        String::new(),
    );
    assert_eq!(s.out_file_path_str(), "/tmp/file.txt.bak");
}

// ── temp_file_names, append_temp_file, name ─────────

#[test]
fn set_get_temp_file_names() {
    let mut s = make_state();
    let names = vec![
        Some("a".to_string()),
        Some("b".to_string()),
        Some("c".to_string()),
    ];
    s.set_temp_file_stack(vec![
        PathBuf::from("/x"),
        PathBuf::from("/y"),
        PathBuf::from("/z"),
    ]);
    s.set_temp_file_names(names.clone());
    assert_eq!(s.temp_file_names(), &names);
}

#[test]
fn set_get_append_temp_file() {
    let mut s = make_state();
    s.set_append_temp_file(Some("7".to_string()));
    assert_eq!(s.append_temp_file(), &Some("7".to_string()));
    s.set_append_temp_file(None);
    assert!(s.append_temp_file().is_none());
}

#[test]
fn set_get_name() {
    let mut s = make_state();
    s.set_name(Some("my-tag".to_string()));
    assert_eq!(s.name(), &Some("my-tag".to_string()));
    s.set_name(None);
    assert!(s.name().is_none());
}

#[test]
fn write_master_round_trip_paths_and_names() {
    let dir = state_test_tmp_dir();
    let master = dir.join("temprs-stack");
    let paths = vec![
        PathBuf::from("/stack/a"),
        PathBuf::from("/stack/b"),
    ];
    let names = vec![Some("first".to_string()), None];
    let s = TempState::new(
        PathBuf::from("/tmp/out"),
        master.clone(),
        dir.clone(),
        paths.clone(),
        names.clone(),
        None,
        String::new(),
    );
    s.write_master();
    let (loaded_paths, loaded_names) = util_file_to_paths_and_names(&master);
    assert_eq!(loaded_paths, paths);
    assert_eq!(loaded_names, names);
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn new_constructor_preserves_temp_file_names() {
    let names = vec![Some("x".to_string()), None, Some("z".to_string())];
    let stack = vec![
        PathBuf::from("/a"),
        PathBuf::from("/b"),
        PathBuf::from("/c"),
    ];
    let s = TempState::new(
        PathBuf::from("/out"),
        PathBuf::from("/master"),
        PathBuf::from("/home"),
        stack,
        names.clone(),
        None,
        String::new(),
    );
    assert_eq!(s.temp_file_names(), &names);
}

// ── mut accessors (used by TempApp) ─────────────────

#[test]
fn holding_buffer_mut_take_and_replace() {
    let mut s = make_state();
    s.set_holding_buffer("before".to_string());
    let taken = std::mem::take(s.holding_buffer_mut());
    assert_eq!(taken, "before");
    assert!(s.holding_buffer().is_empty());
    s.holding_buffer_mut().push_str("after");
    assert_eq!(s.holding_buffer(), "after");
}

#[test]
fn temp_file_stack_mut_push_pop() {
    let mut s = make_state();
    s.temp_file_stack_mut()
        .push(PathBuf::from("/tmp/extra"));
    assert_eq!(s.temp_file_stack().len(), 3);
    let _ = s.temp_file_stack_mut().pop();
    assert_eq!(s.temp_file_stack().len(), 2);
}

#[test]
fn temp_file_names_mut_push_aligns_with_stack() {
    let mut s = make_state();
    s.temp_file_stack_mut().push(PathBuf::from("/p3"));
    s.temp_file_names_mut().push(Some("third".to_string()));
    assert_eq!(s.temp_file_stack().len(), 3);
    assert_eq!(s.temp_file_names().len(), 3);
    assert_eq!(s.temp_file_names()[2], Some("third".to_string()));
}

#[test]
fn temp_file_names_mut_overwrite_slot() {
    let mut s = make_state();
    s.temp_file_names_mut()[0] = Some("tag-a".to_string());
    assert_eq!(s.temp_file_names()[0], Some("tag-a".to_string()));
}

#[test]
fn temp_file_names_mut_clear_name() {
    let mut s = make_state();
    s.temp_file_names_mut()[0] = None;
    assert!(s.temp_file_names()[0].is_none());
}

#[test]
fn temp_file_stack_mut_clear() {
    let mut s = make_state();
    s.temp_file_stack_mut().clear();
    assert!(s.temp_file_stack().is_empty());
}

#[test]
fn temp_file_names_mut_truncate_with_stack() {
    let mut s = make_state();
    s.temp_file_stack_mut().truncate(1);
    s.temp_file_names_mut().truncate(1);
    assert_eq!(s.temp_file_stack().len(), 1);
    assert_eq!(s.temp_file_names().len(), 1);
}

#[test]
fn temp_file_stack_mut_swap_elements() {
    let mut s = make_state();
    let stk = s.temp_file_stack_mut();
    if stk.len() >= 2 {
        stk.swap(0, 1);
    }
    assert_eq!(s.temp_file_stack()[0], PathBuf::from("/tmp/temprs/f2"));
    assert_eq!(s.temp_file_stack()[1], PathBuf::from("/tmp/temprs/f1"));
}

#[test]
fn temp_file_names_mut_extend_from_slice() {
    let mut s = make_state();
    s.temp_file_names_mut().extend([Some("n3".to_string()), None]);
    s.temp_file_stack_mut()
        .extend([PathBuf::from("/p3"), PathBuf::from("/p4")]);
    assert_eq!(s.temp_file_stack().len(), 4);
    assert_eq!(s.temp_file_names().len(), 4);
}

#[test]
fn holding_buffer_mut_clear_via_truncate() {
    let mut s = make_state();
    s.set_holding_buffer("truncate me".to_string());
    s.holding_buffer_mut().truncate(0);
    assert!(s.holding_buffer().is_empty());
}

#[test]
fn holding_buffer_mut_replace_range() {
    let mut s = make_state();
    s.set_holding_buffer("abcdef".to_string());
    s.holding_buffer_mut().replace_range(1..4, "XYZ");
    assert_eq!(s.holding_buffer(), "aXYZef");
}

#[test]
fn temp_file_stack_mut_dedup_paths_allowed() {
    let mut s = make_state();
    let p = PathBuf::from("/same");
    s.set_temp_file_stack(vec![p.clone(), p.clone()]);
    assert_eq!(s.temp_file_stack()[0], s.temp_file_stack()[1]);
}

#[test]
fn all_optional_setters_roundtrip_twice() {
    let mut s = make_state();
    for _ in 0..2 {
        s.set_name(Some("n".to_string()));
        s.set_append_temp_file(Some("1".to_string()));
        assert!(s.name().is_some());
        assert!(s.append_temp_file().is_some());
        s.set_name(None);
        s.set_append_temp_file(None);
        assert!(s.name().is_none());
        assert!(s.append_temp_file().is_none());
    }
}

#[test]
fn temp_file_names_all_some() {
    let mut s = make_state();
    s.set_temp_file_names(vec![
        Some("a".to_string()),
        Some("b".to_string()),
    ]);
    assert!(s.temp_file_names().iter().all(|n| n.is_some()));
}

#[test]
fn write_master_empty_stack_writes_file() {
    let dir = state_test_tmp_dir();
    let master = dir.join("empty-stack");
    let s = TempState::new(
        PathBuf::from("/out"),
        master.clone(),
        dir.clone(),
        vec![],
        vec![],
        None,
        String::new(),
    );
    s.write_master();
    let (p, n) = util_file_to_paths_and_names(&master);
    assert!(p.is_empty());
    assert!(n.is_empty());
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn write_master_single_named_entry() {
    let dir = state_test_tmp_dir();
    let master = dir.join("m");
    let paths = vec![PathBuf::from("/only")];
    let names = vec![Some("solo".to_string())];
    let s = TempState::new(
        PathBuf::from("/out"),
        master.clone(),
        dir.clone(),
        paths.clone(),
        names.clone(),
        None,
        String::new(),
    );
    s.write_master();
    let (p, n) = util_file_to_paths_and_names(&master);
    assert_eq!(p, paths);
    assert_eq!(n, names);
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn temp_file_stack_mut_iter_mut_len() {
    let mut s = make_state();
    let n: usize = s.temp_file_stack_mut().iter().map(|p| p.as_os_str().len()).sum();
    assert!(n > 0);
    assert_eq!(s.temp_file_stack().len(), 2);
}

#[test]
fn temp_file_names_mut_resize_with_default() {
    let mut s = make_state();
    s.temp_file_names_mut().resize(4, None);
    s.temp_file_stack_mut().resize(
        4,
        PathBuf::from("/fill"),
    );
    assert_eq!(s.temp_file_names().len(), 4);
    assert_eq!(s.temp_file_stack().len(), 4);
}

#[test]
fn set_verbose_large_then_zero() {
    let mut s = make_state();
    s.set_verbose(1_000_000);
    assert_eq!(s.verbose(), 1_000_000);
    s.set_verbose(0);
    assert_eq!(s.verbose(), 0);
}

#[test]
fn output_buffer_clone_from_holding() {
    let mut s = make_state();
    s.set_holding_buffer("shared".to_string());
    s.set_output_buffer(s.holding_buffer().to_string());
    s.set_holding_buffer(String::new());
    assert_eq!(s.output_buffer(), "shared");
    assert!(s.holding_buffer().is_empty());
}

#[test]
fn append_temp_file_roundtrip() {
    let mut s = make_state();
    s.set_append_temp_file(Some("9".to_string()));
    assert_eq!(s.append_temp_file(), &Some("9".to_string()));
    s.set_append_temp_file(Some("-1".to_string()));
    assert_eq!(s.append_temp_file(), &Some("-1".to_string()));
}
