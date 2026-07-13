use log::Level;

/// `TEMP_LOG_LEVEL` — Level::Info
pub const TEMP_LOG_LEVEL: Level = Level::Info;
/// `TEMP_DIR` — "temprs"
pub const TEMP_DIR: &str = "temprs";
/// `MASTER_RECORD_FILENAME` — "temprs-stack"
pub const MASTER_RECORD_FILENAME: &str = "temprs-stack";
/// `TEMPFILE_PREFIX` — "tempfile"
pub const TEMPFILE_PREFIX: &str = "tempfile";
/// `ERR_NO_FILE` — "No such file"
pub const ERR_NO_FILE: &str = "No such file";
/// `ERR_PARSE` — "Could not parse line"
pub const ERR_PARSE: &str = "Could not parse line";
/// `ERR_INVALID_RM` — "Could not remove specified temporary file"
pub const ERR_INVALID_RM: &str = "Could not remove specified temporary file";
/// `ERR_INVALID_IDX` — "Invalid specified index"
pub const ERR_INVALID_IDX: &str = "Invalid specified index";
/// `ERR_INVALID_FILE` — "Invalid specified file argument"
pub const ERR_INVALID_FILE: &str = "Invalid specified file argument";
/// `ERR_LOGGER` — "Could not create logger"
pub const ERR_LOGGER: &str = "Could not create logger";
/// `ERR_FILE_READ` — "Could not read file"
pub const ERR_FILE_READ: &str = "Could not read file";
/// `ERR_CLOCK` — "Could not read clock"
pub const ERR_CLOCK: &str = "Could not read clock";
/// `NAME` — "temprs"
pub const NAME: &str = "temprs";
/// `HR_CHAR` — "-"
pub const HR_CHAR: &str = "-";
/// `DIRECTORY` — "directory"
pub const DIRECTORY: &str = "directory";
/// `INPUT` — "input"
pub const INPUT: &str = "input";
/// `OUTPUT` — "output"
pub const OUTPUT: &str = "output";
/// `ADD` — "add"
pub const ADD: &str = "add";
/// `REMOVE` — "remove"
pub const REMOVE: &str = "remove";
/// `POP` — "pop"
pub const POP: &str = "pop";
/// `UNSHIFT` — "unshift"
pub const UNSHIFT: &str = "unshift";
/// `ARGFILE` — "argfile"
pub const ARGFILE: &str = "argfile";
/// `MASTER` — "master"
pub const MASTER: &str = "master";
/// `VERBOSE` — "verbose"
pub const VERBOSE: &str = "verbose";
/// `LIST_FILES` — "list_files"
pub const LIST_FILES: &str = "list_files";
/// `LIST_FILES_NUMBERED` — "list_files_numbered"
pub const LIST_FILES_NUMBERED: &str = "list_files_numbered";
/// `LIST_CONTENTS` — "list_contents"
pub const LIST_CONTENTS: &str = "list_contents";
/// `LIST_CONTENTS_NUMBERED` — "list_contents_numbered"
pub const LIST_CONTENTS_NUMBERED: &str = "list_contents_numbered";
/// `CLEAR` — "clear"
pub const CLEAR: &str = "clear";
/// `SHIFT` — "shift"
pub const SHIFT: &str = "shift";
/// `SILENT` — "silent"
pub const SILENT: &str = "silent";
/// `EDIT` — "edit"
pub const EDIT: &str = "edit";
/// `TAG` — "name"
pub const TAG: &str = "name";
/// `RENAME` — "rename"
pub const RENAME: &str = "rename";
/// `INFO` — "info"
pub const INFO: &str = "info";
/// `GREP` — "grep"
pub const GREP: &str = "grep";
/// `CAT` — "cat"
pub const CAT: &str = "cat";
/// `COUNT` — "count"
pub const COUNT: &str = "count";
/// `DIFF` — "diff"
pub const DIFF: &str = "diff";
/// `MOVE` — "move"
pub const MOVE: &str = "move";
/// `DUP` — "dup"
pub const DUP: &str = "dup";
/// `SWAP` — "swap"
pub const SWAP: &str = "swap";
/// `APPEND` — "append"
pub const APPEND: &str = "append";
/// `REVERSE` — "reverse"
pub const REVERSE: &str = "reverse";
/// `EXPIRE` — "expire"
pub const EXPIRE: &str = "expire";
/// `HEAD` — "head"
pub const HEAD: &str = "head";
/// `TAIL` — "tail"
pub const TAIL: &str = "tail";
/// `WC` — "wc"
pub const WC: &str = "wc";
/// `SIZE` — "size"
pub const SIZE: &str = "size";
/// `SORT` — "sort"
pub const SORT: &str = "sort";
/// `REPLACE` — "replace"
pub const REPLACE: &str = "replace";
/// `PATH` — "path"
pub const PATH: &str = "path";
/// `ERR_INVALID_NAME` — "Invalid or duplicate name"
pub const ERR_INVALID_NAME: &str = "Invalid or duplicate name";
/// `ERR_NAME_NUL` — "Name must not contain null bytes"
pub const ERR_NAME_NUL: &str = "Name must not contain null bytes";
/// `MASTER_FIELD_DELIM` — '\0'
pub const MASTER_FIELD_DELIM: char = '\0';
/// `MASTER_RECORD_DELIM` — "\0\0"
pub const MASTER_RECORD_DELIM: &str = "\0\0";
/// `ERR_EDITOR` — "Could not open editor"
pub const ERR_EDITOR: &str = "Could not open editor";
/// `ERR_MASTER_WRITE` — "Could not write master record"
pub const ERR_MASTER_WRITE: &str = "Could not write master record";
/// `ERR_MASTER_LOCK` — "Could not acquire lock on master record"
pub const ERR_MASTER_LOCK: &str = "Could not acquire lock on master record";
/// `LEASE` — "lease"
pub const LEASE: &str = "lease";
/// `ACK` — "ack"
pub const ACK: &str = "ack";
/// `NACK` — "nack"
pub const NACK: &str = "nack";
/// `LEASE_TTL` — "lease_ttl"
pub const LEASE_TTL: &str = "lease_ttl";
/// `INFLIGHT_RECORD_FILENAME` — "temprs-inflight"
pub const INFLIGHT_RECORD_FILENAME: &str = "temprs-inflight";
/// `DEFAULT_LEASE_TTL_SECS` — 300 (5 minutes)
pub const DEFAULT_LEASE_TTL_SECS: u64 = 300;
/// `ERR_NO_LEASE` — "No such lease token"
pub const ERR_NO_LEASE: &str = "No such lease token";
/// `ERR_EMPTY_STACK` — "Stack is empty"
pub const ERR_EMPTY_STACK: &str = "Stack is empty";
/// `ERR_INFLIGHT_WRITE` — "Could not write inflight record"
pub const ERR_INFLIGHT_WRITE: &str = "Could not write inflight record";

