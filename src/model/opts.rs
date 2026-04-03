use clap::{Arg, ArgAction, Command};

use crate::util::consts::*;

const CYBERPUNK_TEMPLATE: &str = "
{before-help}
{about}

\x1b[33m  USAGE:\x1b[0m {usage}

\x1b[36m  ── DATA I/O ───────────────────────────────────────────\x1b[0m
{options}
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
    "\x1b[36m │ STATUS: ONLINE  // SIGNAL: ████████░░ // v",
    env!("CARGO_PKG_VERSION"),
    "\x1b[36m   │\x1b[0m\n",
    "\x1b[36m └──────────────────────────────────────────────────────┘\x1b[0m\n",
    "\x1b[35m  >> TEMPORARY FILE STACK MANAGER // FULL SPECTRUM <<\x1b[0m"
);

const AFTER: &str = concat!(
    "\x1b[36m  ── SYSTEM ─────────────────────────────────────────\x1b[0m\n",
    "\x1b[35m  v",
    env!("CARGO_PKG_VERSION"),
    " \x1b[0m// \x1b[33m(c) Jacob Menke and contributors\x1b[0m\n",
    "\x1b[35m  The stack is vast and infinite.\x1b[0m\n",
    "\x1b[33m  >>> JACK IN. PUSH YOUR DATA. OWN YOUR TEMP FILES. <<<\x1b[0m\n",
    "\x1b[36m ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░\x1b[0m"
);

pub fn parse_opts() -> Command {
    Command::new(NAME)
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .help_template(CYBERPUNK_TEMPLATE)
        .before_help(BANNER)
        .after_help(AFTER)
        .arg(Arg::new(INPUT)
            .short('i')
            .long("input")
            .value_name("INDEX")
            .help("\x1b[32m//\x1b[0m Write into tempfile at INDEX"))
        .arg(Arg::new(OUTPUT)
            .short('o')
            .long("output")
            .value_name("INDEX")
            .allow_hyphen_values(true)
            .help("\x1b[32m//\x1b[0m Read from tempfile at INDEX"))
        .arg(Arg::new(ADD)
            .short('a')
            .long("add")
            .value_name("INDEX")
            .allow_hyphen_values(true)
            .help("\x1b[32m//\x1b[0m Insert tempfile at INDEX"))
        .arg(Arg::new(REMOVE)
            .short('r')
            .long("remove")
            .value_name("INDEX")
            .allow_hyphen_values(true)
            .help("\x1b[32m//\x1b[0m Remove tempfile at INDEX"))
        .arg(Arg::new(POP)
            .short('p')
            .long("pop")
            .action(ArgAction::SetTrue)
            .help("\x1b[32m//\x1b[0m Pop from top of stack"))
        .arg(Arg::new(UNSHIFT)
            .short('u')
            .long("unshift")
            .action(ArgAction::SetTrue)
            .help("\x1b[32m//\x1b[0m Push to bottom of stack (reads stdin, no stdout)"))
        .arg(Arg::new(SHIFT)
            .short('s')
            .long("shift")
            .action(ArgAction::SetTrue)
            .help("\x1b[32m//\x1b[0m Shift from bottom of stack"))
        .arg(Arg::new(ARGFILE)
            .help("\x1b[32m//\x1b[0m Read input FILE into new tempfile. Stdin takes priority if present.")
            .required(false)
            .index(1))
        .arg(Arg::new(DIRECTORY)
            .short('d')
            .long("dir")
            .action(ArgAction::SetTrue)
            .help("\x1b[32m//\x1b[0m List temprs directory"))
        .arg(Arg::new(MASTER)
            .short('m')
            .long("master")
            .action(ArgAction::SetTrue)
            .help("\x1b[32m//\x1b[0m List temprs master record file"))
        .arg(Arg::new(LIST_FILES)
            .short('l')
            .long("list-files")
            .action(ArgAction::SetTrue)
            .help("\x1b[32m//\x1b[0m List all tempfiles on the stack"))
        .arg(Arg::new(LIST_FILES_NUMBERED)
            .short('n')
            .long("list-files-numbered")
            .action(ArgAction::SetTrue)
            .help("\x1b[32m//\x1b[0m List all tempfiles numbered on the stack"))
        .arg(Arg::new(LIST_CONTENTS)
            .short('L')
            .long("list-contents")
            .action(ArgAction::SetTrue)
            .help("\x1b[32m//\x1b[0m List all tempfile contents on the stack"))
        .arg(Arg::new(LIST_CONTENTS_NUMBERED)
            .short('N')
            .long("list-contents-numbered")
            .action(ArgAction::SetTrue)
            .help("\x1b[32m//\x1b[0m List all tempfiles numbered with contents"))
        .arg(Arg::new(SILENT)
            .short('q')
            .long("quiet")
            .action(ArgAction::SetTrue)
            .help("\x1b[32m//\x1b[0m Suppress output when creating tempfile"))
        .arg(Arg::new(CLEAR)
            .short('c')
            .long("clear")
            .action(ArgAction::SetTrue)
            .help("\x1b[32m//\x1b[0m Purge all tempfiles from the stack"))
        .arg(Arg::new(EDIT)
            .short('e')
            .long("edit")
            .value_name("INDEX")
            .allow_hyphen_values(true)
            .help("\x1b[32m//\x1b[0m Open tempfile at INDEX in $EDITOR"))
        .arg(Arg::new(VERBOSE)
            .short('v')
            .long("verbose")
            .action(ArgAction::Count)
            .help("\x1b[32m//\x1b[0m Increase verbosity level"))
        .arg(Arg::new(TAG)
            .short('w')
            .long("name")
            .value_name("NAME")
            .help("\x1b[32m//\x1b[0m Tag tempfile with NAME for retrieval by alias"))
        .arg(Arg::new(RENAME)
            .short('R')
            .long("rename")
            .value_names(["OLD", "NEW"])
            .num_args(2)
            .help("\x1b[32m//\x1b[0m Rename tag from OLD to NEW"))
        .arg(Arg::new(INFO)
            .short('I')
            .long("info")
            .value_name("INDEX")
            .allow_hyphen_values(true)
            .help("\x1b[32m//\x1b[0m Show metadata for tempfile at INDEX"))
        .arg(Arg::new(GREP)
            .short('g')
            .long("grep")
            .value_name("PATTERN")
            .help("\x1b[32m//\x1b[0m Search tempfile contents for PATTERN"))
        .arg(Arg::new(CAT)
            .short('C')
            .long("cat")
            .value_name("INDEX")
            .num_args(1..)
            .allow_hyphen_values(true)
            .help("\x1b[32m//\x1b[0m Concatenate tempfiles at indices to stdout"))
        .arg(Arg::new(COUNT)
            .short('k')
            .long("count")
            .action(ArgAction::SetTrue)
            .help("\x1b[32m//\x1b[0m Print number of tempfiles on the stack"))
        .arg(Arg::new(DIFF)
            .short('D')
            .long("diff")
            .value_names(["A", "B"])
            .num_args(2)
            .allow_hyphen_values(true)
            .help("\x1b[32m//\x1b[0m Diff two tempfiles by index or name"))
        .arg(Arg::new(MOVE)
            .short('M')
            .long("mv")
            .value_names(["FROM", "TO"])
            .num_args(2)
            .allow_hyphen_values(true)
            .help("\x1b[32m//\x1b[0m Move tempfile from one position to another"))
        .arg(Arg::new(DUP)
            .short('x')
            .long("dup")
            .value_name("INDEX")
            .allow_hyphen_values(true)
            .help("\x1b[32m//\x1b[0m Duplicate tempfile onto top of stack"))
        .arg(Arg::new(SWAP)
            .short('S')
            .long("swap")
            .value_names(["A", "B"])
            .num_args(2)
            .allow_hyphen_values(true)
            .help("\x1b[32m//\x1b[0m Swap two tempfiles by index or name"))
        .arg(Arg::new(APPEND)
            .short('A')
            .long("append")
            .value_name("INDEX")
            .allow_hyphen_values(true)
            .help("\x1b[32m//\x1b[0m Append stdin to tempfile at INDEX"))
        .arg(Arg::new(REVERSE)
            .long("rev")
            .action(ArgAction::SetTrue)
            .help("\x1b[32m//\x1b[0m Reverse the entire stack order"))
        .arg(Arg::new(EXPIRE)
            .long("expire")
            .value_name("HOURS")
            .help("\x1b[32m//\x1b[0m Purge tempfiles older than HOURS hours"))
        .arg(Arg::new(HEAD)
            .long("head")
            .value_names(["INDEX", "N"])
            .num_args(2)
            .allow_hyphen_values(true)
            .help("\x1b[32m//\x1b[0m Show first N lines of tempfile at INDEX"))
        .arg(Arg::new(TAIL)
            .long("tail")
            .value_names(["INDEX", "N"])
            .num_args(2)
            .allow_hyphen_values(true)
            .help("\x1b[32m//\x1b[0m Show last N lines of tempfile at INDEX"))
        .arg(Arg::new(WC)
            .long("wc")
            .value_name("INDEX")
            .allow_hyphen_values(true)
            .help("\x1b[32m//\x1b[0m Print line count of tempfile at INDEX"))
        .arg(Arg::new(SIZE)
            .long("size")
            .value_name("INDEX")
            .allow_hyphen_values(true)
            .help("\x1b[32m//\x1b[0m Print byte size of tempfile at INDEX"))
        .arg(Arg::new(SORT)
            .long("sort")
            .value_name("KEY")
            .default_missing_value("name")
            .num_args(0..=1)
            .help("\x1b[32m//\x1b[0m Sort stack by name, size, or mtime"))
        .arg(Arg::new(REPLACE)
            .long("replace")
            .value_names(["INDEX", "PATTERN", "REPLACEMENT"])
            .num_args(3)
            .allow_hyphen_values(true)
            .help("\x1b[32m//\x1b[0m Replace PATTERN with REPLACEMENT in tempfile"))
        .arg(Arg::new(PATH)
            .long("path")
            .value_name("INDEX")
            .allow_hyphen_values(true)
            .help("\x1b[32m//\x1b[0m Print file path of tempfile at INDEX"))
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── app construction ─────────────────────────────────

    #[test]
    fn parse_opts_returns_app() {
        let app = parse_opts();
        assert_eq!(app.get_name(), NAME);
    }

    // ── flag recognition ─────────────────────────────────

    #[test]
    fn recognizes_input_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-i", "3"]);
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("3"));
    }

    #[test]
    fn recognizes_input_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--input", "5"]);
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("5"));
    }

    #[test]
    fn recognizes_output_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-o", "2"]);
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("2"));
    }

    #[test]
    fn recognizes_output_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--output", "7"]);
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("7"));
    }

    #[test]
    fn recognizes_output_negative_index() {
        let m = parse_opts().get_matches_from(vec!["tp", "-o", "-1"]);
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("-1"));
    }

    #[test]
    fn recognizes_add_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-a", "4"]);
        assert_eq!(m.get_one::<String>(ADD).map(|s| s.as_str()), Some("4"));
    }

    #[test]
    fn recognizes_add_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--add", "4"]);
        assert_eq!(m.get_one::<String>(ADD).map(|s| s.as_str()), Some("4"));
    }

    #[test]
    fn recognizes_add_negative_index() {
        let m = parse_opts().get_matches_from(vec!["tp", "-a", "-2"]);
        assert_eq!(m.get_one::<String>(ADD).map(|s| s.as_str()), Some("-2"));
    }

    #[test]
    fn recognizes_remove_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-r", "1"]);
        assert_eq!(m.get_one::<String>(REMOVE).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_remove_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--remove", "1"]);
        assert_eq!(m.get_one::<String>(REMOVE).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_remove_negative_index() {
        let m = parse_opts().get_matches_from(vec!["tp", "-r", "-3"]);
        assert_eq!(m.get_one::<String>(REMOVE).map(|s| s.as_str()), Some("-3"));
    }

    // ── boolean flags ────────────────────────────────────

    #[test]
    fn recognizes_pop_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-p"]);
        assert!(m.get_flag(POP));
    }

    #[test]
    fn recognizes_pop_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--pop"]);
        assert!(m.get_flag(POP));
    }

    #[test]
    fn recognizes_unshift_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-u"]);
        assert!(m.get_flag(UNSHIFT));
    }

    #[test]
    fn recognizes_unshift_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--unshift"]);
        assert!(m.get_flag(UNSHIFT));
    }

    #[test]
    fn recognizes_shift_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-s"]);
        assert!(m.get_flag(SHIFT));
    }

    #[test]
    fn recognizes_shift_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--shift"]);
        assert!(m.get_flag(SHIFT));
    }

    #[test]
    fn recognizes_dir_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-d"]);
        assert!(m.get_flag(DIRECTORY));
    }

    #[test]
    fn recognizes_dir_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--dir"]);
        assert!(m.get_flag(DIRECTORY));
    }

    #[test]
    fn recognizes_master_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-m"]);
        assert!(m.get_flag(MASTER));
    }

    #[test]
    fn recognizes_master_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--master"]);
        assert!(m.get_flag(MASTER));
    }

    #[test]
    fn recognizes_list_files_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-l"]);
        assert!(m.get_flag(LIST_FILES));
    }

    #[test]
    fn recognizes_list_files_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--list-files"]);
        assert!(m.get_flag(LIST_FILES));
    }

    #[test]
    fn recognizes_list_files_numbered_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-n"]);
        assert!(m.get_flag(LIST_FILES_NUMBERED));
    }

    #[test]
    fn recognizes_list_files_numbered_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--list-files-numbered"]);
        assert!(m.get_flag(LIST_FILES_NUMBERED));
    }

    #[test]
    fn recognizes_list_contents_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-L"]);
        assert!(m.get_flag(LIST_CONTENTS));
    }

    #[test]
    fn recognizes_list_contents_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--list-contents"]);
        assert!(m.get_flag(LIST_CONTENTS));
    }

    #[test]
    fn recognizes_list_contents_numbered_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-N"]);
        assert!(m.get_flag(LIST_CONTENTS_NUMBERED));
    }

    #[test]
    fn recognizes_list_contents_numbered_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--list-contents-numbered"]);
        assert!(m.get_flag(LIST_CONTENTS_NUMBERED));
    }

    #[test]
    fn recognizes_quiet_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-q"]);
        assert!(m.get_flag(SILENT));
    }

    #[test]
    fn recognizes_quiet_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--quiet"]);
        assert!(m.get_flag(SILENT));
    }

    #[test]
    fn recognizes_clear_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-c"]);
        assert!(m.get_flag(CLEAR));
    }

    #[test]
    fn recognizes_clear_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--clear"]);
        assert!(m.get_flag(CLEAR));
    }

    #[test]
    fn recognizes_verbose_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-v"]);
        assert!(m.get_count(VERBOSE) > 0);
    }

    #[test]
    fn recognizes_verbose_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--verbose"]);
        assert!(m.get_count(VERBOSE) > 0);
    }

    // ── defaults (no args) ──────────────────────────────

    #[test]
    fn no_args_all_absent() {
        let m = parse_opts().get_matches_from(vec!["tp"]);
        assert!(!m.get_flag(POP));
        assert!(!m.get_flag(SHIFT));
        assert!(!m.get_flag(UNSHIFT));
        assert!(!m.get_flag(DIRECTORY));
        assert!(!m.get_flag(MASTER));
        assert!(!m.get_flag(LIST_FILES));
        assert!(!m.get_flag(LIST_FILES_NUMBERED));
        assert!(!m.get_flag(LIST_CONTENTS));
        assert!(!m.get_flag(LIST_CONTENTS_NUMBERED));
        assert!(!m.get_flag(SILENT));
        assert!(!m.get_flag(CLEAR));
        assert_eq!(m.get_count(VERBOSE), 0);
        assert!(m.get_one::<String>(INPUT).is_none());
        assert!(m.get_one::<String>(OUTPUT).is_none());
        assert!(m.get_one::<String>(ADD).is_none());
        assert!(m.get_one::<String>(REMOVE).is_none());
        assert!(m.get_one::<String>(ARGFILE).is_none());
    }

    // ── positional argument ─────────────────────────────

    #[test]
    fn recognizes_positional_argfile() {
        let m = parse_opts().get_matches_from(vec!["tp", "myfile.txt"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("myfile.txt")
        );
    }

    #[test]
    fn positional_argfile_with_path() {
        let m = parse_opts().get_matches_from(vec!["tp", "/tmp/data.txt"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("/tmp/data.txt")
        );
    }

    // ── combined flags ──────────────────────────────────

    #[test]
    fn combined_verbose_and_quiet() {
        let m = parse_opts().get_matches_from(vec!["tp", "-v", "-q"]);
        assert!(m.get_count(VERBOSE) > 0);
        assert!(m.get_flag(SILENT));
    }

    #[test]
    fn input_with_quiet() {
        let m = parse_opts().get_matches_from(vec!["tp", "-i", "1", "-q"]);
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("1"));
        assert!(m.get_flag(SILENT));
    }

    #[test]
    fn output_with_verbose() {
        let m = parse_opts().get_matches_from(vec!["tp", "-o", "2", "-v"]);
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("2"));
        assert!(m.get_count(VERBOSE) > 0);
    }

    // ── invalid args ────────────────────────────────────

    #[test]
    fn unknown_flag_errors() {
        let result = parse_opts().try_get_matches_from(vec!["tp", "--nonexistent"]);
        assert!(result.is_err());
    }

    #[test]
    fn input_without_value_errors() {
        let result = parse_opts().try_get_matches_from(vec!["tp", "-i"]);
        assert!(result.is_err());
    }

    #[test]
    fn output_without_value_errors() {
        let result = parse_opts().try_get_matches_from(vec!["tp", "-o"]);
        assert!(result.is_err());
    }

    #[test]
    fn add_without_value_errors() {
        let result = parse_opts().try_get_matches_from(vec!["tp", "-a"]);
        assert!(result.is_err());
    }

    #[test]
    fn remove_without_value_errors() {
        let result = parse_opts().try_get_matches_from(vec!["tp", "-r"]);
        assert!(result.is_err());
    }

    // ── verbose multiple ────────────────────────────────

    #[test]
    fn verbose_counts_occurrences() {
        let m = parse_opts().get_matches_from(vec!["tp", "-vvv"]);
        assert_eq!(m.get_count(VERBOSE), 3);
    }

    #[test]
    fn verbose_single_occurrence() {
        let m = parse_opts().get_matches_from(vec!["tp", "-v"]);
        assert_eq!(m.get_count(VERBOSE), 1);
    }

    // ── value-taking args accept various strings ────────

    #[test]
    fn input_accepts_large_index() {
        let m = parse_opts().get_matches_from(vec!["tp", "-i", "999"]);
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("999"));
    }

    #[test]
    fn output_accepts_negative() {
        let m = parse_opts().get_matches_from(vec!["tp", "-o", "-10"]);
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("-10"));
    }

    #[test]
    fn remove_accepts_negative() {
        let m = parse_opts().get_matches_from(vec!["tp", "-r", "-1"]);
        assert_eq!(m.get_one::<String>(REMOVE).map(|s| s.as_str()), Some("-1"));
    }

    // ── command metadata ────────────────────────────────

    #[test]
    fn command_has_version() {
        let app = parse_opts();
        assert!(app.get_version().is_some());
    }

    #[test]
    fn command_version_matches_cargo() {
        let app = parse_opts();
        assert_eq!(app.get_version().unwrap(), env!("CARGO_PKG_VERSION"));
    }

    #[test]
    fn command_has_about() {
        let app = parse_opts();
        assert!(app.get_about().is_some());
    }

    #[test]
    fn command_has_author() {
        let app = parse_opts();
        assert!(app.get_author().is_some());
    }

    // ── all args present in command ─────────────────────

    #[test]
    fn command_has_all_expected_args() {
        let app = parse_opts();
        let arg_ids: Vec<&str> = app.get_arguments().map(|a| a.get_id().as_str()).collect();
        for expected in &[
            INPUT,
            OUTPUT,
            ADD,
            REMOVE,
            POP,
            UNSHIFT,
            SHIFT,
            ARGFILE,
            DIRECTORY,
            MASTER,
            LIST_FILES,
            LIST_FILES_NUMBERED,
            LIST_CONTENTS,
            LIST_CONTENTS_NUMBERED,
            SILENT,
            CLEAR,
            VERBOSE,
        ] {
            assert!(arg_ids.contains(expected), "missing arg: {}", expected);
        }
    }

    #[test]
    fn command_has_17_custom_args() {
        let app = parse_opts();
        // clap adds --help and --version automatically
        let custom_count = app
            .get_arguments()
            .filter(|a| a.get_id() != "help" && a.get_id() != "version")
            .count();
        assert_eq!(custom_count, 38);
    }

    // ── flag mutual independence ────────────────────────

    #[test]
    fn all_boolean_flags_independent() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "-p", "-u", "-s", "-d", "-m", "-l", "-n", "-L", "-N", "-q", "-c", "-v",
        ]);
        assert!(m.get_flag(POP));
        assert!(m.get_flag(UNSHIFT));
        assert!(m.get_flag(SHIFT));
        assert!(m.get_flag(DIRECTORY));
        assert!(m.get_flag(MASTER));
        assert!(m.get_flag(LIST_FILES));
        assert!(m.get_flag(LIST_FILES_NUMBERED));
        assert!(m.get_flag(LIST_CONTENTS));
        assert!(m.get_flag(LIST_CONTENTS_NUMBERED));
        assert!(m.get_flag(SILENT));
        assert!(m.get_flag(CLEAR));
        assert!(m.get_count(VERBOSE) > 0);
    }

    // ── value args with various strings ─────────────────

    #[test]
    fn input_accepts_zero() {
        let m = parse_opts().get_matches_from(vec!["tp", "-i", "0"]);
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("0"));
    }

    #[test]
    fn output_accepts_zero() {
        let m = parse_opts().get_matches_from(vec!["tp", "-o", "0"]);
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("0"));
    }

    #[test]
    fn add_accepts_zero() {
        let m = parse_opts().get_matches_from(vec!["tp", "-a", "0"]);
        assert_eq!(m.get_one::<String>(ADD).map(|s| s.as_str()), Some("0"));
    }

    #[test]
    fn remove_accepts_zero() {
        let m = parse_opts().get_matches_from(vec!["tp", "-r", "0"]);
        assert_eq!(m.get_one::<String>(REMOVE).map(|s| s.as_str()), Some("0"));
    }

    #[test]
    fn input_accepts_non_numeric() {
        let m = parse_opts().get_matches_from(vec!["tp", "-i", "abc"]);
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("abc"));
    }

    #[test]
    fn output_accepts_negative_large() {
        let m = parse_opts().get_matches_from(vec!["tp", "-o", "-999"]);
        assert_eq!(
            m.get_one::<String>(OUTPUT).map(|s| s.as_str()),
            Some("-999")
        );
    }

    #[test]
    fn add_accepts_negative_large() {
        let m = parse_opts().get_matches_from(vec!["tp", "-a", "-100"]);
        assert_eq!(m.get_one::<String>(ADD).map(|s| s.as_str()), Some("-100"));
    }

    #[test]
    fn remove_accepts_large_value() {
        let m = parse_opts().get_matches_from(vec!["tp", "-r", "99999"]);
        assert_eq!(
            m.get_one::<String>(REMOVE).map(|s| s.as_str()),
            Some("99999")
        );
    }

    // ── positional with flags combined ──────────────────

    #[test]
    fn positional_with_pop() {
        let m = parse_opts().get_matches_from(vec!["tp", "-p", "somefile"]);
        assert!(m.get_flag(POP));
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("somefile")
        );
    }

    #[test]
    fn positional_with_verbose() {
        let m = parse_opts().get_matches_from(vec!["tp", "-v", "myfile"]);
        assert!(m.get_count(VERBOSE) > 0);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("myfile")
        );
    }

    #[test]
    fn positional_with_quiet() {
        let m = parse_opts().get_matches_from(vec!["tp", "-q", "file.txt"]);
        assert!(m.get_flag(SILENT));
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("file.txt")
        );
    }

    // ── multiple verbose levels ─────────────────────────

    #[test]
    fn verbose_two_occurrences() {
        let m = parse_opts().get_matches_from(vec!["tp", "-vv"]);
        assert_eq!(m.get_count(VERBOSE), 2);
    }

    #[test]
    fn verbose_four_occurrences() {
        let m = parse_opts().get_matches_from(vec!["tp", "-vvvv"]);
        assert_eq!(m.get_count(VERBOSE), 4);
    }

    #[test]
    fn verbose_five_occurrences() {
        let m = parse_opts().get_matches_from(vec!["tp", "-vvvvv"]);
        assert_eq!(m.get_count(VERBOSE), 5);
    }

    #[test]
    fn verbose_separate_flags() {
        let m = parse_opts().get_matches_from(vec!["tp", "-v", "-v", "-v"]);
        assert_eq!(m.get_count(VERBOSE), 3);
    }

    #[test]
    fn verbose_mixed_short_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "-v", "--verbose"]);
        assert_eq!(m.get_count(VERBOSE), 2);
    }

    // ── unknown and invalid combinations ────────────────

    #[test]
    fn unknown_short_flag_errors() {
        let result = parse_opts().try_get_matches_from(vec!["tp", "-z"]);
        assert!(result.is_err());
    }

    #[test]
    fn unknown_long_flag_errors() {
        let result = parse_opts().try_get_matches_from(vec!["tp", "--foobar"]);
        assert!(result.is_err());
    }

    #[test]
    fn double_dash_stops_parsing() {
        let m = parse_opts().get_matches_from(vec!["tp", "--", "--not-a-flag"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("--not-a-flag")
        );
    }

    // ── long form with equals sign ──────────────────────

    #[test]
    fn input_long_equals() {
        let m = parse_opts().get_matches_from(vec!["tp", "--input=3"]);
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("3"));
    }

    #[test]
    fn output_long_equals() {
        let m = parse_opts().get_matches_from(vec!["tp", "--output=5"]);
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("5"));
    }

    #[test]
    fn add_long_equals() {
        let m = parse_opts().get_matches_from(vec!["tp", "--add=2"]);
        assert_eq!(m.get_one::<String>(ADD).map(|s| s.as_str()), Some("2"));
    }

    #[test]
    fn remove_long_equals() {
        let m = parse_opts().get_matches_from(vec!["tp", "--remove=4"]);
        assert_eq!(m.get_one::<String>(REMOVE).map(|s| s.as_str()), Some("4"));
    }

    // ── all value args combined ─────────────────────────

    #[test]
    fn all_value_args_together() {
        let m =
            parse_opts().get_matches_from(vec!["tp", "-i", "1", "-o", "2", "-a", "3", "-r", "4"]);
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(ADD).map(|s| s.as_str()), Some("3"));
        assert_eq!(m.get_one::<String>(REMOVE).map(|s| s.as_str()), Some("4"));
    }

    #[test]
    fn value_args_with_boolean_flags() {
        let m = parse_opts().get_matches_from(vec!["tp", "-i", "1", "-q", "-v", "-d"]);
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("1"));
        assert!(m.get_flag(SILENT));
        assert!(m.get_count(VERBOSE) > 0);
        assert!(m.get_flag(DIRECTORY));
    }

    // ── boolean flags default to false ──────────────────

    #[test]
    fn pop_default_false() {
        let m = parse_opts().get_matches_from(vec!["tp"]);
        assert!(!m.get_flag(POP));
    }

    #[test]
    fn unshift_default_false() {
        let m = parse_opts().get_matches_from(vec!["tp"]);
        assert!(!m.get_flag(UNSHIFT));
    }

    #[test]
    fn shift_default_false() {
        let m = parse_opts().get_matches_from(vec!["tp"]);
        assert!(!m.get_flag(SHIFT));
    }

    #[test]
    fn directory_default_false() {
        let m = parse_opts().get_matches_from(vec!["tp"]);
        assert!(!m.get_flag(DIRECTORY));
    }

    #[test]
    fn master_default_false() {
        let m = parse_opts().get_matches_from(vec!["tp"]);
        assert!(!m.get_flag(MASTER));
    }

    #[test]
    fn list_files_default_false() {
        let m = parse_opts().get_matches_from(vec!["tp"]);
        assert!(!m.get_flag(LIST_FILES));
    }

    #[test]
    fn list_files_numbered_default_false() {
        let m = parse_opts().get_matches_from(vec!["tp"]);
        assert!(!m.get_flag(LIST_FILES_NUMBERED));
    }

    #[test]
    fn list_contents_default_false() {
        let m = parse_opts().get_matches_from(vec!["tp"]);
        assert!(!m.get_flag(LIST_CONTENTS));
    }

    #[test]
    fn list_contents_numbered_default_false() {
        let m = parse_opts().get_matches_from(vec!["tp"]);
        assert!(!m.get_flag(LIST_CONTENTS_NUMBERED));
    }

    #[test]
    fn silent_default_false() {
        let m = parse_opts().get_matches_from(vec!["tp"]);
        assert!(!m.get_flag(SILENT));
    }

    #[test]
    fn clear_default_false() {
        let m = parse_opts().get_matches_from(vec!["tp"]);
        assert!(!m.get_flag(CLEAR));
    }

    #[test]
    fn verbose_default_zero() {
        let m = parse_opts().get_matches_from(vec!["tp"]);
        assert_eq!(m.get_count(VERBOSE), 0);
    }

    // ── positional edge cases ───────────────────────────

    #[test]
    fn positional_with_spaces_in_name() {
        let m = parse_opts().get_matches_from(vec!["tp", "my file.txt"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("my file.txt")
        );
    }

    #[test]
    fn positional_with_unicode() {
        let m = parse_opts().get_matches_from(vec!["tp", "日本語.txt"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("日本語.txt")
        );
    }

    #[test]
    fn positional_empty_string() {
        let m = parse_opts().get_matches_from(vec!["tp", ""]);
        assert_eq!(m.get_one::<String>(ARGFILE).map(|s| s.as_str()), Some(""));
    }

    #[test]
    fn positional_dot() {
        let m = parse_opts().get_matches_from(vec!["tp", "."]);
        assert_eq!(m.get_one::<String>(ARGFILE).map(|s| s.as_str()), Some("."));
    }

    #[test]
    fn positional_hidden_file() {
        let m = parse_opts().get_matches_from(vec!["tp", ".hidden"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some(".hidden")
        );
    }

    // ── negative index for output via long flag ─────────

    #[test]
    fn output_long_negative() {
        let m = parse_opts().get_matches_from(vec!["tp", "--output", "-5"]);
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("-5"));
    }

    #[test]
    fn add_long_negative() {
        let m = parse_opts().get_matches_from(vec!["tp", "--add", "-3"]);
        assert_eq!(m.get_one::<String>(ADD).map(|s| s.as_str()), Some("-3"));
    }

    #[test]
    fn remove_long_negative() {
        let m = parse_opts().get_matches_from(vec!["tp", "--remove", "-7"]);
        assert_eq!(m.get_one::<String>(REMOVE).map(|s| s.as_str()), Some("-7"));
    }

    // ── help and version flags ──────────────────────────

    #[test]
    fn help_flag_recognized() {
        let result = parse_opts().try_get_matches_from(vec!["tp", "--help"]);
        // --help causes an early exit, which is an Err
        assert!(result.is_err());
    }

    #[test]
    fn version_flag_recognized() {
        let result = parse_opts().try_get_matches_from(vec!["tp", "--version"]);
        assert!(result.is_err());
    }

    #[test]
    fn help_short_flag_recognized() {
        let result = parse_opts().try_get_matches_from(vec!["tp", "-h"]);
        assert!(result.is_err());
    }

    #[test]
    fn version_short_flag_recognized() {
        let result = parse_opts().try_get_matches_from(vec!["tp", "-V"]);
        assert!(result.is_err());
    }

    // ── extended commands (edit, tag, grep, sort, …) ───

    #[test]
    fn recognizes_edit_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-e", "2"]);
        assert_eq!(m.get_one::<String>(EDIT).map(|s| s.as_str()), Some("2"));
    }

    #[test]
    fn recognizes_edit_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--edit", "-1"]);
        assert_eq!(m.get_one::<String>(EDIT).map(|s| s.as_str()), Some("-1"));
    }

    #[test]
    fn recognizes_name_tag_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-w", "my-alias"]);
        assert_eq!(
            m.get_one::<String>(TAG).map(|s| s.as_str()),
            Some("my-alias")
        );
    }

    #[test]
    fn recognizes_name_tag_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--name", "tag2"]);
        assert_eq!(m.get_one::<String>(TAG).map(|s| s.as_str()), Some("tag2"));
    }

    #[test]
    fn recognizes_rename_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-R", "old", "new"]);
        let names: Vec<String> = m.get_many(RENAME).unwrap().cloned().collect();
        assert_eq!(names, vec!["old", "new"]);
    }

    #[test]
    fn recognizes_rename_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--rename", "a", "b"]);
        let names: Vec<String> = m.get_many(RENAME).unwrap().cloned().collect();
        assert_eq!(names, vec!["a", "b"]);
    }

    #[test]
    fn recognizes_info_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-I", "3"]);
        assert_eq!(m.get_one::<String>(INFO).map(|s| s.as_str()), Some("3"));
    }

    #[test]
    fn recognizes_info_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--info", "-2"]);
        assert_eq!(m.get_one::<String>(INFO).map(|s| s.as_str()), Some("-2"));
    }

    #[test]
    fn recognizes_grep_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-g", "needle"]);
        assert_eq!(
            m.get_one::<String>(GREP).map(|s| s.as_str()),
            Some("needle")
        );
    }

    #[test]
    fn recognizes_grep_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--grep", "foo.*bar"]);
        assert_eq!(
            m.get_one::<String>(GREP).map(|s| s.as_str()),
            Some("foo.*bar")
        );
    }

    #[test]
    fn recognizes_cat_short_one_index() {
        let m = parse_opts().get_matches_from(vec!["tp", "-C", "1"]);
        let vals: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(vals, vec!["1"]);
    }

    #[test]
    fn recognizes_cat_short_multiple_indices() {
        let m = parse_opts().get_matches_from(vec!["tp", "-C", "1", "2", "3"]);
        let vals: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(vals, vec!["1", "2", "3"]);
    }

    #[test]
    fn recognizes_cat_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--cat", "-1", "2"]);
        let vals: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(vals, vec!["-1", "2"]);
    }

    #[test]
    fn recognizes_count_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-k"]);
        assert!(m.get_flag(COUNT));
    }

    #[test]
    fn recognizes_count_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--count"]);
        assert!(m.get_flag(COUNT));
    }

    #[test]
    fn recognizes_diff_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-D", "1", "2"]);
        let v: Vec<String> = m.get_many(DIFF).unwrap().cloned().collect();
        assert_eq!(v, vec!["1", "2"]);
    }

    #[test]
    fn recognizes_diff_long_negative_indices() {
        let m = parse_opts().get_matches_from(vec!["tp", "--diff", "-1", "-2"]);
        let v: Vec<String> = m.get_many(DIFF).unwrap().cloned().collect();
        assert_eq!(v, vec!["-1", "-2"]);
    }

    #[test]
    fn recognizes_mv_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-M", "1", "3"]);
        let v: Vec<String> = m.get_many(MOVE).unwrap().cloned().collect();
        assert_eq!(v, vec!["1", "3"]);
    }

    #[test]
    fn recognizes_mv_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--mv", "2", "4"]);
        let v: Vec<String> = m.get_many(MOVE).unwrap().cloned().collect();
        assert_eq!(v, vec!["2", "4"]);
    }

    #[test]
    fn recognizes_dup_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-x", "1"]);
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_dup_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--dup", "-3"]);
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("-3"));
    }

    #[test]
    fn recognizes_swap_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-S", "1", "2"]);
        let v: Vec<String> = m.get_many(SWAP).unwrap().cloned().collect();
        assert_eq!(v, vec!["1", "2"]);
    }

    #[test]
    fn recognizes_swap_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--swap", "a", "b"]);
        let v: Vec<String> = m.get_many(SWAP).unwrap().cloned().collect();
        assert_eq!(v, vec!["a", "b"]);
    }

    #[test]
    fn recognizes_append_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-A", "2"]);
        assert_eq!(m.get_one::<String>(APPEND).map(|s| s.as_str()), Some("2"));
    }

    #[test]
    fn recognizes_append_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--append", "-1"]);
        assert_eq!(m.get_one::<String>(APPEND).map(|s| s.as_str()), Some("-1"));
    }

    #[test]
    fn recognizes_rev_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--rev"]);
        assert!(m.get_flag(REVERSE));
    }

    #[test]
    fn recognizes_expire_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--expire", "24"]);
        assert_eq!(m.get_one::<String>(EXPIRE).map(|s| s.as_str()), Some("24"));
    }

    #[test]
    fn recognizes_head_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--head", "1", "10"]);
        let v: Vec<String> = m.get_many(HEAD).unwrap().cloned().collect();
        assert_eq!(v, vec!["1", "10"]);
    }

    #[test]
    fn recognizes_tail_long_negative_index() {
        let m = parse_opts().get_matches_from(vec!["tp", "--tail", "-1", "5"]);
        let v: Vec<String> = m.get_many(TAIL).unwrap().cloned().collect();
        assert_eq!(v, vec!["-1", "5"]);
    }

    #[test]
    fn recognizes_wc_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--wc", "2"]);
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("2"));
    }

    #[test]
    fn recognizes_size_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--size", "-2"]);
        assert_eq!(m.get_one::<String>(SIZE).map(|s| s.as_str()), Some("-2"));
    }

    #[test]
    fn recognizes_sort_long_default() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("name"));
    }

    #[test]
    fn recognizes_sort_long_size() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort", "size"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("size"));
    }

    #[test]
    fn recognizes_replace_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--replace", "1", "foo", "bar"]);
        let v: Vec<String> = m.get_many(REPLACE).unwrap().cloned().collect();
        assert_eq!(v, vec!["1", "foo", "bar"]);
    }

    #[test]
    fn recognizes_path_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--path", "3"]);
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("3"));
    }

    #[test]
    fn extended_flags_combine_with_verbose() {
        let m = parse_opts().get_matches_from(vec!["tp", "-v", "-k", "-e", "1"]);
        assert!(m.get_count(VERBOSE) > 0);
        assert!(m.get_flag(COUNT));
        assert_eq!(m.get_one::<String>(EDIT).map(|s| s.as_str()), Some("1"));
    }

    // ── long flag equals syntax (--flag=value) ──────────

    #[test]
    fn recognizes_input_long_equals_value() {
        let m = parse_opts().get_matches_from(vec!["tp", "--input=99"]);
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("99"));
    }

    #[test]
    fn recognizes_output_long_equals_negative() {
        let m = parse_opts().get_matches_from(vec!["tp", "--output=-4"]);
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("-4"));
    }

    #[test]
    fn recognizes_add_long_equals() {
        let m = parse_opts().get_matches_from(vec!["tp", "--add=0"]);
        assert_eq!(m.get_one::<String>(ADD).map(|s| s.as_str()), Some("0"));
    }

    #[test]
    fn recognizes_remove_long_equals() {
        let m = parse_opts().get_matches_from(vec!["tp", "--remove=100"]);
        assert_eq!(m.get_one::<String>(REMOVE).map(|s| s.as_str()), Some("100"));
    }

    #[test]
    fn recognizes_edit_long_equals() {
        let m = parse_opts().get_matches_from(vec!["tp", "--edit=5"]);
        assert_eq!(m.get_one::<String>(EDIT).map(|s| s.as_str()), Some("5"));
    }

    #[test]
    fn recognizes_name_long_equals() {
        let m = parse_opts().get_matches_from(vec!["tp", "--name=stack-tag"]);
        assert_eq!(
            m.get_one::<String>(TAG).map(|s| s.as_str()),
            Some("stack-tag")
        );
    }

    #[test]
    fn recognizes_info_long_equals() {
        let m = parse_opts().get_matches_from(vec!["tp", "--info=-1"]);
        assert_eq!(m.get_one::<String>(INFO).map(|s| s.as_str()), Some("-1"));
    }

    #[test]
    fn recognizes_grep_long_equals() {
        let m = parse_opts().get_matches_from(vec!["tp", "--grep=^start"]);
        assert_eq!(
            m.get_one::<String>(GREP).map(|s| s.as_str()),
            Some("^start")
        );
    }

    #[test]
    fn recognizes_dup_long_equals() {
        let m = parse_opts().get_matches_from(vec!["tp", "--dup=2"]);
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("2"));
    }

    #[test]
    fn recognizes_append_long_equals() {
        let m = parse_opts().get_matches_from(vec!["tp", "--append=3"]);
        assert_eq!(m.get_one::<String>(APPEND).map(|s| s.as_str()), Some("3"));
    }

    #[test]
    fn recognizes_expire_long_equals() {
        let m = parse_opts().get_matches_from(vec!["tp", "--expire=72"]);
        assert_eq!(m.get_one::<String>(EXPIRE).map(|s| s.as_str()), Some("72"));
    }

    #[test]
    fn recognizes_wc_long_equals() {
        let m = parse_opts().get_matches_from(vec!["tp", "--wc=-2"]);
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("-2"));
    }

    #[test]
    fn recognizes_size_long_equals() {
        let m = parse_opts().get_matches_from(vec!["tp", "--size=1"]);
        assert_eq!(m.get_one::<String>(SIZE).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_path_long_equals() {
        let m = parse_opts().get_matches_from(vec!["tp", "--path=-1"]);
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("-1"));
    }

    #[test]
    fn recognizes_sort_long_equals_name() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort=name"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("name"));
    }

    #[test]
    fn recognizes_sort_long_equals_size() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort=size"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("size"));
    }

    #[test]
    fn recognizes_sort_long_equals_mtime() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort=mtime"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("mtime"));
    }

    #[test]
    fn recognizes_verbose_long_equals_not_used() {
        // --verbose is a count flag; no =value in our CLI
        let m = parse_opts().get_matches_from(vec!["tp", "--verbose"]);
        assert!(m.get_count(VERBOSE) > 0);
    }

    #[test]
    fn equals_syntax_combined_with_positional() {
        let m = parse_opts().get_matches_from(vec!["tp", "--output=2", "-q", "file.txt"]);
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("2"));
        assert!(m.get_flag(SILENT));
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("file.txt")
        );
    }

    #[test]
    fn equals_syntax_three_value_flags_still_space_separated() {
        let m = parse_opts().get_matches_from(vec!["tp", "--replace", "2", "a", "b"]);
        let v: Vec<String> = m.get_many(REPLACE).unwrap().cloned().collect();
        assert_eq!(v, vec!["2", "a", "b"]);
    }

    #[test]
    fn recognizes_cat_long_equals_single() {
        let m = parse_opts().get_matches_from(vec!["tp", "--cat=1"]);
        let vals: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(vals, vec!["1"]);
    }

    #[test]
    fn stack_of_value_flags_equals_and_short() {
        let m = parse_opts().get_matches_from(vec![
            "tp",
            "-i",
            "1",
            "--output=2",
            "--append=3",
            "--grep=x",
        ]);
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(APPEND).map(|s| s.as_str()), Some("3"));
        assert_eq!(m.get_one::<String>(GREP).map(|s| s.as_str()), Some("x"));
    }

    #[test]
    fn many_boolean_flags_with_equals_value_args() {
        let m = parse_opts().get_matches_from(vec![
            "tp",
            "-d",
            "-m",
            "-l",
            "--edit=1",
            "--name=foo",
            "--dup=2",
        ]);
        assert!(m.get_flag(DIRECTORY));
        assert!(m.get_flag(MASTER));
        assert!(m.get_flag(LIST_FILES));
        assert_eq!(m.get_one::<String>(EDIT).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(TAG).map(|s| s.as_str()), Some("foo"));
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("2"));
    }

    #[test]
    fn recognizes_diff_space_args_after_equals_elsewhere() {
        let m = parse_opts().get_matches_from(vec!["tp", "--input=1", "-D", "2", "3"]);
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("1"));
        let v: Vec<String> = m.get_many(DIFF).unwrap().cloned().collect();
        assert_eq!(v, vec!["2", "3"]);
    }

    #[test]
    fn recognizes_swap_and_mv_space_args_mixed_with_equals() {
        let m = parse_opts().get_matches_from(vec![
            "tp",
            "--input=1",
            "--mv",
            "2",
            "3",
            "-S",
            "1",
            "2",
        ]);
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("1"));
        let mv: Vec<String> = m.get_many(MOVE).unwrap().cloned().collect();
        assert_eq!(mv, vec!["2", "3"]);
        let sw: Vec<String> = m.get_many(SWAP).unwrap().cloned().collect();
        assert_eq!(sw, vec!["1", "2"]);
    }

    // ── short/long mixes and argument order ─────────────

    #[test]
    fn value_args_order_independent_io() {
        let a = parse_opts().get_matches_from(vec!["tp", "-i", "1", "-o", "2"]);
        let b = parse_opts().get_matches_from(vec!["tp", "-o", "2", "-i", "1"]);
        assert_eq!(
            a.get_one::<String>(INPUT).map(|s| s.as_str()),
            b.get_one::<String>(INPUT).map(|s| s.as_str())
        );
        assert_eq!(
            a.get_one::<String>(OUTPUT).map(|s| s.as_str()),
            b.get_one::<String>(OUTPUT).map(|s| s.as_str())
        );
    }

    #[test]
    fn recognizes_edit_with_name_tag() {
        let m = parse_opts().get_matches_from(vec!["tp", "-e", "2", "-w", "t1"]);
        assert_eq!(m.get_one::<String>(EDIT).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(TAG).map(|s| s.as_str()), Some("t1"));
    }

    #[test]
    fn recognizes_grep_append_input_chain() {
        let m = parse_opts().get_matches_from(vec!["tp", "-g", "pat", "-A", "1", "-i", "2"]);
        assert_eq!(m.get_one::<String>(GREP).map(|s| s.as_str()), Some("pat"));
        assert_eq!(m.get_one::<String>(APPEND).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("2"));
    }

    #[test]
    fn recognizes_head_tail_both_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--head", "1", "5", "--tail", "2", "3"]);
        let h: Vec<String> = m.get_many(HEAD).unwrap().cloned().collect();
        let t: Vec<String> = m.get_many(TAIL).unwrap().cloned().collect();
        assert_eq!(h, vec!["1", "5"]);
        assert_eq!(t, vec!["2", "3"]);
    }

    #[test]
    fn recognizes_replace_with_unicode_values() {
        let m = parse_opts().get_matches_from(vec!["tp", "--replace", "1", "日本", "한국"]);
        let v: Vec<String> = m.get_many(REPLACE).unwrap().cloned().collect();
        assert_eq!(v, vec!["1", "日本", "한국"]);
    }

    #[test]
    fn recognizes_cat_five_indices() {
        let m = parse_opts().get_matches_from(vec!["tp", "-C", "1", "2", "3", "4", "5"]);
        let vals: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(vals, vec!["1", "2", "3", "4", "5"]);
    }

    #[test]
    fn recognizes_verbose_six_times() {
        let m = parse_opts().get_matches_from(vec!["tp", "-vvvvvv"]);
        assert_eq!(m.get_count(VERBOSE), 6);
    }

    #[test]
    fn recognizes_pop_shift_unshift_together_parse() {
        let m = parse_opts().get_matches_from(vec!["tp", "-p", "-s", "-u"]);
        assert!(m.get_flag(POP));
        assert!(m.get_flag(SHIFT));
        assert!(m.get_flag(UNSHIFT));
    }

    #[test]
    fn recognizes_list_all_four_list_modes() {
        let m = parse_opts().get_matches_from(vec!["tp", "-l", "-n", "-L", "-N"]);
        assert!(m.get_flag(LIST_FILES));
        assert!(m.get_flag(LIST_FILES_NUMBERED));
        assert!(m.get_flag(LIST_CONTENTS));
        assert!(m.get_flag(LIST_CONTENTS_NUMBERED));
    }

    #[test]
    fn recognizes_clear_with_output() {
        let m = parse_opts().get_matches_from(vec!["tp", "-c", "-o", "1"]);
        assert!(m.get_flag(CLEAR));
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_reverse_with_sort() {
        let m = parse_opts().get_matches_from(vec!["tp", "--rev", "--sort", "size"]);
        assert!(m.get_flag(REVERSE));
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("size"));
    }

    #[test]
    fn recognizes_dup_swap_diff_triple() {
        let m =
            parse_opts().get_matches_from(vec!["tp", "-x", "1", "-S", "1", "2", "-D", "1", "2"]);
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("1"));
        let sw: Vec<String> = m.get_many(SWAP).unwrap().cloned().collect();
        let df: Vec<String> = m.get_many(DIFF).unwrap().cloned().collect();
        assert_eq!(sw, vec!["1", "2"]);
        assert_eq!(df, vec!["1", "2"]);
    }

    #[test]
    fn recognizes_path_wc_size_same_index() {
        let m = parse_opts().get_matches_from(vec!["tp", "--path=2", "--wc=2", "--size=2"]);
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(SIZE).map(|s| s.as_str()), Some("2"));
    }

    #[test]
    fn recognizes_expire_numeric_string() {
        let m = parse_opts().get_matches_from(vec!["tp", "--expire", "0"]);
        assert_eq!(m.get_one::<String>(EXPIRE).map(|s| s.as_str()), Some("0"));
    }

    #[test]
    fn recognizes_rename_unicode_tags() {
        let m = parse_opts().get_matches_from(vec!["tp", "-R", "旧", "新"]);
        let v: Vec<String> = m.get_many(RENAME).unwrap().cloned().collect();
        assert_eq!(v, vec!["旧", "新"]);
    }

    #[test]
    fn recognizes_positional_after_flags() {
        let m = parse_opts().get_matches_from(vec!["tp", "-v", "-q", "/tmp/in.txt"]);
        assert!(m.get_count(VERBOSE) > 0);
        assert!(m.get_flag(SILENT));
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("/tmp/in.txt")
        );
    }

    #[test]
    fn recognizes_add_remove_both_set() {
        let m = parse_opts().get_matches_from(vec!["tp", "-a", "2", "-r", "3"]);
        assert_eq!(m.get_one::<String>(ADD).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(REMOVE).map(|s| s.as_str()), Some("3"));
    }

    #[test]
    fn recognizes_info_path_dup_chain() {
        let m = parse_opts().get_matches_from(vec!["tp", "-I", "1", "--path=1", "-x", "1"]);
        assert_eq!(m.get_one::<String>(INFO).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_mv_long_equals_via_space() {
        let m = parse_opts().get_matches_from(vec!["tp", "--mv", "1", "2"]);
        let v: Vec<String> = m.get_many(MOVE).unwrap().cloned().collect();
        assert_eq!(v, vec!["1", "2"]);
    }

    #[test]
    fn recognizes_tail_head_wc_no_conflict() {
        let m = parse_opts().get_matches_from(vec!["tp", "--tail", "1", "1", "--wc", "1"]);
        let t: Vec<String> = m.get_many(TAIL).unwrap().cloned().collect();
        assert_eq!(t, vec!["1", "1"]);
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_sort_only_long_no_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("name"));
    }

    #[test]
    fn recognizes_replace_special_chars_pattern() {
        let m = parse_opts().get_matches_from(vec!["tp", "--replace", "1", "\t", "\n"]);
        let v: Vec<String> = m.get_many(REPLACE).unwrap().cloned().collect();
        assert_eq!(v, vec!["1", "\t", "\n"]);
    }

    // ── bulk short/long coverage ─────────────────────────

    #[test]
    fn recognizes_io_add_remove_all_values() {
        let m =
            parse_opts().get_matches_from(vec!["tp", "-i", "9", "-o", "8", "-a", "1", "-r", "2"]);
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("9"));
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("8"));
        assert_eq!(m.get_one::<String>(ADD).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(REMOVE).map(|s| s.as_str()), Some("2"));
    }

    #[test]
    fn recognizes_edit_info_grep_short_chain() {
        let m = parse_opts().get_matches_from(vec!["tp", "-e", "3", "-I", "2", "-g", "x"]);
        assert_eq!(m.get_one::<String>(EDIT).map(|s| s.as_str()), Some("3"));
        assert_eq!(m.get_one::<String>(INFO).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(GREP).map(|s| s.as_str()), Some("x"));
    }

    #[test]
    fn recognizes_dup_append_wc_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-x", "4", "-A", "5", "--wc", "1"]);
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("4"));
        assert_eq!(m.get_one::<String>(APPEND).map(|s| s.as_str()), Some("5"));
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_size_path_expire_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--size=3", "--path=3", "--expire=12"]);
        assert_eq!(m.get_one::<String>(SIZE).map(|s| s.as_str()), Some("3"));
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("3"));
        assert_eq!(m.get_one::<String>(EXPIRE).map(|s| s.as_str()), Some("12"));
    }

    #[test]
    fn recognizes_move_swap_short_letters() {
        let m = parse_opts().get_matches_from(vec!["tp", "-M", "1", "4", "-S", "2", "3"]);
        let mv: Vec<String> = m.get_many(MOVE).unwrap().cloned().collect();
        let sw: Vec<String> = m.get_many(SWAP).unwrap().cloned().collect();
        assert_eq!(mv, vec!["1", "4"]);
        assert_eq!(sw, vec!["2", "3"]);
    }

    #[test]
    fn recognizes_diff_long_equals_not_used_two_values() {
        let m = parse_opts().get_matches_from(vec!["tp", "--diff", "10", "20"]);
        let v: Vec<String> = m.get_many(DIFF).unwrap().cloned().collect();
        assert_eq!(v, vec!["10", "20"]);
    }

    #[test]
    fn recognizes_cat_long_many_indices() {
        let m = parse_opts().get_matches_from(vec!["tp", "--cat", "1", "2", "3", "4", "5", "6"]);
        let vals: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(vals, vec!["1", "2", "3", "4", "5", "6"]);
    }

    #[test]
    fn recognizes_head_tail_pair_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--head", "2", "3", "--tail", "3", "1"]);
        let h: Vec<String> = m.get_many(HEAD).unwrap().cloned().collect();
        let t: Vec<String> = m.get_many(TAIL).unwrap().cloned().collect();
        assert_eq!(h, vec!["2", "3"]);
        assert_eq!(t, vec!["3", "1"]);
    }

    #[test]
    fn recognizes_replace_three_unicode_args() {
        let m = parse_opts().get_matches_from(vec!["tp", "--replace", "1", "α", "β"]);
        let v: Vec<String> = m.get_many(REPLACE).unwrap().cloned().collect();
        assert_eq!(v, vec!["1", "α", "β"]);
    }

    #[test]
    fn recognizes_name_with_hyphen_value() {
        let m = parse_opts().get_matches_from(vec!["tp", "-w", "my-tag"]);
        assert_eq!(m.get_one::<String>(TAG).map(|s| s.as_str()), Some("my-tag"));
    }

    #[test]
    fn recognizes_input_equals_embedded_equals() {
        let m = parse_opts().get_matches_from(vec!["tp", "--input==1"]);
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("=1"));
    }

    #[test]
    fn recognizes_grep_pattern_with_spaces_via_equals() {
        let m = parse_opts().get_matches_from(vec!["tp", "--grep=hello world"]);
        assert_eq!(
            m.get_one::<String>(GREP).map(|s| s.as_str()),
            Some("hello world")
        );
    }

    #[test]
    fn recognizes_verbose_ten_times() {
        let m = parse_opts().get_matches_from(vec!["tp", "-vvvvvvvvvv"]);
        assert_eq!(m.get_count(VERBOSE), 10);
    }

    #[test]
    fn recognizes_all_shift_style_flags() {
        let m = parse_opts().get_matches_from(vec!["tp", "-p", "-s", "-u"]);
        assert!(m.get_flag(POP));
        assert!(m.get_flag(SHIFT));
        assert!(m.get_flag(UNSHIFT));
    }

    #[test]
    fn recognizes_dir_master_list_flags_together() {
        let m = parse_opts().get_matches_from(vec!["tp", "-d", "-m", "-l", "-L"]);
        assert!(m.get_flag(DIRECTORY));
        assert!(m.get_flag(MASTER));
        assert!(m.get_flag(LIST_FILES));
        assert!(m.get_flag(LIST_CONTENTS));
    }

    #[test]
    fn recognizes_sort_mtime_via_equals() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort=mtime"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("mtime"));
    }

    #[test]
    fn recognizes_quiet_verbose_both_parse() {
        let m = parse_opts().get_matches_from(vec!["tp", "-q", "-v"]);
        assert!(m.get_flag(SILENT));
        assert_eq!(m.get_count(VERBOSE), 1);
    }

    #[test]
    fn recognizes_clear_pop_together() {
        let m = parse_opts().get_matches_from(vec!["tp", "-c", "-p"]);
        assert!(m.get_flag(CLEAR));
        assert!(m.get_flag(POP));
    }

    #[test]
    fn recognizes_count_rev_together() {
        let m = parse_opts().get_matches_from(vec!["tp", "-k", "--rev"]);
        assert!(m.get_flag(COUNT));
        assert!(m.get_flag(REVERSE));
    }

    #[test]
    fn recognizes_rename_emoji_values() {
        let m = parse_opts().get_matches_from(vec!["tp", "--rename", "🚀", "🌙"]);
        let v: Vec<String> = m.get_many(RENAME).unwrap().cloned().collect();
        assert_eq!(v, vec!["🚀", "🌙"]);
    }

    #[test]
    fn recognizes_output_positive_large() {
        let m = parse_opts().get_matches_from(vec!["tp", "-o", "1000000"]);
        assert_eq!(
            m.get_one::<String>(OUTPUT).map(|s| s.as_str()),
            Some("1000000")
        );
    }

    #[test]
    fn recognizes_add_negative_one_string() {
        let m = parse_opts().get_matches_from(vec!["tp", "-a", "-1"]);
        assert_eq!(m.get_one::<String>(ADD).map(|s| s.as_str()), Some("-1"));
    }

    #[test]
    fn recognizes_remove_positive_max_ish() {
        let m = parse_opts().get_matches_from(vec!["tp", "-r", "2147483647"]);
        assert_eq!(
            m.get_one::<String>(REMOVE).map(|s| s.as_str()),
            Some("2147483647")
        );
    }

    #[test]
    fn recognizes_cat_single_negative_index() {
        let m = parse_opts().get_matches_from(vec!["tp", "-C", "-1"]);
        let vals: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(vals, vec!["-1"]);
    }

    #[test]
    fn recognizes_double_dash_only_positional() {
        let m = parse_opts().get_matches_from(vec!["tp", "--", "-i"]);
        assert_eq!(m.get_one::<String>(ARGFILE).map(|s| s.as_str()), Some("-i"));
    }

    #[test]
    fn recognizes_positional_with_leading_dotdot() {
        let m = parse_opts().get_matches_from(vec!["tp", "../relative"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("../relative")
        );
    }

    #[test]
    fn recognizes_multiple_verbose_long_mixed() {
        let m = parse_opts().get_matches_from(vec!["tp", "-v", "--verbose", "-v"]);
        assert_eq!(m.get_count(VERBOSE), 3);
    }

    // ── additional clap coverage (round 2) ───────────────

    #[test]
    fn recognizes_short_flags_almost_full_alphabet() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "-i", "1", "-o", "2", "-a", "3", "-r", "4", "-e", "5", "-w", "t", "-I", "6",
            "-g", "p", "-C", "1", "-D", "1", "2", "-M", "1", "2", "-x", "1", "-S", "1", "2", "-A",
            "1",
        ]);
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(TAG).map(|s| s.as_str()), Some("t"));
    }

    #[test]
    fn recognizes_long_expire_wc_size_path_chain() {
        let m = parse_opts().get_matches_from(vec![
            "tp",
            "--expire=96",
            "--wc=3",
            "--size=3",
            "--path=3",
        ]);
        assert_eq!(m.get_one::<String>(EXPIRE).map(|s| s.as_str()), Some("96"));
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("3"));
        assert_eq!(m.get_one::<String>(SIZE).map(|s| s.as_str()), Some("3"));
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("3"));
    }

    #[test]
    fn recognizes_head_tail_replace_long() {
        let m = parse_opts().get_matches_from(vec![
            "tp",
            "--head",
            "1",
            "20",
            "--tail",
            "2",
            "5",
            "--replace",
            "3",
            "a",
            "b",
        ]);
        assert_eq!(
            m.get_many::<String>(HEAD)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["1", "20"]
        );
        assert_eq!(
            m.get_many::<String>(TAIL)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["2", "5"]
        );
        assert_eq!(
            m.get_many::<String>(REPLACE)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["3", "a", "b"]
        );
    }

    #[test]
    fn recognizes_sort_name_explicit_equals() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort=name"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("name"));
    }

    #[test]
    fn recognizes_input_output_both_negative_equals() {
        let m = parse_opts().get_matches_from(vec!["tp", "--input=-5", "--output=-3"]);
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("-5"));
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("-3"));
    }

    #[test]
    fn recognizes_grep_equals_regex_chars() {
        let m = parse_opts().get_matches_from(vec!["tp", "--grep=[0-9]+"]);
        assert_eq!(
            m.get_one::<String>(GREP).map(|s| s.as_str()),
            Some("[0-9]+")
        );
    }

    #[test]
    fn recognizes_dup_append_same_index_different_flags() {
        let m = parse_opts().get_matches_from(vec!["tp", "--dup=1", "--append=1"]);
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(APPEND).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_positional_windows_path_style() {
        let m = parse_opts().get_matches_from(vec!["tp", r"C:\temp\file.txt"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some(r"C:\temp\file.txt"),
        );
    }

    #[test]
    fn recognizes_positional_url_like() {
        let m = parse_opts().get_matches_from(vec!["tp", "s3://bucket/key"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("s3://bucket/key")
        );
    }

    #[test]
    fn recognizes_version_long_try_parse_errors() {
        assert!(
            parse_opts()
                .try_get_matches_from(vec!["tp", "--version"])
                .is_err()
        );
    }

    #[test]
    fn recognizes_help_long_try_parse_errors() {
        assert!(
            parse_opts()
                .try_get_matches_from(vec!["tp", "--help"])
                .is_err()
        );
    }

    #[test]
    fn recognizes_short_h_try_parse_errors() {
        assert!(parse_opts().try_get_matches_from(vec!["tp", "-h"]).is_err());
    }

    #[test]
    fn recognizes_pop_shift_unshift_no_duplicate_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--pop", "--shift", "--unshift"]);
        assert!(m.get_flag(POP));
        assert!(m.get_flag(SHIFT));
        assert!(m.get_flag(UNSHIFT));
    }

    #[test]
    fn recognizes_list_numbered_contents_flags() {
        let m = parse_opts().get_matches_from(vec!["tp", "-n", "-N"]);
        assert!(m.get_flag(LIST_FILES_NUMBERED));
        assert!(m.get_flag(LIST_CONTENTS_NUMBERED));
    }

    #[test]
    fn recognizes_clear_verbose_quiet() {
        let m = parse_opts().get_matches_from(vec!["tp", "-c", "-v", "-q"]);
        assert!(m.get_flag(CLEAR));
        assert_eq!(m.get_count(VERBOSE), 1);
        assert!(m.get_flag(SILENT));
    }

    #[test]
    fn recognizes_rev_sort_mtime_equals() {
        let m = parse_opts().get_matches_from(vec!["tp", "--rev", "--sort=mtime"]);
        assert!(m.get_flag(REVERSE));
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("mtime"));
    }

    #[test]
    fn recognizes_edit_info_path_triple_equals() {
        let m = parse_opts().get_matches_from(vec!["tp", "--edit=2", "--info=2", "--path=2"]);
        assert_eq!(m.get_one::<String>(EDIT).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(INFO).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("2"));
    }

    #[test]
    fn recognizes_cat_seven_indices() {
        let m =
            parse_opts().get_matches_from(vec!["tp", "--cat", "1", "2", "3", "4", "5", "6", "7"]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(v, vec!["1", "2", "3", "4", "5", "6", "7"]);
    }

    #[test]
    fn recognizes_rename_long_form_two_args() {
        let m = parse_opts().get_matches_from(vec!["tp", "--rename", "from", "to"]);
        assert_eq!(
            m.get_many::<String>(RENAME)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["from", "to"]
        );
    }

    #[test]
    fn recognizes_move_long_two_args() {
        let m = parse_opts().get_matches_from(vec!["tp", "--mv", "9", "1"]);
        assert_eq!(
            m.get_many::<String>(MOVE)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["9", "1"]
        );
    }

    #[test]
    fn recognizes_swap_long_two_args() {
        let m = parse_opts().get_matches_from(vec!["tp", "--swap", "9", "1"]);
        assert_eq!(
            m.get_many::<String>(SWAP)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["9", "1"]
        );
    }

    #[test]
    fn recognizes_diff_long_two_args() {
        let m = parse_opts().get_matches_from(vec!["tp", "--diff", "9", "1"]);
        assert_eq!(
            m.get_many::<String>(DIFF)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["9", "1"]
        );
    }

    #[test]
    fn recognizes_replace_long_three_args() {
        let m = parse_opts().get_matches_from(vec!["tp", "--replace", "9", "x", "y"]);
        assert_eq!(
            m.get_many::<String>(REPLACE)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["9", "x", "y"]
        );
    }

    #[test]
    fn recognizes_verbose_eight_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-vvvvvvvv"]);
        assert_eq!(m.get_count(VERBOSE), 8);
    }

    #[test]
    fn recognizes_add_remove_zero_strings() {
        let m = parse_opts().get_matches_from(vec!["tp", "-a", "0", "-r", "0"]);
        assert_eq!(m.get_one::<String>(ADD).map(|s| s.as_str()), Some("0"));
        assert_eq!(m.get_one::<String>(REMOVE).map(|s| s.as_str()), Some("0"));
    }

    #[test]
    fn recognizes_output_equals_only_minus() {
        let m = parse_opts().get_matches_from(vec!["tp", "--output=-"]);
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("-"));
    }

    // ── clap coverage round 3 ────────────────────────────

    #[test]
    fn recognizes_edit_name_grep_chain_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-e", "1", "-w", "tag", "-g", "pat"]);
        assert_eq!(m.get_one::<String>(EDIT).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(TAG).map(|s| s.as_str()), Some("tag"));
        assert_eq!(m.get_one::<String>(GREP).map(|s| s.as_str()), Some("pat"));
    }

    #[test]
    fn recognizes_pop_clear_order_independent() {
        let a = parse_opts().get_matches_from(vec!["tp", "-p", "-c"]);
        let b = parse_opts().get_matches_from(vec!["tp", "-c", "-p"]);
        assert_eq!(a.get_flag(POP), b.get_flag(POP));
        assert_eq!(a.get_flag(CLEAR), b.get_flag(CLEAR));
    }

    #[test]
    fn recognizes_shift_unshift_order_independent() {
        let a = parse_opts().get_matches_from(vec!["tp", "-s", "-u"]);
        let b = parse_opts().get_matches_from(vec!["tp", "-u", "-s"]);
        assert_eq!(a.get_flag(SHIFT), b.get_flag(SHIFT));
        assert_eq!(a.get_flag(UNSHIFT), b.get_flag(UNSHIFT));
    }

    #[test]
    fn recognizes_all_four_list_flags_order_independent() {
        let a = parse_opts().get_matches_from(vec!["tp", "-l", "-n", "-L", "-N"]);
        let b = parse_opts().get_matches_from(vec!["tp", "-N", "-L", "-n", "-l"]);
        assert_eq!(a.get_flag(LIST_FILES), b.get_flag(LIST_FILES));
        assert_eq!(
            a.get_flag(LIST_FILES_NUMBERED),
            b.get_flag(LIST_FILES_NUMBERED)
        );
        assert_eq!(a.get_flag(LIST_CONTENTS), b.get_flag(LIST_CONTENTS));
        assert_eq!(
            a.get_flag(LIST_CONTENTS_NUMBERED),
            b.get_flag(LIST_CONTENTS_NUMBERED)
        );
    }

    #[test]
    fn recognizes_expire_fractional_string() {
        let m = parse_opts().get_matches_from(vec!["tp", "--expire", "0.5"]);
        assert_eq!(m.get_one::<String>(EXPIRE).map(|s| s.as_str()), Some("0.5"));
    }

    #[test]
    fn recognizes_sort_size_explicit_equals() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort=size"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("size"));
    }

    #[test]
    fn recognizes_input_short_equals_via_space() {
        let m = parse_opts().get_matches_from(vec!["tp", "-i", "42"]);
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("42"));
    }

    #[test]
    fn recognizes_remove_long_positive() {
        let m = parse_opts().get_matches_from(vec!["tp", "--remove", "7"]);
        assert_eq!(m.get_one::<String>(REMOVE).map(|s| s.as_str()), Some("7"));
    }

    #[test]
    fn recognizes_add_short_positive() {
        let m = parse_opts().get_matches_from(vec!["tp", "-a", "6"]);
        assert_eq!(m.get_one::<String>(ADD).map(|s| s.as_str()), Some("6"));
    }

    #[test]
    fn recognizes_dup_short_equals_via_space() {
        let m = parse_opts().get_matches_from(vec!["tp", "-x", "9"]);
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("9"));
    }

    #[test]
    fn recognizes_append_short_equals_via_space() {
        let m = parse_opts().get_matches_from(vec!["tp", "-A", "8"]);
        assert_eq!(m.get_one::<String>(APPEND).map(|s| s.as_str()), Some("8"));
    }

    #[test]
    fn recognizes_info_short_capital_i() {
        let m = parse_opts().get_matches_from(vec!["tp", "-I", "-3"]);
        assert_eq!(m.get_one::<String>(INFO).map(|s| s.as_str()), Some("-3"));
    }

    #[test]
    fn recognizes_edit_short_lowercase_e() {
        let m = parse_opts().get_matches_from(vec!["tp", "-e", "100"]);
        assert_eq!(m.get_one::<String>(EDIT).map(|s| s.as_str()), Some("100"));
    }

    #[test]
    fn recognizes_count_short_k() {
        let m = parse_opts().get_matches_from(vec!["tp", "-k"]);
        assert!(m.get_flag(COUNT));
    }

    #[test]
    fn recognizes_reverse_only_long_rev() {
        let m = parse_opts().get_matches_from(vec!["tp", "--rev"]);
        assert!(m.get_flag(REVERSE));
    }

    #[test]
    fn recognizes_positional_only_dot() {
        let m = parse_opts().get_matches_from(vec!["tp", ".."]);
        assert_eq!(m.get_one::<String>(ARGFILE).map(|s| s.as_str()), Some(".."));
    }

    #[test]
    fn recognizes_positional_single_slash() {
        let m = parse_opts().get_matches_from(vec!["tp", "/"]);
        assert_eq!(m.get_one::<String>(ARGFILE).map(|s| s.as_str()), Some("/"));
    }

    #[test]
    fn recognizes_cat_two_indices_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-C", "2", "4"]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(v, vec!["2", "4"]);
    }

    #[test]
    fn recognizes_head_tail_minimal() {
        let m = parse_opts().get_matches_from(vec!["tp", "--head", "1", "1", "--tail", "1", "1"]);
        assert_eq!(
            m.get_many::<String>(HEAD)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["1", "1"]
        );
        assert_eq!(
            m.get_many::<String>(TAIL)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["1", "1"]
        );
    }

    #[test]
    fn recognizes_verbose_long_only_twice() {
        let m = parse_opts().get_matches_from(vec!["tp", "--verbose", "--verbose"]);
        assert_eq!(m.get_count(VERBOSE), 2);
    }

    #[test]
    fn recognizes_quiet_long_twice_try_parse_errors() {
        assert!(
            parse_opts()
                .try_get_matches_from(vec!["tp", "--quiet", "--quiet"])
                .is_err()
        );
    }

    #[test]
    fn recognizes_directory_master_both_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--dir", "--master"]);
        assert!(m.get_flag(DIRECTORY));
        assert!(m.get_flag(MASTER));
    }

    #[test]
    fn recognizes_list_files_both_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--list-files"]);
        assert!(m.get_flag(LIST_FILES));
    }

    #[test]
    fn recognizes_list_files_numbered_both_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--list-files-numbered"]);
        assert!(m.get_flag(LIST_FILES_NUMBERED));
    }

    #[test]
    fn recognizes_list_contents_both_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--list-contents"]);
        assert!(m.get_flag(LIST_CONTENTS));
    }

    #[test]
    fn recognizes_list_contents_numbered_both_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--list-contents-numbered"]);
        assert!(m.get_flag(LIST_CONTENTS_NUMBERED));
    }

    #[test]
    fn recognizes_clear_both_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--clear"]);
        assert!(m.get_flag(CLEAR));
    }

    #[test]
    fn recognizes_input_output_append_triple_equals() {
        let m = parse_opts().get_matches_from(vec!["tp", "--input=1", "--output=2", "--append=3"]);
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(APPEND).map(|s| s.as_str()), Some("3"));
    }

    // ── clap coverage round 4 ────────────────────────────

    #[test]
    fn recognizes_vvvv_short_verbose_count_four() {
        let m = parse_opts().get_matches_from(vec!["tp", "-vvvv"]);
        assert_eq!(m.get_count(VERBOSE), 4);
    }

    #[test]
    fn recognizes_dup_equals_negative_index() {
        let m = parse_opts().get_matches_from(vec!["tp", "--dup=-1"]);
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("-1"));
    }

    #[test]
    fn recognizes_dup_short_negative_index() {
        let m = parse_opts().get_matches_from(vec!["tp", "-x", "-2"]);
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("-2"));
    }

    #[test]
    fn recognizes_append_equals_negative_index() {
        let m = parse_opts().get_matches_from(vec!["tp", "--append=-3"]);
        assert_eq!(m.get_one::<String>(APPEND).map(|s| s.as_str()), Some("-3"));
    }

    #[test]
    fn recognizes_input_output_append_short_chain() {
        let m = parse_opts().get_matches_from(vec!["tp", "-i", "2", "-o", "1", "-A", "-1"]);
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(APPEND).map(|s| s.as_str()), Some("-1"));
    }

    #[test]
    fn recognizes_count_grep_quiet_parse() {
        let m = parse_opts().get_matches_from(vec!["tp", "-k", "-g", "needle", "-q"]);
        assert!(m.get_flag(COUNT));
        assert_eq!(
            m.get_one::<String>(GREP).map(|s| s.as_str()),
            Some("needle")
        );
        assert!(m.get_flag(SILENT));
    }

    #[test]
    fn recognizes_cat_five_indices_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--cat", "1", "2", "3", "4", "5"]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(v, vec!["1", "2", "3", "4", "5"]);
    }

    #[test]
    fn recognizes_rename_short_two_args() {
        let m = parse_opts().get_matches_from(vec!["tp", "-R", "old", "new"]);
        let v: Vec<String> = m.get_many(RENAME).unwrap().cloned().collect();
        assert_eq!(v, vec!["old", "new"]);
    }

    #[test]
    fn recognizes_diff_short_negative_pair() {
        let m = parse_opts().get_matches_from(vec!["tp", "-D", "-1", "-2"]);
        let v: Vec<String> = m.get_many(DIFF).unwrap().cloned().collect();
        assert_eq!(v, vec!["-1", "-2"]);
    }

    #[test]
    fn recognizes_swap_short_negative_pair() {
        let m = parse_opts().get_matches_from(vec!["tp", "-S", "-2", "-1"]);
        let v: Vec<String> = m.get_many(SWAP).unwrap().cloned().collect();
        assert_eq!(v, vec!["-2", "-1"]);
    }

    #[test]
    fn recognizes_mv_short_two_args() {
        let m = parse_opts().get_matches_from(vec!["tp", "-M", "2", "4"]);
        let v: Vec<String> = m.get_many(MOVE).unwrap().cloned().collect();
        assert_eq!(v, vec!["2", "4"]);
    }

    #[test]
    fn recognizes_wc_equals_negative_one() {
        let m = parse_opts().get_matches_from(vec!["tp", "--wc=-1"]);
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("-1"));
    }

    #[test]
    fn recognizes_size_equals_negative_one() {
        let m = parse_opts().get_matches_from(vec!["tp", "--size=-1"]);
        assert_eq!(m.get_one::<String>(SIZE).map(|s| s.as_str()), Some("-1"));
    }

    #[test]
    fn recognizes_path_equals_negative_two() {
        let m = parse_opts().get_matches_from(vec!["tp", "--path=-2"]);
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("-2"));
    }

    #[test]
    fn recognizes_head_tail_negative_index_first_arg() {
        let m =
            parse_opts().get_matches_from(vec!["tp", "--head", "-1", "10", "--tail", "-2", "3"]);
        let h: Vec<String> = m.get_many(HEAD).unwrap().cloned().collect();
        let t: Vec<String> = m.get_many(TAIL).unwrap().cloned().collect();
        assert_eq!(h, vec!["-1", "10"]);
        assert_eq!(t, vec!["-2", "3"]);
    }

    #[test]
    fn recognizes_expire_zero_equals() {
        let m = parse_opts().get_matches_from(vec!["tp", "--expire=0"]);
        assert_eq!(m.get_one::<String>(EXPIRE).map(|s| s.as_str()), Some("0"));
    }

    #[test]
    fn recognizes_expire_large_hours() {
        let m = parse_opts().get_matches_from(vec!["tp", "--expire", "8760"]);
        assert_eq!(
            m.get_one::<String>(EXPIRE).map(|s| s.as_str()),
            Some("8760")
        );
    }

    #[test]
    fn recognizes_positional_with_multiple_flags() {
        let m = parse_opts().get_matches_from(vec!["tp", "-q", "-c", "myfile.txt"]);
        assert!(m.get_flag(SILENT));
        assert!(m.get_flag(CLEAR));
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("myfile.txt")
        );
    }

    #[test]
    fn recognizes_list_rev_count_parse() {
        let m = parse_opts().get_matches_from(vec!["tp", "-l", "--rev", "-k"]);
        assert!(m.get_flag(LIST_FILES));
        assert!(m.get_flag(REVERSE));
        assert!(m.get_flag(COUNT));
    }

    #[test]
    fn recognizes_pop_unshift_shift_all_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-p", "-u", "-s"]);
        assert!(m.get_flag(POP));
        assert!(m.get_flag(UNSHIFT));
        assert!(m.get_flag(SHIFT));
    }

    #[test]
    fn recognizes_edit_info_path_chain_long() {
        let m =
            parse_opts().get_matches_from(vec!["tp", "--edit", "2", "--info", "1", "--path", "3"]);
        assert_eq!(m.get_one::<String>(EDIT).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(INFO).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("3"));
    }

    #[test]
    fn recognizes_add_remove_same_argv_parse() {
        let m = parse_opts().get_matches_from(vec!["tp", "-a", "1", "-r", "2"]);
        assert_eq!(m.get_one::<String>(ADD).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(REMOVE).map(|s| s.as_str()), Some("2"));
    }

    #[test]
    fn recognizes_name_tag_with_output() {
        let m = parse_opts().get_matches_from(vec!["tp", "-w", "tag", "-o", "1"]);
        assert_eq!(m.get_one::<String>(TAG).map(|s| s.as_str()), Some("tag"));
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_grep_pattern_double_dash_prefix_via_equals() {
        let m = parse_opts().get_matches_from(vec!["tp", "--grep=--verbose"]);
        assert_eq!(
            m.get_one::<String>(GREP).map(|s| s.as_str()),
            Some("--verbose")
        );
    }

    #[test]
    fn recognizes_verbose_mixed_short_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "-v", "--verbose", "-v"]);
        assert_eq!(m.get_count(VERBOSE), 3);
    }

    #[test]
    fn recognizes_clear_master_dir_parse() {
        let m = parse_opts().get_matches_from(vec!["tp", "-c", "-m", "-d"]);
        assert!(m.get_flag(CLEAR));
        assert!(m.get_flag(MASTER));
        assert!(m.get_flag(DIRECTORY));
    }

    // ── clap coverage round 5 ──────────────────────────────

    #[test]
    fn recognizes_vvvvv_short_verbose_count_five() {
        let m = parse_opts().get_matches_from(vec!["tp", "-vvvvv"]);
        assert_eq!(m.get_count(VERBOSE), 5);
    }

    #[test]
    fn recognizes_input_output_both_equals_adjacent() {
        let m = parse_opts().get_matches_from(vec!["tp", "--input=9", "--output=8"]);
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("9"));
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("8"));
    }

    #[test]
    fn recognizes_remove_equals_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--remove=-1"]);
        assert_eq!(m.get_one::<String>(REMOVE).map(|s| s.as_str()), Some("-1"));
    }

    #[test]
    fn recognizes_add_equals_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--add=3"]);
        assert_eq!(m.get_one::<String>(ADD).map(|s| s.as_str()), Some("3"));
    }

    #[test]
    fn recognizes_edit_equals_negative() {
        let m = parse_opts().get_matches_from(vec!["tp", "--edit=-1"]);
        assert_eq!(m.get_one::<String>(EDIT).map(|s| s.as_str()), Some("-1"));
    }

    #[test]
    fn recognizes_info_equals_negative() {
        let m = parse_opts().get_matches_from(vec!["tp", "--info=-2"]);
        assert_eq!(m.get_one::<String>(INFO).map(|s| s.as_str()), Some("-2"));
    }

    #[test]
    fn recognizes_dup_short_equals_via_equals() {
        let m = parse_opts().get_matches_from(vec!["tp", "-x=5"]);
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("5"));
    }

    #[test]
    fn recognizes_swap_long_space_args_negative() {
        let m = parse_opts().get_matches_from(vec!["tp", "--swap", "-1", "-2"]);
        let v: Vec<String> = m.get_many(SWAP).unwrap().cloned().collect();
        assert_eq!(v, vec!["-1", "-2"]);
    }

    #[test]
    fn recognizes_mv_long_two_args() {
        let m = parse_opts().get_matches_from(vec!["tp", "--mv", "1", "9"]);
        let v: Vec<String> = m.get_many(MOVE).unwrap().cloned().collect();
        assert_eq!(v, vec!["1", "9"]);
    }

    #[test]
    fn recognizes_replace_all_space_separated() {
        let m = parse_opts().get_matches_from(vec!["tp", "--replace", "2", "a", "b"]);
        let v: Vec<String> = m.get_many::<String>(REPLACE).unwrap().cloned().collect();
        assert_eq!(v, vec!["2", "a", "b"]);
    }

    #[test]
    fn recognizes_cat_six_indices_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--cat", "1", "2", "3", "4", "5", "6"]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(v, vec!["1", "2", "3", "4", "5", "6"]);
    }

    #[test]
    fn recognizes_sort_long_only_default_via_flag() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("name"));
    }

    #[test]
    fn recognizes_expire_decimal_string() {
        let m = parse_opts().get_matches_from(vec!["tp", "--expire", "0.5"]);
        assert_eq!(m.get_one::<String>(EXPIRE).map(|s| s.as_str()), Some("0.5"));
    }

    #[test]
    fn recognizes_count_long_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "--count"]);
        assert!(m.get_flag(COUNT));
    }

    #[test]
    fn recognizes_reverse_long_only_rev() {
        let m = parse_opts().get_matches_from(vec!["tp", "--rev"]);
        assert!(m.get_flag(REVERSE));
    }

    #[test]
    fn recognizes_list_files_short_l_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "-l"]);
        assert!(m.get_flag(LIST_FILES));
    }

    #[test]
    fn recognizes_list_contents_short_cap_l_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "-L"]);
        assert!(m.get_flag(LIST_CONTENTS));
    }

    #[test]
    fn recognizes_master_short_m() {
        let m = parse_opts().get_matches_from(vec!["tp", "-m"]);
        assert!(m.get_flag(MASTER));
    }

    #[test]
    fn recognizes_directory_short_d() {
        let m = parse_opts().get_matches_from(vec!["tp", "-d"]);
        assert!(m.get_flag(DIRECTORY));
    }

    #[test]
    fn recognizes_clear_short_c() {
        let m = parse_opts().get_matches_from(vec!["tp", "-c"]);
        assert!(m.get_flag(CLEAR));
    }

    #[test]
    fn recognizes_pop_short_p() {
        let m = parse_opts().get_matches_from(vec!["tp", "-p"]);
        assert!(m.get_flag(POP));
    }

    #[test]
    fn recognizes_shift_short_s() {
        let m = parse_opts().get_matches_from(vec!["tp", "-s"]);
        assert!(m.get_flag(SHIFT));
    }

    #[test]
    fn recognizes_unshift_short_u() {
        let m = parse_opts().get_matches_from(vec!["tp", "-u"]);
        assert!(m.get_flag(UNSHIFT));
    }

    #[test]
    fn recognizes_quiet_short_q() {
        let m = parse_opts().get_matches_from(vec!["tp", "-q"]);
        assert!(m.get_flag(SILENT));
    }

    #[test]
    fn recognizes_verbose_short_v_single() {
        let m = parse_opts().get_matches_from(vec!["tp", "-v"]);
        assert_eq!(m.get_count(VERBOSE), 1);
    }

    #[test]
    fn recognizes_diff_short_d_uppercase() {
        let m = parse_opts().get_matches_from(vec!["tp", "-D", "1", "2"]);
        let v: Vec<String> = m.get_many(DIFF).unwrap().cloned().collect();
        assert_eq!(v, vec!["1", "2"]);
    }

    #[test]
    fn recognizes_swap_short_s_uppercase() {
        let m = parse_opts().get_matches_from(vec!["tp", "-S", "1", "2"]);
        let v: Vec<String> = m.get_many(SWAP).unwrap().cloned().collect();
        assert_eq!(v, vec!["1", "2"]);
    }

    #[test]
    fn recognizes_mv_short_m_uppercase() {
        let m = parse_opts().get_matches_from(vec!["tp", "-M", "1", "2"]);
        let v: Vec<String> = m.get_many(MOVE).unwrap().cloned().collect();
        assert_eq!(v, vec!["1", "2"]);
    }

    #[test]
    fn recognizes_dup_short_x() {
        let m = parse_opts().get_matches_from(vec!["tp", "-x", "1"]);
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_append_short_a_uppercase() {
        let m = parse_opts().get_matches_from(vec!["tp", "-A", "1"]);
        assert_eq!(m.get_one::<String>(APPEND).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_rename_short_r_uppercase() {
        let m = parse_opts().get_matches_from(vec!["tp", "-R", "a", "b"]);
        let v: Vec<String> = m.get_many(RENAME).unwrap().cloned().collect();
        assert_eq!(v, vec!["a", "b"]);
    }

    #[test]
    fn recognizes_info_short_i_uppercase() {
        let m = parse_opts().get_matches_from(vec!["tp", "-I", "1"]);
        assert_eq!(m.get_one::<String>(INFO).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_grep_short_g() {
        let m = parse_opts().get_matches_from(vec!["tp", "-g", "x"]);
        assert_eq!(m.get_one::<String>(GREP).map(|s| s.as_str()), Some("x"));
    }

    #[test]
    fn recognizes_cat_short_c_uppercase() {
        let m = parse_opts().get_matches_from(vec!["tp", "-C", "1", "2"]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(v, vec!["1", "2"]);
    }

    #[test]
    fn recognizes_diff_short_d_uppercase_two_names() {
        let m = parse_opts().get_matches_from(vec!["tp", "-D", "a", "b"]);
        let v: Vec<String> = m.get_many(DIFF).unwrap().cloned().collect();
        assert_eq!(v, vec!["a", "b"]);
    }

    // ── clap coverage round 6 ──────────────────────────────

    #[test]
    fn recognizes_positional_equals_sign_in_filename() {
        let m = parse_opts().get_matches_from(vec!["tp", "a=b.txt"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("a=b.txt")
        );
    }

    #[test]
    fn recognizes_positional_at_sign_prefix() {
        let m = parse_opts().get_matches_from(vec!["tp", "@config"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("@config")
        );
    }

    #[test]
    fn recognizes_input_equals_embedded_plus() {
        let m = parse_opts().get_matches_from(vec!["tp", "--input=+1"]);
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("+1"));
    }

    #[test]
    fn recognizes_output_equals_leading_zeros() {
        let m = parse_opts().get_matches_from(vec!["tp", "--output=007"]);
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("007"));
    }

    #[test]
    fn recognizes_append_long_space_negative_three() {
        let m = parse_opts().get_matches_from(vec!["tp", "--append", "-3"]);
        assert_eq!(m.get_one::<String>(APPEND).map(|s| s.as_str()), Some("-3"));
    }

    #[test]
    fn recognizes_remove_long_space_max_i32_string() {
        let m = parse_opts().get_matches_from(vec!["tp", "--remove", "2147483647"]);
        assert_eq!(
            m.get_one::<String>(REMOVE).map(|s| s.as_str()),
            Some("2147483647")
        );
    }

    #[test]
    fn recognizes_edit_long_space_name_like() {
        let m = parse_opts().get_matches_from(vec!["tp", "--edit", "my-tag"]);
        assert_eq!(
            m.get_one::<String>(EDIT).map(|s| s.as_str()),
            Some("my-tag")
        );
    }

    #[test]
    fn recognizes_grep_equals_empty_via_space_not_equals() {
        let m = parse_opts().get_matches_from(vec!["tp", "--grep", ""]);
        assert_eq!(m.get_one::<String>(GREP).map(|s| s.as_str()), Some(""));
    }

    #[test]
    fn recognizes_head_long_two_numeric_strings() {
        let m = parse_opts().get_matches_from(vec!["tp", "--head", "3", "100"]);
        let v: Vec<String> = m.get_many(HEAD).unwrap().cloned().collect();
        assert_eq!(v, vec!["3", "100"]);
    }

    #[test]
    fn recognizes_tail_long_two_numeric_strings() {
        let m = parse_opts().get_matches_from(vec!["tp", "--tail", "2", "50"]);
        let v: Vec<String> = m.get_many(TAIL).unwrap().cloned().collect();
        assert_eq!(v, vec!["2", "50"]);
    }

    #[test]
    fn recognizes_replace_pattern_with_slash() {
        let m = parse_opts().get_matches_from(vec!["tp", "--replace", "1", "a/b", "c"]);
        let v: Vec<String> = m.get_many::<String>(REPLACE).unwrap().cloned().collect();
        assert_eq!(v, vec!["1", "a/b", "c"]);
    }

    #[test]
    fn recognizes_mv_from_to_zero_strings() {
        let m = parse_opts().get_matches_from(vec!["tp", "--mv", "0", "0"]);
        let v: Vec<String> = m.get_many(MOVE).unwrap().cloned().collect();
        assert_eq!(v, vec!["0", "0"]);
    }

    #[test]
    fn recognizes_swap_both_zero() {
        let m = parse_opts().get_matches_from(vec!["tp", "--swap", "0", "0"]);
        let v: Vec<String> = m.get_many(SWAP).unwrap().cloned().collect();
        assert_eq!(v, vec!["0", "0"]);
    }

    #[test]
    fn recognizes_diff_both_zero() {
        let m = parse_opts().get_matches_from(vec!["tp", "--diff", "0", "0"]);
        let v: Vec<String> = m.get_many(DIFF).unwrap().cloned().collect();
        assert_eq!(v, vec!["0", "0"]);
    }

    #[test]
    fn recognizes_rename_unicode_tags_two_args() {
        let m = parse_opts().get_matches_from(vec!["tp", "--rename", "旧", "新"]);
        let v: Vec<String> = m.get_many(RENAME).unwrap().cloned().collect();
        assert_eq!(v, vec!["旧", "新"]);
    }

    #[test]
    fn recognizes_sort_mtime_space() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort", "mtime"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("mtime"));
    }

    #[test]
    fn recognizes_expire_negative_string_equals_form() {
        let m = parse_opts().get_matches_from(vec!["tp", "--expire=-1"]);
        assert_eq!(m.get_one::<String>(EXPIRE).map(|s| s.as_str()), Some("-1"));
    }

    #[test]
    fn recognizes_wc_path_size_same_line_three_flags() {
        let m =
            parse_opts().get_matches_from(vec!["tp", "--wc", "1", "--path", "1", "--size", "1"]);
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(SIZE).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_pop_shift_order_reversed_argv() {
        let m = parse_opts().get_matches_from(vec!["tp", "-s", "-p"]);
        assert!(m.get_flag(SHIFT));
        assert!(m.get_flag(POP));
    }

    #[test]
    fn recognizes_list_ln_ln_flags() {
        let m = parse_opts().get_matches_from(vec!["tp", "-l", "-n", "-L", "-N"]);
        assert!(m.get_flag(LIST_FILES));
        assert!(m.get_flag(LIST_FILES_NUMBERED));
        assert!(m.get_flag(LIST_CONTENTS));
        assert!(m.get_flag(LIST_CONTENTS_NUMBERED));
    }

    #[test]
    fn recognizes_verbose_five_short_mixed_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "-vv", "--verbose", "-vv"]);
        assert_eq!(m.get_count(VERBOSE), 5);
    }

    #[test]
    fn recognizes_quiet_with_expire_and_clear() {
        let m = parse_opts().get_matches_from(vec!["tp", "-q", "--expire", "24", "-c"]);
        assert!(m.get_flag(SILENT));
        assert_eq!(m.get_one::<String>(EXPIRE).map(|s| s.as_str()), Some("24"));
        assert!(m.get_flag(CLEAR));
    }

    #[test]
    fn recognizes_input_output_only_no_positional() {
        let m = parse_opts().get_matches_from(vec!["tp", "-i", "1", "-o", "2"]);
        assert!(m.get_one::<String>(ARGFILE).is_none());
    }

    #[test]
    fn recognizes_positional_trailing_after_all_flags() {
        let m = parse_opts().get_matches_from(vec!["tp", "-v", "-q", "readme.md"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("readme.md")
        );
    }

    #[test]
    fn recognizes_cat_eight_indices() {
        let m = parse_opts()
            .get_matches_from(vec!["tp", "--cat", "1", "2", "3", "4", "5", "6", "7", "8"]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(v, vec!["1", "2", "3", "4", "5", "6", "7", "8"]);
    }

    #[test]
    fn recognizes_dup_cat_chain() {
        let m = parse_opts().get_matches_from(vec!["tp", "-x", "2", "-C", "1", "2"]);
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("2"));
        let c: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(c, vec!["1", "2"]);
    }

    #[test]
    fn recognizes_reverse_sort_name_order() {
        let m = parse_opts().get_matches_from(vec!["tp", "--rev", "--sort", "name"]);
        assert!(m.get_flag(REVERSE));
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("name"));
    }

    // ── clap coverage round 7 ──────────────────────────────

    #[test]
    fn recognizes_positional_colon_path() {
        let m = parse_opts().get_matches_from(vec!["tp", "C:\\temp\\file.txt"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("C:\\temp\\file.txt")
        );
    }

    #[test]
    fn recognizes_input_long_space_tab_index() {
        let m = parse_opts().get_matches_from(vec!["tp", "--input", "1"]);
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_output_short_space_negative_five() {
        let m = parse_opts().get_matches_from(vec!["tp", "-o", "-5"]);
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("-5"));
    }

    #[test]
    fn recognizes_add_short_space_plus_one() {
        let m = parse_opts().get_matches_from(vec!["tp", "-a", "+1"]);
        assert_eq!(m.get_one::<String>(ADD).map(|s| s.as_str()), Some("+1"));
    }

    #[test]
    fn recognizes_remove_short_r_space() {
        let m = parse_opts().get_matches_from(vec!["tp", "-r", "3"]);
        assert_eq!(m.get_one::<String>(REMOVE).map(|s| s.as_str()), Some("3"));
    }

    #[test]
    fn recognizes_name_w_short_equals() {
        let m = parse_opts().get_matches_from(vec!["tp", "-w=my-tag"]);
        assert_eq!(m.get_one::<String>(TAG).map(|s| s.as_str()), Some("my-tag"));
    }

    #[test]
    fn recognizes_edit_e_short_equals_index() {
        let m = parse_opts().get_matches_from(vec!["tp", "-e=2"]);
        assert_eq!(m.get_one::<String>(EDIT).map(|s| s.as_str()), Some("2"));
    }

    #[test]
    fn recognizes_info_long_space_numeric() {
        let m = parse_opts().get_matches_from(vec!["tp", "--info", "42"]);
        assert_eq!(m.get_one::<String>(INFO).map(|s| s.as_str()), Some("42"));
    }

    #[test]
    fn recognizes_grep_long_space_regex_or() {
        let m = parse_opts().get_matches_from(vec!["tp", "--grep", "a|b"]);
        assert_eq!(m.get_one::<String>(GREP).map(|s| s.as_str()), Some("a|b"));
    }

    #[test]
    fn recognizes_cat_ten_indices() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--cat", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10",
        ]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(v, vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"]);
    }

    #[test]
    fn recognizes_replace_spaces_in_replacement() {
        let m = parse_opts().get_matches_from(vec!["tp", "--replace", "1", "x", "y z"]);
        let v: Vec<String> = m.get_many::<String>(REPLACE).unwrap().cloned().collect();
        assert_eq!(v, vec!["1", "x", "y z"]);
    }

    #[test]
    fn recognizes_expire_equals_fraction() {
        let m = parse_opts().get_matches_from(vec!["tp", "--expire=0.25"]);
        assert_eq!(
            m.get_one::<String>(EXPIRE).map(|s| s.as_str()),
            Some("0.25")
        );
    }

    #[test]
    fn recognizes_sort_size_explicit_space() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort", "size"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("size"));
    }

    #[test]
    fn recognizes_pop_clear_unshift_short_chain() {
        let m = parse_opts().get_matches_from(vec!["tp", "-p", "-c", "-u"]);
        assert!(m.get_flag(POP));
        assert!(m.get_flag(CLEAR));
        assert!(m.get_flag(UNSHIFT));
    }

    #[test]
    fn recognizes_master_dir_quiet() {
        let m = parse_opts().get_matches_from(vec!["tp", "-m", "-d", "-q"]);
        assert!(m.get_flag(MASTER));
        assert!(m.get_flag(DIRECTORY));
        assert!(m.get_flag(SILENT));
    }

    #[test]
    fn recognizes_verbose_short_seven() {
        let m = parse_opts().get_matches_from(vec!["tp", "-vvvvvvv"]);
        assert_eq!(m.get_count(VERBOSE), 7);
    }

    #[test]
    fn recognizes_input_output_verbose_quiet() {
        let m = parse_opts().get_matches_from(vec!["tp", "-i", "1", "-o", "2", "-v", "-q"]);
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_count(VERBOSE), 1);
        assert!(m.get_flag(SILENT));
    }

    #[test]
    fn recognizes_append_dup_same_stack_index() {
        let m = parse_opts().get_matches_from(vec!["tp", "-A", "1", "-x", "1"]);
        assert_eq!(m.get_one::<String>(APPEND).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_path_wc_size_triple_equals() {
        let m = parse_opts().get_matches_from(vec!["tp", "--path=3", "--wc=3", "--size=3"]);
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("3"));
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("3"));
        assert_eq!(m.get_one::<String>(SIZE).map(|s| s.as_str()), Some("3"));
    }

    #[test]
    fn recognizes_double_dash_only_positional_still() {
        let m = parse_opts().get_matches_from(vec!["tp", "--", "--not-a-flag"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("--not-a-flag")
        );
    }

    // ── clap coverage round 8 ──────────────────────────────

    #[test]
    fn recognizes_positional_query_string_looking() {
        let m = parse_opts().get_matches_from(vec!["tp", "file.txt?raw=1"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("file.txt?raw=1")
        );
    }

    #[test]
    fn recognizes_input_long_equals_tab_character_value() {
        let m = parse_opts().get_matches_from(vec!["tp", "--input=\t1"]);
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("\t1"));
    }

    #[test]
    fn recognizes_output_short_space_zero_string() {
        let m = parse_opts().get_matches_from(vec!["tp", "-o", "0"]);
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("0"));
    }

    #[test]
    fn recognizes_add_long_equals_negative_twelve() {
        let m = parse_opts().get_matches_from(vec!["tp", "--add=-12"]);
        assert_eq!(m.get_one::<String>(ADD).map(|s| s.as_str()), Some("-12"));
    }

    #[test]
    fn recognizes_remove_short_r_equals_via_space() {
        let m = parse_opts().get_matches_from(vec!["tp", "-r", "0"]);
        assert_eq!(m.get_one::<String>(REMOVE).map(|s| s.as_str()), Some("0"));
    }

    #[test]
    fn recognizes_tag_long_equals_unicode() {
        let m = parse_opts().get_matches_from(vec!["tp", "--name=标签"]);
        assert_eq!(m.get_one::<String>(TAG).map(|s| s.as_str()), Some("标签"));
    }

    #[test]
    fn recognizes_grep_equals_dot_star() {
        let m = parse_opts().get_matches_from(vec!["tp", "--grep=.*"]);
        assert_eq!(m.get_one::<String>(GREP).map(|s| s.as_str()), Some(".*"));
    }

    #[test]
    fn recognizes_cat_eleven_indices_long() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--cat", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11",
        ]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(
            v,
            vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11"]
        );
    }

    #[test]
    fn recognizes_replace_three_unicode_args_space() {
        let m = parse_opts().get_matches_from(vec!["tp", "--replace", "1", "α", "β"]);
        let v: Vec<String> = m.get_many::<String>(REPLACE).unwrap().cloned().collect();
        assert_eq!(v, vec!["1", "α", "β"]);
    }

    #[test]
    fn recognizes_mv_long_from_to_same_numeric() {
        let m = parse_opts().get_matches_from(vec!["tp", "--mv", "3", "3"]);
        let v: Vec<String> = m.get_many(MOVE).unwrap().cloned().collect();
        assert_eq!(v, vec!["3", "3"]);
    }

    #[test]
    fn recognizes_swap_long_identical_indices() {
        let m = parse_opts().get_matches_from(vec!["tp", "--swap", "2", "2"]);
        let v: Vec<String> = m.get_many(SWAP).unwrap().cloned().collect();
        assert_eq!(v, vec!["2", "2"]);
    }

    #[test]
    fn recognizes_diff_long_identical_indices() {
        let m = parse_opts().get_matches_from(vec!["tp", "--diff", "5", "5"]);
        let v: Vec<String> = m.get_many(DIFF).unwrap().cloned().collect();
        assert_eq!(v, vec!["5", "5"]);
    }

    #[test]
    fn recognizes_expire_equals_scientific_like_string() {
        let m = parse_opts().get_matches_from(vec!["tp", "--expire=1e2"]);
        assert_eq!(m.get_one::<String>(EXPIRE).map(|s| s.as_str()), Some("1e2"));
    }

    #[test]
    fn recognizes_sort_mtime_equals_form() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort=mtime"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("mtime"));
    }

    #[test]
    fn recognizes_list_numbered_short_n_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "-n"]);
        assert!(m.get_flag(LIST_FILES_NUMBERED));
    }

    #[test]
    fn recognizes_list_contents_numbered_short_cap_n_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "-N"]);
        assert!(m.get_flag(LIST_CONTENTS_NUMBERED));
    }

    #[test]
    fn recognizes_count_pop_chain() {
        let m = parse_opts().get_matches_from(vec!["tp", "-k", "-p"]);
        assert!(m.get_flag(COUNT));
        assert!(m.get_flag(POP));
    }

    #[test]
    fn recognizes_rev_count_chain() {
        let m = parse_opts().get_matches_from(vec!["tp", "--rev", "-k"]);
        assert!(m.get_flag(REVERSE));
        assert!(m.get_flag(COUNT));
    }

    #[test]
    fn recognizes_quiet_with_list_files() {
        let m = parse_opts().get_matches_from(vec!["tp", "-q", "-l"]);
        assert!(m.get_flag(SILENT));
        assert!(m.get_flag(LIST_FILES));
    }

    #[test]
    fn recognizes_input_append_same_index_values() {
        let m = parse_opts().get_matches_from(vec!["tp", "-i", "2", "-A", "2"]);
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(APPEND).map(|s| s.as_str()), Some("2"));
    }

    #[test]
    fn recognizes_output_wc_path_negative_one_equals() {
        let m = parse_opts().get_matches_from(vec!["tp", "--wc=-1", "--path=-1", "--size=-1"]);
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("-1"));
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("-1"));
        assert_eq!(m.get_one::<String>(SIZE).map(|s| s.as_str()), Some("-1"));
    }

    #[test]
    fn recognizes_rename_long_two_unicode_args() {
        let m = parse_opts().get_matches_from(vec!["tp", "--rename", "α", "β"]);
        let v: Vec<String> = m.get_many(RENAME).unwrap().cloned().collect();
        assert_eq!(v, vec!["α", "β"]);
    }

    #[test]
    fn recognizes_head_tail_same_index_different_n() {
        let m = parse_opts().get_matches_from(vec!["tp", "--head", "1", "5", "--tail", "1", "3"]);
        let h: Vec<String> = m.get_many(HEAD).unwrap().cloned().collect();
        let t: Vec<String> = m.get_many(TAIL).unwrap().cloned().collect();
        assert_eq!(h, vec!["1", "5"]);
        assert_eq!(t, vec!["1", "3"]);
    }

    #[test]
    fn recognizes_edit_dup_chain_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-e", "1", "-x", "2"]);
        assert_eq!(m.get_one::<String>(EDIT).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("2"));
    }

    #[test]
    fn recognizes_grep_info_short_chain() {
        let m = parse_opts().get_matches_from(vec!["tp", "-g", "pat", "-I", "1"]);
        assert_eq!(m.get_one::<String>(GREP).map(|s| s.as_str()), Some("pat"));
        assert_eq!(m.get_one::<String>(INFO).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_clear_pop_shift_long_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "--clear", "--pop", "--shift"]);
        assert!(m.get_flag(CLEAR));
        assert!(m.get_flag(POP));
        assert!(m.get_flag(SHIFT));
    }
}
