use clap::{crate_authors, crate_version, crate_description, App, AppSettings, Arg};

use crate::util::consts::*;

const CYBERPUNK_TEMPLATE: &str = "
{before-help}
{about}

\x1b[33m  USAGE:\x1b[0m {usage}

\x1b[36m  ── DATA I/O ───────────────────────────────────────────\x1b[0m
{unified}
\x1b[36m  ── POSITIONAL ─────────────────────────────────────────\x1b[0m
{positionals}
{after-help}";

const BANNER: &str = concat!(
 "\x1b[36m ████████╗███████╗███╗   ███╗██████╗ ██████╗ ███████╗\x1b[0m\n",
 "\x1b[36m ╚══██╔══╝██╔════╝████╗ ████║██╔══██╗██╔══██╗██╔════╝\x1b[0m\n",
 "\x1b[35m    ██║   █████╗  ██╔████╔██║██████╔╝██████╔╝███████╗\x1b[0m\n",
 "\x1b[35m    ██║   ██╔══╝  ██║╚██╔╝██║██╔═══╝ ██╔══██╗╚════██║\x1b[0m\n",
 "\x1b[31m    ██║   ███████╗██║ ╚═╝ ██║██║     ██║  ██║███████║\x1b[0m\n",
 "\x1b[31m    ╚═╝   ╚══════╝╚═╝     ╚═╝╚═╝     ╚═╝  ╚═╝╚══════╝\x1b[0m\n",
 "\x1b[36m ┌──────────────────────────────────────────────────────┐\x1b[0m\n",
 "\x1b[36m │ STATUS: ONLINE  // SIGNAL: ████████░░ // v", env!("CARGO_PKG_VERSION"), "\x1b[36m   │\x1b[0m\n",
 "\x1b[36m └──────────────────────────────────────────────────────┘\x1b[0m\n",
 "\x1b[35m  >> TEMPORARY FILE STACK MANAGER // FULL SPECTRUM <<\x1b[0m");

const AFTER: &str = concat!(
"\x1b[36m  ── SYSTEM ─────────────────────────────────────────\x1b[0m\n",
"\x1b[35m  v", env!("CARGO_PKG_VERSION"), " \x1b[0m// \x1b[33m(c) Jacob Menke and contributors\x1b[0m\n",
"\x1b[35m  The stack is vast and infinite.\x1b[0m\n",
"\x1b[33m  >>> JACK IN. PUSH YOUR DATA. OWN YOUR TEMP FILES. <<<\x1b[0m\n",
"\x1b[36m ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░\x1b[0m");

pub fn parse_opts() -> App<'static, 'static> {
    App::new(NAME)
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(AppSettings::ColoredHelp)
        .template(CYBERPUNK_TEMPLATE)
        .before_help(BANNER)
        .after_help(AFTER)
        .arg(Arg::with_name(INPUT)
            .short("i")
            .long("input")
            .value_name("INDEX")
            .help("\x1b[32m//\x1b[0m Write into tempfile at INDEX")
            .takes_value(true))
        .arg(Arg::with_name(OUTPUT)
            .short("o")
            .long("output")
            .value_name("INDEX")
            .allow_hyphen_values(true)
            .help("\x1b[32m//\x1b[0m Read from tempfile at INDEX")
            .takes_value(true))
        .arg(Arg::with_name(ADD)
            .short("a")
            .long("add")
            .value_name("INDEX")
            .allow_hyphen_values(true)
            .help("\x1b[32m//\x1b[0m Insert tempfile at INDEX")
            .takes_value(true))
        .arg(Arg::with_name(REMOVE)
            .short("r")
            .long("remove")
            .value_name("INDEX")
            .allow_hyphen_values(true)
            .help("\x1b[32m//\x1b[0m Remove tempfile at INDEX")
            .takes_value(true))
        .arg(Arg::with_name(POP)
            .short("p")
            .long("pop")
            .help("\x1b[32m//\x1b[0m Pop from top of stack"))
        .arg(Arg::with_name(UNSHIFT)
            .short("u")
            .long("unshift")
            .help("\x1b[32m//\x1b[0m Push to bottom of stack"))
        .arg(Arg::with_name(SHIFT)
            .short("s")
            .long("shift")
            .help("\x1b[32m//\x1b[0m Shift from bottom of stack"))
        .arg(Arg::with_name(ARGFILE)
            .help("\x1b[32m//\x1b[0m Read input FILE into new tempfile. Stdin takes priority if present.")
            .required(false)
            .index(1))
        .arg(Arg::with_name(DIRECTORY)
            .short("d")
            .long("dir")
            .help("\x1b[32m//\x1b[0m List temprs directory"))
        .arg(Arg::with_name(MASTER)
            .short("m")
            .long("master")
            .help("\x1b[32m//\x1b[0m List temprs master record file"))
        .arg(Arg::with_name(LIST_FILES)
            .short("l")
            .long("list-files")
            .help("\x1b[32m//\x1b[0m List all tempfiles on the stack"))
        .arg(Arg::with_name(LIST_FILES_NUMBERED)
            .short("n")
            .long("list-files-numbered")
            .help("\x1b[32m//\x1b[0m List all tempfiles numbered on the stack"))
        .arg(Arg::with_name(LIST_CONTENTS)
            .short("L")
            .long("list-contents")
            .help("\x1b[32m//\x1b[0m List all tempfile contents on the stack"))
        .arg(Arg::with_name(LIST_CONTENTS_NUMBERED)
            .short("N")
            .long("list-contents-numbered")
            .help("\x1b[32m//\x1b[0m List all tempfiles numbered with contents"))
        .arg(Arg::with_name(SILENT)
            .short("q")
            .long("quiet")
            .help("\x1b[32m//\x1b[0m Suppress output when creating tempfile"))
        .arg(Arg::with_name(CLEAR)
            .short("c")
            .long("clear")
            .help("\x1b[32m//\x1b[0m Purge all tempfiles from the stack"))
        .arg(Arg::with_name(VERBOSE)
            .short("v")
            .long("verbose")
            .multiple(true)
            .help("\x1b[32m//\x1b[0m Increase verbosity level"))
}
