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
    fn err_file_read_mentions_read() {
        assert!(ERR_FILE_READ.to_lowercase().contains("read"));
    }
}