#[cfg(test)]
mod tests {
    use super::*;

    // ── directory and file constants ─────────────────────

    #[test]
    fn temp_dir_is_temprs() {
        assert_eq!(TEMP_DIR, "temprs");
    }

    #[test]
    fn master_record_filename_value() {
        assert_eq!(MASTER_RECORD_FILENAME, "temprs-stack");
    }

    #[test]
    fn tempfile_prefix_value() {
        assert_eq!(TEMPFILE_PREFIX, "tempfile");
    }

    #[test]
    fn name_is_temprs() {
        assert_eq!(NAME, "temprs");
    }

    #[test]
    fn hr_char_is_single_dash() {
        assert_eq!(HR_CHAR, "-");
        assert_eq!(HR_CHAR.len(), 1);
    }

    // ── error message constants are non-empty ────────────

    #[test]
    fn err_no_file_not_empty() {
        assert!(!ERR_NO_FILE.is_empty());
    }

    #[test]
    fn err_parse_not_empty() {
        assert!(!ERR_PARSE.is_empty());
    }

    #[test]
    fn err_invalid_rm_not_empty() {
        assert!(!ERR_INVALID_RM.is_empty());
    }

    #[test]
    fn err_invalid_idx_not_empty() {
        assert!(!ERR_INVALID_IDX.is_empty());
    }

    #[test]
    fn err_invalid_file_not_empty() {
        assert!(!ERR_INVALID_FILE.is_empty());
    }

    #[test]
    fn err_logger_not_empty() {
        assert!(!ERR_LOGGER.is_empty());
    }

    #[test]
    fn err_file_read_not_empty() {
        assert!(!ERR_FILE_READ.is_empty());
    }

    #[test]
    fn err_clock_not_empty() {
        assert!(!ERR_CLOCK.is_empty());
    }

    #[test]
    fn err_master_write_not_empty() {
        assert!(!ERR_MASTER_WRITE.is_empty());
    }

