#![allow(dead_code)]
#![allow(unused_must_use)]

use log::Level;

pub const TEMP_LOG_LEVEL: Level = Level::Info;
pub const TEMP_DIR: &'static str = "temprs";
pub const MASTER_RECORD_FILENAME: &'static str = "temprs-stack";
pub const TEMPFILE_PREFIX: &'static str = "tempfile";
pub const ERR_NO_FILE: &'static str = "No such file";
pub const ERR_PARSE: &'static str = "Could not parse line";
pub const ERR_INVALID_OUTFILE: &'static str = "Could not read from specified temporary file";
pub const ERR_INVALID_INFILE: &'static str = "Could not write to specified temporary file";
pub const ERR_INVALID_RM: &'static str = "Could not remove specified temporary file";
pub const ERR_INVALID_INSERT: &'static str = "Could not insert at specified index";
pub const ERR_INVALID_IDX: &'static str = "Invalid specified index";
pub const ERR_INVALID_FILE: &'static str = "Invalid specified file argument";
pub const ERR_LOGGER: &'static str = "Could not create logger";
pub const ERR_FILE_READ: &'static str = "Could not read file";
pub const ERR_CLOCK: &'static str = "Could not read clock";
pub const NAME: &'static str = "temprs";
pub const HR_CHAR: &'static str = "-";
pub const DIRECTORY: &'static str = "directory";
pub const INPUT: &'static str = "input";
pub const OUTPUT: &'static str = "output";
pub const ADD: &'static str = "add";
pub const REMOVE: &'static str = "remove";
pub const POP: &'static str = "pop";
pub const UNSHIFT: &'static str = "unshift";
pub const ARGFILE: &'static str = "argfile";
pub const MASTER: &'static str = "master";
pub const VERBOSE: &'static str = "verbose";
pub const LIST_FILES: &'static str = "list_files";
pub const LIST_FILES_NUMBERED: &'static str = "list_files_numbered";
pub const LIST_CONTENTS: &'static str = "list_contents";
pub const LIST_CONTENTS_NUMBERED: &'static str = "list_contents_numbered";
pub const CLEAR: &'static str = "clear";
pub const SHIFT: &'static str = "shift";
pub const SILENT: &'static str = "silent";

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
    fn err_invalid_outfile_not_empty() {
        assert!(!ERR_INVALID_OUTFILE.is_empty());
    }

    #[test]
    fn err_invalid_infile_not_empty() {
        assert!(!ERR_INVALID_INFILE.is_empty());
    }

    #[test]
    fn err_invalid_rm_not_empty() {
        assert!(!ERR_INVALID_RM.is_empty());
    }

    #[test]
    fn err_invalid_insert_not_empty() {
        assert!(!ERR_INVALID_INSERT.is_empty());
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
            LIST_CONTENTS, LIST_CONTENTS_NUMBERED, CLEAR, SHIFT, SILENT,
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
            LIST_CONTENTS, LIST_CONTENTS_NUMBERED, CLEAR, SHIFT, SILENT,
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
    fn err_invalid_insert_mentions_insert() {
        assert!(ERR_INVALID_INSERT.to_lowercase().contains("insert"));
    }

    #[test]
    fn err_file_read_mentions_read() {
        assert!(ERR_FILE_READ.to_lowercase().contains("read"));
    }
}
