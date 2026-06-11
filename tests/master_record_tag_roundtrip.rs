//! Round-trip / alignment invariants for the master-record serialization in
//! `util_paths_and_names_to_file` <-> `util_file_to_paths_and_names`.
//!
//! The master record stores two parallel arrays (paths + optional tag names)
//! flattened into one file using `MASTER_FIELD_DELIM` ('\0') between a path and
//! its tag, and `MASTER_RECORD_DELIM` ("\0\0") between records. Existing tests
//! cover the single-tagged + single-untagged case and newline-in-path cases.
//! They do NOT cover:
//!   - several CONSECUTIVE tagged records (the joining logic emits a record
//!     that ends with a name, then `\0\0`, then the next path — a field-vs-
//!     record delimiter mixup or an off-by-one in the join would corrupt the
//!     parallel-array alignment here, not in the 1-tag case).
//!   - a tag whose text is identical to a path component (catches a swapped
//!     `split_once` order — splitting on the wrong delimiter would mis-assign
//!     which side is the path and which is the tag).
//!   - distinguishing "tagged with empty string" from "untagged": the writer
//!     emits nothing for a `None` tag and the reader maps an empty field back
//!     to `None`, so `Some(String::new())` must NOT survive as `Some("")`.
//!
//! These are alignment/format invariants, not smoke tests: a passing-for-any-
//! impl mirror test would not detect a record-vs-field delimiter swap.

use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use temprs::util::utils::{util_file_to_paths_and_names, util_paths_and_names_to_file};

static COUNTER: AtomicU64 = AtomicU64::new(0);

fn tmp_master() -> PathBuf {
    let n = COUNTER.fetch_add(1, Ordering::SeqCst);
    let dir = std::env::temp_dir().join(format!("temprs_tag_rt_{}_{}", std::process::id(), n));
    fs::create_dir_all(&dir).unwrap();
    dir.join("master")
}

/// Several consecutive tagged records plus an untagged one in the middle.
/// A field/record delimiter mixup or a join off-by-one would desync the
/// returned paths from their names; we assert exact parallel-array equality.
#[test]
fn multiple_consecutive_tags_keep_alignment() {
    let master = tmp_master();
    let paths = vec![
        PathBuf::from("/tmp/alpha"),
        PathBuf::from("/tmp/bravo"),
        PathBuf::from("/tmp/charlie"),
        PathBuf::from("/tmp/delta"),
    ];
    let names = vec![
        Some("first".to_string()),
        Some("second".to_string()),
        None,
        Some("fourth".to_string()),
    ];

    util_paths_and_names_to_file(&paths, &names, &master);
    let (loaded_paths, loaded_names) = util_file_to_paths_and_names(master.as_path());

    assert_eq!(
        loaded_paths, paths,
        "paths desynced across multi-tag records"
    );
    assert_eq!(
        loaded_names, names,
        "names desynced across multi-tag records"
    );
    // Parallel arrays must stay the same length, or downstream index logic
    // would read a name belonging to a different path.
    assert_eq!(loaded_paths.len(), loaded_names.len());

    fs::remove_dir_all(master.parent().unwrap()).unwrap();
}

/// A tag whose text equals a path component. If the reader split on the wrong
/// delimiter or in the wrong order it could assign "shared" to the path side.
#[test]
fn tag_text_equal_to_path_component_round_trips() {
    let master = tmp_master();
    let paths = vec![PathBuf::from("/tmp/shared"), PathBuf::from("/tmp/other")];
    let names = vec![Some("shared".to_string()), None];

    util_paths_and_names_to_file(&paths, &names, &master);
    let (loaded_paths, loaded_names) = util_file_to_paths_and_names(master.as_path());

    assert_eq!(loaded_paths[0], PathBuf::from("/tmp/shared"));
    assert_eq!(loaded_names[0], Some("shared".to_string()));
    assert_eq!(loaded_paths[1], PathBuf::from("/tmp/other"));
    assert_eq!(loaded_names[1], None);

    fs::remove_dir_all(master.parent().unwrap()).unwrap();
}

/// An empty-string tag is written by the format as "no tag" (the writer emits
/// just the path), and the reader maps the empty field back to `None`. So a
/// `Some(String::new())` input must come back as `None`, never `Some("")` —
/// otherwise tag-equality checks and `@`-name resolution would treat the empty
/// string as a usable tag.
#[test]
fn empty_string_tag_collapses_to_none() {
    let master = tmp_master();
    let paths = vec![PathBuf::from("/tmp/x")];
    let names = vec![Some(String::new())];

    util_paths_and_names_to_file(&paths, &names, &master);
    let (loaded_paths, loaded_names) = util_file_to_paths_and_names(master.as_path());

    assert_eq!(loaded_paths, vec![PathBuf::from("/tmp/x")]);
    assert_eq!(
        loaded_names,
        vec![None],
        "empty-string tag must collapse to None, not Some(\"\")"
    );

    fs::remove_dir_all(master.parent().unwrap()).unwrap();
}
