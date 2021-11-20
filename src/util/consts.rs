use log::Level;

pub const VERSION: &'static str = "0.3.0";
pub const TEMP_LOG_LEVEL: Level = Level::Info;
pub const TEMP_DIR: &'static str = "temprs";
pub const FILE_LIST_FILE: &'static str = "temprs-stack";
pub const TEMPFILE_PREFIX: &'static str = "tempfile";
pub const ERR_NO_FILE: &'static str = "no such file";
pub const ERR_PARSE: &'static str = "Could not parse line";
pub const ERR_INVALID_OUTFILE: &'static str = "Could not read from specified temporary file";
pub const ERR_INVALID_INFILE: &'static str = "Could not write to specified temporary file";
pub const ERR_INVALID_RM: &'static str = "Could not remove specified temporary file";
pub const ERR_INVALID_INSERT: &'static str = "Could not insert at specified index";
pub const DESC: &'static str = "A temporary file manager with stack mechanism";
pub const NAME: &'static str = "temprs";