    #[test]
    fn err_master_lock_not_empty() {
        assert!(!ERR_MASTER_LOCK.is_empty());
    }

    #[test]
    fn err_name_nul_not_empty() {
        assert!(!ERR_NAME_NUL.is_empty());
    }

    // ── master file delimiter constants ──────────────────

    #[test]
    fn master_field_delim_is_null() {
        assert_eq!(MASTER_FIELD_DELIM, '\0');
    }

    #[test]
    fn master_record_delim_is_double_null() {
        assert_eq!(MASTER_RECORD_DELIM, "\0\0");
        assert_eq!(MASTER_RECORD_DELIM.len(), 2);
    }

    // ── CLI flag name constants ──────────────────────────

    #[test]
    fn flag_constants_are_unique() {
        let flags = vec![
            DIRECTORY,
            INPUT,
            OUTPUT,
            ADD,
            REMOVE,
            POP,
            UNSHIFT,
            ARGFILE,
            MASTER,
            VERBOSE,
            LIST_FILES,
            LIST_FILES_NUMBERED,
            LIST_CONTENTS,
            LIST_CONTENTS_NUMBERED,
            CLEAR,
            SHIFT,
            SILENT,
            EDIT,
            TAG,
            RENAME,
            INFO,
            GREP,
            CAT,
            COUNT,
            DIFF,
            MOVE,
            DUP,
            SWAP,
            APPEND,
            REVERSE,
            EXPIRE,
            HEAD,
            TAIL,
            WC,
            SIZE,
            SORT,
            REPLACE,
            PATH,
            LEASE,
            ACK,
            NACK,
            LEASE_TTL,
        ];
        let mut sorted = flags.clone();
        sorted.sort();
        sorted.dedup();
        assert_eq!(flags.len(), sorted.len(), "flag constants must be unique");
    }

    #[test]
    fn flag_constants_are_lowercase() {
        let flags = vec![
            DIRECTORY,
            INPUT,
            OUTPUT,
            ADD,
            REMOVE,
            POP,
            UNSHIFT,
            ARGFILE,
            MASTER,
            VERBOSE,
            LIST_FILES,
            LIST_FILES_NUMBERED,
            LIST_CONTENTS,
            LIST_CONTENTS_NUMBERED,
            CLEAR,
            SHIFT,
            SILENT,
            EDIT,
            TAG,
            RENAME,
            INFO,
            GREP,
            CAT,
            COUNT,
            DIFF,
            MOVE,
            DUP,
            SWAP,
            APPEND,
            REVERSE,
            EXPIRE,
            HEAD,
            TAIL,
            WC,
            SIZE,
            SORT,
            REPLACE,
            PATH,
            LEASE,
            ACK,
            NACK,
            LEASE_TTL,
        ];
        for f in flags {
            assert_eq!(f, f.to_lowercase(), "flag '{}' should be lowercase", f);
        }
    }

    #[test]
    fn log_level_is_info() {
        assert_eq!(TEMP_LOG_LEVEL, Level::Info);
    }

    // ── error messages contain descriptive text ──────────

    #[test]
    fn err_invalid_idx_mentions_index() {
        assert!(ERR_INVALID_IDX.to_lowercase().contains("index"));
    }

    #[test]
    fn err_invalid_file_mentions_file() {
        assert!(ERR_INVALID_FILE.to_lowercase().contains("file"));
    }

    #[test]
    fn err_no_file_mentions_file() {
        assert!(ERR_NO_FILE.to_lowercase().contains("file"));
    }

    #[test]
    fn err_invalid_rm_mentions_remove() {
        assert!(ERR_INVALID_RM.to_lowercase().contains("remove"));
    }

    #[test]
    fn err_file_read_mentions_read() {
        assert!(ERR_FILE_READ.to_lowercase().contains("read"));
    }

    // ── error messages mention expected keywords ────────

    #[test]
    fn err_parse_mentions_parse() {
        assert!(ERR_PARSE.to_lowercase().contains("parse"));
    }

    #[test]
    fn err_logger_mentions_logger() {
        assert!(ERR_LOGGER.to_lowercase().contains("logger"));
    }

    #[test]
    fn err_clock_mentions_clock() {
        assert!(ERR_CLOCK.to_lowercase().contains("clock"));
    }

    // ── constant value exact assertions ─────────────────

    #[test]
    fn err_no_file_exact() {
        assert_eq!(ERR_NO_FILE, "No such file");
    }

