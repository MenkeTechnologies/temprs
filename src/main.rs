use fs::create_dir;
use io::stdin;
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::{Path, PathBuf};

use atty::Stream;
use log::debug;

use model::app::TempApp;
use util::consts::{FILE_LIST_FILE, TEMP_DIR, TEMP_LOG_LEVEL};
use util::consts::TEMPFILE_PREFIX;
use util::utils::{append_file, paths_from_file};
use util::utils::{file_contents, get_ms};

mod util;
mod model;


fn main() {
    let mut app = TempApp::new();

    app.run();
}
