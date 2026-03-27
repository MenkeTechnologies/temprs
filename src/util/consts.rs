use log::Level;

pub const TEMP_LOG_LEVEL: Level = Level::Info;
pub const TEMP_DIR: &str = "temprs";
pub const MASTER_RECORD_FILENAME: &str = "temprs-stack";
pub const TEMPFILE_PREFIX: &str = "tempfile";
pub const ERR_NO_FILE: &str = "No such file";
pub const ERR_PARSE: &str = "Could not parse line";
pub const ERR_INVALID_RM: &str = "Could not remove specified temporary file";
pub const ERR_INVALID_IDX: &str = "Invalid specified index";
pub const ERR_INVALID_FILE: &str = "Invalid specified file argument";
pub const ERR_LOGGER: &str = "Could not create logger";
pub const ERR_FILE_READ: &str = "Could not read file";
pub const ERR_CLOCK: &str = "Could not read clock";
pub const NAME: &str = "temprs";
pub const HR_CHAR: &str = "-";
pub const DIRECTORY: &str = "directory";
pub const INPUT: &str = "input";
pub const OUTPUT: &str = "output";
pub const ADD: &str = "add";
pub const REMOVE: &str = "remove";
pub const POP: &str = "pop";
pub const UNSHIFT: &str = "unshift";
pub const ARGFILE: &str = "argfile";
pub const MASTER: &str = "master";
pub const VERBOSE: &str = "verbose";
pub const LIST_FILES: &str = "list_files";
pub const LIST_FILES_NUMBERED: &str = "list_files_numbered";
pub const LIST_CONTENTS: &str = "list_contents";
pub const LIST_CONTENTS_NUMBERED: &str = "list_contents_numbered";
pub const CLEAR: &str = "clear";
pub const SHIFT: &str = "shift";
pub const SILENT: &str = "silent";
pub const EDIT: &str = "edit";
pub const TAG: &str = "name";
pub const RENAME: &str = "rename";
pub const INFO: &str = "info";
pub const GREP: &str = "grep";
pub const CAT: &str = "cat";
pub const ERR_INVALID_NAME: &str = "Invalid or duplicate name";
pub const ERR_EDITOR: &str = "Could not open editor";

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

    // ── CLI flag name constants ──────────────────────────

    #[test]
    fn flag_constants_are_unique() {
        let flags = vec![
            DIRECTORY, INPUT, OUTPUT, ADD, REMOVE, POP, UNSHIFT,
            ARGFILE, MASTER, VERBOSE, LIST_FILES, LIST_FILES_NUMBERED,
            LIST_CONTENTS, LIST_CONTENTS_NUMBERED, CLEAR, SHIFT, SILENT, EDIT, TAG, RENAME, INFO, GREP, CAT,
        ];
        let mut sorted = flags.clone();
        sorted.sort();
        sorted.dedup();
        assert_eq!(flags.len(), sorted.len(), "flag constants must be unique");
    }

    #[test]
    fn flag_constants_are_lowercase() {
        let flags = vec![
            DIRECTORY, INPUT, OUTPUT, ADD, REMOVE, POP, UNSHIFT,
            ARGFILE, MASTER, VERBOSE, LIST_FILES, LIST_FILES_NUMBERED,
            LIST_CONTENTS, LIST_CONTENTS_NUMBERED, CLEAR, SHIFT, SILENT, EDIT, TAG, RENAME, INFO, GREP, CAT,
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
            DIRECTORY, INPUT, OUTPUT, ADD, REMOVE, POP, UNSHIFT,
            ARGFILE, MASTER, VERBOSE, LIST_FILES, LIST_FILES_NUMBERED,
            LIST_CONTENTS, LIST_CONTENTS_NUMBERED, CLEAR, SHIFT, SILENT, EDIT, TAG, RENAME, INFO, GREP, CAT,
        ];
        for f in flags {
            assert!(!f.contains(' '), "flag '{}' should not contain spaces", f);
        }
    }

    // ── flag constants are non-empty ────────────────────

    #[test]
    fn flag_constants_not_empty() {
        let flags = vec![
            DIRECTORY, INPUT, OUTPUT, ADD, REMOVE, POP, UNSHIFT,
            ARGFILE, MASTER, VERBOSE, LIST_FILES, LIST_FILES_NUMBERED,
            LIST_CONTENTS, LIST_CONTENTS_NUMBERED, CLEAR, SHIFT, SILENT, EDIT, TAG, RENAME, INFO, GREP, CAT,
        ];
        for f in flags {
            assert!(!f.is_empty(), "flag constant should not be empty");
        }
    }

    // ── flag constants are ASCII ────────────────────────

    #[test]
    fn flag_constants_are_ascii() {
        let flags = vec![
            DIRECTORY, INPUT, OUTPUT, ADD, REMOVE, POP, UNSHIFT,
            ARGFILE, MASTER, VERBOSE, LIST_FILES, LIST_FILES_NUMBERED,
            LIST_CONTENTS, LIST_CONTENTS_NUMBERED, CLEAR, SHIFT, SILENT, EDIT, TAG, RENAME, INFO, GREP, CAT,
        ];
        for f in flags {
            assert!(f.is_ascii(), "flag '{}' should be ASCII", f);
        }
    }

    // ── flag count ──────────────────────────────────────

    #[test]
    fn total_flag_count_is_17() {
        let flags = vec![
            DIRECTORY, INPUT, OUTPUT, ADD, REMOVE, POP, UNSHIFT,
            ARGFILE, MASTER, VERBOSE, LIST_FILES, LIST_FILES_NUMBERED,
            LIST_CONTENTS, LIST_CONTENTS_NUMBERED, CLEAR, SHIFT, SILENT, EDIT, TAG, RENAME, INFO, GREP, CAT,
        ];
        assert_eq!(flags.len(), 23);
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
            ERR_NO_FILE, ERR_PARSE, ERR_INVALID_RM, ERR_INVALID_IDX,
            ERR_INVALID_FILE, ERR_LOGGER, ERR_FILE_READ, ERR_CLOCK,
        ];
        for e in errors {
            let first = e.chars().next().unwrap();
            assert!(first.is_uppercase(), "error '{}' should start with uppercase", e);
        }
    }

    // ── error messages don't end with punctuation ───────

    #[test]
    fn error_messages_no_trailing_period() {
        let errors = vec![
            ERR_NO_FILE, ERR_PARSE, ERR_INVALID_RM, ERR_INVALID_IDX,
            ERR_INVALID_FILE, ERR_LOGGER, ERR_FILE_READ, ERR_CLOCK,
        ];
        for e in errors {
            assert!(!e.ends_with('.'), "error '{}' should not end with period", e);
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
}