    #[test]
    fn err_parse_exact() {
        assert_eq!(ERR_PARSE, "Could not parse line");
    }

    #[test]
    fn err_invalid_rm_exact() {
        assert_eq!(ERR_INVALID_RM, "Could not remove specified temporary file");
    }

    #[test]
    fn err_invalid_idx_exact() {
        assert_eq!(ERR_INVALID_IDX, "Invalid specified index");
    }

    #[test]
    fn err_invalid_file_exact() {
        assert_eq!(ERR_INVALID_FILE, "Invalid specified file argument");
    }

    #[test]
    fn err_logger_exact() {
        assert_eq!(ERR_LOGGER, "Could not create logger");
    }

    #[test]
    fn err_file_read_exact() {
        assert_eq!(ERR_FILE_READ, "Could not read file");
    }

    #[test]
    fn err_clock_exact() {
        assert_eq!(ERR_CLOCK, "Could not read clock");
    }

    // ── CLI flag names match expected values ────────────

    #[test]
    fn directory_flag_value() {
        assert_eq!(DIRECTORY, "directory");
    }

    #[test]
    fn input_flag_value() {
        assert_eq!(INPUT, "input");
    }

    #[test]
    fn output_flag_value() {
        assert_eq!(OUTPUT, "output");
    }

    #[test]
    fn add_flag_value() {
        assert_eq!(ADD, "add");
    }

    #[test]
    fn remove_flag_value() {
        assert_eq!(REMOVE, "remove");
    }

    #[test]
    fn pop_flag_value() {
        assert_eq!(POP, "pop");
    }

    #[test]
    fn unshift_flag_value() {
        assert_eq!(UNSHIFT, "unshift");
    }

    #[test]
    fn argfile_flag_value() {
        assert_eq!(ARGFILE, "argfile");
    }

    #[test]
    fn master_flag_value() {
        assert_eq!(MASTER, "master");
    }

    #[test]
    fn verbose_flag_value() {
        assert_eq!(VERBOSE, "verbose");
    }

    #[test]
    fn list_files_flag_value() {
        assert_eq!(LIST_FILES, "list_files");
    }

    #[test]
    fn list_files_numbered_flag_value() {
        assert_eq!(LIST_FILES_NUMBERED, "list_files_numbered");
    }

    #[test]
    fn list_contents_flag_value() {
        assert_eq!(LIST_CONTENTS, "list_contents");
    }

    #[test]
    fn list_contents_numbered_flag_value() {
        assert_eq!(LIST_CONTENTS_NUMBERED, "list_contents_numbered");
    }

    #[test]
    fn clear_flag_value() {
        assert_eq!(CLEAR, "clear");
    }

    #[test]
    fn shift_flag_value() {
        assert_eq!(SHIFT, "shift");
    }

    #[test]
    fn silent_flag_value() {
        assert_eq!(SILENT, "silent");
    }

    // ── constants are non-empty strings ─────────────────

    #[test]
    fn temp_dir_not_empty() {
        assert!(!TEMP_DIR.is_empty());
    }

    #[test]
    fn master_record_filename_not_empty() {
        assert!(!MASTER_RECORD_FILENAME.is_empty());
    }

    #[test]
    fn tempfile_prefix_not_empty() {
        assert!(!TEMPFILE_PREFIX.is_empty());
    }

    #[test]
    fn name_not_empty() {
        assert!(!NAME.is_empty());
    }

    #[test]
    fn hr_char_not_empty() {
        assert!(!HR_CHAR.is_empty());
    }

    // ── flag constants do not contain spaces ────────────

    #[test]
    fn flag_constants_no_spaces() {
        let flags = vec![
            DIRECTORY,
            INPUT,
            OUTPUT,
            ADD,
            REMOVE,
            POP,
            UNSHIFT,
            ARGFILE,
            MASTER,
            VERBOSE,
            LIST_FILES,
            LIST_FILES_NUMBERED,
            LIST_CONTENTS,
            LIST_CONTENTS_NUMBERED,
            CLEAR,
            SHIFT,
            SILENT,
            EDIT,
            TAG,
            RENAME,
            INFO,
            GREP,
            CAT,
            COUNT,
            DIFF,
            MOVE,
            DUP,
            SWAP,
            APPEND,
            REVERSE,
            EXPIRE,
            HEAD,
            TAIL,
            WC,
            SIZE,
            SORT,
            REPLACE,
            PATH,
            LEASE,
            ACK,
            NACK,
            LEASE_TTL,
        ];
        for f in flags {
            assert!(!f.contains(' '), "flag '{}' should not contain spaces", f);
        }
    }

