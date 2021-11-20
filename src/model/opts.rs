use clap::{App, Arg};

use crate::util::consts::{DESC, VERSION};

pub fn parse_opts() -> App<'static, 'static> {
    App::new("Temp")
        .version(VERSION)
        .author("Jacob Menke. <linux.dev25@gmail.com>")
        .about(DESC)
        .arg(Arg::with_name("input")
            .short("i")
            .long("input")
            .value_name("INDEX")
            .help("Sets temp file index to read from")
            .takes_value(true))
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .value_name("INDEX")
            .help("Sets temp file index to write to")
            .takes_value(true))
        .arg(Arg::with_name("remove")
            .short("r")
            .long("remove")
            .value_name("INDEX")
            .help("Sets temp file index to remove")
            .takes_value(true))
        .arg(Arg::with_name("FILE")
            .help("Read input FILE into a new temp file. If stdin is present the tempfile created from stdin instead.")
            .required(false)
            .index(1))
        .arg(Arg::with_name("list_files")
            .short("l")
            .long("list-files")
            .help("list all temp files on the stack to stdout"))
        .arg(Arg::with_name("list_contents")
            .short("L")
            .long("list-contents")
            .help("list all temp files with contents on the stack to stdout"))
        .arg(Arg::with_name("silent")
            .short("s")
            .long("silent")
            .help("no output"))
        .arg(Arg::with_name("clear")
            .short("c")
            .long("clear")
            .help("remove all temp files"))
        .arg(Arg::with_name("v")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity"))
}
