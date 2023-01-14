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
pub const LIST_FILES: &'static str = "list_files";
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
pub const LIST_CONTENTS: &'static str = "list_contents";
pub const CLEAR: &'static str = "clear";
pub const SHIFT: &'static str = "shift";
pub const SILENT: &'static str = "silent";
