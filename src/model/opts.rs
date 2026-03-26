use clap::{crate_authors, crate_version, crate_description, App, Arg};

use crate::util::consts::*;

const CYBERPUNK_TEMPLATE: &str = r#"
{before-help}
{about}

  USAGE: {usage}

── DATA I/O ───────────────────────────────────────────
{unified}
── POSITIONAL ─────────────────────────────────────────
{positionals}
{after-help}"#;

const BANNER: &str = concat!(r#"
 ████████╗███████╗███╗   ███╗██████╗ ██████╗ ███████╗
 ╚══██╔══╝██╔════╝████╗ ████║██╔══██╗██╔══██╗██╔════╝
    ██║   █████╗  ██╔████╔██║██████╔╝██████╔╝███████╗
    ██║   ██╔══╝  ██║╚██╔╝██║██╔═══╝ ██╔══██╗╚════██║
    ██║   ███████╗██║ ╚═╝ ██║██║     ██║  ██║███████║
    ╚═╝   ╚══════╝╚═╝     ╚═╝╚═╝     ╚═╝  ╚═╝╚══════╝
 ┌──────────────────────────────────────────────────────┐
 │ STATUS: ONLINE  // SIGNAL: ████████░░ // v"#, env!("CARGO_PKG_VERSION"), r#"   │
 └──────────────────────────────────────────────────────┘
  >> TEMPORARY FILE STACK MANAGER // FULL SPECTRUM <<"#);

const AFTER: &str = r#"
 ─────────────────────────────────────────────────────
  The stack is vast and infinite.
  >>> JACK IN. PUSH YOUR DATA. OWN YOUR TEMP FILES. <<<
 ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░"#;

pub fn parse_opts() -> App<'static, 'static> {
    App::new(NAME)
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .template(CYBERPUNK_TEMPLATE)
        .before_help(BANNER)
        .after_help(AFTER)
        .arg(Arg::with_name(INPUT)
            .short("i")
            .long("input")
            .value_name("INDEX")
            .help("// Write into tempfile at INDEX")
            .takes_value(true))
        .arg(Arg::with_name(OUTPUT)
            .short("o")
            .long("output")
            .value_name("INDEX")
            .allow_hyphen_values(true)
            .help("// Read from tempfile at INDEX")
            .takes_value(true))
        .arg(Arg::with_name(ADD)
            .short("a")
            .long("add")
            .value_name("INDEX")
            .allow_hyphen_values(true)
            .help("// Insert tempfile at INDEX")
            .takes_value(true))
        .arg(Arg::with_name(REMOVE)
            .short("r")
            .long("remove")
            .value_name("INDEX")
            .allow_hyphen_values(true)
            .help("// Remove tempfile at INDEX")
            .takes_value(true))
        .arg(Arg::with_name(POP)
            .short("p")
            .long("pop")
            .help("// Pop from top of stack"))
        .arg(Arg::with_name(UNSHIFT)
            .short("u")
            .long("unshift")
            .help("// Push to bottom of stack"))
        .arg(Arg::with_name(SHIFT)
            .short("s")
            .long("shift")
            .help("// Shift from bottom of stack"))
        .arg(Arg::with_name(ARGFILE)
            .help("// Read input FILE into new tempfile. Stdin takes priority if present.")
            .required(false)
            .index(1))
        .arg(Arg::with_name(DIRECTORY)
            .short("d")
            .long("dir")
            .help("// List temprs directory"))
        .arg(Arg::with_name(MASTER)
            .short("m")
            .long("master")
            .help("// List temprs master record file"))
        .arg(Arg::with_name(LIST_FILES)
            .short("l")
            .long("list-files")
            .help("// List all tempfiles on the stack"))
        .arg(Arg::with_name(LIST_FILES_NUMBERED)
            .short("n")
            .long("list-files-numbered")
            .help("// List all tempfiles numbered on the stack"))
        .arg(Arg::with_name(LIST_CONTENTS)
            .short("L")
            .long("list-contents")
            .help("// List all tempfile contents on the stack"))
        .arg(Arg::with_name(LIST_CONTENTS_NUMBERED)
            .short("N")
            .long("list-contents-numbered")
            .help("// List all tempfiles numbered with contents"))
        .arg(Arg::with_name(SILENT)
            .short("q")
            .long("quiet")
            .help("// Suppress output when creating tempfile"))
        .arg(Arg::with_name(CLEAR)
            .short("c")
            .long("clear")
            .help("// Purge all tempfiles from the stack"))
        .arg(Arg::with_name(VERBOSE)
            .short("v")
            .long("verbose")
            .multiple(true)
            .help("// Increase verbosity level"))
}
