use clap::{crate_authors, crate_description, crate_version, App, Arg};

use crate::util::consts::*;

pub fn parse_opts() -> App<'static, 'static> {
    App::new(NAME)
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::with_name(INPUT)
            .short("i")
            .long("input")
            .value_name("INDEX")
            .help("Set tempfile INDEX to write into")
            .takes_value(true))
        .arg(Arg::with_name(OUTPUT)
            .short("o")
            .long("output")
            .value_name("INDEX")
            .allow_hyphen_values(true)
            .help("Set tempfile INDEX to read from")
            .takes_value(true))
        .arg(Arg::with_name(ADD)
            .short("a")
            .long("add")
            .value_name("INDEX")
            .allow_hyphen_values(true)
            .help("Insert tempfile at INDEX")
            .takes_value(true))
        .arg(Arg::with_name(REMOVE)
            .short("r")
            .long("remove")
            .value_name("INDEX")
            .allow_hyphen_values(true)
            .help("Set tempfile INDEX to remove")
            .takes_value(true))
        .arg(Arg::with_name(POP)
            .short("p")
            .long("pop")
            .help("Remove from top of stack"))
        .arg(Arg::with_name(UNSHIFT)
            .short("u")
            .long("unshift")
            .help("Add to bottom of stack"))
        .arg(Arg::with_name(SHIFT)
            .short("s")
            .long("shift")
            .help("Remove from bottom of stack"))
        .arg(Arg::with_name(ARGFILE)
            .help("Read input FILE into a new tempfile. If stdin is present the tempfile is created from stdin instead.")
            .required(false)
            .index(1))
        .arg(Arg::with_name(DIRECTORY)
            .short("d")
            .long("dir")
            .help("List temprs directory"))
        .arg(Arg::with_name(MASTER)
            .short("m")
            .long("master")
            .help("List temprs master record file"))
        .arg(Arg::with_name(LIST_FILES)
            .short("l")
            .long("list-files")
            .help("List all tempfiles on the stack to stdout"))
        .arg(Arg::with_name(LIST_FILES_NUMBERED)
            .short("n")
            .long("list-files-numbered")
            .help("List all tempfiles numbered on the stack to stdout"))
        .arg(Arg::with_name(LIST_CONTENTS)
            .short("L")
            .long("list-contents")
            .help("List all tempfiles contents on the stack to stdout"))
        .arg(Arg::with_name(LIST_CONTENTS_NUMBERED)
            .short("N")
            .long("list-contents-numbered")
            .help("List all tempfiles numbered with contents on the stack to stdout"))
        .arg(Arg::with_name(SILENT)
            .short("q")
            .long("quiet")
            .help("No output when creating tempfile"))
        .arg(Arg::with_name(CLEAR)
            .short("c")
            .long("clear")
            .help("Remove all tempfiles"))
        .arg(Arg::with_name(VERBOSE)
            .short("v")
            .long("verbose")
            .multiple(true)
            .help("Set the level of verbosity"))
}