    // ── flag constants are non-empty ────────────────────

    #[test]
    fn flag_constants_not_empty() {
        let flags = vec![
            DIRECTORY,
            INPUT,
            OUTPUT,
            ADD,
            REMOVE,
            POP,
            UNSHIFT,
            ARGFILE,
            MASTER,
            VERBOSE,
            LIST_FILES,
            LIST_FILES_NUMBERED,
            LIST_CONTENTS,
            LIST_CONTENTS_NUMBERED,
            CLEAR,
            SHIFT,
            SILENT,
            EDIT,
            TAG,
            RENAME,
            INFO,
            GREP,
            CAT,
            COUNT,
            DIFF,
            MOVE,
            DUP,
            SWAP,
            APPEND,
            REVERSE,
            EXPIRE,
            HEAD,
            TAIL,
            WC,
            SIZE,
            SORT,
            REPLACE,
            PATH,
            LEASE,
            ACK,
            NACK,
            LEASE_TTL,
        ];
        for f in flags {
            assert!(!f.is_empty(), "flag constant should not be empty");
        }
    }

    // ── flag constants are ASCII ────────────────────────

    #[test]
    fn flag_constants_are_ascii() {
        let flags = vec![
            DIRECTORY,
            INPUT,
            OUTPUT,
            ADD,
            REMOVE,
            POP,
            UNSHIFT,
            ARGFILE,
            MASTER,
            VERBOSE,
            LIST_FILES,
            LIST_FILES_NUMBERED,
            LIST_CONTENTS,
            LIST_CONTENTS_NUMBERED,
            CLEAR,
            SHIFT,
            SILENT,
            EDIT,
            TAG,
            RENAME,
            INFO,
            GREP,
            CAT,
            COUNT,
            DIFF,
            MOVE,
            DUP,
            SWAP,
            APPEND,
            REVERSE,
            EXPIRE,
            HEAD,
            TAIL,
            WC,
            SIZE,
            SORT,
            REPLACE,
            PATH,
            LEASE,
            ACK,
            NACK,
            LEASE_TTL,
        ];
        for f in flags {
            assert!(f.is_ascii(), "flag '{}' should be ASCII", f);
        }
    }

    // ── flag count ──────────────────────────────────────

    #[test]
    fn total_flag_count_is_17() {
        let flags = vec![
            DIRECTORY,
            INPUT,
            OUTPUT,
            ADD,
            REMOVE,
            POP,
            UNSHIFT,
            ARGFILE,
            MASTER,
            VERBOSE,
            LIST_FILES,
            LIST_FILES_NUMBERED,
            LIST_CONTENTS,
            LIST_CONTENTS_NUMBERED,
            CLEAR,
            SHIFT,
            SILENT,
            EDIT,
            TAG,
            RENAME,
            INFO,
            GREP,
            CAT,
            COUNT,
            DIFF,
            MOVE,
            DUP,
            SWAP,
            APPEND,
            REVERSE,
            EXPIRE,
            HEAD,
            TAIL,
            WC,
            SIZE,
            SORT,
            REPLACE,
            PATH,
            LEASE,
            ACK,
            NACK,
            LEASE_TTL,
        ];
        assert_eq!(flags.len(), 42);
    }

    // ── naming conventions ──────────────────────────────

    #[test]
    fn master_record_filename_contains_stack() {
        assert!(MASTER_RECORD_FILENAME.contains("stack"));
    }

    #[test]
    fn master_record_filename_starts_with_temprs() {
        assert!(MASTER_RECORD_FILENAME.starts_with("temprs"));
    }

    #[test]
    fn tempfile_prefix_starts_with_temp() {
        assert!(TEMPFILE_PREFIX.starts_with("temp"));
    }

    #[test]
    fn temp_dir_equals_name() {
        assert_eq!(TEMP_DIR, NAME);
    }

    // ── error messages are capitalized ──────────────────

    #[test]
    fn error_messages_start_with_uppercase() {
        let errors = vec![
            ERR_NO_FILE,
            ERR_PARSE,
            ERR_INVALID_RM,
            ERR_INVALID_IDX,
            ERR_INVALID_FILE,
            ERR_LOGGER,
            ERR_FILE_READ,
            ERR_CLOCK,
        ];
        for e in errors {
            let first = e.chars().next().unwrap();
            assert!(
                first.is_uppercase(),
                "error '{}' should start with uppercase",
                e
            );
        }
    }

    // ── error messages don't end with punctuation ───────

    #[test]
    fn error_messages_no_trailing_period() {
        let errors = vec![
            ERR_NO_FILE,
            ERR_PARSE,
            ERR_INVALID_RM,
            ERR_INVALID_IDX,
            ERR_INVALID_FILE,
            ERR_LOGGER,
            ERR_FILE_READ,
            ERR_CLOCK,
        ];
        for e in errors {
            assert!(
                !e.ends_with('.'),
                "error '{}' should not end with period",
                e
            );
        }
    }

    // ── log level variants ──────────────────────────────

    #[test]
    fn log_level_is_not_debug() {
        assert_ne!(TEMP_LOG_LEVEL, Level::Debug);
    }

    #[test]
    fn log_level_is_not_error() {
        assert_ne!(TEMP_LOG_LEVEL, Level::Error);
    }

    #[test]
    fn log_level_is_not_warn() {
        assert_ne!(TEMP_LOG_LEVEL, Level::Warn);
    }

    #[test]
    fn log_level_is_not_trace() {
        assert_ne!(TEMP_LOG_LEVEL, Level::Trace);
    }

    // ── additional error strings ─────────────────────────

    #[test]
    fn err_invalid_name_not_empty() {
        assert!(!ERR_INVALID_NAME.is_empty());
    }

    #[test]
    fn err_editor_not_empty() {
        assert!(!ERR_EDITOR.is_empty());
    }

    #[test]
    fn err_invalid_name_mentions_name() {
        assert!(ERR_INVALID_NAME.to_lowercase().contains("name"));
    }

    #[test]
    fn err_name_nul_mentions_null() {
        assert!(ERR_NAME_NUL.to_lowercase().contains("null"));
    }

    #[test]
    fn err_master_write_mentions_write_or_record() {
        let l = ERR_MASTER_WRITE.to_lowercase();
        assert!(l.contains("write") || l.contains("record"));
    }

    #[test]
    fn err_master_lock_mentions_lock() {
        assert!(ERR_MASTER_LOCK.to_lowercase().contains("lock"));
    }

    #[test]
    fn err_editor_mentions_editor() {
        assert!(ERR_EDITOR.to_lowercase().contains("editor"));
    }

    #[test]
    fn extended_errors_start_uppercase() {
        for e in [
            ERR_INVALID_NAME,
            ERR_NAME_NUL,
            ERR_MASTER_WRITE,
            ERR_MASTER_LOCK,
            ERR_EDITOR,
        ] {
            assert!(e.chars().next().unwrap().is_uppercase(), "{}", e);
        }
    }

    #[test]
    fn extended_errors_no_trailing_period() {
        for e in [
            ERR_INVALID_NAME,
            ERR_NAME_NUL,
            ERR_MASTER_WRITE,
            ERR_MASTER_LOCK,
            ERR_EDITOR,
        ] {
            assert!(!e.ends_with('.'), "{}", e);
        }
    }

    #[test]
    fn hr_char_is_ascii_dash() {
        assert_eq!(HR_CHAR.as_bytes(), b"-");
    }

    #[test]
    fn name_constant_is_valid_dir_name() {
        assert!(!NAME.contains('/'));
        assert!(!NAME.contains('\\'));
    }

    #[test]
    fn tempfile_prefix_has_no_whitespace() {
        assert!(!TEMPFILE_PREFIX.chars().any(|c| c.is_whitespace()));
    }

    #[test]
    fn master_record_filename_ends_with_stack_literal() {
        assert!(MASTER_RECORD_FILENAME.ends_with("stack"));
    }

    #[test]
    fn all_error_constants_distinct() {
        let errs = [
            ERR_NO_FILE,
            ERR_PARSE,
            ERR_INVALID_RM,
            ERR_INVALID_IDX,
            ERR_INVALID_FILE,
            ERR_LOGGER,
            ERR_FILE_READ,
            ERR_CLOCK,
            ERR_INVALID_NAME,
            ERR_NAME_NUL,
            ERR_MASTER_WRITE,
            ERR_MASTER_LOCK,
            ERR_EDITOR,
        ];
        let mut v = errs.to_vec();
        v.sort();
        v.dedup();
        assert_eq!(v.len(), errs.len());
    }
}
