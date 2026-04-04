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

    // ── clap coverage round 9 ──────────────────────────────

    #[test]
    fn recognizes_positional_semicolon_in_name() {
        let m = parse_opts().get_matches_from(vec!["tp", "data;foo=bar"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("data;foo=bar")
        );
    }

    #[test]
    fn recognizes_input_short_space_value_99() {
        let m = parse_opts().get_matches_from(vec!["tp", "-i", "99"]);
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("99"));
    }

    #[test]
    fn recognizes_output_long_space_only_minus_sign() {
        let m = parse_opts().get_matches_from(vec!["tp", "--output", "-"]);
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("-"));
    }

    #[test]
    fn recognizes_add_remove_long_both_equals() {
        let m = parse_opts().get_matches_from(vec!["tp", "--add=1", "--remove=2"]);
        assert_eq!(m.get_one::<String>(ADD).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(REMOVE).map(|s| s.as_str()), Some("2"));
    }

    #[test]
    fn recognizes_pop_shift_unshift_long_all() {
        let m = parse_opts().get_matches_from(vec!["tp", "--pop", "--shift", "--unshift"]);
        assert!(m.get_flag(POP));
        assert!(m.get_flag(SHIFT));
        assert!(m.get_flag(UNSHIFT));
    }

    #[test]
    fn recognizes_cat_twelve_indices_long() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--cat", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12",
        ]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(
            v,
            vec![
                "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12"
            ]
        );
    }

    #[test]
    fn recognizes_replace_backslash_in_pattern() {
        let m = parse_opts().get_matches_from(vec!["tp", "--replace", "1", r"\n", " "]);
        let v: Vec<String> = m.get_many::<String>(REPLACE).unwrap().cloned().collect();
        assert_eq!(v, vec!["1", r"\n", " "]);
    }

    #[test]
    fn recognizes_expire_space_fraction() {
        let m = parse_opts().get_matches_from(vec!["tp", "--expire", "3.14"]);
        assert_eq!(
            m.get_one::<String>(EXPIRE).map(|s| s.as_str()),
            Some("3.14")
        );
    }

    #[test]
    fn recognizes_sort_name_space_explicit() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort", "name"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("name"));
    }

    #[test]
    fn recognizes_dup_wc_short_chain() {
        let m = parse_opts().get_matches_from(vec!["tp", "-x", "1", "--wc", "2"]);
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("2"));
    }

    #[test]
    fn recognizes_path_size_expire_long_chain() {
        let m = parse_opts()
            .get_matches_from(vec!["tp", "--path", "1", "--size", "1", "--expire", "1"]);
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(SIZE).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(EXPIRE).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_verbose_nine_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-vvvvvvvvv"]);
        assert_eq!(m.get_count(VERBOSE), 9);
    }

    #[test]
    fn recognizes_quiet_with_verbose_count_still_parses() {
        let m = parse_opts().get_matches_from(vec!["tp", "-q", "-vvv"]);
        assert!(m.get_flag(SILENT));
        assert_eq!(m.get_count(VERBOSE), 3);
    }

    #[test]
    fn recognizes_move_swap_diff_short_letters_chain() {
        let m = parse_opts()
            .get_matches_from(vec!["tp", "-M", "1", "2", "-S", "1", "2", "-D", "1", "2"]);
        let mv: Vec<String> = m.get_many(MOVE).unwrap().cloned().collect();
        let sw: Vec<String> = m.get_many(SWAP).unwrap().cloned().collect();
        let df: Vec<String> = m.get_many(DIFF).unwrap().cloned().collect();
        assert_eq!(mv, vec!["1", "2"]);
        assert_eq!(sw, vec!["1", "2"]);
        assert_eq!(df, vec!["1", "2"]);
    }

    #[test]
    fn recognizes_name_output_input_chain() {
        let m = parse_opts().get_matches_from(vec!["tp", "-w", "t", "-o", "1", "-i", "2"]);
        assert_eq!(m.get_one::<String>(TAG).map(|s| s.as_str()), Some("t"));
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("2"));
    }

    #[test]
    fn recognizes_append_input_same_argv() {
        let m = parse_opts().get_matches_from(vec!["tp", "-A", "3", "-i", "3"]);
        assert_eq!(m.get_one::<String>(APPEND).map(|s| s.as_str()), Some("3"));
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("3"));
    }

    #[test]
    fn recognizes_list_all_four_short_flags_together() {
        let m = parse_opts().get_matches_from(vec!["tp", "-l", "-n", "-L", "-N"]);
        assert!(m.get_flag(LIST_FILES));
        assert!(m.get_flag(LIST_FILES_NUMBERED));
        assert!(m.get_flag(LIST_CONTENTS));
        assert!(m.get_flag(LIST_CONTENTS_NUMBERED));
    }

    #[test]
    fn recognizes_double_dash_positional_with_spaces() {
        let m = parse_opts().get_matches_from(vec!["tp", "--", "my file.txt"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("my file.txt")
        );
    }

    #[test]
    fn recognizes_info_path_dup_short_chain() {
        let m = parse_opts().get_matches_from(vec!["tp", "-I", "1", "--path", "2", "-x", "3"]);
        assert_eq!(m.get_one::<String>(INFO).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("3"));
    }

    #[test]
    fn recognizes_rename_long_space_two_tags() {
        let m = parse_opts().get_matches_from(vec!["tp", "--rename", "from-tag", "to-tag"]);
        let v: Vec<String> = m.get_many(RENAME).unwrap().cloned().collect();
        assert_eq!(v, vec!["from-tag", "to-tag"]);
    }

    #[test]
    fn recognizes_head_tail_wc_long_only() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--head", "1", "1", "--tail", "2", "2", "--wc", "3",
        ]);
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
            vec!["2", "2"]
        );
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("3"));
    }

    // ── clap coverage round 10 ─────────────────────────────

    #[test]
    fn recognizes_clap10_edit_equals_negative_three() {
        let m = parse_opts().get_matches_from(vec!["tp", "--edit=-3"]);
        assert_eq!(m.get_one::<String>(EDIT).map(|s| s.as_str()), Some("-3"));
    }

    #[test]
    fn recognizes_clap10_size_tail_short_chain() {
        let m = parse_opts().get_matches_from(vec!["tp", "--size", "1", "--tail", "1", "5"]);
        assert_eq!(m.get_one::<String>(SIZE).map(|s| s.as_str()), Some("1"));
        assert_eq!(
            m.get_many::<String>(TAIL)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["1", "5"]
        );
    }

    #[test]
    fn recognizes_clap10_head_wc_path_long_chain() {
        let m = parse_opts()
            .get_matches_from(vec!["tp", "--head", "2", "3", "--wc", "2", "--path", "2"]);
        assert_eq!(
            m.get_many::<String>(HEAD)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["2", "3"]
        );
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("2"));
    }

    #[test]
    fn recognizes_clap10_cat_two_negative_indices() {
        let m = parse_opts().get_matches_from(vec!["tp", "-C", "-2", "-1"]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(v, vec!["-2", "-1"]);
    }

    #[test]
    fn recognizes_clap10_grep_count_quiet_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-g", "x", "-k", "-q"]);
        assert_eq!(m.get_one::<String>(GREP).map(|s| s.as_str()), Some("x"));
        assert!(m.get_flag(COUNT));
        assert!(m.get_flag(SILENT));
    }

    #[test]
    fn recognizes_clap10_directory_master_count_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--dir", "--master", "--count"]);
        assert!(m.get_flag(DIRECTORY));
        assert!(m.get_flag(MASTER));
        assert!(m.get_flag(COUNT));
    }

    #[test]
    fn recognizes_clap10_reverse_expire_long_chain() {
        let m = parse_opts().get_matches_from(vec!["tp", "--rev", "--expire", "0"]);
        assert!(m.get_flag(REVERSE));
        assert_eq!(m.get_one::<String>(EXPIRE).map(|s| s.as_str()), Some("0"));
    }

    #[test]
    fn recognizes_clap10_replace_long_three_space_args() {
        let m = parse_opts().get_matches_from(vec!["tp", "--replace", "2", "pat", "sub"]);
        let v: Vec<String> = m.get_many(REPLACE).unwrap().cloned().collect();
        assert_eq!(v, vec!["2", "pat", "sub"]);
    }

    #[test]
    fn recognizes_clap10_append_input_dup_long() {
        let m = parse_opts()
            .get_matches_from(vec!["tp", "--append", "4", "--input", "3", "--dup", "2"]);
        assert_eq!(m.get_one::<String>(APPEND).map(|s| s.as_str()), Some("4"));
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("3"));
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("2"));
    }

    #[test]
    fn recognizes_clap10_rename_short_space_two_tags() {
        let m = parse_opts().get_matches_from(vec!["tp", "-R", "from-here", "to-there"]);
        let v: Vec<String> = m.get_many(RENAME).unwrap().cloned().collect();
        assert_eq!(v, vec!["from-here", "to-there"]);
    }

    #[test]
    fn recognizes_clap10_info_output_short_chain() {
        let m = parse_opts().get_matches_from(vec!["tp", "-I", "-1", "-o", "2"]);
        assert_eq!(m.get_one::<String>(INFO).map(|s| s.as_str()), Some("-1"));
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("2"));
    }

    #[test]
    fn recognizes_clap10_sort_mtime_space_explicit() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort", "mtime"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("mtime"));
    }

    #[test]
    fn recognizes_clap10_verbose_long_repeat_count() {
        let m = parse_opts().get_matches_from(vec!["tp", "--verbose", "--verbose", "--verbose"]);
        assert_eq!(m.get_count(VERBOSE), 3);
    }

    #[test]
    fn recognizes_clap10_list_files_quiet_clear_parse() {
        let m = parse_opts().get_matches_from(vec!["tp", "-l", "-q", "-c"]);
        assert!(m.get_flag(LIST_FILES));
        assert!(m.get_flag(SILENT));
        assert!(m.get_flag(CLEAR));
    }

    #[test]
    fn recognizes_clap10_add_remove_short_equals() {
        let m = parse_opts().get_matches_from(vec!["tp", "-a=5", "-r=6"]);
        assert_eq!(m.get_one::<String>(ADD).map(|s| s.as_str()), Some("5"));
        assert_eq!(m.get_one::<String>(REMOVE).map(|s| s.as_str()), Some("6"));
    }

    #[test]
    fn recognizes_clap10_output_input_add_chain() {
        let m = parse_opts().get_matches_from(vec!["tp", "-o", "7", "-i", "8", "-a", "9"]);
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("7"));
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("8"));
        assert_eq!(m.get_one::<String>(ADD).map(|s| s.as_str()), Some("9"));
    }

    #[test]
    fn recognizes_clap10_cat_three_short_indices() {
        let m = parse_opts().get_matches_from(vec!["tp", "-C", "10", "11", "12"]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(v, vec!["10", "11", "12"]);
    }

    #[test]
    fn recognizes_clap10_tail_positive_index_large_lines() {
        let m = parse_opts().get_matches_from(vec!["tp", "--tail", "3", "999"]);
        assert_eq!(
            m.get_many::<String>(TAIL)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["3", "999"]
        );
    }

    #[test]
    fn recognizes_clap10_head_negative_index_zero_lines() {
        let m = parse_opts().get_matches_from(vec!["tp", "--head", "-1", "0"]);
        assert_eq!(
            m.get_many::<String>(HEAD)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["-1", "0"]
        );
    }

    #[test]
    fn recognizes_clap10_path_size_dup_chain_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "--path", "1", "--size", "1", "-x", "1"]);
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(SIZE).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_clap10_expire_zero_hours() {
        let m = parse_opts().get_matches_from(vec!["tp", "--expire", "0"]);
        assert_eq!(m.get_one::<String>(EXPIRE).map(|s| s.as_str()), Some("0"));
    }

    #[test]
    fn recognizes_clap10_remove_short_positive_large() {
        let m = parse_opts().get_matches_from(vec!["tp", "-r", "1000"]);
        assert_eq!(
            m.get_one::<String>(REMOVE).map(|s| s.as_str()),
            Some("1000")
        );
    }

    #[test]
    fn recognizes_clap10_double_dash_empty_string_positional() {
        let m = parse_opts().get_matches_from(vec!["tp", "--", ""]);
        assert_eq!(m.get_one::<String>(ARGFILE).map(|s| s.as_str()), Some(""));
    }

    #[test]
    fn recognizes_clap10_positional_unicode_filename() {
        let m = parse_opts().get_matches_from(vec!["tp", "文件.txt"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("文件.txt")
        );
    }

    #[test]
    fn recognizes_clap10_grep_dot_pattern() {
        let m = parse_opts().get_matches_from(vec!["tp", "--grep", "."]);
        assert_eq!(m.get_one::<String>(GREP).map(|s| s.as_str()), Some("."));
    }

    #[test]
    fn recognizes_clap10_tag_hyphen_starting_name_long_equals() {
        let m = parse_opts().get_matches_from(vec!["tp", "--name=-leading"]);
        assert_eq!(
            m.get_one::<String>(TAG).map(|s| s.as_str()),
            Some("-leading")
        );
    }

    #[test]
    fn recognizes_clap10_diff_negative_indices_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--diff", "-1", "-2"]);
        let v: Vec<String> = m.get_many(DIFF).unwrap().cloned().collect();
        assert_eq!(v, vec!["-1", "-2"]);
    }

    #[test]
    fn recognizes_clap10_swap_short_space_pair() {
        let m = parse_opts().get_matches_from(vec!["tp", "-S", "7", "8"]);
        let v: Vec<String> = m.get_many(SWAP).unwrap().cloned().collect();
        assert_eq!(v, vec!["7", "8"]);
    }

    #[test]
    fn recognizes_clap10_mv_short_space_pair() {
        let m = parse_opts().get_matches_from(vec!["tp", "-M", "9", "10"]);
        let v: Vec<String> = m.get_many(MOVE).unwrap().cloned().collect();
        assert_eq!(v, vec!["9", "10"]);
    }

    #[test]
    fn recognizes_clap10_list_contents_numbered_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "-N"]);
        assert!(m.get_flag(LIST_CONTENTS_NUMBERED));
    }

    #[test]
    fn recognizes_clap10_unshift_only_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-u"]);
        assert!(m.get_flag(UNSHIFT));
    }

    #[test]
    fn recognizes_clap10_shift_only_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-s"]);
        assert!(m.get_flag(SHIFT));
    }

    #[test]
    fn recognizes_clap10_pop_only_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-p"]);
        assert!(m.get_flag(POP));
    }

    #[test]
    fn recognizes_clap10_rev_only_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--rev"]);
        assert!(m.get_flag(REVERSE));
    }

    #[test]
    fn recognizes_clap10_count_only_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-k"]);
        assert!(m.get_flag(COUNT));
    }

    #[test]
    fn recognizes_clap10_clear_only_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-c"]);
        assert!(m.get_flag(CLEAR));
    }

    #[test]
    fn recognizes_clap10_master_only_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-m"]);
        assert!(m.get_flag(MASTER));
    }

    #[test]
    fn recognizes_clap10_directory_only_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-d"]);
        assert!(m.get_flag(DIRECTORY));
    }

    #[test]
    fn recognizes_clap10_replace_tab_in_replacement() {
        let m = parse_opts().get_matches_from(vec!["tp", "--replace", "1", "a", "\t"]);
        let v: Vec<String> = m.get_many(REPLACE).unwrap().cloned().collect();
        assert_eq!(v, vec!["1", "a", "\t"]);
    }

    #[test]
    fn recognizes_clap10_cat_one_index_zero_string() {
        let m = parse_opts().get_matches_from(vec!["tp", "--cat", "0"]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(v, vec!["0"]);
    }

    #[test]
    fn recognizes_clap10_sort_only_flag_then_positional() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort", "size", "readme.md"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("size"));
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("readme.md")
        );
    }

    #[test]
    fn recognizes_clap10_wc_equals_negative_one() {
        let m = parse_opts().get_matches_from(vec!["tp", "--wc=-1"]);
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("-1"));
    }

    #[test]
    fn recognizes_clap10_size_equals_positive_large() {
        let m = parse_opts().get_matches_from(vec!["tp", "--size=9999"]);
        assert_eq!(m.get_one::<String>(SIZE).map(|s| s.as_str()), Some("9999"));
    }

    #[test]
    fn recognizes_clap10_path_grep_info_chain() {
        let m = parse_opts().get_matches_from(vec!["tp", "--path", "1", "-g", "pat", "-I", "2"]);
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(GREP).map(|s| s.as_str()), Some("pat"));
        assert_eq!(m.get_one::<String>(INFO).map(|s| s.as_str()), Some("2"));
    }

    // ── clap coverage round 11 ─────────────────────────────

    #[test]
    fn recognizes_clap11_edit_long_space_negative_five() {
        let m = parse_opts().get_matches_from(vec!["tp", "--edit", "-5"]);
        assert_eq!(m.get_one::<String>(EDIT).map(|s| s.as_str()), Some("-5"));
    }

    #[test]
    fn recognizes_clap11_grep_pattern_with_embedded_equals() {
        let m = parse_opts().get_matches_from(vec!["tp", "--grep", "a=b"]);
        assert_eq!(m.get_one::<String>(GREP).map(|s| s.as_str()), Some("a=b"));
    }

    #[test]
    fn recognizes_clap11_cat_four_indices_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-C", "1", "2", "3", "4"]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(v, vec!["1", "2", "3", "4"]);
    }

    #[test]
    fn recognizes_clap11_expire_large_hours() {
        let m = parse_opts().get_matches_from(vec!["tp", "--expire", "8760"]);
        assert_eq!(
            m.get_one::<String>(EXPIRE).map(|s| s.as_str()),
            Some("8760")
        );
    }

    #[test]
    fn recognizes_clap11_verbose_eleven_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-vvvvvvvvvvv"]);
        assert_eq!(m.get_count(VERBOSE), 11);
    }

    #[test]
    fn recognizes_clap11_four_list_long_flags() {
        let m = parse_opts().get_matches_from(vec![
            "tp",
            "--list-files",
            "--list-files-numbered",
            "--list-contents",
            "--list-contents-numbered",
        ]);
        assert!(m.get_flag(LIST_FILES));
        assert!(m.get_flag(LIST_FILES_NUMBERED));
        assert!(m.get_flag(LIST_CONTENTS));
        assert!(m.get_flag(LIST_CONTENTS_NUMBERED));
    }

    #[test]
    fn recognizes_clap11_clear_pop_shift_unshift_long() {
        let m =
            parse_opts().get_matches_from(vec!["tp", "--clear", "--pop", "--shift", "--unshift"]);
        assert!(m.get_flag(CLEAR));
        assert!(m.get_flag(POP));
        assert!(m.get_flag(SHIFT));
        assert!(m.get_flag(UNSHIFT));
    }

    #[test]
    fn recognizes_clap11_add_remove_input_output_chain() {
        let m =
            parse_opts().get_matches_from(vec!["tp", "-a", "1", "-r", "2", "-i", "3", "-o", "4"]);
        assert_eq!(m.get_one::<String>(ADD).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(REMOVE).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("3"));
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("4"));
    }

    #[test]
    fn recognizes_clap11_head_tail_minimal_pairs() {
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
    fn recognizes_clap11_path_wc_size_expire_long() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--path", "1", "--wc", "1", "--size", "1", "--expire", "1",
        ]);
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(SIZE).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(EXPIRE).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_clap11_dup_append_reverse_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--dup", "2", "--append", "1", "--rev"]);
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(APPEND).map(|s| s.as_str()), Some("1"));
        assert!(m.get_flag(REVERSE));
    }

    #[test]
    fn recognizes_clap11_info_edit_grep_short_chain() {
        let m = parse_opts().get_matches_from(vec!["tp", "-I", "1", "-e", "2", "-g", "pat"]);
        assert_eq!(m.get_one::<String>(INFO).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(EDIT).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(GREP).map(|s| s.as_str()), Some("pat"));
    }

    #[test]
    fn recognizes_clap11_diff_long_two_positive() {
        let m = parse_opts().get_matches_from(vec!["tp", "--diff", "10", "20"]);
        let v: Vec<String> = m.get_many(DIFF).unwrap().cloned().collect();
        assert_eq!(v, vec!["10", "20"]);
    }

    #[test]
    fn recognizes_clap11_swap_long_two_mixed_sign() {
        let m = parse_opts().get_matches_from(vec!["tp", "--swap", "1", "-1"]);
        let v: Vec<String> = m.get_many(SWAP).unwrap().cloned().collect();
        assert_eq!(v, vec!["1", "-1"]);
    }

    #[test]
    fn recognizes_clap11_move_long_two_args() {
        let m = parse_opts().get_matches_from(vec!["tp", "--mv", "3", "4"]);
        let v: Vec<String> = m.get_many(MOVE).unwrap().cloned().collect();
        assert_eq!(v, vec!["3", "4"]);
    }

    #[test]
    fn recognizes_clap11_rename_long_two_tags() {
        let m = parse_opts().get_matches_from(vec!["tp", "--rename", "alpha", "beta"]);
        let v: Vec<String> = m.get_many(RENAME).unwrap().cloned().collect();
        assert_eq!(v, vec!["alpha", "beta"]);
    }

    #[test]
    fn recognizes_clap11_replace_three_args_brackets() {
        let m = parse_opts().get_matches_from(vec!["tp", "--replace", "1", "[x]", "[y]"]);
        let v: Vec<String> = m.get_many(REPLACE).unwrap().cloned().collect();
        assert_eq!(v, vec!["1", "[x]", "[y]"]);
    }

    #[test]
    fn recognizes_clap11_positional_single_dash_filename() {
        let m = parse_opts().get_matches_from(vec!["tp", "-"]);
        assert_eq!(m.get_one::<String>(ARGFILE).map(|s| s.as_str()), Some("-"));
    }

    #[test]
    fn recognizes_clap11_positional_dot_slash_path() {
        let m = parse_opts().get_matches_from(vec!["tp", "./local.txt"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("./local.txt")
        );
    }

    #[test]
    fn recognizes_clap11_master_long_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "--master"]);
        assert!(m.get_flag(MASTER));
    }

    #[test]
    fn recognizes_clap11_directory_long_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "--dir"]);
        assert!(m.get_flag(DIRECTORY));
    }

    #[test]
    fn recognizes_clap11_count_rev_quiet_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-k", "--rev", "-q"]);
        assert!(m.get_flag(COUNT));
        assert!(m.get_flag(REVERSE));
        assert!(m.get_flag(SILENT));
    }

    #[test]
    fn recognizes_clap11_input_output_long_equals() {
        let m = parse_opts().get_matches_from(vec!["tp", "--input=5", "--output=6"]);
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("5"));
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("6"));
    }

    #[test]
    fn recognizes_clap11_grep_long_equals_pattern() {
        let m = parse_opts().get_matches_from(vec!["tp", "--grep=^start"]);
        assert_eq!(
            m.get_one::<String>(GREP).map(|s| s.as_str()),
            Some("^start")
        );
    }

    #[test]
    fn recognizes_clap11_cat_three_negative_indices() {
        let m = parse_opts().get_matches_from(vec!["tp", "--cat", "-3", "-2", "-1"]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(v, vec!["-3", "-2", "-1"]);
    }

    #[test]
    fn recognizes_clap11_sort_explicit_name_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort", "name"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("name"));
    }

    #[test]
    fn recognizes_clap11_sort_explicit_size_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort", "size"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("size"));
    }

    #[test]
    fn recognizes_clap11_append_dup_wc_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-A", "1", "-x", "2", "--wc", "1"]);
        assert_eq!(m.get_one::<String>(APPEND).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_clap11_name_tag_short_then_output() {
        let m = parse_opts().get_matches_from(vec!["tp", "-w", "tagged", "-o", "1"]);
        assert_eq!(m.get_one::<String>(TAG).map(|s| s.as_str()), Some("tagged"));
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_clap11_pop_then_verbose_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-p", "-vv"]);
        assert!(m.get_flag(POP));
        assert_eq!(m.get_count(VERBOSE), 2);
    }

    #[test]
    fn recognizes_clap11_shift_then_list_files() {
        let m = parse_opts().get_matches_from(vec!["tp", "-s", "-l"]);
        assert!(m.get_flag(SHIFT));
        assert!(m.get_flag(LIST_FILES));
    }

    #[test]
    fn recognizes_clap11_unshift_only_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--unshift"]);
        assert!(m.get_flag(UNSHIFT));
    }

    #[test]
    fn recognizes_clap11_double_dash_positional_with_leading_space_name() {
        let m = parse_opts().get_matches_from(vec!["tp", "--", " leading"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some(" leading")
        );
    }

    #[test]
    fn recognizes_clap11_expire_decimal_hours() {
        let m = parse_opts().get_matches_from(vec!["tp", "--expire", "0.5"]);
        assert_eq!(m.get_one::<String>(EXPIRE).map(|s| s.as_str()), Some("0.5"));
    }

    #[test]
    fn recognizes_clap11_size_path_short_long_mix() {
        let m = parse_opts().get_matches_from(vec!["tp", "--size", "2", "--path", "-1"]);
        assert_eq!(m.get_one::<String>(SIZE).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("-1"));
    }

    #[test]
    fn recognizes_clap11_tail_head_chain() {
        let m = parse_opts().get_matches_from(vec!["tp", "--tail", "1", "2", "--head", "1", "3"]);
        assert_eq!(
            m.get_many::<String>(TAIL)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["1", "2"]
        );
        assert_eq!(
            m.get_many::<String>(HEAD)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["1", "3"]
        );
    }

    #[test]
    fn recognizes_clap11_quiet_verbose_parse_both() {
        let m = parse_opts().get_matches_from(vec!["tp", "-q", "-v"]);
        assert!(m.get_flag(SILENT));
        assert_eq!(m.get_count(VERBOSE), 1);
    }

    #[test]
    fn recognizes_clap11_count_only_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--count"]);
        assert!(m.get_flag(COUNT));
    }

    #[test]
    fn recognizes_clap11_clear_only_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--clear"]);
        assert!(m.get_flag(CLEAR));
    }

    #[test]
    fn recognizes_clap11_list_files_only_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--list-files"]);
        assert!(m.get_flag(LIST_FILES));
    }

    #[test]
    fn recognizes_clap11_list_contents_only_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--list-contents"]);
        assert!(m.get_flag(LIST_CONTENTS));
    }

    // ── clap coverage round 12 ─────────────────────────────

    #[test]
    fn recognizes_clap12_output_short_equals_positive() {
        let m = parse_opts().get_matches_from(vec!["tp", "-o=42"]);
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("42"));
    }

    #[test]
    fn recognizes_clap12_input_short_equals_positive() {
        let m = parse_opts().get_matches_from(vec!["tp", "-i=99"]);
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("99"));
    }

    #[test]
    fn recognizes_clap12_append_wc_dup_short_chain() {
        let m = parse_opts().get_matches_from(vec!["tp", "-A", "1", "--wc", "2", "-x", "3"]);
        assert_eq!(m.get_one::<String>(APPEND).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("3"));
    }

    #[test]
    fn recognizes_clap12_expire_sort_rev_long() {
        let m =
            parse_opts().get_matches_from(vec!["tp", "--expire", "24", "--sort", "mtime", "--rev"]);
        assert_eq!(m.get_one::<String>(EXPIRE).map(|s| s.as_str()), Some("24"));
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("mtime"));
        assert!(m.get_flag(REVERSE));
    }

    #[test]
    fn recognizes_clap12_cat_five_short_indices() {
        let m = parse_opts().get_matches_from(vec!["tp", "-C", "a", "b", "c", "d", "e"]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(v, vec!["a", "b", "c", "d", "e"]);
    }

    #[test]
    fn recognizes_clap12_grep_star_pattern() {
        let m = parse_opts().get_matches_from(vec!["tp", "-g", "*"]);
        assert_eq!(m.get_one::<String>(GREP).map(|s| s.as_str()), Some("*"));
    }

    #[test]
    fn recognizes_clap12_edit_long_at_symbol() {
        let m = parse_opts().get_matches_from(vec!["tp", "--edit", "@mytag"]);
        assert_eq!(
            m.get_one::<String>(EDIT).map(|s| s.as_str()),
            Some("@mytag")
        );
    }

    #[test]
    fn recognizes_clap12_info_long_negative_large() {
        let m = parse_opts().get_matches_from(vec!["tp", "--info", "-100"]);
        assert_eq!(m.get_one::<String>(INFO).map(|s| s.as_str()), Some("-100"));
    }

    #[test]
    fn recognizes_clap12_path_long_zero_string() {
        let m = parse_opts().get_matches_from(vec!["tp", "--path", "0"]);
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("0"));
    }

    #[test]
    fn recognizes_clap12_size_wc_head_chain() {
        let m = parse_opts()
            .get_matches_from(vec!["tp", "--size", "1", "--wc", "1", "--head", "1", "10"]);
        assert_eq!(m.get_one::<String>(SIZE).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("1"));
        assert_eq!(
            m.get_many::<String>(HEAD)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["1", "10"]
        );
    }

    #[test]
    fn recognizes_clap12_remove_add_short_chain() {
        let m = parse_opts().get_matches_from(vec!["tp", "-r", "2", "-a", "1"]);
        assert_eq!(m.get_one::<String>(REMOVE).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(ADD).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_clap12_name_long_underscore() {
        let m = parse_opts().get_matches_from(vec!["tp", "--name", "my_tag"]);
        assert_eq!(m.get_one::<String>(TAG).map(|s| s.as_str()), Some("my_tag"));
    }

    #[test]
    fn recognizes_clap12_rename_short_two_words() {
        let m = parse_opts().get_matches_from(vec!["tp", "-R", "one", "two"]);
        let v: Vec<String> = m.get_many(RENAME).unwrap().cloned().collect();
        assert_eq!(v, vec!["one", "two"]);
    }

    #[test]
    fn recognizes_clap12_diff_swap_move_all_short() {
        let m = parse_opts()
            .get_matches_from(vec!["tp", "-D", "1", "2", "-S", "3", "4", "-M", "5", "6"]);
        assert_eq!(
            m.get_many(DIFF).unwrap().cloned().collect::<Vec<String>>(),
            vec!["1", "2"]
        );
        assert_eq!(
            m.get_many(SWAP).unwrap().cloned().collect::<Vec<String>>(),
            vec!["3", "4"]
        );
        assert_eq!(
            m.get_many(MOVE).unwrap().cloned().collect::<Vec<String>>(),
            vec!["5", "6"]
        );
    }

    #[test]
    fn recognizes_clap12_replace_newline_in_pattern() {
        let m = parse_opts().get_matches_from(vec!["tp", "--replace", "1", "\n", " "]);
        let v: Vec<String> = m.get_many(REPLACE).unwrap().cloned().collect();
        assert_eq!(v, vec!["1", "\n", " "]);
    }

    #[test]
    fn recognizes_clap12_positional_only_spaces() {
        let m = parse_opts().get_matches_from(vec!["tp", "   "]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("   ")
        );
    }

    #[test]
    fn recognizes_clap12_quiet_long_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "--quiet"]);
        assert!(m.get_flag(SILENT));
    }

    #[test]
    fn recognizes_clap12_verbose_long_single() {
        let m = parse_opts().get_matches_from(vec!["tp", "--verbose"]);
        assert_eq!(m.get_count(VERBOSE), 1);
    }

    #[test]
    fn recognizes_clap12_list_files_numbered_long_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "--list-files-numbered"]);
        assert!(m.get_flag(LIST_FILES_NUMBERED));
    }

    #[test]
    fn recognizes_clap12_list_contents_numbered_long_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "--list-contents-numbered"]);
        assert!(m.get_flag(LIST_CONTENTS_NUMBERED));
    }

    #[test]
    fn recognizes_clap12_pop_long_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "--pop"]);
        assert!(m.get_flag(POP));
    }

    #[test]
    fn recognizes_clap12_shift_long_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "--shift"]);
        assert!(m.get_flag(SHIFT));
    }

    #[test]
    fn recognizes_clap12_dup_long_equals_index() {
        let m = parse_opts().get_matches_from(vec!["tp", "--dup=-2"]);
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("-2"));
    }

    #[test]
    fn recognizes_clap12_append_long_equals_index() {
        let m = parse_opts().get_matches_from(vec!["tp", "--append=3"]);
        assert_eq!(m.get_one::<String>(APPEND).map(|s| s.as_str()), Some("3"));
    }

    #[test]
    fn recognizes_clap12_tail_head_wc_long_chain() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--tail", "1", "5", "--head", "1", "5", "--wc", "1",
        ]);
        assert_eq!(
            m.get_many::<String>(TAIL)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["1", "5"]
        );
        assert_eq!(
            m.get_many::<String>(HEAD)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["1", "5"]
        );
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_clap12_expire_path_size_long() {
        let m = parse_opts()
            .get_matches_from(vec!["tp", "--expire", "1", "--path", "1", "--size", "1"]);
        assert_eq!(m.get_one::<String>(EXPIRE).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(SIZE).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_clap12_count_directory_master_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-k", "-d", "-m"]);
        assert!(m.get_flag(COUNT));
        assert!(m.get_flag(DIRECTORY));
        assert!(m.get_flag(MASTER));
    }

    #[test]
    fn recognizes_clap12_clear_list_files_chain() {
        let m = parse_opts().get_matches_from(vec!["tp", "-c", "-l"]);
        assert!(m.get_flag(CLEAR));
        assert!(m.get_flag(LIST_FILES));
    }

    #[test]
    fn recognizes_clap12_input_output_both_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-i", "1", "-o", "2"]);
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("2"));
    }

    #[test]
    fn recognizes_clap12_verbose_five_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-vvvvv"]);
        assert_eq!(m.get_count(VERBOSE), 5);
    }

    #[test]
    fn recognizes_clap12_double_dash_positional_hash() {
        let m = parse_opts().get_matches_from(vec!["tp", "--", "#file"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("#file")
        );
    }

    #[test]
    fn recognizes_clap12_sort_mtime_then_positional_file() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort", "mtime", "stack.txt"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("mtime"));
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("stack.txt")
        );
    }

    #[test]
    fn recognizes_clap12_grep_long_dollar_pattern() {
        let m = parse_opts().get_matches_from(vec!["tp", "--grep", "end$"]);
        assert_eq!(m.get_one::<String>(GREP).map(|s| s.as_str()), Some("end$"));
    }

    #[test]
    fn recognizes_clap12_cat_short_two_indices_mixed_sign() {
        let m = parse_opts().get_matches_from(vec!["tp", "-C", "1", "-1"]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(v, vec!["1", "-1"]);
    }

    #[test]
    fn recognizes_clap12_move_long_from_to_names() {
        let m = parse_opts().get_matches_from(vec!["tp", "--mv", "from", "to"]);
        let v: Vec<String> = m.get_many(MOVE).unwrap().cloned().collect();
        assert_eq!(v, vec!["from", "to"]);
    }

    #[test]
    fn recognizes_clap12_swap_long_two_negative() {
        let m = parse_opts().get_matches_from(vec!["tp", "--swap", "-2", "-1"]);
        let v: Vec<String> = m.get_many(SWAP).unwrap().cloned().collect();
        assert_eq!(v, vec!["-2", "-1"]);
    }

    #[test]
    fn recognizes_clap12_diff_long_two_zero() {
        let m = parse_opts().get_matches_from(vec!["tp", "--diff", "0", "0"]);
        let v: Vec<String> = m.get_many(DIFF).unwrap().cloned().collect();
        assert_eq!(v, vec!["0", "0"]);
    }

    #[test]
    fn recognizes_clap12_replace_three_args_empty_replacement() {
        let m = parse_opts().get_matches_from(vec!["tp", "--replace", "1", "x", ""]);
        let v: Vec<String> = m.get_many(REPLACE).unwrap().cloned().collect();
        assert_eq!(v, vec!["1", "x", ""]);
    }

    // ── clap coverage round 13 ─────────────────────────────

    #[test]
    fn recognizes_clap13_version_string_not_parsed_as_subcommand() {
        let m = parse_opts().get_matches_from(vec!["tp", "v1.0.0.tgz"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("v1.0.0.tgz")
        );
    }

    #[test]
    fn recognizes_clap13_output_long_equals_minus_one() {
        let m = parse_opts().get_matches_from(vec!["tp", "--output=-1"]);
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("-1"));
    }

    #[test]
    fn recognizes_clap13_input_long_equals_at_name() {
        let m = parse_opts().get_matches_from(vec!["tp", "--input=@tag"]);
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("@tag"));
    }

    #[test]
    fn recognizes_clap13_add_remove_long_both_negative() {
        let m = parse_opts().get_matches_from(vec!["tp", "--add", "-1", "--remove", "-2"]);
        assert_eq!(m.get_one::<String>(ADD).map(|s| s.as_str()), Some("-1"));
        assert_eq!(m.get_one::<String>(REMOVE).map(|s| s.as_str()), Some("-2"));
    }

    #[test]
    fn recognizes_clap13_cat_six_indices_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--cat", "1", "2", "3", "4", "5", "6"]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(v, vec!["1", "2", "3", "4", "5", "6"]);
    }

    #[test]
    fn recognizes_clap13_grep_pipe_in_pattern() {
        let m = parse_opts().get_matches_from(vec!["tp", "-g", "a|b"]);
        assert_eq!(m.get_one::<String>(GREP).map(|s| s.as_str()), Some("a|b"));
    }

    #[test]
    fn recognizes_clap13_expire_equals_negative_zero() {
        let m = parse_opts().get_matches_from(vec!["tp", "--expire=-0"]);
        assert_eq!(m.get_one::<String>(EXPIRE).map(|s| s.as_str()), Some("-0"));
    }

    #[test]
    fn recognizes_clap13_sort_size_then_positional() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort", "size", "in.txt"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("size"));
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("in.txt")
        );
    }

    #[test]
    fn recognizes_clap13_rev_sort_name_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--rev", "--sort", "name"]);
        assert!(m.get_flag(REVERSE));
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("name"));
    }

    #[test]
    fn recognizes_clap13_dup_x_append_a_chain() {
        let m = parse_opts().get_matches_from(vec!["tp", "-x", "1", "-A", "1"]);
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(APPEND).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_clap13_path_wc_size_short_long_mix() {
        let m =
            parse_opts().get_matches_from(vec!["tp", "--path", "2", "--wc", "-1", "--size", "-1"]);
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("-1"));
        assert_eq!(m.get_one::<String>(SIZE).map(|s| s.as_str()), Some("-1"));
    }

    #[test]
    fn recognizes_clap13_head_tail_max_u32_lines() {
        let m = parse_opts().get_matches_from(vec![
            "tp",
            "--head",
            "1",
            "4294967295",
            "--tail",
            "1",
            "0",
        ]);
        assert_eq!(
            m.get_many::<String>(HEAD)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["1", "4294967295"]
        );
        assert_eq!(
            m.get_many::<String>(TAIL)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["1", "0"]
        );
    }

    #[test]
    fn recognizes_clap13_replace_unicode_three_args() {
        let m = parse_opts().get_matches_from(vec!["tp", "--replace", "1", "α", "β"]);
        let v: Vec<String> = m.get_many(REPLACE).unwrap().cloned().collect();
        assert_eq!(v, vec!["1", "α", "β"]);
    }

    #[test]
    fn recognizes_clap13_rename_long_two_unicode() {
        let m = parse_opts().get_matches_from(vec!["tp", "--rename", "旧", "新"]);
        let v: Vec<String> = m.get_many(RENAME).unwrap().cloned().collect();
        assert_eq!(v, vec!["旧", "新"]);
    }

    #[test]
    fn recognizes_clap13_diff_short_two_unicode_indices() {
        let m = parse_opts().get_matches_from(vec!["tp", "-D", "一", "二"]);
        let v: Vec<String> = m.get_many(DIFF).unwrap().cloned().collect();
        assert_eq!(v, vec!["一", "二"]);
    }

    #[test]
    fn recognizes_clap13_verbose_twelve_long() {
        let m = parse_opts().get_matches_from(vec![
            "tp",
            "--verbose",
            "--verbose",
            "--verbose",
            "--verbose",
            "--verbose",
            "--verbose",
            "--verbose",
            "--verbose",
            "--verbose",
            "--verbose",
            "--verbose",
            "--verbose",
        ]);
        assert_eq!(m.get_count(VERBOSE), 12);
    }

    #[test]
    fn recognizes_clap13_list_numbered_short_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "-n"]);
        assert!(m.get_flag(LIST_FILES_NUMBERED));
    }

    #[test]
    fn recognizes_clap13_list_contents_short_uppercase() {
        let m = parse_opts().get_matches_from(vec!["tp", "-L"]);
        assert!(m.get_flag(LIST_CONTENTS));
    }

    #[test]
    fn recognizes_clap13_list_contents_numbered_short_uppercase() {
        let m = parse_opts().get_matches_from(vec!["tp", "-N"]);
        assert!(m.get_flag(LIST_CONTENTS_NUMBERED));
    }

    #[test]
    fn recognizes_clap13_pop_shift_verbose_chain() {
        let m = parse_opts().get_matches_from(vec!["tp", "-p", "-s", "-vvv"]);
        assert!(m.get_flag(POP));
        assert!(m.get_flag(SHIFT));
        assert_eq!(m.get_count(VERBOSE), 3);
    }

    #[test]
    fn recognizes_clap13_unshift_input_chain() {
        let m = parse_opts().get_matches_from(vec!["tp", "-u", "-i", "1"]);
        assert!(m.get_flag(UNSHIFT));
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_clap13_edit_grep_info_chain() {
        let m = parse_opts().get_matches_from(vec!["tp", "-e", "1", "-g", "x", "-I", "1"]);
        assert_eq!(m.get_one::<String>(EDIT).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(GREP).map(|s| s.as_str()), Some("x"));
        assert_eq!(m.get_one::<String>(INFO).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_clap13_count_clear_quiet_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--count", "--clear", "--quiet"]);
        assert!(m.get_flag(COUNT));
        assert!(m.get_flag(CLEAR));
        assert!(m.get_flag(SILENT));
    }

    #[test]
    fn recognizes_clap13_double_dash_positional_only_plus() {
        let m = parse_opts().get_matches_from(vec!["tp", "--", "+special"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("+special")
        );
    }

    #[test]
    fn recognizes_clap13_positional_colon_path() {
        let m = parse_opts().get_matches_from(vec!["tp", "C:temp.txt"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("C:temp.txt")
        );
    }

    #[test]
    fn recognizes_clap13_move_swap_diff_order() {
        let m = parse_opts()
            .get_matches_from(vec!["tp", "-M", "1", "2", "-S", "3", "4", "-D", "5", "6"]);
        assert_eq!(
            m.get_many(MOVE).unwrap().cloned().collect::<Vec<String>>(),
            vec!["1", "2"]
        );
        assert_eq!(
            m.get_many(SWAP).unwrap().cloned().collect::<Vec<String>>(),
            vec!["3", "4"]
        );
        assert_eq!(
            m.get_many(DIFF).unwrap().cloned().collect::<Vec<String>>(),
            vec!["5", "6"]
        );
    }

    #[test]
    fn recognizes_clap13_name_short_equals_value() {
        let m = parse_opts().get_matches_from(vec!["tp", "-w=label"]);
        assert_eq!(m.get_one::<String>(TAG).map(|s| s.as_str()), Some("label"));
    }

    #[test]
    fn recognizes_clap13_wc_short_equals() {
        let m = parse_opts().get_matches_from(vec!["tp", "--wc=2"]);
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("2"));
    }

    #[test]
    fn recognizes_clap13_size_short_long_same_index() {
        let m = parse_opts().get_matches_from(vec!["tp", "--size", "1", "--path", "1"]);
        assert_eq!(m.get_one::<String>(SIZE).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_clap13_expire_path_dup_chain() {
        let m =
            parse_opts().get_matches_from(vec!["tp", "--expire", "99", "--path", "1", "-x", "1"]);
        assert_eq!(m.get_one::<String>(EXPIRE).map(|s| s.as_str()), Some("99"));
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_clap13_cat_short_one_index_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "-C", "7"]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(v, vec!["7"]);
    }

    #[test]
    fn recognizes_clap13_tail_head_same_index_different_n() {
        let m = parse_opts().get_matches_from(vec!["tp", "--tail", "2", "1", "--head", "2", "100"]);
        assert_eq!(
            m.get_many::<String>(TAIL)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["2", "1"]
        );
        assert_eq!(
            m.get_many::<String>(HEAD)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["2", "100"]
        );
    }

    #[test]
    fn recognizes_clap13_grep_backslash_pattern() {
        let m = parse_opts().get_matches_from(vec!["tp", "-g", r"\d+"]);
        assert_eq!(m.get_one::<String>(GREP).map(|s| s.as_str()), Some(r"\d+"));
    }

    // ── clap coverage round 14 ─────────────────────────────

    #[test]
    fn recognizes_clap14_output_input_append_short_chain() {
        let m = parse_opts().get_matches_from(vec!["tp", "-o", "1", "-i", "2", "-A", "3"]);
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(APPEND).map(|s| s.as_str()), Some("3"));
    }

    #[test]
    fn recognizes_clap14_add_remove_dup_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-a", "1", "-r", "2", "-x", "3"]);
        assert_eq!(m.get_one::<String>(ADD).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(REMOVE).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("3"));
    }

    #[test]
    fn recognizes_clap14_cat_seven_indices_long() {
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
    fn recognizes_clap14_grep_question_mark_pattern() {
        let m = parse_opts().get_matches_from(vec!["tp", "--grep", "colou?r"]);
        assert_eq!(
            m.get_one::<String>(GREP).map(|s| s.as_str()),
            Some("colou?r")
        );
    }

    #[test]
    fn recognizes_clap14_expire_scientific_string() {
        let m = parse_opts().get_matches_from(vec!["tp", "--expire", "1e-3"]);
        assert_eq!(
            m.get_one::<String>(EXPIRE).map(|s| s.as_str()),
            Some("1e-3")
        );
    }

    #[test]
    fn recognizes_clap14_sort_default_long_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("name"));
    }

    #[test]
    fn recognizes_clap14_rev_only_short_parse() {
        let m = parse_opts().get_matches_from(vec!["tp", "--rev"]);
        assert!(m.get_flag(REVERSE));
    }

    #[test]
    fn recognizes_clap14_count_list_files_chain() {
        let m = parse_opts().get_matches_from(vec!["tp", "-k", "-l"]);
        assert!(m.get_flag(COUNT));
        assert!(m.get_flag(LIST_FILES));
    }

    #[test]
    fn recognizes_clap14_master_directory_verbose() {
        let m = parse_opts().get_matches_from(vec!["tp", "-m", "-d", "-v"]);
        assert!(m.get_flag(MASTER));
        assert!(m.get_flag(DIRECTORY));
        assert_eq!(m.get_count(VERBOSE), 1);
    }

    #[test]
    fn recognizes_clap14_head_wc_only_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--head", "1", "0", "--wc", "1"]);
        assert_eq!(
            m.get_many::<String>(HEAD)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["1", "0"]
        );
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_clap14_tail_only_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--tail", "-1", "5"]);
        assert_eq!(
            m.get_many::<String>(TAIL)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["-1", "5"]
        );
    }

    #[test]
    fn recognizes_clap14_replace_percent_in_replacement() {
        let m = parse_opts().get_matches_from(vec!["tp", "--replace", "1", "x", "%"]);
        let v: Vec<String> = m.get_many(REPLACE).unwrap().cloned().collect();
        assert_eq!(v, vec!["1", "x", "%"]);
    }

    #[test]
    fn recognizes_clap14_rename_short_hyphenated_tags() {
        let m = parse_opts().get_matches_from(vec!["tp", "-R", "old-name", "new-name"]);
        let v: Vec<String> = m.get_many(RENAME).unwrap().cloned().collect();
        assert_eq!(v, vec!["old-name", "new-name"]);
    }

    #[test]
    fn recognizes_clap14_diff_long_mixed_name_index() {
        let m = parse_opts().get_matches_from(vec!["tp", "--diff", "1", "@tag"]);
        let v: Vec<String> = m.get_many(DIFF).unwrap().cloned().collect();
        assert_eq!(v, vec!["1", "@tag"]);
    }

    #[test]
    fn recognizes_clap14_swap_short_numeric_strings() {
        let m = parse_opts().get_matches_from(vec!["tp", "-S", "001", "002"]);
        let v: Vec<String> = m.get_many(SWAP).unwrap().cloned().collect();
        assert_eq!(v, vec!["001", "002"]);
    }

    #[test]
    fn recognizes_clap14_move_short_large_indices() {
        let m = parse_opts().get_matches_from(vec!["tp", "-M", "1000", "2000"]);
        let v: Vec<String> = m.get_many(MOVE).unwrap().cloned().collect();
        assert_eq!(v, vec!["1000", "2000"]);
    }

    #[test]
    fn recognizes_clap14_verbose_thirteen_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-vvvvvvvvvvvvv"]);
        assert_eq!(m.get_count(VERBOSE), 13);
    }

    #[test]
    fn recognizes_clap14_positional_tab_in_name() {
        let m = parse_opts().get_matches_from(vec!["tp", "file\tname.txt"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("file\tname.txt")
        );
    }

    #[test]
    fn recognizes_clap14_double_dash_positional_newline_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "--", "\n"]);
        assert_eq!(m.get_one::<String>(ARGFILE).map(|s| s.as_str()), Some("\n"));
    }

    #[test]
    fn recognizes_clap14_info_path_dup_long_chain() {
        let m =
            parse_opts().get_matches_from(vec!["tp", "--info", "1", "--path", "2", "--dup", "3"]);
        assert_eq!(m.get_one::<String>(INFO).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("3"));
    }

    #[test]
    fn recognizes_clap14_edit_long_equals_index() {
        let m = parse_opts().get_matches_from(vec!["tp", "--edit=42"]);
        assert_eq!(m.get_one::<String>(EDIT).map(|s| s.as_str()), Some("42"));
    }

    #[test]
    fn recognizes_clap14_grep_long_equals_empty_pattern() {
        let m = parse_opts().get_matches_from(vec!["tp", "--grep="]);
        assert_eq!(m.get_one::<String>(GREP).map(|s| s.as_str()), Some(""));
    }

    #[test]
    fn recognizes_clap14_name_long_equals_simple() {
        let m = parse_opts().get_matches_from(vec!["tp", "--name=simple"]);
        assert_eq!(m.get_one::<String>(TAG).map(|s| s.as_str()), Some("simple"));
    }

    #[test]
    fn recognizes_clap14_clear_pop_unshift_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-c", "-p", "-u"]);
        assert!(m.get_flag(CLEAR));
        assert!(m.get_flag(POP));
        assert!(m.get_flag(UNSHIFT));
    }

    #[test]
    fn recognizes_clap14_shift_reverse_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-s", "--rev"]);
        assert!(m.get_flag(SHIFT));
        assert!(m.get_flag(REVERSE));
    }

    #[test]
    fn recognizes_clap14_size_wc_path_expire_long() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--size", "1", "--wc", "1", "--path", "1", "--expire", "0",
        ]);
        assert_eq!(m.get_one::<String>(SIZE).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(EXPIRE).map(|s| s.as_str()), Some("0"));
    }

    #[test]
    fn recognizes_clap14_append_long_negative_index() {
        let m = parse_opts().get_matches_from(vec!["tp", "--append", "-3"]);
        assert_eq!(m.get_one::<String>(APPEND).map(|s| s.as_str()), Some("-3"));
    }

    #[test]
    fn recognizes_clap14_dup_short_positive_large() {
        let m = parse_opts().get_matches_from(vec!["tp", "-x", "99999"]);
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("99999"));
    }

    #[test]
    fn recognizes_clap14_remove_long_equals_positive() {
        let m = parse_opts().get_matches_from(vec!["tp", "--remove=8"]);
        assert_eq!(m.get_one::<String>(REMOVE).map(|s| s.as_str()), Some("8"));
    }

    #[test]
    fn recognizes_clap14_add_long_equals_positive() {
        let m = parse_opts().get_matches_from(vec!["tp", "--add=9"]);
        assert_eq!(m.get_one::<String>(ADD).map(|s| s.as_str()), Some("9"));
    }

    #[test]
    fn recognizes_clap14_cat_long_eight_indices() {
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
    fn recognizes_clap14_quiet_long_verbose_count_parse() {
        let m = parse_opts().get_matches_from(vec!["tp", "--quiet", "--verbose", "--verbose"]);
        assert!(m.get_flag(SILENT));
        assert_eq!(m.get_count(VERBOSE), 2);
    }

    #[test]
    fn recognizes_clap14_list_contents_numbered_long_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "--list-contents-numbered"]);
        assert!(m.get_flag(LIST_CONTENTS_NUMBERED));
    }

    #[test]
    fn recognizes_clap14_list_files_numbered_long_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "--list-files-numbered"]);
        assert!(m.get_flag(LIST_FILES_NUMBERED));
    }

    // ── clap coverage round 15 ─────────────────────────────

    #[test]
    fn recognizes_clap15_positional_backslash_windows_style() {
        let m = parse_opts().get_matches_from(vec![r"tp", r"folder\file.txt"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some(r"folder\file.txt")
        );
    }

    #[test]
    fn recognizes_clap15_output_short_space_negative_large() {
        let m = parse_opts().get_matches_from(vec!["tp", "-o", "-999"]);
        assert_eq!(
            m.get_one::<String>(OUTPUT).map(|s| s.as_str()),
            Some("-999")
        );
    }

    #[test]
    fn recognizes_clap15_input_long_multiline_not_special() {
        let m = parse_opts().get_matches_from(vec!["tp", "--input", "1\n2"]);
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("1\n2"));
    }

    #[test]
    fn recognizes_clap15_cat_nine_indices_short() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "-C", "1", "2", "3", "4", "5", "6", "7", "8", "9",
        ]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(v, vec!["1", "2", "3", "4", "5", "6", "7", "8", "9"]);
    }

    #[test]
    fn recognizes_clap15_grep_caret_anchor() {
        let m = parse_opts().get_matches_from(vec!["tp", "-g", "^start"]);
        assert_eq!(
            m.get_one::<String>(GREP).map(|s| s.as_str()),
            Some("^start")
        );
    }

    #[test]
    fn recognizes_clap15_expire_long_fraction_no_leading_digit_issue() {
        let m = parse_opts().get_matches_from(vec!["tp", "--expire", ".25"]);
        assert_eq!(m.get_one::<String>(EXPIRE).map(|s| s.as_str()), Some(".25"));
    }

    #[test]
    fn recognizes_clap15_sort_name_explicit_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort", "name"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("name"));
    }

    #[test]
    fn recognizes_clap15_dup_append_wc_short_chain() {
        let m = parse_opts().get_matches_from(vec!["tp", "-x", "1", "-A", "2", "--wc", "1"]);
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(APPEND).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_clap15_head_tail_wc_all_long() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--head", "1", "1", "--tail", "1", "1", "--wc", "1",
        ]);
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
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_clap15_path_size_only_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--path", "2", "--size", "2"]);
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(SIZE).map(|s| s.as_str()), Some("2"));
    }

    #[test]
    fn recognizes_clap15_replace_three_args_dollar_in_pattern() {
        let m = parse_opts().get_matches_from(vec!["tp", "--replace", "1", "$", "dollar"]);
        let v: Vec<String> = m.get_many(REPLACE).unwrap().cloned().collect();
        assert_eq!(v, vec!["1", "$", "dollar"]);
    }

    #[test]
    fn recognizes_clap15_rename_long_numeric_tags() {
        let m = parse_opts().get_matches_from(vec!["tp", "--rename", "100", "200"]);
        let v: Vec<String> = m.get_many(RENAME).unwrap().cloned().collect();
        assert_eq!(v, vec!["100", "200"]);
    }

    #[test]
    fn recognizes_clap15_diff_long_both_at_names() {
        let m = parse_opts().get_matches_from(vec!["tp", "--diff", "@a", "@b"]);
        let v: Vec<String> = m.get_many(DIFF).unwrap().cloned().collect();
        assert_eq!(v, vec!["@a", "@b"]);
    }

    #[test]
    fn recognizes_clap15_swap_short_identical_args() {
        let m = parse_opts().get_matches_from(vec!["tp", "-S", "1", "1"]);
        let v: Vec<String> = m.get_many(SWAP).unwrap().cloned().collect();
        assert_eq!(v, vec!["1", "1"]);
    }

    #[test]
    fn recognizes_clap15_move_short_identical_args() {
        let m = parse_opts().get_matches_from(vec!["tp", "-M", "2", "2"]);
        let v: Vec<String> = m.get_many(MOVE).unwrap().cloned().collect();
        assert_eq!(v, vec!["2", "2"]);
    }

    #[test]
    fn recognizes_clap15_verbose_fourteen_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-vvvvvvvvvvvvvv"]);
        assert_eq!(m.get_count(VERBOSE), 14);
    }

    #[test]
    fn recognizes_clap15_double_dash_positional_percent() {
        let m = parse_opts().get_matches_from(vec!["tp", "--", "%TEMP%"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("%TEMP%")
        );
    }

    #[test]
    fn recognizes_clap15_info_edit_output_chain() {
        let m = parse_opts().get_matches_from(vec!["tp", "-I", "1", "-e", "2", "-o", "3"]);
        assert_eq!(m.get_one::<String>(INFO).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(EDIT).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("3"));
    }

    #[test]
    fn recognizes_clap15_grep_info_path_chain() {
        let m = parse_opts().get_matches_from(vec!["tp", "-g", "pat", "-I", "1", "--path", "1"]);
        assert_eq!(m.get_one::<String>(GREP).map(|s| s.as_str()), Some("pat"));
        assert_eq!(m.get_one::<String>(INFO).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_clap15_count_pop_shift_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-k", "-p", "-s"]);
        assert!(m.get_flag(COUNT));
        assert!(m.get_flag(POP));
        assert!(m.get_flag(SHIFT));
    }

    #[test]
    fn recognizes_clap15_list_all_short_ln_ln() {
        let m = parse_opts().get_matches_from(vec!["tp", "-l", "-n", "-L", "-N"]);
        assert!(m.get_flag(LIST_FILES));
        assert!(m.get_flag(LIST_FILES_NUMBERED));
        assert!(m.get_flag(LIST_CONTENTS));
        assert!(m.get_flag(LIST_CONTENTS_NUMBERED));
    }

    #[test]
    fn recognizes_clap15_unshift_rev_quiet_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--unshift", "--rev", "--quiet"]);
        assert!(m.get_flag(UNSHIFT));
        assert!(m.get_flag(REVERSE));
        assert!(m.get_flag(SILENT));
    }

    #[test]
    fn recognizes_clap15_add_remove_both_long_equals() {
        let m = parse_opts().get_matches_from(vec!["tp", "--add=-1", "--remove=-2"]);
        assert_eq!(m.get_one::<String>(ADD).map(|s| s.as_str()), Some("-1"));
        assert_eq!(m.get_one::<String>(REMOVE).map(|s| s.as_str()), Some("-2"));
    }

    #[test]
    fn recognizes_clap15_wc_path_size_short_long_mix() {
        let m =
            parse_opts().get_matches_from(vec!["tp", "--wc", "3", "--path", "3", "--size", "3"]);
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("3"));
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("3"));
        assert_eq!(m.get_one::<String>(SIZE).map(|s| s.as_str()), Some("3"));
    }

    #[test]
    fn recognizes_clap15_expire_rev_count_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--expire", "1", "--rev", "--count"]);
        assert_eq!(m.get_one::<String>(EXPIRE).map(|s| s.as_str()), Some("1"));
        assert!(m.get_flag(REVERSE));
        assert!(m.get_flag(COUNT));
    }

    #[test]
    fn recognizes_clap15_name_output_input_long_chain() {
        let m = parse_opts()
            .get_matches_from(vec!["tp", "--name", "t", "--output", "1", "--input", "2"]);
        assert_eq!(m.get_one::<String>(TAG).map(|s| s.as_str()), Some("t"));
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("2"));
    }

    #[test]
    fn recognizes_clap15_append_short_equals_negative() {
        let m = parse_opts().get_matches_from(vec!["tp", "-A=-1"]);
        assert_eq!(m.get_one::<String>(APPEND).map(|s| s.as_str()), Some("-1"));
    }

    #[test]
    fn recognizes_clap15_dup_long_equals_positive() {
        let m = parse_opts().get_matches_from(vec!["tp", "--dup=5"]);
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("5"));
    }

    #[test]
    fn recognizes_clap15_clear_master_directory_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-c", "-m", "-d"]);
        assert!(m.get_flag(CLEAR));
        assert!(m.get_flag(MASTER));
        assert!(m.get_flag(DIRECTORY));
    }

    #[test]
    fn recognizes_clap15_tail_head_order_independent_values() {
        let m = parse_opts().get_matches_from(vec!["tp", "--head", "3", "10", "--tail", "3", "2"]);
        assert_eq!(
            m.get_many::<String>(HEAD)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["3", "10"]
        );
        assert_eq!(
            m.get_many::<String>(TAIL)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["3", "2"]
        );
    }

    #[test]
    fn recognizes_clap15_sort_mtime_explicit_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort", "mtime"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("mtime"));
    }

    #[test]
    fn recognizes_clap15_positional_only_brackets() {
        let m = parse_opts().get_matches_from(vec!["tp", "[draft].txt"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("[draft].txt")
        );
    }

    #[test]
    fn recognizes_clap15_grep_long_bracket_class() {
        let m = parse_opts().get_matches_from(vec!["tp", "--grep", "[abc]"]);
        assert_eq!(m.get_one::<String>(GREP).map(|s| s.as_str()), Some("[abc]"));
    }

    #[test]
    fn recognizes_clap15_replace_crlf_pattern() {
        let m = parse_opts().get_matches_from(vec!["tp", "--replace", "1", "\r\n", "\n"]);
        let v: Vec<String> = m.get_many(REPLACE).unwrap().cloned().collect();
        assert_eq!(v, vec!["1", "\r\n", "\n"]);
    }

    #[test]
    fn recognizes_clap15_cat_ten_indices_long() {
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

    // ── clap coverage round 16 ─────────────────────────────

    #[test]
    fn recognizes_clap16_positional_url_scheme() {
        let m = parse_opts().get_matches_from(vec!["tp", "https://example.com/x"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("https://example.com/x")
        );
    }

    #[test]
    fn recognizes_clap16_output_long_equals_at_name() {
        let m = parse_opts().get_matches_from(vec!["tp", "--output=@mytag"]);
        assert_eq!(
            m.get_one::<String>(OUTPUT).map(|s| s.as_str()),
            Some("@mytag")
        );
    }

    #[test]
    fn recognizes_clap16_input_short_equals_negative() {
        let m = parse_opts().get_matches_from(vec!["tp", "-i=-2"]);
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("-2"));
    }

    #[test]
    fn recognizes_clap16_cat_eleven_indices_long() {
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
    fn recognizes_clap16_grep_escaped_parens() {
        let m = parse_opts().get_matches_from(vec!["tp", "-g", r"\(\)"]);
        assert_eq!(m.get_one::<String>(GREP).map(|s| s.as_str()), Some(r"\(\)"));
    }

    #[test]
    fn recognizes_clap16_expire_long_plus_prefix() {
        let m = parse_opts().get_matches_from(vec!["tp", "--expire", "+1"]);
        assert_eq!(m.get_one::<String>(EXPIRE).map(|s| s.as_str()), Some("+1"));
    }

    #[test]
    fn recognizes_clap16_sort_default_then_second_positional_arg() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort", "name", "extra.txt"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("name"));
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("extra.txt")
        );
    }

    #[test]
    fn recognizes_clap16_rev_pop_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "--rev", "-p"]);
        assert!(m.get_flag(REVERSE));
        assert!(m.get_flag(POP));
    }

    #[test]
    fn recognizes_clap16_dup_append_input_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-x", "1", "-A", "1", "-i", "1"]);
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(APPEND).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_clap16_head_only_long_minimal() {
        let m = parse_opts().get_matches_from(vec!["tp", "--head", "1", "0"]);
        assert_eq!(
            m.get_many::<String>(HEAD)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["1", "0"]
        );
    }

    #[test]
    fn recognizes_clap16_tail_wc_chain_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--tail", "1", "3", "--wc", "1"]);
        assert_eq!(
            m.get_many::<String>(TAIL)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["1", "3"]
        );
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_clap16_path_expire_size_chain() {
        let m = parse_opts()
            .get_matches_from(vec!["tp", "--path", "1", "--expire", "0", "--size", "1"]);
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(EXPIRE).map(|s| s.as_str()), Some("0"));
        assert_eq!(m.get_one::<String>(SIZE).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_clap16_replace_three_args_slash_in_pattern() {
        let m = parse_opts().get_matches_from(vec!["tp", "--replace", "1", "a/b", "c"]);
        let v: Vec<String> = m.get_many(REPLACE).unwrap().cloned().collect();
        assert_eq!(v, vec!["1", "a/b", "c"]);
    }

    #[test]
    fn recognizes_clap16_rename_long_spaces_in_tags() {
        let m = parse_opts().get_matches_from(vec!["tp", "--rename", "a b", "c d"]);
        let v: Vec<String> = m.get_many(RENAME).unwrap().cloned().collect();
        assert_eq!(v, vec!["a b", "c d"]);
    }

    #[test]
    fn recognizes_clap16_diff_long_hex_strings() {
        let m = parse_opts().get_matches_from(vec!["tp", "--diff", "0x1", "0x2"]);
        let v: Vec<String> = m.get_many(DIFF).unwrap().cloned().collect();
        assert_eq!(v, vec!["0x1", "0x2"]);
    }

    #[test]
    fn recognizes_clap16_swap_move_short_chain() {
        let m = parse_opts().get_matches_from(vec!["tp", "-S", "1", "2", "-M", "3", "4"]);
        assert_eq!(
            m.get_many(SWAP).unwrap().cloned().collect::<Vec<String>>(),
            vec!["1", "2"]
        );
        assert_eq!(
            m.get_many(MOVE).unwrap().cloned().collect::<Vec<String>>(),
            vec!["3", "4"]
        );
    }

    #[test]
    fn recognizes_clap16_verbose_fifteen_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-vvvvvvvvvvvvvvv"]);
        assert_eq!(m.get_count(VERBOSE), 15);
    }

    #[test]
    fn recognizes_clap16_double_dash_positional_semicolon() {
        let m = parse_opts().get_matches_from(vec!["tp", "--", "a;b"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("a;b")
        );
    }

    #[test]
    fn recognizes_clap16_info_wc_path_long() {
        let m =
            parse_opts().get_matches_from(vec!["tp", "--info", "1", "--wc", "1", "--path", "1"]);
        assert_eq!(m.get_one::<String>(INFO).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_clap16_edit_grep_dup_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-e", "1", "-g", "x", "-x", "2"]);
        assert_eq!(m.get_one::<String>(EDIT).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(GREP).map(|s| s.as_str()), Some("x"));
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("2"));
    }

    #[test]
    fn recognizes_clap16_count_list_numbered_chain() {
        let m = parse_opts().get_matches_from(vec!["tp", "-k", "-n"]);
        assert!(m.get_flag(COUNT));
        assert!(m.get_flag(LIST_FILES_NUMBERED));
    }

    #[test]
    fn recognizes_clap16_clear_list_contents_chain() {
        let m = parse_opts().get_matches_from(vec!["tp", "-c", "-L"]);
        assert!(m.get_flag(CLEAR));
        assert!(m.get_flag(LIST_CONTENTS));
    }

    #[test]
    fn recognizes_clap16_shift_unshift_both_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-s", "-u"]);
        assert!(m.get_flag(SHIFT));
        assert!(m.get_flag(UNSHIFT));
    }

    #[test]
    fn recognizes_clap16_add_remove_output_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-a", "1", "-r", "2", "-o", "3"]);
        assert_eq!(m.get_one::<String>(ADD).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(REMOVE).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("3"));
    }

    #[test]
    fn recognizes_clap16_name_short_equals_unicode() {
        let m = parse_opts().get_matches_from(vec!["tp", "-w=标签"]);
        assert_eq!(m.get_one::<String>(TAG).map(|s| s.as_str()), Some("标签"));
    }

    #[test]
    fn recognizes_clap16_input_long_equals_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "--input=1"]);
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_clap16_output_short_equals_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "-o=2"]);
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("2"));
    }

    #[test]
    fn recognizes_clap16_remove_short_equals() {
        let m = parse_opts().get_matches_from(vec!["tp", "-r=4"]);
        assert_eq!(m.get_one::<String>(REMOVE).map(|s| s.as_str()), Some("4"));
    }

    #[test]
    fn recognizes_clap16_add_short_equals() {
        let m = parse_opts().get_matches_from(vec!["tp", "-a=5"]);
        assert_eq!(m.get_one::<String>(ADD).map(|s| s.as_str()), Some("5"));
    }

    #[test]
    fn recognizes_clap16_grep_short_equals_pattern() {
        let m = parse_opts().get_matches_from(vec!["tp", "-g=foo"]);
        assert_eq!(m.get_one::<String>(GREP).map(|s| s.as_str()), Some("foo"));
    }

    #[test]
    fn recognizes_clap16_info_short_equals_index() {
        let m = parse_opts().get_matches_from(vec!["tp", "-I=3"]);
        assert_eq!(m.get_one::<String>(INFO).map(|s| s.as_str()), Some("3"));
    }

    #[test]
    fn recognizes_clap16_edit_short_equals_index() {
        let m = parse_opts().get_matches_from(vec!["tp", "-e=4"]);
        assert_eq!(m.get_one::<String>(EDIT).map(|s| s.as_str()), Some("4"));
    }

    #[test]
    fn recognizes_clap16_quiet_verbose_mixed_long_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "--quiet", "-vv"]);
        assert!(m.get_flag(SILENT));
        assert_eq!(m.get_count(VERBOSE), 2);
    }

    #[test]
    fn recognizes_clap16_master_only_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--master"]);
        assert!(m.get_flag(MASTER));
    }

    #[test]
    fn recognizes_clap16_directory_only_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--dir"]);
        assert!(m.get_flag(DIRECTORY));
    }

    #[test]
    fn recognizes_clap16_replace_three_args_only_spaces() {
        let m = parse_opts().get_matches_from(vec!["tp", "--replace", "1", " ", "  "]);
        let v: Vec<String> = m.get_many(REPLACE).unwrap().cloned().collect();
        assert_eq!(v, vec!["1", " ", "  "]);
    }

    // ── clap coverage round 17 ─────────────────────────────

    #[test]
    fn recognizes_clap17_positional_query_string() {
        let m = parse_opts().get_matches_from(vec!["tp", "file.txt?raw=1"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("file.txt?raw=1")
        );
    }

    #[test]
    fn recognizes_clap17_output_input_long_both_equals() {
        let m = parse_opts().get_matches_from(vec!["tp", "--output=9", "--input=8"]);
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("9"));
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("8"));
    }

    #[test]
    fn recognizes_clap17_cat_twelve_indices_long() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--cat", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12",
        ]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(
            v,
            vec![
                "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12"
            ]
        );
    }

    #[test]
    fn recognizes_clap17_grep_curly_braces_alternation() {
        let m = parse_opts().get_matches_from(vec!["tp", "-g", r"{foo,bar}"]);
        assert_eq!(
            m.get_one::<String>(GREP).map(|s| s.as_str()),
            Some(r"{foo,bar}")
        );
    }

    #[test]
    fn recognizes_clap17_expire_infinity_string() {
        let m = parse_opts().get_matches_from(vec!["tp", "--expire", "inf"]);
        assert_eq!(m.get_one::<String>(EXPIRE).map(|s| s.as_str()), Some("inf"));
    }

    #[test]
    fn recognizes_clap17_sort_size_then_positional() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort", "size", "data.bin"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("size"));
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("data.bin")
        );
    }

    #[test]
    fn recognizes_clap17_clear_rev_count_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--clear", "--rev", "--count"]);
        assert!(m.get_flag(CLEAR));
        assert!(m.get_flag(REVERSE));
        assert!(m.get_flag(COUNT));
    }

    #[test]
    fn recognizes_clap17_dup_wc_path_short_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "-x", "1", "--wc", "1", "--path", "1"]);
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_clap17_head_tail_both_negative_indices() {
        let m = parse_opts().get_matches_from(vec!["tp", "--head", "-1", "5", "--tail", "-1", "3"]);
        assert_eq!(
            m.get_many::<String>(HEAD)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["-1", "5"]
        );
        assert_eq!(
            m.get_many::<String>(TAIL)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["-1", "3"]
        );
    }

    #[test]
    fn recognizes_clap17_size_wc_only_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--size", "2", "--wc", "2"]);
        assert_eq!(m.get_one::<String>(SIZE).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("2"));
    }

    #[test]
    fn recognizes_clap17_replace_three_args_backtick() {
        let m = parse_opts().get_matches_from(vec!["tp", "--replace", "1", "`", "'"]);
        let v: Vec<String> = m.get_many(REPLACE).unwrap().cloned().collect();
        assert_eq!(v, vec!["1", "`", "'"]);
    }

    #[test]
    fn recognizes_clap17_diff_swap_long_pairs() {
        let m = parse_opts().get_matches_from(vec!["tp", "--diff", "a", "b", "--swap", "c", "d"]);
        assert_eq!(
            m.get_many(DIFF).unwrap().cloned().collect::<Vec<String>>(),
            vec!["a", "b"]
        );
        assert_eq!(
            m.get_many(SWAP).unwrap().cloned().collect::<Vec<String>>(),
            vec!["c", "d"]
        );
    }

    #[test]
    fn recognizes_clap17_move_long_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "--mv", "1", "2"]);
        let v: Vec<String> = m.get_many(MOVE).unwrap().cloned().collect();
        assert_eq!(v, vec!["1", "2"]);
    }

    #[test]
    fn recognizes_clap17_verbose_sixteen_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-vvvvvvvvvvvvvvvv"]);
        assert_eq!(m.get_count(VERBOSE), 16);
    }

    #[test]
    fn recognizes_clap17_double_dash_positional_only_comma() {
        let m = parse_opts().get_matches_from(vec!["tp", "--", ","]);
        assert_eq!(m.get_one::<String>(ARGFILE).map(|s| s.as_str()), Some(","));
    }

    #[test]
    fn recognizes_clap17_append_expire_path_long() {
        let m = parse_opts()
            .get_matches_from(vec!["tp", "--append", "1", "--expire", "2", "--path", "1"]);
        assert_eq!(m.get_one::<String>(APPEND).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(EXPIRE).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_clap17_info_output_append_chain() {
        let m = parse_opts().get_matches_from(vec!["tp", "-I", "1", "-o", "2", "-A", "3"]);
        assert_eq!(m.get_one::<String>(INFO).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(APPEND).map(|s| s.as_str()), Some("3"));
    }

    #[test]
    fn recognizes_clap17_grep_cat_short_chain() {
        let m = parse_opts().get_matches_from(vec!["tp", "-g", "x", "-C", "1", "2"]);
        assert_eq!(m.get_one::<String>(GREP).map(|s| s.as_str()), Some("x"));
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(v, vec!["1", "2"]);
    }

    #[test]
    fn recognizes_clap17_pop_unshift_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-p", "-u"]);
        assert!(m.get_flag(POP));
        assert!(m.get_flag(UNSHIFT));
    }

    #[test]
    fn recognizes_clap17_list_numbered_contents_numbered_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-n", "-N"]);
        assert!(m.get_flag(LIST_FILES_NUMBERED));
        assert!(m.get_flag(LIST_CONTENTS_NUMBERED));
    }

    #[test]
    fn recognizes_clap17_add_remove_append_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-a", "1", "-r", "2", "-A", "3"]);
        assert_eq!(m.get_one::<String>(ADD).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(REMOVE).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(APPEND).map(|s| s.as_str()), Some("3"));
    }

    #[test]
    fn recognizes_clap17_name_long_equals_quoted_looking() {
        let m = parse_opts().get_matches_from(vec!["tp", "--name=\"quoted\""]);
        assert_eq!(
            m.get_one::<String>(TAG).map(|s| s.as_str()),
            Some("\"quoted\"")
        );
    }

    #[test]
    fn recognizes_clap17_edit_long_equals_at() {
        let m = parse_opts().get_matches_from(vec!["tp", "--edit=@x"]);
        assert_eq!(m.get_one::<String>(EDIT).map(|s| s.as_str()), Some("@x"));
    }

    #[test]
    fn recognizes_clap17_path_long_equals_negative() {
        let m = parse_opts().get_matches_from(vec!["tp", "--path=-3"]);
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("-3"));
    }

    #[test]
    fn recognizes_clap17_size_long_equals_zero() {
        let m = parse_opts().get_matches_from(vec!["tp", "--size=0"]);
        assert_eq!(m.get_one::<String>(SIZE).map(|s| s.as_str()), Some("0"));
    }

    #[test]
    fn recognizes_clap17_wc_long_equals_zero() {
        let m = parse_opts().get_matches_from(vec!["tp", "--wc=0"]);
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("0"));
    }

    #[test]
    fn recognizes_clap17_expire_long_equals_zero() {
        let m = parse_opts().get_matches_from(vec!["tp", "--expire=0"]);
        assert_eq!(m.get_one::<String>(EXPIRE).map(|s| s.as_str()), Some("0"));
    }

    #[test]
    fn recognizes_clap17_tail_head_wc_long_chain() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--tail", "1", "1", "--head", "1", "1", "--wc", "1",
        ]);
        assert_eq!(
            m.get_many::<String>(TAIL)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["1", "1"]
        );
        assert_eq!(
            m.get_many::<String>(HEAD)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["1", "1"]
        );
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_clap17_diff_short_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "-D", "1", "2"]);
        let v: Vec<String> = m.get_many(DIFF).unwrap().cloned().collect();
        assert_eq!(v, vec!["1", "2"]);
    }

    #[test]
    fn recognizes_clap17_swap_long_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "--swap", "3", "4"]);
        let v: Vec<String> = m.get_many(SWAP).unwrap().cloned().collect();
        assert_eq!(v, vec!["3", "4"]);
    }

    #[test]
    fn recognizes_clap17_rename_short_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "-R", "x", "y"]);
        let v: Vec<String> = m.get_many(RENAME).unwrap().cloned().collect();
        assert_eq!(v, vec!["x", "y"]);
    }

    #[test]
    fn recognizes_clap17_replace_long_three_args() {
        let m = parse_opts().get_matches_from(vec!["tp", "--replace", "9", "old", "new"]);
        let v: Vec<String> = m.get_many(REPLACE).unwrap().cloned().collect();
        assert_eq!(v, vec!["9", "old", "new"]);
    }

    #[test]
    fn recognizes_clap17_count_only_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-k"]);
        assert!(m.get_flag(COUNT));
    }

    #[test]
    fn recognizes_clap17_rev_only_short_alias() {
        let m = parse_opts().get_matches_from(vec!["tp", "--rev"]);
        assert!(m.get_flag(REVERSE));
    }

    // ── clap coverage round 18 ─────────────────────────────

    #[test]
    fn recognizes_clap18_positional_fragment_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "#section"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("#section")
        );
    }

    #[test]
    fn recognizes_clap18_output_short_at_tag() {
        let m = parse_opts().get_matches_from(vec!["tp", "-o", "@foo"]);
        assert_eq!(
            m.get_one::<String>(OUTPUT).map(|s| s.as_str()),
            Some("@foo")
        );
    }

    #[test]
    fn recognizes_clap18_input_long_space_multibyte() {
        let m = parse_opts().get_matches_from(vec!["tp", "--input", "索引"]);
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("索引"));
    }

    #[test]
    fn recognizes_clap18_cat_thirteen_indices_long() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--cat", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13",
        ]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(
            v,
            vec![
                "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13"
            ]
        );
    }

    #[test]
    fn recognizes_clap18_grep_escaped_brackets() {
        let m = parse_opts().get_matches_from(vec!["tp", "-g", r"\[x\]"]);
        assert_eq!(
            m.get_one::<String>(GREP).map(|s| s.as_str()),
            Some(r"\[x\]")
        );
    }

    #[test]
    fn recognizes_clap18_expire_nan_string() {
        let m = parse_opts().get_matches_from(vec!["tp", "--expire", "NaN"]);
        assert_eq!(m.get_one::<String>(EXPIRE).map(|s| s.as_str()), Some("NaN"));
    }

    #[test]
    fn recognizes_clap18_sort_mtime_then_positional() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort", "mtime", "sortme.txt"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("mtime"));
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("sortme.txt")
        );
    }

    #[test]
    fn recognizes_clap18_pop_shift_rev_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-p", "-s", "--rev"]);
        assert!(m.get_flag(POP));
        assert!(m.get_flag(SHIFT));
        assert!(m.get_flag(REVERSE));
    }

    #[test]
    fn recognizes_clap18_append_dup_remove_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-A", "1", "-x", "2", "-r", "3"]);
        assert_eq!(m.get_one::<String>(APPEND).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(REMOVE).map(|s| s.as_str()), Some("3"));
    }

    #[test]
    fn recognizes_clap18_head_tail_max_line_counts() {
        let m =
            parse_opts().get_matches_from(vec!["tp", "--head", "1", "999999", "--tail", "1", "1"]);
        assert_eq!(
            m.get_many::<String>(HEAD)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["1", "999999"]
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
    fn recognizes_clap18_path_size_expire_wc_long() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--path", "1", "--size", "1", "--expire", "1", "--wc", "1",
        ]);
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(SIZE).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(EXPIRE).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_clap18_replace_three_args_unicode_replacement() {
        let m = parse_opts().get_matches_from(vec!["tp", "--replace", "1", "a", "β"]);
        let v: Vec<String> = m.get_many(REPLACE).unwrap().cloned().collect();
        assert_eq!(v, vec!["1", "a", "β"]);
    }

    #[test]
    fn recognizes_clap18_diff_move_long_pairs() {
        let m = parse_opts().get_matches_from(vec!["tp", "--diff", "1", "2", "--mv", "3", "4"]);
        assert_eq!(
            m.get_many(DIFF).unwrap().cloned().collect::<Vec<String>>(),
            vec!["1", "2"]
        );
        assert_eq!(
            m.get_many(MOVE).unwrap().cloned().collect::<Vec<String>>(),
            vec!["3", "4"]
        );
    }

    #[test]
    fn recognizes_clap18_swap_short_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "-S", "9", "8"]);
        let v: Vec<String> = m.get_many(SWAP).unwrap().cloned().collect();
        assert_eq!(v, vec!["9", "8"]);
    }

    #[test]
    fn recognizes_clap18_verbose_seventeen_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-vvvvvvvvvvvvvvvvv"]);
        assert_eq!(m.get_count(VERBOSE), 17);
    }

    #[test]
    fn recognizes_clap18_double_dash_positional_equals_in_name() {
        let m = parse_opts().get_matches_from(vec!["tp", "--", "a=b.txt"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("a=b.txt")
        );
    }

    #[test]
    fn recognizes_clap18_edit_info_path_chain() {
        let m = parse_opts().get_matches_from(vec!["tp", "-e", "1", "-I", "2", "--path", "3"]);
        assert_eq!(m.get_one::<String>(EDIT).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(INFO).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("3"));
    }

    #[test]
    fn recognizes_clap18_grep_wc_dup_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-g", "pat", "--wc", "1", "-x", "2"]);
        assert_eq!(m.get_one::<String>(GREP).map(|s| s.as_str()), Some("pat"));
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("2"));
    }

    #[test]
    fn recognizes_clap18_count_master_directory_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-k", "-m", "-d"]);
        assert!(m.get_flag(COUNT));
        assert!(m.get_flag(MASTER));
        assert!(m.get_flag(DIRECTORY));
    }

    #[test]
    fn recognizes_clap18_clear_pop_verbose_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-c", "-p", "-vv"]);
        assert!(m.get_flag(CLEAR));
        assert!(m.get_flag(POP));
        assert_eq!(m.get_count(VERBOSE), 2);
    }

    #[test]
    fn recognizes_clap18_list_files_list_contents_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-l", "-L"]);
        assert!(m.get_flag(LIST_FILES));
        assert!(m.get_flag(LIST_CONTENTS));
    }

    #[test]
    fn recognizes_clap18_add_output_input_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-a", "1", "-o", "2", "-i", "3"]);
        assert_eq!(m.get_one::<String>(ADD).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("3"));
    }

    #[test]
    fn recognizes_clap18_name_short_space_value() {
        let m = parse_opts().get_matches_from(vec!["tp", "-w", "my name"]);
        assert_eq!(
            m.get_one::<String>(TAG).map(|s| s.as_str()),
            Some("my name")
        );
    }

    #[test]
    fn recognizes_clap18_unshift_quiet_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--unshift", "--quiet"]);
        assert!(m.get_flag(UNSHIFT));
        assert!(m.get_flag(SILENT));
    }

    #[test]
    fn recognizes_clap18_shift_count_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-s", "-k"]);
        assert!(m.get_flag(SHIFT));
        assert!(m.get_flag(COUNT));
    }

    #[test]
    fn recognizes_clap18_dup_long_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "--dup", "7"]);
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("7"));
    }

    #[test]
    fn recognizes_clap18_append_long_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "--append", "6"]);
        assert_eq!(m.get_one::<String>(APPEND).map(|s| s.as_str()), Some("6"));
    }

    #[test]
    fn recognizes_clap18_remove_long_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "--remove", "5"]);
        assert_eq!(m.get_one::<String>(REMOVE).map(|s| s.as_str()), Some("5"));
    }

    #[test]
    fn recognizes_clap18_add_long_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "--add", "4"]);
        assert_eq!(m.get_one::<String>(ADD).map(|s| s.as_str()), Some("4"));
    }

    #[test]
    fn recognizes_clap18_cat_short_two_mixed_long_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-C", "1", "2"]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(v, vec!["1", "2"]);
    }

    #[test]
    fn recognizes_clap18_quiet_long_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "--quiet"]);
        assert!(m.get_flag(SILENT));
    }

    #[test]
    fn recognizes_clap18_verbose_long_triple() {
        let m = parse_opts().get_matches_from(vec!["tp", "--verbose", "--verbose", "--verbose"]);
        assert_eq!(m.get_count(VERBOSE), 3);
    }

    #[test]
    fn recognizes_clap18_rev_expire_sort_long() {
        let m =
            parse_opts().get_matches_from(vec!["tp", "--rev", "--expire", "1", "--sort", "size"]);
        assert!(m.get_flag(REVERSE));
        assert_eq!(m.get_one::<String>(EXPIRE).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("size"));
    }

    // ── clap coverage round 19 ─────────────────────────────

    #[test]
    fn recognizes_clap19_positional_mailto_uri() {
        let m = parse_opts().get_matches_from(vec!["tp", "mailto:user@example.com"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("mailto:user@example.com")
        );
    }

    #[test]
    fn recognizes_clap19_output_long_space_positive() {
        let m = parse_opts().get_matches_from(vec!["tp", "--output", "100"]);
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("100"));
    }

    #[test]
    fn recognizes_clap19_input_short_positive_large() {
        let m = parse_opts().get_matches_from(vec!["tp", "-i", "5000"]);
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("5000"));
    }

    #[test]
    fn recognizes_clap19_cat_fourteen_indices_long() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--cat", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13",
            "14",
        ]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(
            v,
            vec![
                "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14"
            ]
        );
    }

    #[test]
    fn recognizes_clap19_grep_escaped_dollar() {
        let m = parse_opts().get_matches_from(vec!["tp", "-g", r"\$"]);
        assert_eq!(m.get_one::<String>(GREP).map(|s| s.as_str()), Some(r"\$"));
    }

    #[test]
    fn recognizes_clap19_expire_underscore_numeric() {
        let m = parse_opts().get_matches_from(vec!["tp", "--expire", "1_000"]);
        assert_eq!(
            m.get_one::<String>(EXPIRE).map(|s| s.as_str()),
            Some("1_000")
        );
    }

    #[test]
    fn recognizes_clap19_sort_explicit_then_positional() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort", "name", "a.txt"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("name"));
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("a.txt")
        );
    }

    #[test]
    fn recognizes_clap19_clear_count_rev_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-c", "-k", "--rev"]);
        assert!(m.get_flag(CLEAR));
        assert!(m.get_flag(COUNT));
        assert!(m.get_flag(REVERSE));
    }

    #[test]
    fn recognizes_clap19_dup_append_output_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-x", "1", "-A", "2", "-o", "3"]);
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(APPEND).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("3"));
    }

    #[test]
    fn recognizes_clap19_head_wc_tail_long() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--head", "1", "2", "--wc", "1", "--tail", "1", "3",
        ]);
        assert_eq!(
            m.get_many::<String>(HEAD)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["1", "2"]
        );
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("1"));
        assert_eq!(
            m.get_many::<String>(TAIL)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["1", "3"]
        );
    }

    #[test]
    fn recognizes_clap19_path_expire_only_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--path", "1", "--expire", "24"]);
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(EXPIRE).map(|s| s.as_str()), Some("24"));
    }

    #[test]
    fn recognizes_clap19_replace_three_args_tab_in_replacement() {
        let m = parse_opts().get_matches_from(vec!["tp", "--replace", "1", "x", "\t"]);
        let v: Vec<String> = m.get_many(REPLACE).unwrap().cloned().collect();
        assert_eq!(v, vec!["1", "x", "\t"]);
    }

    #[test]
    fn recognizes_clap19_rename_long_emoji_tags() {
        let m = parse_opts().get_matches_from(vec!["tp", "--rename", "🙂", "🎉"]);
        let v: Vec<String> = m.get_many(RENAME).unwrap().cloned().collect();
        assert_eq!(v, vec!["🙂", "🎉"]);
    }

    #[test]
    fn recognizes_clap19_diff_swap_move_long_chain() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--diff", "1", "2", "--swap", "3", "4", "--mv", "5", "6",
        ]);
        assert_eq!(
            m.get_many(DIFF).unwrap().cloned().collect::<Vec<String>>(),
            vec!["1", "2"]
        );
        assert_eq!(
            m.get_many(SWAP).unwrap().cloned().collect::<Vec<String>>(),
            vec!["3", "4"]
        );
        assert_eq!(
            m.get_many(MOVE).unwrap().cloned().collect::<Vec<String>>(),
            vec!["5", "6"]
        );
    }

    #[test]
    fn recognizes_clap19_verbose_eighteen_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-vvvvvvvvvvvvvvvvvv"]);
        assert_eq!(m.get_count(VERBOSE), 18);
    }

    #[test]
    fn recognizes_clap19_double_dash_positional_bang() {
        let m = parse_opts().get_matches_from(vec!["tp", "--", "!tmp"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("!tmp")
        );
    }

    #[test]
    fn recognizes_clap19_info_grep_edit_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-I", "1", "-g", "x", "-e", "2"]);
        assert_eq!(m.get_one::<String>(INFO).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(GREP).map(|s| s.as_str()), Some("x"));
        assert_eq!(m.get_one::<String>(EDIT).map(|s| s.as_str()), Some("2"));
    }

    #[test]
    fn recognizes_clap19_wc_size_path_short_long() {
        let m =
            parse_opts().get_matches_from(vec!["tp", "--wc", "1", "--size", "1", "--path", "1"]);
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(SIZE).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_clap19_list_numbered_contents_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-n", "-L"]);
        assert!(m.get_flag(LIST_FILES_NUMBERED));
        assert!(m.get_flag(LIST_CONTENTS));
    }

    #[test]
    fn recognizes_clap19_pop_shift_unshift_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-p", "-s", "-u"]);
        assert!(m.get_flag(POP));
        assert!(m.get_flag(SHIFT));
        assert!(m.get_flag(UNSHIFT));
    }

    #[test]
    fn recognizes_clap19_add_dup_remove_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-a", "1", "-x", "2", "-r", "3"]);
        assert_eq!(m.get_one::<String>(ADD).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(REMOVE).map(|s| s.as_str()), Some("3"));
    }

    #[test]
    fn recognizes_clap19_name_long_space_value() {
        let m = parse_opts().get_matches_from(vec!["tp", "--name", "tag with spaces"]);
        assert_eq!(
            m.get_one::<String>(TAG).map(|s| s.as_str()),
            Some("tag with spaces")
        );
    }

    #[test]
    fn recognizes_clap19_input_output_append_long() {
        let m = parse_opts()
            .get_matches_from(vec!["tp", "--input", "1", "--output", "2", "--append", "3"]);
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(APPEND).map(|s| s.as_str()), Some("3"));
    }

    #[test]
    fn recognizes_clap19_master_verbose_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-m", "-vv"]);
        assert!(m.get_flag(MASTER));
        assert_eq!(m.get_count(VERBOSE), 2);
    }

    #[test]
    fn recognizes_clap19_directory_verbose_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-d", "-v"]);
        assert!(m.get_flag(DIRECTORY));
        assert_eq!(m.get_count(VERBOSE), 1);
    }

    #[test]
    fn recognizes_clap19_quiet_verbose_both_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--quiet", "--verbose"]);
        assert!(m.get_flag(SILENT));
        assert_eq!(m.get_count(VERBOSE), 1);
    }

    #[test]
    fn recognizes_clap19_cat_short_three_indices() {
        let m = parse_opts().get_matches_from(vec!["tp", "-C", "a", "b", "c"]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(v, vec!["a", "b", "c"]);
    }

    #[test]
    fn recognizes_clap19_expire_path_wc_size_long() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--expire", "1", "--path", "1", "--wc", "1", "--size", "1",
        ]);
        assert_eq!(m.get_one::<String>(EXPIRE).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(SIZE).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_clap19_rev_sort_name_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--rev", "--sort", "name"]);
        assert!(m.get_flag(REVERSE));
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("name"));
    }

    #[test]
    fn recognizes_clap19_positional_dotfiles() {
        let m = parse_opts().get_matches_from(vec!["tp", ".hidden"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some(".hidden")
        );
    }

    #[test]
    fn recognizes_clap19_grep_long_wildcard() {
        let m = parse_opts().get_matches_from(vec!["tp", "--grep", "*.rs"]);
        assert_eq!(m.get_one::<String>(GREP).map(|s| s.as_str()), Some("*.rs"));
    }

    #[test]
    fn recognizes_clap19_shift_only_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--shift"]);
        assert!(m.get_flag(SHIFT));
    }

    #[test]
    fn recognizes_clap19_unshift_only_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--unshift"]);
        assert!(m.get_flag(UNSHIFT));
    }

    // ── clap coverage round 20 ─────────────────────────────

    #[test]
    fn recognizes_clap20_positional_data_uri() {
        let m = parse_opts().get_matches_from(vec!["tp", "data:,hello"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("data:,hello")
        );
    }

    #[test]
    fn recognizes_clap20_output_equals_positive() {
        let m = parse_opts().get_matches_from(vec!["tp", "--output=11"]);
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("11"));
    }

    #[test]
    fn recognizes_clap20_input_equals_negative() {
        let m = parse_opts().get_matches_from(vec!["tp", "--input=-5"]);
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("-5"));
    }

    #[test]
    fn recognizes_clap20_cat_fifteen_indices_long() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--cat", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13",
            "14", "15",
        ]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(
            v,
            vec![
                "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15"
            ]
        );
    }

    #[test]
    fn recognizes_clap20_grep_non_ascii_pattern() {
        let m = parse_opts().get_matches_from(vec!["tp", "-g", "café"]);
        assert_eq!(m.get_one::<String>(GREP).map(|s| s.as_str()), Some("café"));
    }

    #[test]
    fn recognizes_clap20_expire_iso8601_looking() {
        let m = parse_opts().get_matches_from(vec!["tp", "--expire", "2026-01-01T00:00:00Z"]);
        assert_eq!(
            m.get_one::<String>(EXPIRE).map(|s| s.as_str()),
            Some("2026-01-01T00:00:00Z")
        );
    }

    #[test]
    fn recognizes_clap20_sort_size_then_positional() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort", "size", "blob.bin"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("size"));
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("blob.bin")
        );
    }

    #[test]
    fn recognizes_clap20_clear_pop_shift_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--clear", "--pop", "--shift"]);
        assert!(m.get_flag(CLEAR));
        assert!(m.get_flag(POP));
        assert!(m.get_flag(SHIFT));
    }

    #[test]
    fn recognizes_clap20_dup_input_append_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-x", "1", "-i", "2", "-A", "3"]);
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(APPEND).map(|s| s.as_str()), Some("3"));
    }

    #[test]
    fn recognizes_clap20_wc_head_tail_long() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--wc", "1", "--head", "1", "2", "--tail", "1", "2",
        ]);
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("1"));
        assert_eq!(
            m.get_many::<String>(HEAD)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["1", "2"]
        );
        assert_eq!(
            m.get_many::<String>(TAIL)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["1", "2"]
        );
    }

    #[test]
    fn recognizes_clap20_path_wc_only_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--path", "3", "--wc", "3"]);
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("3"));
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("3"));
    }

    #[test]
    fn recognizes_clap20_replace_three_args_empty_pattern() {
        let m = parse_opts().get_matches_from(vec!["tp", "--replace", "1", "", "x"]);
        let v: Vec<String> = m.get_many(REPLACE).unwrap().cloned().collect();
        assert_eq!(v, vec!["1", "", "x"]);
    }

    #[test]
    fn recognizes_clap20_rename_short_equals_style() {
        let m = parse_opts().get_matches_from(vec!["tp", "-R", "from", "to"]);
        let v: Vec<String> = m.get_many(RENAME).unwrap().cloned().collect();
        assert_eq!(v, vec!["from", "to"]);
    }

    #[test]
    fn recognizes_clap20_diff_long_swap_short_mixed() {
        let m = parse_opts().get_matches_from(vec!["tp", "--diff", "a", "b", "-S", "c", "d"]);
        assert_eq!(
            m.get_many(DIFF).unwrap().cloned().collect::<Vec<String>>(),
            vec!["a", "b"]
        );
        assert_eq!(
            m.get_many(SWAP).unwrap().cloned().collect::<Vec<String>>(),
            vec!["c", "d"]
        );
    }

    #[test]
    fn recognizes_clap20_move_long_only_pair() {
        let m = parse_opts().get_matches_from(vec!["tp", "--mv", "9", "10"]);
        let v: Vec<String> = m.get_many(MOVE).unwrap().cloned().collect();
        assert_eq!(v, vec!["9", "10"]);
    }

    #[test]
    fn recognizes_clap20_verbose_nineteen_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-vvvvvvvvvvvvvvvvvvv"]);
        assert_eq!(m.get_count(VERBOSE), 19);
    }

    #[test]
    fn recognizes_clap20_double_dash_positional_tilde() {
        let m = parse_opts().get_matches_from(vec!["tp", "--", "~/file"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("~/file")
        );
    }

    #[test]
    fn recognizes_clap20_info_path_wc_short_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "-I", "1", "--path", "1", "--wc", "1"]);
        assert_eq!(m.get_one::<String>(INFO).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_clap20_grep_append_dup_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-g", "p", "-A", "1", "-x", "2"]);
        assert_eq!(m.get_one::<String>(GREP).map(|s| s.as_str()), Some("p"));
        assert_eq!(m.get_one::<String>(APPEND).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("2"));
    }

    #[test]
    fn recognizes_clap20_count_list_contents_numbered_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-k", "-N"]);
        assert!(m.get_flag(COUNT));
        assert!(m.get_flag(LIST_CONTENTS_NUMBERED));
    }

    #[test]
    fn recognizes_clap20_rev_count_quiet_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--rev", "--count", "--quiet"]);
        assert!(m.get_flag(REVERSE));
        assert!(m.get_flag(COUNT));
        assert!(m.get_flag(SILENT));
    }

    #[test]
    fn recognizes_clap20_add_remove_input_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-a", "1", "-r", "2", "-i", "3"]);
        assert_eq!(m.get_one::<String>(ADD).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(REMOVE).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("3"));
    }

    #[test]
    fn recognizes_clap20_output_input_short_equals() {
        let m = parse_opts().get_matches_from(vec!["tp", "-o=1", "-i=2"]);
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("2"));
    }

    #[test]
    fn recognizes_clap20_edit_long_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "--edit", "1"]);
        assert_eq!(m.get_one::<String>(EDIT).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_clap20_grep_long_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "--grep", "pat"]);
        assert_eq!(m.get_one::<String>(GREP).map(|s| s.as_str()), Some("pat"));
    }

    #[test]
    fn recognizes_clap20_expire_long_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "--expire", "42"]);
        assert_eq!(m.get_one::<String>(EXPIRE).map(|s| s.as_str()), Some("42"));
    }

    #[test]
    fn recognizes_clap20_sort_long_only_default() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("name"));
    }

    #[test]
    fn recognizes_clap20_cat_short_four_indices() {
        let m = parse_opts().get_matches_from(vec!["tp", "-C", "1", "2", "3", "4"]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(v, vec!["1", "2", "3", "4"]);
    }

    #[test]
    fn recognizes_clap20_size_expire_path_long() {
        let m = parse_opts()
            .get_matches_from(vec!["tp", "--size", "1", "--expire", "2", "--path", "1"]);
        assert_eq!(m.get_one::<String>(SIZE).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(EXPIRE).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_clap20_pop_only_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--pop"]);
        assert!(m.get_flag(POP));
    }

    #[test]
    fn recognizes_clap20_clear_only_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-c"]);
        assert!(m.get_flag(CLEAR));
    }

    #[test]
    fn recognizes_clap20_list_files_only_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-l"]);
        assert!(m.get_flag(LIST_FILES));
    }

    // ── clap coverage round 21 ─────────────────────────────

    #[test]
    fn recognizes_clap21_positional_sftp_style() {
        let m = parse_opts().get_matches_from(vec!["tp", "host:/remote/path"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("host:/remote/path")
        );
    }

    #[test]
    fn recognizes_clap21_output_short_zero_string() {
        let m = parse_opts().get_matches_from(vec!["tp", "-o", "0"]);
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("0"));
    }

    #[test]
    fn recognizes_clap21_input_long_at_tag() {
        let m = parse_opts().get_matches_from(vec!["tp", "--input", "@stack"]);
        assert_eq!(
            m.get_one::<String>(INPUT).map(|s| s.as_str()),
            Some("@stack")
        );
    }

    #[test]
    fn recognizes_clap21_cat_sixteen_indices_long() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--cat", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13",
            "14", "15", "16",
        ]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(
            v,
            vec![
                "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15",
                "16"
            ]
        );
    }

    #[test]
    fn recognizes_clap21_grep_multiline_escape() {
        let m = parse_opts().get_matches_from(vec!["tp", "-g", "(?s).+"]);
        assert_eq!(
            m.get_one::<String>(GREP).map(|s| s.as_str()),
            Some("(?s).+")
        );
    }

    #[test]
    fn recognizes_clap21_expire_ratio_string() {
        let m = parse_opts().get_matches_from(vec!["tp", "--expire", "1:2"]);
        assert_eq!(m.get_one::<String>(EXPIRE).map(|s| s.as_str()), Some("1:2"));
    }

    #[test]
    fn recognizes_clap21_sort_mtime_then_positional() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort", "mtime", "x.dat"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("mtime"));
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("x.dat")
        );
    }

    #[test]
    fn recognizes_clap21_unshift_shift_pop_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-u", "-s", "-p"]);
        assert!(m.get_flag(UNSHIFT));
        assert!(m.get_flag(SHIFT));
        assert!(m.get_flag(POP));
    }

    #[test]
    fn recognizes_clap21_remove_add_dup_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-r", "1", "-a", "2", "-x", "3"]);
        assert_eq!(m.get_one::<String>(REMOVE).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(ADD).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("3"));
    }

    #[test]
    fn recognizes_clap21_head_tail_same_line_counts_different_indices() {
        let m = parse_opts().get_matches_from(vec!["tp", "--head", "1", "5", "--tail", "2", "5"]);
        assert_eq!(
            m.get_many::<String>(HEAD)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["1", "5"]
        );
        assert_eq!(
            m.get_many::<String>(TAIL)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["2", "5"]
        );
    }

    #[test]
    fn recognizes_clap21_size_path_wc_expire_long() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--size", "1", "--path", "1", "--wc", "1", "--expire", "0",
        ]);
        assert_eq!(m.get_one::<String>(SIZE).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(EXPIRE).map(|s| s.as_str()), Some("0"));
    }

    #[test]
    fn recognizes_clap21_replace_three_args_equal_signs() {
        let m = parse_opts().get_matches_from(vec!["tp", "--replace", "1", "a=b", "c=d"]);
        let v: Vec<String> = m.get_many(REPLACE).unwrap().cloned().collect();
        assert_eq!(v, vec!["1", "a=b", "c=d"]);
    }

    #[test]
    fn recognizes_clap21_diff_long_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "--diff", "x", "y"]);
        let v: Vec<String> = m.get_many(DIFF).unwrap().cloned().collect();
        assert_eq!(v, vec!["x", "y"]);
    }

    #[test]
    fn recognizes_clap21_swap_move_diff_short_chain() {
        let m = parse_opts()
            .get_matches_from(vec!["tp", "-S", "1", "2", "-M", "3", "4", "-D", "5", "6"]);
        assert_eq!(
            m.get_many(SWAP).unwrap().cloned().collect::<Vec<String>>(),
            vec!["1", "2"]
        );
        assert_eq!(
            m.get_many(MOVE).unwrap().cloned().collect::<Vec<String>>(),
            vec!["3", "4"]
        );
        assert_eq!(
            m.get_many(DIFF).unwrap().cloned().collect::<Vec<String>>(),
            vec!["5", "6"]
        );
    }

    #[test]
    fn recognizes_clap21_verbose_twenty_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-vvvvvvvvvvvvvvvvvvvv"]);
        assert_eq!(m.get_count(VERBOSE), 20);
    }

    #[test]
    fn recognizes_clap21_double_dash_positional_utf8() {
        let m = parse_opts().get_matches_from(vec!["tp", "--", "你好.txt"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("你好.txt")
        );
    }

    #[test]
    fn recognizes_clap21_edit_info_wc_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-e", "1", "-I", "2", "--wc", "1"]);
        assert_eq!(m.get_one::<String>(EDIT).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(INFO).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_clap21_grep_path_size_short_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "-g", "z", "--path", "1", "--size", "1"]);
        assert_eq!(m.get_one::<String>(GREP).map(|s| s.as_str()), Some("z"));
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(SIZE).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_clap21_count_clear_rev_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-k", "-c", "--rev"]);
        assert!(m.get_flag(COUNT));
        assert!(m.get_flag(CLEAR));
        assert!(m.get_flag(REVERSE));
    }

    #[test]
    fn recognizes_clap21_list_contents_numbered_verbose_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-N", "-vv"]);
        assert!(m.get_flag(LIST_CONTENTS_NUMBERED));
        assert_eq!(m.get_count(VERBOSE), 2);
    }

    #[test]
    fn recognizes_clap21_append_output_input_long() {
        let m = parse_opts()
            .get_matches_from(vec!["tp", "--append", "1", "--output", "2", "--input", "3"]);
        assert_eq!(m.get_one::<String>(APPEND).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("3"));
    }

    #[test]
    fn recognizes_clap21_name_short_equals_utf8() {
        let m = parse_opts().get_matches_from(vec!["tp", "-w=名前"]);
        assert_eq!(m.get_one::<String>(TAG).map(|s| s.as_str()), Some("名前"));
    }

    #[test]
    fn recognizes_clap21_dup_short_equals() {
        let m = parse_opts().get_matches_from(vec!["tp", "-x=8"]);
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("8"));
    }

    #[test]
    fn recognizes_clap21_append_short_equals() {
        let m = parse_opts().get_matches_from(vec!["tp", "-A=9"]);
        assert_eq!(m.get_one::<String>(APPEND).map(|s| s.as_str()), Some("9"));
    }

    #[test]
    fn recognizes_clap21_cat_short_five_indices() {
        let m = parse_opts().get_matches_from(vec!["tp", "-C", "1", "2", "3", "4", "5"]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(v, vec!["1", "2", "3", "4", "5"]);
    }

    #[test]
    fn recognizes_clap21_rev_expire_path_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--rev", "--expire", "1", "--path", "1"]);
        assert!(m.get_flag(REVERSE));
        assert_eq!(m.get_one::<String>(EXPIRE).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_clap21_sort_size_explicit_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort", "size"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("size"));
    }

    #[test]
    fn recognizes_clap21_positional_pipe_in_name() {
        let m = parse_opts().get_matches_from(vec!["tp", "a|b.txt"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("a|b.txt")
        );
    }

    #[test]
    fn recognizes_clap21_master_short_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "-m"]);
        assert!(m.get_flag(MASTER));
    }

    #[test]
    fn recognizes_clap21_directory_short_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "-d"]);
        assert!(m.get_flag(DIRECTORY));
    }

    #[test]
    fn recognizes_clap21_quiet_short_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "-q"]);
        assert!(m.get_flag(SILENT));
    }

    // ── clap coverage round 22 ─────────────────────────────

    #[test]
    fn recognizes_clap22_positional_file_url() {
        let m = parse_opts().get_matches_from(vec!["tp", "file:///tmp/x"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("file:///tmp/x")
        );
    }

    #[test]
    fn recognizes_clap22_positional_query_string() {
        let m = parse_opts().get_matches_from(vec!["tp", "api?foo=1&bar=2"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("api?foo=1&bar=2")
        );
    }

    #[test]
    fn recognizes_clap22_output_short_large_index() {
        let m = parse_opts().get_matches_from(vec!["tp", "-o", "9999"]);
        assert_eq!(
            m.get_one::<String>(OUTPUT).map(|s| s.as_str()),
            Some("9999")
        );
    }

    #[test]
    fn recognizes_clap22_input_equals_tagged() {
        let m = parse_opts().get_matches_from(vec!["tp", "--input=@0"]);
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("@0"));
    }

    #[test]
    fn recognizes_clap22_cat_seventeen_indices_long() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--cat", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12",
            "13", "14", "15", "16",
        ]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(
            v,
            vec![
                "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                "15", "16"
            ]
        );
    }

    #[test]
    fn recognizes_clap22_grep_escaped_brackets() {
        let m = parse_opts().get_matches_from(vec!["tp", "-g", r"\[a\]"]);
        assert_eq!(
            m.get_one::<String>(GREP).map(|s| s.as_str()),
            Some(r"\[a\]")
        );
    }

    #[test]
    fn recognizes_clap22_expire_decimal_hours() {
        let m = parse_opts().get_matches_from(vec!["tp", "--expire", "0.25"]);
        assert_eq!(
            m.get_one::<String>(EXPIRE).map(|s| s.as_str()),
            Some("0.25")
        );
    }

    #[test]
    fn recognizes_clap22_sort_name_then_positional() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort", "name", "y.dat"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("name"));
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("y.dat")
        );
    }

    #[test]
    fn recognizes_clap22_clear_pop_unshift_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--clear", "--pop", "--unshift"]);
        assert!(m.get_flag(CLEAR));
        assert!(m.get_flag(POP));
        assert!(m.get_flag(UNSHIFT));
    }

    #[test]
    fn recognizes_clap22_remove_add_equals_style() {
        let m = parse_opts().get_matches_from(vec!["tp", "--remove=-1", "--add=0"]);
        assert_eq!(m.get_one::<String>(REMOVE).map(|s| s.as_str()), Some("-1"));
        assert_eq!(m.get_one::<String>(ADD).map(|s| s.as_str()), Some("0"));
    }

    #[test]
    fn recognizes_clap22_head_tail_size_long() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--head", "0", "3", "--tail", "1", "4", "--size", "0",
        ]);
        assert_eq!(
            m.get_many::<String>(HEAD)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["0", "3"]
        );
        assert_eq!(
            m.get_many::<String>(TAIL)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["1", "4"]
        );
        assert_eq!(m.get_one::<String>(SIZE).map(|s| s.as_str()), Some("0"));
    }

    #[test]
    fn recognizes_clap22_replace_newline_in_replacement() {
        let m = parse_opts().get_matches_from(vec!["tp", "--replace", "0", "a", "b\nc"]);
        let v: Vec<String> = m.get_many(REPLACE).unwrap().cloned().collect();
        assert_eq!(v, vec!["0", "a", "b\nc"]);
    }

    #[test]
    fn recognizes_clap22_diff_swap_short_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "-D", "p", "q", "-S", "r", "s"]);
        assert_eq!(
            m.get_many(DIFF).unwrap().cloned().collect::<Vec<String>>(),
            vec!["p", "q"]
        );
        assert_eq!(
            m.get_many(SWAP).unwrap().cloned().collect::<Vec<String>>(),
            vec!["r", "s"]
        );
    }

    #[test]
    fn recognizes_clap22_move_short_pair() {
        let m = parse_opts().get_matches_from(vec!["tp", "-M", "7", "8"]);
        let v: Vec<String> = m.get_many(MOVE).unwrap().cloned().collect();
        assert_eq!(v, vec!["7", "8"]);
    }

    #[test]
    fn recognizes_clap22_verbose_twenty_one_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-vvvvvvvvvvvvvvvvvvvvv"]);
        assert_eq!(m.get_count(VERBOSE), 21);
    }

    #[test]
    fn recognizes_clap22_double_dash_positional_spaces() {
        let m = parse_opts().get_matches_from(vec!["tp", "--", "my file.txt"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("my file.txt")
        );
    }

    #[test]
    fn recognizes_clap22_edit_rename_info_long() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--edit", "-2", "--rename", "old", "new", "--info", "0",
        ]);
        assert_eq!(m.get_one::<String>(EDIT).map(|s| s.as_str()), Some("-2"));
        assert_eq!(
            m.get_many(RENAME)
                .unwrap()
                .cloned()
                .collect::<Vec<String>>(),
            vec!["old", "new"]
        );
        assert_eq!(m.get_one::<String>(INFO).map(|s| s.as_str()), Some("0"));
    }

    #[test]
    fn recognizes_clap22_grep_list_files_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-g", "needle", "-l"]);
        assert_eq!(
            m.get_one::<String>(GREP).map(|s| s.as_str()),
            Some("needle")
        );
        assert!(m.get_flag(LIST_FILES));
    }

    #[test]
    fn recognizes_clap22_count_list_contents_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-k", "-L"]);
        assert!(m.get_flag(COUNT));
        assert!(m.get_flag(LIST_CONTENTS));
    }

    #[test]
    fn recognizes_clap22_rev_sort_quiet_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--rev", "--sort", "mtime", "--quiet"]);
        assert!(m.get_flag(REVERSE));
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("mtime"));
        assert!(m.get_flag(SILENT));
    }

    #[test]
    fn recognizes_clap22_append_output_input_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-A", "0", "-o", "1", "-i", "2"]);
        assert_eq!(m.get_one::<String>(APPEND).map(|s| s.as_str()), Some("0"));
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("2"));
    }

    #[test]
    fn recognizes_clap22_name_long_equals() {
        let m = parse_opts().get_matches_from(vec!["tp", "--name=alias"]);
        assert_eq!(m.get_one::<String>(TAG).map(|s| s.as_str()), Some("alias"));
    }

    #[test]
    fn recognizes_clap22_dup_long_equals() {
        let m = parse_opts().get_matches_from(vec!["tp", "--dup=-3"]);
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("-3"));
    }

    #[test]
    fn recognizes_clap22_append_long_equals() {
        let m = parse_opts().get_matches_from(vec!["tp", "--append=99"]);
        assert_eq!(m.get_one::<String>(APPEND).map(|s| s.as_str()), Some("99"));
    }

    #[test]
    fn recognizes_clap22_cat_short_six_indices() {
        let m = parse_opts().get_matches_from(vec!["tp", "-C", "9", "8", "7", "6", "5", "4"]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(v, vec!["9", "8", "7", "6", "5", "4"]);
    }

    #[test]
    fn recognizes_clap22_rev_path_wc_expire() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--rev", "--path", "2", "--wc", "2", "--expire", "48",
        ]);
        assert!(m.get_flag(REVERSE));
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(EXPIRE).map(|s| s.as_str()), Some("48"));
    }

    #[test]
    fn recognizes_clap22_sort_mtime_long_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort", "mtime"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("mtime"));
    }

    #[test]
    fn recognizes_clap22_positional_windows_drive() {
        let m = parse_opts().get_matches_from(vec!["tp", r"C:\temp\out.txt"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some(r"C:\temp\out.txt")
        );
    }

    #[test]
    fn recognizes_clap22_master_list_files_numbered_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-m", "-n"]);
        assert!(m.get_flag(MASTER));
        assert!(m.get_flag(LIST_FILES_NUMBERED));
    }

    #[test]
    fn recognizes_clap22_directory_list_contents_numbered_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--dir", "--list-contents-numbered"]);
        assert!(m.get_flag(DIRECTORY));
        assert!(m.get_flag(LIST_CONTENTS_NUMBERED));
    }

    #[test]
    fn recognizes_clap22_program_name_temprs_binary() {
        let m = parse_opts().get_matches_from(vec!["temprs", "--count"]);
        assert!(m.get_flag(COUNT));
    }

    // ── clap coverage round 23 ─────────────────────────────

    #[test]
    fn recognizes_clap23_positional_dot_slash() {
        let m = parse_opts().get_matches_from(vec!["tp", "./local.bin"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("./local.bin")
        );
    }

    #[test]
    fn recognizes_clap23_positional_parent_dirs() {
        let m = parse_opts().get_matches_from(vec!["tp", "../out/../in.txt"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("../out/../in.txt")
        );
    }

    #[test]
    fn recognizes_clap23_output_equals_negative_two() {
        let m = parse_opts().get_matches_from(vec!["tp", "--output=-2"]);
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("-2"));
    }

    #[test]
    fn recognizes_clap23_input_short_plus_index() {
        let m = parse_opts().get_matches_from(vec!["tp", "-i", "+1"]);
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("+1"));
    }

    #[test]
    fn recognizes_clap23_cat_eighteen_indices_long() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--cat", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13",
            "14", "15", "16", "17", "18",
        ]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(
            v,
            vec![
                "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15",
                "16", "17", "18"
            ]
        );
    }

    #[test]
    fn recognizes_clap23_grep_dollar_anchor() {
        let m = parse_opts().get_matches_from(vec!["tp", "-g", r"line$"]);
        assert_eq!(m.get_one::<String>(GREP).map(|s| s.as_str()), Some("line$"));
    }

    #[test]
    fn recognizes_clap23_expire_human_readable_suffix() {
        let m = parse_opts().get_matches_from(vec!["tp", "--expire", "72h"]);
        assert_eq!(m.get_one::<String>(EXPIRE).map(|s| s.as_str()), Some("72h"));
    }

    #[test]
    fn recognizes_clap23_sort_size_then_positional() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort", "size", "archive.tgz"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("size"));
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("archive.tgz")
        );
    }

    #[test]
    fn recognizes_clap23_pop_shift_unshift_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--pop", "--shift", "--unshift"]);
        assert!(m.get_flag(POP));
        assert!(m.get_flag(SHIFT));
        assert!(m.get_flag(UNSHIFT));
    }

    #[test]
    fn recognizes_clap23_edit_info_wc_path_short() {
        let m = parse_opts()
            .get_matches_from(vec!["tp", "-e", "0", "-I", "1", "--wc", "2", "--path", "2"]);
        assert_eq!(m.get_one::<String>(EDIT).map(|s| s.as_str()), Some("0"));
        assert_eq!(m.get_one::<String>(INFO).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("2"));
    }

    #[test]
    fn recognizes_clap23_replace_unicode_pattern_and_repl() {
        let m = parse_opts().get_matches_from(vec!["tp", "--replace", "1", "β", "γ"]);
        let v: Vec<String> = m.get_many(REPLACE).unwrap().cloned().collect();
        assert_eq!(v, vec!["1", "β", "γ"]);
    }

    #[test]
    fn recognizes_clap23_diff_move_swap_long_short_mix() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--diff", "α", "ω", "--mv", "10", "11", "-S", "12", "13",
        ]);
        assert_eq!(
            m.get_many(DIFF).unwrap().cloned().collect::<Vec<String>>(),
            vec!["α", "ω"]
        );
        assert_eq!(
            m.get_many(MOVE).unwrap().cloned().collect::<Vec<String>>(),
            vec!["10", "11"]
        );
        assert_eq!(
            m.get_many(SWAP).unwrap().cloned().collect::<Vec<String>>(),
            vec!["12", "13"]
        );
    }

    #[test]
    fn recognizes_clap23_verbose_twenty_two_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-vvvvvvvvvvvvvvvvvvvvvv"]);
        assert_eq!(m.get_count(VERBOSE), 22);
    }

    #[test]
    fn recognizes_clap23_double_dash_positional_leading_hyphen() {
        let m = parse_opts().get_matches_from(vec!["tp", "--", "-weird-name"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("-weird-name")
        );
    }

    #[test]
    fn recognizes_clap23_all_list_modes_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-l", "-n", "-L", "-N"]);
        assert!(m.get_flag(LIST_FILES));
        assert!(m.get_flag(LIST_FILES_NUMBERED));
        assert!(m.get_flag(LIST_CONTENTS));
        assert!(m.get_flag(LIST_CONTENTS_NUMBERED));
    }

    #[test]
    fn recognizes_clap23_rename_long_pair() {
        let m = parse_opts().get_matches_from(vec!["tp", "--rename", "before", "after"]);
        assert_eq!(
            m.get_many(RENAME)
                .unwrap()
                .cloned()
                .collect::<Vec<String>>(),
            vec!["before", "after"]
        );
    }

    #[test]
    fn recognizes_clap23_grep_tag_quiet_long() {
        let m =
            parse_opts().get_matches_from(vec!["tp", "--grep", "pat", "--name", "t", "--quiet"]);
        assert_eq!(m.get_one::<String>(GREP).map(|s| s.as_str()), Some("pat"));
        assert_eq!(m.get_one::<String>(TAG).map(|s| s.as_str()), Some("t"));
        assert!(m.get_flag(SILENT));
    }

    #[test]
    fn recognizes_clap23_append_dup_remove_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-A", "1", "-x", "2", "-r", "3"]);
        assert_eq!(m.get_one::<String>(APPEND).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(REMOVE).map(|s| s.as_str()), Some("3"));
    }

    #[test]
    fn recognizes_clap23_head_tail_negative_line_count() {
        let m = parse_opts().get_matches_from(vec!["tp", "--head", "0", "-1", "--tail", "1", "-2"]);
        assert_eq!(
            m.get_many::<String>(HEAD)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["0", "-1"]
        );
        assert_eq!(
            m.get_many::<String>(TAIL)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["1", "-2"]
        );
    }

    #[test]
    fn recognizes_clap23_size_wc_only_short_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--size", "-1", "-k", "--wc", "-1"]);
        assert_eq!(m.get_one::<String>(SIZE).map(|s| s.as_str()), Some("-1"));
        assert!(m.get_flag(COUNT));
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("-1"));
    }

    #[test]
    fn recognizes_clap23_clear_rev_count_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--clear", "--rev", "--count"]);
        assert!(m.get_flag(CLEAR));
        assert!(m.get_flag(REVERSE));
        assert!(m.get_flag(COUNT));
    }

    #[test]
    fn recognizes_clap23_add_remove_equals_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-a=5", "-r=6"]);
        assert_eq!(m.get_one::<String>(ADD).map(|s| s.as_str()), Some("5"));
        assert_eq!(m.get_one::<String>(REMOVE).map(|s| s.as_str()), Some("6"));
    }

    #[test]
    fn recognizes_clap23_output_input_add_long_chain() {
        let m = parse_opts()
            .get_matches_from(vec!["tp", "--output", "7", "--input", "8", "--add", "9"]);
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("7"));
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("8"));
        assert_eq!(m.get_one::<String>(ADD).map(|s| s.as_str()), Some("9"));
    }

    #[test]
    fn recognizes_clap23_cat_short_seven_indices() {
        let m = parse_opts().get_matches_from(vec!["tp", "-C", "0", "1", "2", "3", "4", "5", "6"]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(v, vec!["0", "1", "2", "3", "4", "5", "6"]);
    }

    #[test]
    fn recognizes_clap23_sort_bare_long_then_rev() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort", "--rev"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("name"));
        assert!(m.get_flag(REVERSE));
    }

    #[test]
    fn recognizes_clap23_positional_scp_style() {
        let m = parse_opts().get_matches_from(vec!["tp", "user@host:path/to/file"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("user@host:path/to/file")
        );
    }

    #[test]
    fn recognizes_clap23_program_name_tp_alias() {
        let m = parse_opts().get_matches_from(vec!["tp", "--master", "--list-files"]);
        assert!(m.get_flag(MASTER));
        assert!(m.get_flag(LIST_FILES));
    }

    #[test]
    fn recognizes_clap23_expire_zero_string() {
        let m = parse_opts().get_matches_from(vec!["tp", "--expire", "0"]);
        assert_eq!(m.get_one::<String>(EXPIRE).map(|s| s.as_str()), Some("0"));
    }

    #[test]
    fn recognizes_clap23_grep_empty_pattern_allowed() {
        let m = parse_opts().get_matches_from(vec!["tp", "-g", ""]);
        assert_eq!(m.get_one::<String>(GREP).map(|s| s.as_str()), Some(""));
    }

    #[test]
    fn recognizes_clap23_positional_dot_hidden_file() {
        let m = parse_opts().get_matches_from(vec!["tp", ".env.local"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some(".env.local")
        );
    }

    #[test]
    fn recognizes_clap23_sort_equals_name_with_rev_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort=name", "--rev"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("name"));
        assert!(m.get_flag(REVERSE));
    }

    #[test]
    fn recognizes_clap23_list_files_verbose_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-l", "-v"]);
        assert!(m.get_flag(LIST_FILES));
        assert_eq!(m.get_count(VERBOSE), 1);
    }

    // ── clap coverage round 24 ─────────────────────────────

    #[test]
    fn recognizes_clap24_positional_tilde_path() {
        let m = parse_opts().get_matches_from(vec!["tp", "~/Documents/x.dat"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("~/Documents/x.dat")
        );
    }

    #[test]
    fn recognizes_clap24_positional_percent_encoded() {
        let m = parse_opts().get_matches_from(vec!["tp", "a%20b%2Fc"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("a%20b%2Fc")
        );
    }

    #[test]
    fn recognizes_clap24_output_short_negative_one() {
        let m = parse_opts().get_matches_from(vec!["tp", "-o", "-1"]);
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("-1"));
    }

    #[test]
    fn recognizes_clap24_input_equals_zero() {
        let m = parse_opts().get_matches_from(vec!["tp", "--input=0"]);
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("0"));
    }

    #[test]
    fn recognizes_clap24_cat_nineteen_indices_long() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--cat", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
            "15", "16", "17", "18", "19", "20",
        ]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(
            v,
            vec![
                "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16",
                "17", "18", "19", "20"
            ]
        );
    }

    #[test]
    fn recognizes_clap24_grep_word_boundary() {
        let m = parse_opts().get_matches_from(vec!["tp", "-g", r"\bfoo\b"]);
        assert_eq!(
            m.get_one::<String>(GREP).map(|s| s.as_str()),
            Some(r"\bfoo\b")
        );
    }

    #[test]
    fn recognizes_clap24_expire_one_week_string() {
        let m = parse_opts().get_matches_from(vec!["tp", "--expire", "1w"]);
        assert_eq!(m.get_one::<String>(EXPIRE).map(|s| s.as_str()), Some("1w"));
    }

    #[test]
    fn recognizes_clap24_sort_mtime_positional_then_rev() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort", "mtime", "--rev", "heap.bin"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("mtime"));
        assert!(m.get_flag(REVERSE));
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("heap.bin")
        );
    }

    #[test]
    fn recognizes_clap24_clear_count_quiet_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-c", "-k", "-q"]);
        assert!(m.get_flag(CLEAR));
        assert!(m.get_flag(COUNT));
        assert!(m.get_flag(SILENT));
    }

    #[test]
    fn recognizes_clap24_edit_wc_size_short_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "-e", "3", "--wc", "3", "--size", "3"]);
        assert_eq!(m.get_one::<String>(EDIT).map(|s| s.as_str()), Some("3"));
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("3"));
        assert_eq!(m.get_one::<String>(SIZE).map(|s| s.as_str()), Some("3"));
    }

    #[test]
    fn recognizes_clap24_replace_three_args_tabs() {
        let m = parse_opts().get_matches_from(vec!["tp", "--replace", "0", "\t", "\t\t"]);
        let v: Vec<String> = m.get_many(REPLACE).unwrap().cloned().collect();
        assert_eq!(v, vec!["0", "\t", "\t\t"]);
    }

    #[test]
    fn recognizes_clap24_diff_short_only_pair() {
        let m = parse_opts().get_matches_from(vec!["tp", "-D", "i", "j"]);
        assert_eq!(
            m.get_many(DIFF).unwrap().cloned().collect::<Vec<String>>(),
            vec!["i", "j"]
        );
    }

    #[test]
    fn recognizes_clap24_swap_long_only_pair() {
        let m = parse_opts().get_matches_from(vec!["tp", "--swap", "m", "n"]);
        assert_eq!(
            m.get_many(SWAP).unwrap().cloned().collect::<Vec<String>>(),
            vec!["m", "n"]
        );
    }

    #[test]
    fn recognizes_clap24_verbose_twenty_three_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-vvvvvvvvvvvvvvvvvvvvvvv"]);
        assert_eq!(m.get_count(VERBOSE), 23);
    }

    #[test]
    fn recognizes_clap24_double_dash_positional_equals_sign() {
        let m = parse_opts().get_matches_from(vec!["tp", "--", "a=b"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("a=b")
        );
    }

    #[test]
    fn recognizes_clap24_grep_list_files_numbered_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-g", "x", "-n"]);
        assert_eq!(m.get_one::<String>(GREP).map(|s| s.as_str()), Some("x"));
        assert!(m.get_flag(LIST_FILES_NUMBERED));
    }

    #[test]
    fn recognizes_clap24_rev_expire_size_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--rev", "--expire", "99", "--size", "0"]);
        assert!(m.get_flag(REVERSE));
        assert_eq!(m.get_one::<String>(EXPIRE).map(|s| s.as_str()), Some("99"));
        assert_eq!(m.get_one::<String>(SIZE).map(|s| s.as_str()), Some("0"));
    }

    #[test]
    fn recognizes_clap24_name_long_with_spaces() {
        let m = parse_opts().get_matches_from(vec!["tp", "--name", "my tag"]);
        assert_eq!(m.get_one::<String>(TAG).map(|s| s.as_str()), Some("my tag"));
    }

    #[test]
    fn recognizes_clap24_dup_append_long_chain() {
        let m = parse_opts().get_matches_from(vec!["tp", "--dup", "4", "--append", "5"]);
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("4"));
        assert_eq!(m.get_one::<String>(APPEND).map(|s| s.as_str()), Some("5"));
    }

    #[test]
    fn recognizes_clap24_cat_short_eight_indices() {
        let m =
            parse_opts().get_matches_from(vec!["tp", "-C", "a", "b", "c", "d", "e", "f", "g", "h"]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(v, vec!["a", "b", "c", "d", "e", "f", "g", "h"]);
    }

    #[test]
    fn recognizes_clap24_path_head_tail_wc_long() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--path", "1", "--head", "1", "10", "--tail", "1", "10", "--wc", "1",
        ]);
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("1"));
        assert_eq!(
            m.get_many::<String>(HEAD)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["1", "10"]
        );
        assert_eq!(
            m.get_many::<String>(TAIL)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["1", "10"]
        );
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_clap24_master_directory_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-m", "-d"]);
        assert!(m.get_flag(MASTER));
        assert!(m.get_flag(DIRECTORY));
    }

    #[test]
    fn recognizes_clap24_sort_equals_mtime_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort=mtime"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("mtime"));
    }

    #[test]
    fn recognizes_clap24_output_long_equals_negative() {
        let m = parse_opts().get_matches_from(vec!["tp", "--output=-99"]);
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("-99"));
    }

    #[test]
    fn recognizes_clap24_remove_long_equals_positive() {
        let m = parse_opts().get_matches_from(vec!["tp", "--remove=42"]);
        assert_eq!(m.get_one::<String>(REMOVE).map(|s| s.as_str()), Some("42"));
    }

    #[test]
    fn recognizes_clap24_add_short_equals_zero() {
        let m = parse_opts().get_matches_from(vec!["tp", "-a=0"]);
        assert_eq!(m.get_one::<String>(ADD).map(|s| s.as_str()), Some("0"));
    }

    #[test]
    fn recognizes_clap24_move_diff_short_chain() {
        let m = parse_opts().get_matches_from(vec!["tp", "-M", "a", "b", "-D", "c", "d"]);
        assert_eq!(
            m.get_many(MOVE).unwrap().cloned().collect::<Vec<String>>(),
            vec!["a", "b"]
        );
        assert_eq!(
            m.get_many(DIFF).unwrap().cloned().collect::<Vec<String>>(),
            vec!["c", "d"]
        );
    }

    #[test]
    fn recognizes_clap24_list_contents_only_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--list-contents"]);
        assert!(m.get_flag(LIST_CONTENTS));
    }

    #[test]
    fn recognizes_clap24_pop_unshift_only_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-p", "-u"]);
        assert!(m.get_flag(POP));
        assert!(m.get_flag(UNSHIFT));
    }

    #[test]
    fn recognizes_clap24_program_name_temprs_with_list_files() {
        let m = parse_opts().get_matches_from(vec!["temprs", "-l"]);
        assert!(m.get_flag(LIST_FILES));
    }

    #[test]
    fn recognizes_clap24_positional_hash_fragment() {
        let m = parse_opts().get_matches_from(vec!["tp", "page.html#section"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("page.html#section")
        );
    }

    #[test]
    fn recognizes_clap24_info_edit_negative_indices() {
        let m = parse_opts().get_matches_from(vec!["tp", "-I", "-3", "-e", "-4"]);
        assert_eq!(m.get_one::<String>(INFO).map(|s| s.as_str()), Some("-3"));
        assert_eq!(m.get_one::<String>(EDIT).map(|s| s.as_str()), Some("-4"));
    }

    // ── clap coverage round 25 ─────────────────────────────

    #[test]
    fn recognizes_clap25_positional_semver_path() {
        let m = parse_opts().get_matches_from(vec!["tp", "pkg@1.2.3.tgz"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("pkg@1.2.3.tgz")
        );
    }

    #[test]
    fn recognizes_clap25_positional_blob_uri() {
        let m = parse_opts().get_matches_from(vec!["tp", "blob:https://example.com/uuid"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("blob:https://example.com/uuid")
        );
    }

    #[test]
    fn recognizes_clap25_output_short_large_positive() {
        let m = parse_opts().get_matches_from(vec!["tp", "-o", "65535"]);
        assert_eq!(
            m.get_one::<String>(OUTPUT).map(|s| s.as_str()),
            Some("65535")
        );
    }

    #[test]
    fn recognizes_clap25_input_long_negative_large() {
        let m = parse_opts().get_matches_from(vec!["tp", "--input=-128"]);
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("-128"));
    }

    #[test]
    fn recognizes_clap25_cat_twenty_indices_long() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--cat", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13",
            "14", "15", "16", "17", "18", "19", "20",
        ]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(
            v,
            vec![
                "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15",
                "16", "17", "18", "19", "20"
            ]
        );
    }

    #[test]
    fn recognizes_clap25_grep_lookahead_style() {
        let m = parse_opts().get_matches_from(vec!["tp", "-g", r"(?<=a)b"]);
        assert_eq!(
            m.get_one::<String>(GREP).map(|s| s.as_str()),
            Some(r"(?<=a)b")
        );
    }

    #[test]
    fn recognizes_clap25_expire_fraction_slash() {
        let m = parse_opts().get_matches_from(vec!["tp", "--expire", "1/2"]);
        assert_eq!(m.get_one::<String>(EXPIRE).map(|s| s.as_str()), Some("1/2"));
    }

    #[test]
    fn recognizes_clap25_sort_name_positional_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort", "name", "only.dat"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("name"));
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("only.dat")
        );
    }

    #[test]
    fn recognizes_clap25_rev_clear_count_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--rev", "--clear", "--count"]);
        assert!(m.get_flag(REVERSE));
        assert!(m.get_flag(CLEAR));
        assert!(m.get_flag(COUNT));
    }

    #[test]
    fn recognizes_clap25_shift_pop_only_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-s", "-p"]);
        assert!(m.get_flag(SHIFT));
        assert!(m.get_flag(POP));
    }

    #[test]
    fn recognizes_clap25_remove_add_input_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-r", "9", "-a", "8", "-i", "7"]);
        assert_eq!(m.get_one::<String>(REMOVE).map(|s| s.as_str()), Some("9"));
        assert_eq!(m.get_one::<String>(ADD).map(|s| s.as_str()), Some("8"));
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("7"));
    }

    #[test]
    fn recognizes_clap25_replace_three_args_backslashes() {
        let m = parse_opts().get_matches_from(vec!["tp", "--replace", "0", r"\n", r"\\n"]);
        let v: Vec<String> = m.get_many(REPLACE).unwrap().cloned().collect();
        assert_eq!(v, vec!["0", r"\n", r"\\n"]);
    }

    #[test]
    fn recognizes_clap25_diff_swap_move_long_short_mix() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--diff", "u", "v", "--swap", "w", "x", "-M", "y", "z",
        ]);
        assert_eq!(
            m.get_many(DIFF).unwrap().cloned().collect::<Vec<String>>(),
            vec!["u", "v"]
        );
        assert_eq!(
            m.get_many(SWAP).unwrap().cloned().collect::<Vec<String>>(),
            vec!["w", "x"]
        );
        assert_eq!(
            m.get_many(MOVE).unwrap().cloned().collect::<Vec<String>>(),
            vec!["y", "z"]
        );
    }

    #[test]
    fn recognizes_clap25_verbose_twenty_four_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-vvvvvvvvvvvvvvvvvvvvvvvv"]);
        assert_eq!(m.get_count(VERBOSE), 24);
    }

    #[test]
    fn recognizes_clap25_double_dash_positional_at_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "--", "@"]);
        assert_eq!(m.get_one::<String>(ARGFILE).map(|s| s.as_str()), Some("@"));
    }

    #[test]
    fn recognizes_clap25_grep_quiet_list_files_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--grep", "z", "--quiet", "--list-files"]);
        assert_eq!(m.get_one::<String>(GREP).map(|s| s.as_str()), Some("z"));
        assert!(m.get_flag(SILENT));
        assert!(m.get_flag(LIST_FILES));
    }

    #[test]
    fn recognizes_clap25_path_size_wc_head_long() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--path", "5", "--size", "5", "--wc", "5", "--head", "5", "100",
        ]);
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("5"));
        assert_eq!(m.get_one::<String>(SIZE).map(|s| s.as_str()), Some("5"));
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("5"));
        assert_eq!(
            m.get_many::<String>(HEAD)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["5", "100"]
        );
    }

    #[test]
    fn recognizes_clap25_dup_x_append_long_equals() {
        let m = parse_opts().get_matches_from(vec!["tp", "-x", "1", "--append=2"]);
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(APPEND).map(|s| s.as_str()), Some("2"));
    }

    #[test]
    fn recognizes_clap25_cat_short_nine_indices() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "-C", "1", "2", "3", "4", "5", "6", "7", "8", "9",
        ]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(v, vec!["1", "2", "3", "4", "5", "6", "7", "8", "9"]);
    }

    #[test]
    fn recognizes_clap25_sort_size_equals_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort=size"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("size"));
    }

    #[test]
    fn recognizes_clap25_output_input_long_equals_chain() {
        let m = parse_opts().get_matches_from(vec!["tp", "--output=3", "--input=4"]);
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("3"));
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("4"));
    }

    #[test]
    fn recognizes_clap25_rename_short_equals_style() {
        let m = parse_opts().get_matches_from(vec!["tp", "-R", "src", "dst"]);
        assert_eq!(
            m.get_many(RENAME)
                .unwrap()
                .cloned()
                .collect::<Vec<String>>(),
            vec!["src", "dst"]
        );
    }

    #[test]
    fn recognizes_clap25_list_files_numbered_contents_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-n", "-L"]);
        assert!(m.get_flag(LIST_FILES_NUMBERED));
        assert!(m.get_flag(LIST_CONTENTS));
    }

    #[test]
    fn recognizes_clap25_master_dir_list_numbered_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-m", "-d", "-n"]);
        assert!(m.get_flag(MASTER));
        assert!(m.get_flag(DIRECTORY));
        assert!(m.get_flag(LIST_FILES_NUMBERED));
    }

    #[test]
    fn recognizes_clap25_expire_sort_rev_long() {
        let m =
            parse_opts().get_matches_from(vec!["tp", "--expire", "12", "--sort", "size", "--rev"]);
        assert_eq!(m.get_one::<String>(EXPIRE).map(|s| s.as_str()), Some("12"));
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("size"));
        assert!(m.get_flag(REVERSE));
    }

    #[test]
    fn recognizes_clap25_tail_head_negative_line_counts() {
        let m = parse_opts().get_matches_from(vec!["tp", "--tail", "2", "-5", "--head", "3", "-4"]);
        assert_eq!(
            m.get_many::<String>(TAIL)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["2", "-5"]
        );
        assert_eq!(
            m.get_many::<String>(HEAD)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["3", "-4"]
        );
    }

    #[test]
    fn recognizes_clap25_positional_colon_windows_device() {
        let m = parse_opts().get_matches_from(vec!["tp", "\\\\?\\C:\\temp"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some(r"\\?\C:\temp")
        );
    }

    #[test]
    fn recognizes_clap25_program_name_tp_with_count() {
        let m = parse_opts().get_matches_from(vec!["tp", "-k"]);
        assert!(m.get_flag(COUNT));
    }

    #[test]
    fn recognizes_clap25_tag_short_equals_spaces() {
        let m = parse_opts().get_matches_from(vec!["tp", "-w=tag one"]);
        assert_eq!(
            m.get_one::<String>(TAG).map(|s| s.as_str()),
            Some("tag one")
        );
    }

    #[test]
    fn recognizes_clap25_edit_info_output_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-e", "2", "-I", "3", "-o", "1"]);
        assert_eq!(m.get_one::<String>(EDIT).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(INFO).map(|s| s.as_str()), Some("3"));
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_clap25_positional_git_ssh_scp() {
        let m = parse_opts().get_matches_from(vec!["tp", "git@github.com:org/repo.git"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("git@github.com:org/repo.git")
        );
    }

    #[test]
    fn recognizes_clap25_wc_only_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "--wc", "0"]);
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("0"));
    }

    // ── clap coverage round 26 ─────────────────────────────

    #[test]
    fn recognizes_clap26_positional_magnet_uri() {
        let m = parse_opts().get_matches_from(vec!["tp", "magnet:?xt=urn:btih:abc"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("magnet:?xt=urn:btih:abc")
        );
    }

    #[test]
    fn recognizes_clap26_output_long_double_hyphen_value() {
        let m = parse_opts().get_matches_from(vec!["tp", "--output=--"]);
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("--"));
    }

    #[test]
    fn recognizes_clap26_input_long_double_hyphen_value() {
        let m = parse_opts().get_matches_from(vec!["tp", "--input=--"]);
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("--"));
    }

    #[test]
    fn recognizes_clap26_cat_twenty_one_indices_long() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--cat", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12",
            "13", "14", "15", "16", "17", "18", "19", "20",
        ]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(
            v,
            vec![
                "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                "15", "16", "17", "18", "19", "20"
            ]
        );
    }

    #[test]
    fn recognizes_clap26_grep_alternation() {
        let m = parse_opts().get_matches_from(vec!["tp", "-g", "a|b|c"]);
        assert_eq!(m.get_one::<String>(GREP).map(|s| s.as_str()), Some("a|b|c"));
    }

    #[test]
    fn recognizes_clap26_expire_infinity_string() {
        let m = parse_opts().get_matches_from(vec!["tp", "--expire", "inf"]);
        assert_eq!(m.get_one::<String>(EXPIRE).map(|s| s.as_str()), Some("inf"));
    }

    #[test]
    fn recognizes_clap26_sort_mtime_positional_file() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort", "mtime", "stack.dump"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("mtime"));
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("stack.dump")
        );
    }

    #[test]
    fn recognizes_clap26_unshift_shift_only_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-u", "-s"]);
        assert!(m.get_flag(UNSHIFT));
        assert!(m.get_flag(SHIFT));
    }

    #[test]
    fn recognizes_clap26_pop_clear_quiet_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-p", "-c", "-q"]);
        assert!(m.get_flag(POP));
        assert!(m.get_flag(CLEAR));
        assert!(m.get_flag(SILENT));
    }

    #[test]
    fn recognizes_clap26_dup_remove_append_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-x", "0", "-r", "1", "-A", "2"]);
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("0"));
        assert_eq!(m.get_one::<String>(REMOVE).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(APPEND).map(|s| s.as_str()), Some("2"));
    }

    #[test]
    fn recognizes_clap26_replace_three_args_unicode_replacement() {
        let m = parse_opts().get_matches_from(vec!["tp", "--replace", "2", "x", "★"]);
        let v: Vec<String> = m.get_many(REPLACE).unwrap().cloned().collect();
        assert_eq!(v, vec!["2", "x", "★"]);
    }

    #[test]
    fn recognizes_clap26_move_swap_diff_short_chain() {
        let m = parse_opts()
            .get_matches_from(vec!["tp", "-M", "1", "2", "-S", "3", "4", "-D", "5", "6"]);
        assert_eq!(
            m.get_many(MOVE).unwrap().cloned().collect::<Vec<String>>(),
            vec!["1", "2"]
        );
        assert_eq!(
            m.get_many(SWAP).unwrap().cloned().collect::<Vec<String>>(),
            vec!["3", "4"]
        );
        assert_eq!(
            m.get_many(DIFF).unwrap().cloned().collect::<Vec<String>>(),
            vec!["5", "6"]
        );
    }

    #[test]
    fn recognizes_clap26_verbose_twenty_five_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-vvvvvvvvvvvvvvvvvvvvvvvvv"]);
        assert_eq!(m.get_count(VERBOSE), 25);
    }

    #[test]
    fn recognizes_clap26_double_dash_positional_colon_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "--", ":"]);
        assert_eq!(m.get_one::<String>(ARGFILE).map(|s| s.as_str()), Some(":"));
    }

    #[test]
    fn recognizes_clap26_grep_count_list_numbered_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-g", "p", "-k", "-N"]);
        assert_eq!(m.get_one::<String>(GREP).map(|s| s.as_str()), Some("p"));
        assert!(m.get_flag(COUNT));
        assert!(m.get_flag(LIST_CONTENTS_NUMBERED));
    }

    #[test]
    fn recognizes_clap26_path_tail_wc_long() {
        let m = parse_opts()
            .get_matches_from(vec!["tp", "--path", "4", "--tail", "4", "7", "--wc", "4"]);
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("4"));
        assert_eq!(
            m.get_many::<String>(TAIL)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["4", "7"]
        );
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("4"));
    }

    #[test]
    fn recognizes_clap26_size_head_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--size", "1", "--head", "1", "20"]);
        assert_eq!(m.get_one::<String>(SIZE).map(|s| s.as_str()), Some("1"));
        assert_eq!(
            m.get_many::<String>(HEAD)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["1", "20"]
        );
    }

    #[test]
    fn recognizes_clap26_edit_long_equals() {
        let m = parse_opts().get_matches_from(vec!["tp", "--edit=6"]);
        assert_eq!(m.get_one::<String>(EDIT).map(|s| s.as_str()), Some("6"));
    }

    #[test]
    fn recognizes_clap26_info_long_equals() {
        let m = parse_opts().get_matches_from(vec!["tp", "--info=7"]);
        assert_eq!(m.get_one::<String>(INFO).map(|s| s.as_str()), Some("7"));
    }

    #[test]
    fn recognizes_clap26_cat_short_ten_indices() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "-C", "10", "11", "12", "13", "14", "15", "16", "17", "18", "19",
        ]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(
            v,
            vec!["10", "11", "12", "13", "14", "15", "16", "17", "18", "19"]
        );
    }

    #[test]
    fn recognizes_clap26_sort_bare_long_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("name"));
    }

    #[test]
    fn recognizes_clap26_rev_expire_path_wc_short() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--rev", "--expire", "3", "--path", "0", "--wc", "0",
        ]);
        assert!(m.get_flag(REVERSE));
        assert_eq!(m.get_one::<String>(EXPIRE).map(|s| s.as_str()), Some("3"));
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("0"));
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("0"));
    }

    #[test]
    fn recognizes_clap26_rename_move_long_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "--rename", "x", "y", "--mv", "0", "1"]);
        assert_eq!(
            m.get_many(RENAME)
                .unwrap()
                .cloned()
                .collect::<Vec<String>>(),
            vec!["x", "y"]
        );
        assert_eq!(
            m.get_many(MOVE).unwrap().cloned().collect::<Vec<String>>(),
            vec!["0", "1"]
        );
    }

    #[test]
    fn recognizes_clap26_list_files_list_contents_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-l", "-L"]);
        assert!(m.get_flag(LIST_FILES));
        assert!(m.get_flag(LIST_CONTENTS));
    }

    #[test]
    fn recognizes_clap26_master_quiet_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--master", "--quiet"]);
        assert!(m.get_flag(MASTER));
        assert!(m.get_flag(SILENT));
    }

    #[test]
    fn recognizes_clap26_directory_verbose_twice_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-d", "-vv"]);
        assert!(m.get_flag(DIRECTORY));
        assert_eq!(m.get_count(VERBOSE), 2);
    }

    #[test]
    fn recognizes_clap26_positional_comma_in_name() {
        let m = parse_opts().get_matches_from(vec!["tp", "a,b,c.txt"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("a,b,c.txt")
        );
    }

    #[test]
    fn recognizes_clap26_program_name_temprs_with_clear() {
        let m = parse_opts().get_matches_from(vec!["temprs", "--clear"]);
        assert!(m.get_flag(CLEAR));
    }

    #[test]
    fn recognizes_clap26_output_add_input_short_chain() {
        let m = parse_opts().get_matches_from(vec!["tp", "-o", "9", "-a", "8", "-i", "7"]);
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("9"));
        assert_eq!(m.get_one::<String>(ADD).map(|s| s.as_str()), Some("8"));
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("7"));
    }

    #[test]
    fn recognizes_clap26_tag_long_multiline_escape() {
        let m = parse_opts().get_matches_from(vec!["tp", "--name", "line1\\nline2"]);
        assert_eq!(
            m.get_one::<String>(TAG).map(|s| s.as_str()),
            Some("line1\\nline2")
        );
    }

    #[test]
    fn recognizes_clap26_positional_nfs_style() {
        let m = parse_opts().get_matches_from(vec!["tp", "nfs://host/export/path"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("nfs://host/export/path")
        );
    }

    #[test]
    fn recognizes_clap26_size_path_only_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--size", "2", "--path", "2"]);
        assert_eq!(m.get_one::<String>(SIZE).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("2"));
    }

    // ── clap coverage round 27 ─────────────────────────────

    #[test]
    fn recognizes_clap27_positional_smb_uri() {
        let m = parse_opts().get_matches_from(vec!["tp", r"\\server\share\file.txt"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some(r"\\server\share\file.txt")
        );
    }

    #[test]
    fn recognizes_clap27_positional_data_image_svg() {
        let m = parse_opts().get_matches_from(vec!["tp", "data:image/svg+xml,<svg/>"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("data:image/svg+xml,<svg/>")
        );
    }

    #[test]
    fn recognizes_clap27_output_equals_max_u16() {
        let m = parse_opts().get_matches_from(vec!["tp", "--output=65535"]);
        assert_eq!(
            m.get_one::<String>(OUTPUT).map(|s| s.as_str()),
            Some("65535")
        );
    }

    #[test]
    fn recognizes_clap27_input_long_zero() {
        let m = parse_opts().get_matches_from(vec!["tp", "--input=0"]);
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("0"));
    }

    #[test]
    fn recognizes_clap27_cat_twenty_two_indices_long() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--cat", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13",
            "14", "15", "16", "17", "18", "19", "20", "21", "22",
        ]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(
            v,
            vec![
                "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15",
                "16", "17", "18", "19", "20", "21", "22"
            ]
        );
    }

    #[test]
    fn recognizes_clap27_grep_non_greedy() {
        let m = parse_opts().get_matches_from(vec!["tp", "-g", "a+?"]);
        assert_eq!(m.get_one::<String>(GREP).map(|s| s.as_str()), Some("a+?"));
    }

    #[test]
    fn recognizes_clap27_expire_nan_string() {
        let m = parse_opts().get_matches_from(vec!["tp", "--expire", "nan"]);
        assert_eq!(m.get_one::<String>(EXPIRE).map(|s| s.as_str()), Some("nan"));
    }

    #[test]
    fn recognizes_clap27_sort_size_rev_positional() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort", "size", "--rev", "out.log"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("size"));
        assert!(m.get_flag(REVERSE));
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("out.log")
        );
    }

    #[test]
    fn recognizes_clap27_shift_unshift_pop_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-s", "-u", "-p"]);
        assert!(m.get_flag(SHIFT));
        assert!(m.get_flag(UNSHIFT));
        assert!(m.get_flag(POP));
    }

    #[test]
    fn recognizes_clap27_add_remove_output_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-a", "1", "-r", "2", "-o", "3"]);
        assert_eq!(m.get_one::<String>(ADD).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(REMOVE).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("3"));
    }

    #[test]
    fn recognizes_clap27_replace_three_args_carriage_return() {
        let m = parse_opts().get_matches_from(vec!["tp", "--replace", "0", "\r", "\n"]);
        let v: Vec<String> = m.get_many(REPLACE).unwrap().cloned().collect();
        assert_eq!(v, vec!["0", "\r", "\n"]);
    }

    #[test]
    fn recognizes_clap27_diff_swap_move_long_short_mix() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--diff", "A", "B", "--swap", "C", "D", "-M", "E", "F",
        ]);
        assert_eq!(
            m.get_many(DIFF).unwrap().cloned().collect::<Vec<String>>(),
            vec!["A", "B"]
        );
        assert_eq!(
            m.get_many(SWAP).unwrap().cloned().collect::<Vec<String>>(),
            vec!["C", "D"]
        );
        assert_eq!(
            m.get_many(MOVE).unwrap().cloned().collect::<Vec<String>>(),
            vec!["E", "F"]
        );
    }

    #[test]
    fn recognizes_clap27_verbose_twenty_six_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-vvvvvvvvvvvvvvvvvvvvvvvvvv"]);
        assert_eq!(m.get_count(VERBOSE), 26);
    }

    #[test]
    fn recognizes_clap27_double_dash_positional_only_slash() {
        let m = parse_opts().get_matches_from(vec!["tp", "--", "/"]);
        assert_eq!(m.get_one::<String>(ARGFILE).map(|s| s.as_str()), Some("/"));
    }

    #[test]
    fn recognizes_clap27_grep_append_dup_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-g", "pat", "-A", "1", "-x", "2"]);
        assert_eq!(m.get_one::<String>(GREP).map(|s| s.as_str()), Some("pat"));
        assert_eq!(m.get_one::<String>(APPEND).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("2"));
    }

    #[test]
    fn recognizes_clap27_head_wc_size_long() {
        let m = parse_opts()
            .get_matches_from(vec!["tp", "--head", "1", "50", "--wc", "1", "--size", "1"]);
        assert_eq!(
            m.get_many::<String>(HEAD)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["1", "50"]
        );
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(SIZE).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_clap27_tail_path_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--tail", "2", "99", "--path", "2"]);
        assert_eq!(
            m.get_many::<String>(TAIL)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["2", "99"]
        );
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("2"));
    }

    #[test]
    fn recognizes_clap27_cat_short_eleven_indices() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "-C", "20", "21", "22", "23", "24", "25", "26", "27", "28", "29", "30",
        ]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(
            v,
            vec![
                "20", "21", "22", "23", "24", "25", "26", "27", "28", "29", "30"
            ]
        );
    }

    #[test]
    fn recognizes_clap27_sort_equals_size_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort=size"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("size"));
    }

    #[test]
    fn recognizes_clap27_remove_add_long_equals() {
        let m = parse_opts().get_matches_from(vec!["tp", "--remove=10", "--add=11"]);
        assert_eq!(m.get_one::<String>(REMOVE).map(|s| s.as_str()), Some("10"));
        assert_eq!(m.get_one::<String>(ADD).map(|s| s.as_str()), Some("11"));
    }

    #[test]
    fn recognizes_clap27_count_list_files_numbered_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-k", "-n"]);
        assert!(m.get_flag(COUNT));
        assert!(m.get_flag(LIST_FILES_NUMBERED));
    }

    #[test]
    fn recognizes_clap27_rev_sort_expire_long() {
        let m =
            parse_opts().get_matches_from(vec!["tp", "--rev", "--sort", "name", "--expire", "24"]);
        assert!(m.get_flag(REVERSE));
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("name"));
        assert_eq!(m.get_one::<String>(EXPIRE).map(|s| s.as_str()), Some("24"));
    }

    #[test]
    fn recognizes_clap27_head_tail_positive_only() {
        let m =
            parse_opts().get_matches_from(vec!["tp", "--head", "3", "100", "--tail", "4", "200"]);
        assert_eq!(
            m.get_many::<String>(HEAD)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["3", "100"]
        );
        assert_eq!(
            m.get_many::<String>(TAIL)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["4", "200"]
        );
    }

    #[test]
    fn recognizes_clap27_edit_info_cat_short() {
        let m =
            parse_opts().get_matches_from(vec!["tp", "-e", "5", "-I", "6", "-C", "5", "6", "7"]);
        assert_eq!(m.get_one::<String>(EDIT).map(|s| s.as_str()), Some("5"));
        assert_eq!(m.get_one::<String>(INFO).map(|s| s.as_str()), Some("6"));
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(v, vec!["5", "6", "7"]);
    }

    #[test]
    fn recognizes_clap27_rename_diff_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-R", "old", "new", "-D", "0", "1"]);
        assert_eq!(
            m.get_many(RENAME)
                .unwrap()
                .cloned()
                .collect::<Vec<String>>(),
            vec!["old", "new"]
        );
        assert_eq!(
            m.get_many(DIFF).unwrap().cloned().collect::<Vec<String>>(),
            vec!["0", "1"]
        );
    }

    #[test]
    fn recognizes_clap27_program_name_tp_with_directory() {
        let m = parse_opts().get_matches_from(vec!["tp", "--dir"]);
        assert!(m.get_flag(DIRECTORY));
    }

    #[test]
    fn recognizes_clap27_positional_semicolon_path() {
        let m = parse_opts().get_matches_from(vec!["tp", "C:;D:;file.txt"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("C:;D:;file.txt")
        );
    }

    #[test]
    fn recognizes_clap27_dup_short_equals_negative() {
        let m = parse_opts().get_matches_from(vec!["tp", "-x=-1"]);
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("-1"));
    }

    #[test]
    fn recognizes_clap27_append_short_equals_negative() {
        let m = parse_opts().get_matches_from(vec!["tp", "-A=-2"]);
        assert_eq!(m.get_one::<String>(APPEND).map(|s| s.as_str()), Some("-2"));
    }

    #[test]
    fn recognizes_clap27_clear_rev_quiet_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--clear", "--rev", "--quiet"]);
        assert!(m.get_flag(CLEAR));
        assert!(m.get_flag(REVERSE));
        assert!(m.get_flag(SILENT));
    }

    #[test]
    fn recognizes_clap27_positional_s3_style() {
        let m = parse_opts().get_matches_from(vec!["tp", "s3://bucket/key/object"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("s3://bucket/key/object")
        );
    }

    #[test]
    fn recognizes_clap27_program_name_temprs_with_master() {
        let m = parse_opts().get_matches_from(vec!["temprs", "--master"]);
        assert!(m.get_flag(MASTER));
    }

    // ── clap coverage round 28 ─────────────────────────────

    #[test]
    fn recognizes_clap28_positional_gs_uri() {
        let m = parse_opts().get_matches_from(vec!["tp", "gs://bucket/object/path"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("gs://bucket/object/path")
        );
    }

    #[test]
    fn recognizes_clap28_positional_azure_blob() {
        let m = parse_opts().get_matches_from(vec!["tp", "https://acct.blob.core.windows.net/c/f"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("https://acct.blob.core.windows.net/c/f")
        );
    }

    #[test]
    fn recognizes_clap28_output_short_one() {
        let m = parse_opts().get_matches_from(vec!["tp", "-o", "1"]);
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_clap28_input_long_negative_one() {
        let m = parse_opts().get_matches_from(vec!["tp", "--input=-1"]);
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("-1"));
    }

    #[test]
    fn recognizes_clap28_cat_twenty_three_indices_long() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--cat", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12",
            "13", "14", "15", "16", "17", "18", "19", "20", "21", "22",
        ]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(
            v,
            vec![
                "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                "15", "16", "17", "18", "19", "20", "21", "22"
            ]
        );
    }

    #[test]
    fn recognizes_clap28_grep_caret_anchor() {
        let m = parse_opts().get_matches_from(vec!["tp", "-g", "^start"]);
        assert_eq!(
            m.get_one::<String>(GREP).map(|s| s.as_str()),
            Some("^start")
        );
    }

    #[test]
    fn recognizes_clap28_expire_negative_string() {
        let m = parse_opts().get_matches_from(vec!["tp", "--expire=-1"]);
        assert_eq!(m.get_one::<String>(EXPIRE).map(|s| s.as_str()), Some("-1"));
    }

    #[test]
    fn recognizes_clap28_sort_name_positional_rev() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort", "name", "--rev", "in.txt"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("name"));
        assert!(m.get_flag(REVERSE));
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("in.txt")
        );
    }

    #[test]
    fn recognizes_clap28_pop_shift_only_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-p", "-s"]);
        assert!(m.get_flag(POP));
        assert!(m.get_flag(SHIFT));
    }

    #[test]
    fn recognizes_clap28_unshift_clear_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-u", "-c"]);
        assert!(m.get_flag(UNSHIFT));
        assert!(m.get_flag(CLEAR));
    }

    #[test]
    fn recognizes_clap28_remove_dup_input_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-r", "0", "-x", "1", "-i", "2"]);
        assert_eq!(m.get_one::<String>(REMOVE).map(|s| s.as_str()), Some("0"));
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("2"));
    }

    #[test]
    fn recognizes_clap28_replace_three_args_null_byte_display() {
        let m = parse_opts().get_matches_from(vec!["tp", "--replace", "1", "a", "\0"]);
        let v: Vec<String> = m.get_many(REPLACE).unwrap().cloned().collect();
        assert_eq!(v, vec!["1", "a", "\0"]);
    }

    #[test]
    fn recognizes_clap28_swap_diff_move_short_chain() {
        let m = parse_opts()
            .get_matches_from(vec!["tp", "-S", "a", "b", "-D", "c", "d", "-M", "e", "f"]);
        assert_eq!(
            m.get_many(SWAP).unwrap().cloned().collect::<Vec<String>>(),
            vec!["a", "b"]
        );
        assert_eq!(
            m.get_many(DIFF).unwrap().cloned().collect::<Vec<String>>(),
            vec!["c", "d"]
        );
        assert_eq!(
            m.get_many(MOVE).unwrap().cloned().collect::<Vec<String>>(),
            vec!["e", "f"]
        );
    }

    #[test]
    fn recognizes_clap28_verbose_twenty_seven_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-vvvvvvvvvvvvvvvvvvvvvvvvvvv"]);
        assert_eq!(m.get_count(VERBOSE), 27);
    }

    #[test]
    fn recognizes_clap28_double_dash_positional_empty_string() {
        let m = parse_opts().get_matches_from(vec!["tp", "--", ""]);
        assert_eq!(m.get_one::<String>(ARGFILE).map(|s| s.as_str()), Some(""));
    }

    #[test]
    fn recognizes_clap28_grep_list_contents_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--grep", "pat", "--list-contents"]);
        assert_eq!(m.get_one::<String>(GREP).map(|s| s.as_str()), Some("pat"));
        assert!(m.get_flag(LIST_CONTENTS));
    }

    #[test]
    fn recognizes_clap28_wc_size_path_long() {
        let m =
            parse_opts().get_matches_from(vec!["tp", "--wc", "3", "--size", "3", "--path", "3"]);
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("3"));
        assert_eq!(m.get_one::<String>(SIZE).map(|s| s.as_str()), Some("3"));
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("3"));
    }

    #[test]
    fn recognizes_clap28_head_tail_wc_long() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--head", "1", "5", "--tail", "2", "6", "--wc", "1",
        ]);
        assert_eq!(
            m.get_many::<String>(HEAD)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["1", "5"]
        );
        assert_eq!(
            m.get_many::<String>(TAIL)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["2", "6"]
        );
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_clap28_cat_short_twelve_indices() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "-C", "31", "32", "33", "34", "35", "36", "37", "38", "39", "40", "41", "42",
        ]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(
            v,
            vec![
                "31", "32", "33", "34", "35", "36", "37", "38", "39", "40", "41", "42"
            ]
        );
    }

    #[test]
    fn recognizes_clap28_sort_equals_mtime_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort=mtime"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("mtime"));
    }

    #[test]
    fn recognizes_clap28_output_append_long_equals() {
        let m = parse_opts().get_matches_from(vec!["tp", "--output=4", "--append=5"]);
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("4"));
        assert_eq!(m.get_one::<String>(APPEND).map(|s| s.as_str()), Some("5"));
    }

    #[test]
    fn recognizes_clap28_rename_swap_long_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "--rename", "u", "v", "--swap", "1", "2"]);
        assert_eq!(
            m.get_many(RENAME)
                .unwrap()
                .cloned()
                .collect::<Vec<String>>(),
            vec!["u", "v"]
        );
        assert_eq!(
            m.get_many(SWAP).unwrap().cloned().collect::<Vec<String>>(),
            vec!["1", "2"]
        );
    }

    #[test]
    fn recognizes_clap28_list_numbered_verbose_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-N", "-vvv"]);
        assert!(m.get_flag(LIST_CONTENTS_NUMBERED));
        assert_eq!(m.get_count(VERBOSE), 3);
    }

    #[test]
    fn recognizes_clap28_expire_path_rev_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--expire", "6", "--path", "1", "--rev"]);
        assert_eq!(m.get_one::<String>(EXPIRE).map(|s| s.as_str()), Some("6"));
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("1"));
        assert!(m.get_flag(REVERSE));
    }

    #[test]
    fn recognizes_clap28_edit_info_long_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "--edit", "8", "--info", "9"]);
        assert_eq!(m.get_one::<String>(EDIT).map(|s| s.as_str()), Some("8"));
        assert_eq!(m.get_one::<String>(INFO).map(|s| s.as_str()), Some("9"));
    }

    #[test]
    fn recognizes_clap28_positional_backtick_name() {
        let m = parse_opts().get_matches_from(vec!["tp", "`echo`.txt"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("`echo`.txt")
        );
    }

    #[test]
    fn recognizes_clap28_program_name_temprs_with_count() {
        let m = parse_opts().get_matches_from(vec!["temprs", "--count"]);
        assert!(m.get_flag(COUNT));
    }

    #[test]
    fn recognizes_clap28_tag_short_equals_emoji() {
        let m = parse_opts().get_matches_from(vec!["tp", "-w=🔖"]);
        assert_eq!(m.get_one::<String>(TAG).map(|s| s.as_str()), Some("🔖"));
    }

    #[test]
    fn recognizes_clap28_add_remove_long_chain() {
        let m = parse_opts().get_matches_from(vec!["tp", "--add", "100", "--remove", "101"]);
        assert_eq!(m.get_one::<String>(ADD).map(|s| s.as_str()), Some("100"));
        assert_eq!(m.get_one::<String>(REMOVE).map(|s| s.as_str()), Some("101"));
    }

    #[test]
    fn recognizes_clap28_master_list_files_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-m", "-l"]);
        assert!(m.get_flag(MASTER));
        assert!(m.get_flag(LIST_FILES));
    }

    #[test]
    fn recognizes_clap28_positional_ftp_url() {
        let m = parse_opts().get_matches_from(vec!["tp", "ftp://user:pass@host/path"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("ftp://user:pass@host/path")
        );
    }

    #[test]
    fn recognizes_clap28_directory_quiet_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-d", "-q"]);
        assert!(m.get_flag(DIRECTORY));
        assert!(m.get_flag(SILENT));
    }

    // ── clap coverage round 29 ─────────────────────────────

    #[test]
    fn recognizes_clap29_positional_crates_io_path() {
        let m = parse_opts().get_matches_from(vec!["tp", "crates.io/crates/foo/1.0.0"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("crates.io/crates/foo/1.0.0")
        );
    }

    #[test]
    fn recognizes_clap29_positional_mailto_uri() {
        let m = parse_opts().get_matches_from(vec!["tp", "mailto:user@example.com"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("mailto:user@example.com")
        );
    }

    #[test]
    fn recognizes_clap29_output_long_equals_plus_index() {
        let m = parse_opts().get_matches_from(vec!["tp", "--output=+0"]);
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("+0"));
    }

    #[test]
    fn recognizes_clap29_input_long_equals_tag() {
        let m = parse_opts().get_matches_from(vec!["tp", "--input=@named"]);
        assert_eq!(
            m.get_one::<String>(INPUT).map(|s| s.as_str()),
            Some("@named")
        );
    }

    #[test]
    fn recognizes_clap29_cat_twenty_four_indices_long() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--cat", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13",
            "14", "15", "16", "17", "18", "19", "20", "21", "22", "23", "24",
        ]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(
            v,
            vec![
                "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15",
                "16", "17", "18", "19", "20", "21", "22", "23", "24"
            ]
        );
    }

    #[test]
    fn recognizes_clap29_grep_possessive_quantifier() {
        let m = parse_opts().get_matches_from(vec!["tp", "-g", "a++"]);
        assert_eq!(m.get_one::<String>(GREP).map(|s| s.as_str()), Some("a++"));
    }

    #[test]
    fn recognizes_clap29_expire_iso_duration_style() {
        let m = parse_opts().get_matches_from(vec!["tp", "--expire", "P1DT12H"]);
        assert_eq!(
            m.get_one::<String>(EXPIRE).map(|s| s.as_str()),
            Some("P1DT12H")
        );
    }

    #[test]
    fn recognizes_clap29_sort_mtime_positional_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort", "mtime", "data.bin"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("mtime"));
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("data.bin")
        );
    }

    #[test]
    fn recognizes_clap29_shift_unshift_pop_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-s", "-u", "-p"]);
        assert!(m.get_flag(SHIFT));
        assert!(m.get_flag(UNSHIFT));
        assert!(m.get_flag(POP));
    }

    #[test]
    fn recognizes_clap29_append_output_input_long() {
        let m = parse_opts()
            .get_matches_from(vec!["tp", "--append", "1", "--output", "2", "--input", "3"]);
        assert_eq!(m.get_one::<String>(APPEND).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("3"));
    }

    #[test]
    fn recognizes_clap29_replace_three_args_only_spaces() {
        let m = parse_opts().get_matches_from(vec!["tp", "--replace", "0", "  ", "   "]);
        let v: Vec<String> = m.get_many(REPLACE).unwrap().cloned().collect();
        assert_eq!(v, vec!["0", "  ", "   "]);
    }

    #[test]
    fn recognizes_clap29_move_diff_swap_long_short_mix() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--mv", "1", "2", "--diff", "3", "4", "-S", "5", "6",
        ]);
        assert_eq!(
            m.get_many(MOVE).unwrap().cloned().collect::<Vec<String>>(),
            vec!["1", "2"]
        );
        assert_eq!(
            m.get_many(DIFF).unwrap().cloned().collect::<Vec<String>>(),
            vec!["3", "4"]
        );
        assert_eq!(
            m.get_many(SWAP).unwrap().cloned().collect::<Vec<String>>(),
            vec!["5", "6"]
        );
    }

    #[test]
    fn recognizes_clap29_verbose_twenty_eight_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-vvvvvvvvvvvvvvvvvvvvvvvvvvvv"]);
        assert_eq!(m.get_count(VERBOSE), 28);
    }

    #[test]
    fn recognizes_clap29_double_dash_positional_percent() {
        let m = parse_opts().get_matches_from(vec!["tp", "--", "%"]);
        assert_eq!(m.get_one::<String>(ARGFILE).map(|s| s.as_str()), Some("%"));
    }

    #[test]
    fn recognizes_clap29_grep_count_quiet_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-g", "x", "-k", "-q"]);
        assert_eq!(m.get_one::<String>(GREP).map(|s| s.as_str()), Some("x"));
        assert!(m.get_flag(COUNT));
        assert!(m.get_flag(SILENT));
    }

    #[test]
    fn recognizes_clap29_path_tail_size_long() {
        let m = parse_opts()
            .get_matches_from(vec!["tp", "--path", "7", "--tail", "7", "3", "--size", "7"]);
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("7"));
        assert_eq!(
            m.get_many::<String>(TAIL)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["7", "3"]
        );
        assert_eq!(m.get_one::<String>(SIZE).map(|s| s.as_str()), Some("7"));
    }

    #[test]
    fn recognizes_clap29_head_wc_only_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--head", "0", "12", "--wc", "0"]);
        assert_eq!(
            m.get_many::<String>(HEAD)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["0", "12"]
        );
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("0"));
    }

    #[test]
    fn recognizes_clap29_cat_short_thirteen_indices() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "-C", "43", "44", "45", "46", "47", "48", "49", "50", "51", "52", "53", "54",
            "55",
        ]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(
            v,
            vec![
                "43", "44", "45", "46", "47", "48", "49", "50", "51", "52", "53", "54", "55"
            ]
        );
    }

    #[test]
    fn recognizes_clap29_sort_bare_long_with_positional() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort", "sorted.txt"]);
        assert_eq!(
            m.get_one::<String>(SORT).map(|s| s.as_str()),
            Some("sorted.txt")
        );
        assert_eq!(m.get_one::<String>(ARGFILE).map(|s| s.as_str()), None);
    }

    #[test]
    fn recognizes_clap29_remove_add_short_equals() {
        let m = parse_opts().get_matches_from(vec!["tp", "-r=7", "-a=8"]);
        assert_eq!(m.get_one::<String>(REMOVE).map(|s| s.as_str()), Some("7"));
        assert_eq!(m.get_one::<String>(ADD).map(|s| s.as_str()), Some("8"));
    }

    #[test]
    fn recognizes_clap29_rename_move_long_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "--rename", "p", "q", "--mv", "8", "9"]);
        assert_eq!(
            m.get_many(RENAME)
                .unwrap()
                .cloned()
                .collect::<Vec<String>>(),
            vec!["p", "q"]
        );
        assert_eq!(
            m.get_many(MOVE).unwrap().cloned().collect::<Vec<String>>(),
            vec!["8", "9"]
        );
    }

    #[test]
    fn recognizes_clap29_list_files_contents_numbered_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-l", "-L", "-N"]);
        assert!(m.get_flag(LIST_FILES));
        assert!(m.get_flag(LIST_CONTENTS));
        assert!(m.get_flag(LIST_CONTENTS_NUMBERED));
    }

    #[test]
    fn recognizes_clap29_rev_expire_sort_long() {
        let m =
            parse_opts().get_matches_from(vec!["tp", "--rev", "--expire", "2", "--sort", "size"]);
        assert!(m.get_flag(REVERSE));
        assert_eq!(m.get_one::<String>(EXPIRE).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("size"));
    }

    #[test]
    fn recognizes_clap29_head_tail_negative_line_counts() {
        let m = parse_opts().get_matches_from(vec!["tp", "--head", "1", "-9", "--tail", "2", "-8"]);
        assert_eq!(
            m.get_many::<String>(HEAD)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["1", "-9"]
        );
        assert_eq!(
            m.get_many::<String>(TAIL)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["2", "-8"]
        );
    }

    #[test]
    fn recognizes_clap29_edit_dup_append_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-e", "4", "-x", "5", "-A", "6"]);
        assert_eq!(m.get_one::<String>(EDIT).map(|s| s.as_str()), Some("4"));
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("5"));
        assert_eq!(m.get_one::<String>(APPEND).map(|s| s.as_str()), Some("6"));
    }

    #[test]
    fn recognizes_clap29_positional_bracket_glob() {
        let m = parse_opts().get_matches_from(vec!["tp", "file[0-9].txt"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("file[0-9].txt")
        );
    }

    #[test]
    fn recognizes_clap29_program_name_temprs_with_directory() {
        let m = parse_opts().get_matches_from(vec!["temprs", "--dir"]);
        assert!(m.get_flag(DIRECTORY));
    }

    #[test]
    fn recognizes_clap29_tag_long_zwj_sequence() {
        let m = parse_opts().get_matches_from(vec!["tp", "--name", "👨\u{200d}👩\u{200d}👧"]);
        assert_eq!(
            m.get_one::<String>(TAG).map(|s| s.as_str()),
            Some("👨\u{200d}👩\u{200d}👧")
        );
    }

    #[test]
    fn recognizes_clap29_clear_count_rev_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-c", "-k", "--rev"]);
        assert!(m.get_flag(CLEAR));
        assert!(m.get_flag(COUNT));
        assert!(m.get_flag(REVERSE));
    }

    #[test]
    fn recognizes_clap29_info_path_wc_short_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "-I", "0", "--path", "0", "--wc", "0"]);
        assert_eq!(m.get_one::<String>(INFO).map(|s| s.as_str()), Some("0"));
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("0"));
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("0"));
    }

    #[test]
    fn recognizes_clap29_positional_ws_uri() {
        let m = parse_opts().get_matches_from(vec!["tp", "ws://host.example/socket"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("ws://host.example/socket")
        );
    }

    #[test]
    fn recognizes_clap29_diff_only_long_pair() {
        let m = parse_opts().get_matches_from(vec!["tp", "--diff", "left", "right"]);
        assert_eq!(
            m.get_many(DIFF).unwrap().cloned().collect::<Vec<String>>(),
            vec!["left", "right"]
        );
    }

    // ── clap coverage round 30 ─────────────────────────────

    #[test]
    fn recognizes_clap30_positional_wss_uri() {
        let m = parse_opts().get_matches_from(vec!["tp", "wss://secure.example/ws"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("wss://secure.example/ws")
        );
    }

    #[test]
    fn recognizes_clap30_positional_redis_url() {
        let m = parse_opts().get_matches_from(vec!["tp", "redis://:secret@localhost:6379/0"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("redis://:secret@localhost:6379/0")
        );
    }

    #[test]
    fn recognizes_clap30_output_short_zero() {
        let m = parse_opts().get_matches_from(vec!["tp", "-o", "0"]);
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("0"));
    }

    #[test]
    fn recognizes_clap30_input_long_equals_empty() {
        let m = parse_opts().get_matches_from(vec!["tp", "--input="]);
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some(""));
    }

    #[test]
    fn recognizes_clap30_cat_twenty_five_indices_long() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--cat", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13",
            "14", "15", "16", "17", "18", "19", "20", "21", "22", "23", "24", "25",
        ]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(
            v,
            vec![
                "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15",
                "16", "17", "18", "19", "20", "21", "22", "23", "24", "25"
            ]
        );
    }

    #[test]
    fn recognizes_clap30_grep_comment_pattern() {
        let m = parse_opts().get_matches_from(vec!["tp", "-g", "(?#comment)^foo"]);
        assert_eq!(
            m.get_one::<String>(GREP).map(|s| s.as_str()),
            Some("(?#comment)^foo")
        );
    }

    #[test]
    fn recognizes_clap30_expire_plus_infinity() {
        let m = parse_opts().get_matches_from(vec!["tp", "--expire", "+∞"]);
        assert_eq!(m.get_one::<String>(EXPIRE).map(|s| s.as_str()), Some("+∞"));
    }

    #[test]
    fn recognizes_clap30_sort_size_positional_rev() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort", "size", "--rev", "big.dat"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("size"));
        assert!(m.get_flag(REVERSE));
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("big.dat")
        );
    }

    #[test]
    fn recognizes_clap30_pop_unshift_clear_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-p", "-u", "-c"]);
        assert!(m.get_flag(POP));
        assert!(m.get_flag(UNSHIFT));
        assert!(m.get_flag(CLEAR));
    }

    #[test]
    fn recognizes_clap30_add_remove_output_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-a", "3", "-r", "4", "-o", "5"]);
        assert_eq!(m.get_one::<String>(ADD).map(|s| s.as_str()), Some("3"));
        assert_eq!(m.get_one::<String>(REMOVE).map(|s| s.as_str()), Some("4"));
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("5"));
    }

    #[test]
    fn recognizes_clap30_replace_three_args_mixed_quotes() {
        let m = parse_opts().get_matches_from(vec!["tp", "--replace", "1", "'", "\""]);
        let v: Vec<String> = m.get_many(REPLACE).unwrap().cloned().collect();
        assert_eq!(v, vec!["1", "'", "\""]);
    }

    #[test]
    fn recognizes_clap30_diff_move_swap_long_chain() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--diff", "a", "b", "--mv", "c", "d", "--swap", "e", "f",
        ]);
        assert_eq!(
            m.get_many(DIFF).unwrap().cloned().collect::<Vec<String>>(),
            vec!["a", "b"]
        );
        assert_eq!(
            m.get_many(MOVE).unwrap().cloned().collect::<Vec<String>>(),
            vec!["c", "d"]
        );
        assert_eq!(
            m.get_many(SWAP).unwrap().cloned().collect::<Vec<String>>(),
            vec!["e", "f"]
        );
    }

    #[test]
    fn recognizes_clap30_verbose_twenty_nine_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-vvvvvvvvvvvvvvvvvvvvvvvvvvvvv"]);
        assert_eq!(m.get_count(VERBOSE), 29);
    }

    #[test]
    fn recognizes_clap30_double_dash_positional_dotdot() {
        let m = parse_opts().get_matches_from(vec!["tp", "--", ".."]);
        assert_eq!(m.get_one::<String>(ARGFILE).map(|s| s.as_str()), Some(".."));
    }

    #[test]
    fn recognizes_clap30_grep_append_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-g", "rx", "-A", "0"]);
        assert_eq!(m.get_one::<String>(GREP).map(|s| s.as_str()), Some("rx"));
        assert_eq!(m.get_one::<String>(APPEND).map(|s| s.as_str()), Some("0"));
    }

    #[test]
    fn recognizes_clap30_wc_path_head_long() {
        let m = parse_opts()
            .get_matches_from(vec!["tp", "--wc", "6", "--path", "6", "--head", "6", "20"]);
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("6"));
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("6"));
        assert_eq!(
            m.get_many::<String>(HEAD)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["6", "20"]
        );
    }

    #[test]
    fn recognizes_clap30_tail_size_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--tail", "1", "15", "--size", "1"]);
        assert_eq!(
            m.get_many::<String>(TAIL)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["1", "15"]
        );
        assert_eq!(m.get_one::<String>(SIZE).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_clap30_cat_short_fourteen_indices() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "-C", "56", "57", "58", "59", "60", "61", "62", "63", "64", "65", "66", "67",
            "68", "69",
        ]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(
            v,
            vec![
                "56", "57", "58", "59", "60", "61", "62", "63", "64", "65", "66", "67", "68", "69"
            ]
        );
    }

    #[test]
    fn recognizes_clap30_sort_name_equals_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort=name"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("name"));
    }

    #[test]
    fn recognizes_clap30_dup_append_long_equals() {
        let m = parse_opts().get_matches_from(vec!["tp", "--dup=3", "--append=4"]);
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("3"));
        assert_eq!(m.get_one::<String>(APPEND).map(|s| s.as_str()), Some("4"));
    }

    #[test]
    fn recognizes_clap30_rename_diff_short_long_mix() {
        let m = parse_opts().get_matches_from(vec!["tp", "-R", "x", "y", "--diff", "0", "1"]);
        assert_eq!(
            m.get_many(RENAME)
                .unwrap()
                .cloned()
                .collect::<Vec<String>>(),
            vec!["x", "y"]
        );
        assert_eq!(
            m.get_many(DIFF).unwrap().cloned().collect::<Vec<String>>(),
            vec!["0", "1"]
        );
    }

    #[test]
    fn recognizes_clap30_master_verbose_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-m", "-v"]);
        assert!(m.get_flag(MASTER));
        assert_eq!(m.get_count(VERBOSE), 1);
    }

    #[test]
    fn recognizes_clap30_rev_quiet_expire_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--rev", "--quiet", "--expire", "168"]);
        assert!(m.get_flag(REVERSE));
        assert!(m.get_flag(SILENT));
        assert_eq!(m.get_one::<String>(EXPIRE).map(|s| s.as_str()), Some("168"));
    }

    #[test]
    fn recognizes_clap30_head_tail_wc_short_long() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--head", "2", "8", "--tail", "3", "9", "--wc", "2",
        ]);
        assert_eq!(
            m.get_many::<String>(HEAD)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["2", "8"]
        );
        assert_eq!(
            m.get_many::<String>(TAIL)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["3", "9"]
        );
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("2"));
    }

    #[test]
    fn recognizes_clap30_edit_info_cat_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-e", "1", "-I", "2", "-C", "1", "2"]);
        assert_eq!(m.get_one::<String>(EDIT).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(INFO).map(|s| s.as_str()), Some("2"));
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(v, vec!["1", "2"]);
    }

    #[test]
    fn recognizes_clap30_positional_curly_brace_glob() {
        let m = parse_opts().get_matches_from(vec!["tp", "file.{a,b,c}"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("file.{a,b,c}")
        );
    }

    #[test]
    fn recognizes_clap30_program_name_temprs_with_list_numbered() {
        let m = parse_opts().get_matches_from(vec!["temprs", "-n"]);
        assert!(m.get_flag(LIST_FILES_NUMBERED));
    }

    #[test]
    fn recognizes_clap30_tag_short_equals_comma() {
        let m = parse_opts().get_matches_from(vec!["tp", "-w=a,b"]);
        assert_eq!(m.get_one::<String>(TAG).map(|s| s.as_str()), Some("a,b"));
    }

    #[test]
    fn recognizes_clap30_list_contents_only_numbered_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-N"]);
        assert!(m.get_flag(LIST_CONTENTS_NUMBERED));
    }

    #[test]
    fn recognizes_clap30_shift_pop_quiet_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-s", "-p", "-q"]);
        assert!(m.get_flag(SHIFT));
        assert!(m.get_flag(POP));
        assert!(m.get_flag(SILENT));
    }

    #[test]
    fn recognizes_clap30_count_only_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-k"]);
        assert!(m.get_flag(COUNT));
    }

    #[test]
    fn recognizes_clap30_swap_only_long_pair() {
        let m = parse_opts().get_matches_from(vec!["tp", "--swap", "first", "second"]);
        assert_eq!(
            m.get_many(SWAP).unwrap().cloned().collect::<Vec<String>>(),
            vec!["first", "second"]
        );
    }

    // ── clap coverage round 31 ─────────────────────────────

    #[test]
    fn recognizes_clap31_positional_jdbc_url() {
        let m = parse_opts().get_matches_from(vec!["tp", "jdbc:postgresql://localhost:5432/db"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("jdbc:postgresql://localhost:5432/db")
        );
    }

    #[test]
    fn recognizes_clap31_positional_odbc_style() {
        let m = parse_opts().get_matches_from(vec!["tp", "odbc:DSN=mydsn;UID=u;PWD=p"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("odbc:DSN=mydsn;UID=u;PWD=p")
        );
    }

    #[test]
    fn recognizes_clap31_output_long_equals_negative() {
        let m = parse_opts().get_matches_from(vec!["tp", "--output=-10"]);
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("-10"));
    }

    #[test]
    fn recognizes_clap31_input_long_equals_numeric() {
        let m = parse_opts().get_matches_from(vec!["tp", "--input=99"]);
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("99"));
    }

    #[test]
    fn recognizes_clap31_cat_twenty_six_indices_long() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--cat", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12",
            "13", "14", "15", "16", "17", "18", "19", "20", "21", "22", "23", "24", "25",
        ]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(
            v,
            vec![
                "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                "15", "16", "17", "18", "19", "20", "21", "22", "23", "24", "25"
            ]
        );
    }

    #[test]
    fn recognizes_clap31_grep_atomic_group() {
        let m = parse_opts().get_matches_from(vec!["tp", "-g", "(?>a+)b"]);
        assert_eq!(
            m.get_one::<String>(GREP).map(|s| s.as_str()),
            Some("(?>a+)b")
        );
    }

    #[test]
    fn recognizes_clap31_expire_underscore_style() {
        let m = parse_opts().get_matches_from(vec!["tp", "--expire", "1_000"]);
        assert_eq!(
            m.get_one::<String>(EXPIRE).map(|s| s.as_str()),
            Some("1_000")
        );
    }

    #[test]
    fn recognizes_clap31_sort_name_positional_file() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort", "name", "alpha.txt"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("name"));
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("alpha.txt")
        );
    }

    #[test]
    fn recognizes_clap31_unshift_pop_shift_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-u", "-p", "-s"]);
        assert!(m.get_flag(UNSHIFT));
        assert!(m.get_flag(POP));
        assert!(m.get_flag(SHIFT));
    }

    #[test]
    fn recognizes_clap31_remove_add_dup_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-r", "1", "-a", "2", "-x", "3"]);
        assert_eq!(m.get_one::<String>(REMOVE).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(ADD).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("3"));
    }

    #[test]
    fn recognizes_clap31_replace_three_args_bom() {
        let m = parse_opts().get_matches_from(vec!["tp", "--replace", "0", "\u{feff}", ""]);
        let v: Vec<String> = m.get_many(REPLACE).unwrap().cloned().collect();
        assert_eq!(v, vec!["0", "\u{feff}", ""]);
    }

    #[test]
    fn recognizes_clap31_swap_move_diff_short_chain() {
        let m = parse_opts()
            .get_matches_from(vec!["tp", "-S", "1", "2", "-M", "3", "4", "-D", "5", "6"]);
        assert_eq!(
            m.get_many(SWAP).unwrap().cloned().collect::<Vec<String>>(),
            vec!["1", "2"]
        );
        assert_eq!(
            m.get_many(MOVE).unwrap().cloned().collect::<Vec<String>>(),
            vec!["3", "4"]
        );
        assert_eq!(
            m.get_many(DIFF).unwrap().cloned().collect::<Vec<String>>(),
            vec!["5", "6"]
        );
    }

    #[test]
    fn recognizes_clap31_verbose_thirty_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-vvvvvvvvvvvvvvvvvvvvvvvvvvvvvv"]);
        assert_eq!(m.get_count(VERBOSE), 30);
    }

    #[test]
    fn recognizes_clap31_double_dash_positional_backslash_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "--", r"\"]);
        assert_eq!(m.get_one::<String>(ARGFILE).map(|s| s.as_str()), Some(r"\"));
    }

    #[test]
    fn recognizes_clap31_grep_path_size_short_long() {
        let m =
            parse_opts().get_matches_from(vec!["tp", "-g", "pat", "--path", "1", "--size", "1"]);
        assert_eq!(m.get_one::<String>(GREP).map(|s| s.as_str()), Some("pat"));
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(SIZE).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_clap31_wc_tail_head_long() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--wc", "4", "--tail", "4", "11", "--head", "4", "11",
        ]);
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("4"));
        assert_eq!(
            m.get_many::<String>(TAIL)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["4", "11"]
        );
        assert_eq!(
            m.get_many::<String>(HEAD)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["4", "11"]
        );
    }

    #[test]
    fn recognizes_clap31_cat_short_fifteen_indices() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "-C", "70", "71", "72", "73", "74", "75", "76", "77", "78", "79", "80", "81",
            "82", "83", "84",
        ]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(
            v,
            vec![
                "70", "71", "72", "73", "74", "75", "76", "77", "78", "79", "80", "81", "82", "83",
                "84"
            ]
        );
    }

    #[test]
    fn recognizes_clap31_sort_mtime_equals_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort=mtime"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("mtime"));
    }

    #[test]
    fn recognizes_clap31_output_input_short_chain() {
        let m = parse_opts().get_matches_from(vec!["tp", "-o", "2", "-i", "3"]);
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("3"));
    }

    #[test]
    fn recognizes_clap31_rename_swap_long_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "--rename", "a", "b", "--swap", "0", "1"]);
        assert_eq!(
            m.get_many(RENAME)
                .unwrap()
                .cloned()
                .collect::<Vec<String>>(),
            vec!["a", "b"]
        );
        assert_eq!(
            m.get_many(SWAP).unwrap().cloned().collect::<Vec<String>>(),
            vec!["0", "1"]
        );
    }

    #[test]
    fn recognizes_clap31_list_files_numbered_contents_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-n", "-L"]);
        assert!(m.get_flag(LIST_FILES_NUMBERED));
        assert!(m.get_flag(LIST_CONTENTS));
    }

    #[test]
    fn recognizes_clap31_rev_sort_quiet_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--rev", "--sort", "name", "--quiet"]);
        assert!(m.get_flag(REVERSE));
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("name"));
        assert!(m.get_flag(SILENT));
    }

    #[test]
    fn recognizes_clap31_head_tail_negative_line_counts() {
        let m = parse_opts().get_matches_from(vec!["tp", "--head", "0", "-7", "--tail", "1", "-6"]);
        assert_eq!(
            m.get_many::<String>(HEAD)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["0", "-7"]
        );
        assert_eq!(
            m.get_many::<String>(TAIL)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["1", "-6"]
        );
    }

    #[test]
    fn recognizes_clap31_append_edit_info_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-A", "0", "-e", "1", "-I", "2"]);
        assert_eq!(m.get_one::<String>(APPEND).map(|s| s.as_str()), Some("0"));
        assert_eq!(m.get_one::<String>(EDIT).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(INFO).map(|s| s.as_str()), Some("2"));
    }

    #[test]
    fn recognizes_clap31_positional_angle_brackets() {
        let m = parse_opts().get_matches_from(vec!["tp", "<stdin>"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("<stdin>")
        );
    }

    #[test]
    fn recognizes_clap31_program_name_temprs_with_pop() {
        let m = parse_opts().get_matches_from(vec!["temprs", "--pop"]);
        assert!(m.get_flag(POP));
    }

    #[test]
    fn recognizes_clap31_tag_long_rtl_mark() {
        let m = parse_opts().get_matches_from(vec!["tp", "--name", "a\u{200f}b"]);
        assert_eq!(
            m.get_one::<String>(TAG).map(|s| s.as_str()),
            Some("a\u{200f}b")
        );
    }

    #[test]
    fn recognizes_clap31_clear_rev_count_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--clear", "--rev", "--count"]);
        assert!(m.get_flag(CLEAR));
        assert!(m.get_flag(REVERSE));
        assert!(m.get_flag(COUNT));
    }

    #[test]
    fn recognizes_clap31_move_only_long_pair() {
        let m = parse_opts().get_matches_from(vec!["tp", "--mv", "from", "to"]);
        assert_eq!(
            m.get_many(MOVE).unwrap().cloned().collect::<Vec<String>>(),
            vec!["from", "to"]
        );
    }

    #[test]
    fn recognizes_clap31_positional_mongodb_srv() {
        let m = parse_opts().get_matches_from(vec!["tp", "mongodb+srv://cluster.example/db"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("mongodb+srv://cluster.example/db")
        );
    }

    #[test]
    fn recognizes_clap31_directory_quiet_list_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-d", "-q", "-l"]);
        assert!(m.get_flag(DIRECTORY));
        assert!(m.get_flag(SILENT));
        assert!(m.get_flag(LIST_FILES));
    }

    #[test]
    fn recognizes_clap31_expire_path_wc_long() {
        let m =
            parse_opts().get_matches_from(vec!["tp", "--expire", "7", "--path", "5", "--wc", "5"]);
        assert_eq!(m.get_one::<String>(EXPIRE).map(|s| s.as_str()), Some("7"));
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("5"));
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("5"));
    }

    // ── clap coverage round 32 ─────────────────────────────

    #[test]
    fn recognizes_clap32_positional_amqp_uri() {
        let m =
            parse_opts().get_matches_from(vec!["tp", "amqp://guest:guest@localhost:5672/vhost"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("amqp://guest:guest@localhost:5672/vhost")
        );
    }

    #[test]
    fn recognizes_clap32_positional_kafka_broker() {
        let m =
            parse_opts().get_matches_from(vec!["tp", "kafka://broker1:9092,broker2:9092/topic"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("kafka://broker1:9092,broker2:9092/topic")
        );
    }

    #[test]
    fn recognizes_clap32_output_short_equals() {
        let m = parse_opts().get_matches_from(vec!["tp", "-o=42"]);
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("42"));
    }

    #[test]
    fn recognizes_clap32_input_long_equals_negative() {
        let m = parse_opts().get_matches_from(vec!["tp", "--input=-5"]);
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("-5"));
    }

    #[test]
    fn recognizes_clap32_cat_twenty_seven_indices_long() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--cat", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13",
            "14", "15", "16", "17", "18", "19", "20", "21", "22", "23", "24", "25", "26", "27",
        ]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(
            v,
            vec![
                "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15",
                "16", "17", "18", "19", "20", "21", "22", "23", "24", "25", "26", "27"
            ]
        );
    }

    #[test]
    fn recognizes_clap32_grep_named_capture() {
        let m = parse_opts().get_matches_from(vec!["tp", "-g", "(?P<name>\\w+)"]);
        assert_eq!(
            m.get_one::<String>(GREP).map(|s| s.as_str()),
            Some("(?P<name>\\w+)")
        );
    }

    #[test]
    fn recognizes_clap32_expire_scientific_string() {
        let m = parse_opts().get_matches_from(vec!["tp", "--expire", "1e-6"]);
        assert_eq!(
            m.get_one::<String>(EXPIRE).map(|s| s.as_str()),
            Some("1e-6")
        );
    }

    #[test]
    fn recognizes_clap32_sort_mtime_rev_positional() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort", "mtime", "--rev", "ts.log"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("mtime"));
        assert!(m.get_flag(REVERSE));
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("ts.log")
        );
    }

    #[test]
    fn recognizes_clap32_shift_pop_unshift_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-s", "-p", "-u"]);
        assert!(m.get_flag(SHIFT));
        assert!(m.get_flag(POP));
        assert!(m.get_flag(UNSHIFT));
    }

    #[test]
    fn recognizes_clap32_append_remove_dup_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-A", "1", "-r", "2", "-x", "3"]);
        assert_eq!(m.get_one::<String>(APPEND).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(REMOVE).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("3"));
    }

    #[test]
    fn recognizes_clap32_replace_three_args_vertical_tab() {
        let m = parse_opts().get_matches_from(vec!["tp", "--replace", "0", "\u{000b}", "|"]);
        let v: Vec<String> = m.get_many(REPLACE).unwrap().cloned().collect();
        assert_eq!(v, vec!["0", "\u{000b}", "|"]);
    }

    #[test]
    fn recognizes_clap32_move_diff_swap_long_short_mix() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--mv", "a", "b", "--diff", "c", "d", "-S", "e", "f",
        ]);
        assert_eq!(
            m.get_many(MOVE).unwrap().cloned().collect::<Vec<String>>(),
            vec!["a", "b"]
        );
        assert_eq!(
            m.get_many(DIFF).unwrap().cloned().collect::<Vec<String>>(),
            vec!["c", "d"]
        );
        assert_eq!(
            m.get_many(SWAP).unwrap().cloned().collect::<Vec<String>>(),
            vec!["e", "f"]
        );
    }

    #[test]
    fn recognizes_clap32_verbose_thirty_one_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv"]);
        assert_eq!(m.get_count(VERBOSE), 31);
    }

    #[test]
    fn recognizes_clap32_double_dash_positional_newline_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "--", "\n"]);
        assert_eq!(m.get_one::<String>(ARGFILE).map(|s| s.as_str()), Some("\n"));
    }

    #[test]
    fn recognizes_clap32_grep_wc_quiet_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--grep", "z", "--wc", "0", "--quiet"]);
        assert_eq!(m.get_one::<String>(GREP).map(|s| s.as_str()), Some("z"));
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("0"));
        assert!(m.get_flag(SILENT));
    }

    #[test]
    fn recognizes_clap32_size_path_tail_long() {
        let m = parse_opts()
            .get_matches_from(vec!["tp", "--size", "8", "--path", "8", "--tail", "8", "4"]);
        assert_eq!(m.get_one::<String>(SIZE).map(|s| s.as_str()), Some("8"));
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("8"));
        assert_eq!(
            m.get_many::<String>(TAIL)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["8", "4"]
        );
    }

    #[test]
    fn recognizes_clap32_head_wc_only_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--head", "2", "30", "--wc", "2"]);
        assert_eq!(
            m.get_many::<String>(HEAD)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["2", "30"]
        );
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("2"));
    }

    #[test]
    fn recognizes_clap32_cat_short_sixteen_indices() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "-C", "85", "86", "87", "88", "89", "90", "91", "92", "93", "94", "95", "96",
            "97", "98", "99", "100",
        ]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(
            v,
            vec![
                "85", "86", "87", "88", "89", "90", "91", "92", "93", "94", "95", "96", "97", "98",
                "99", "100"
            ]
        );
    }

    #[test]
    fn recognizes_clap32_sort_size_equals_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort=size"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("size"));
    }

    #[test]
    fn recognizes_clap32_add_remove_long_equals() {
        let m = parse_opts().get_matches_from(vec!["tp", "--add=12", "--remove=13"]);
        assert_eq!(m.get_one::<String>(ADD).map(|s| s.as_str()), Some("12"));
        assert_eq!(m.get_one::<String>(REMOVE).map(|s| s.as_str()), Some("13"));
    }

    #[test]
    fn recognizes_clap32_rename_move_long_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "--rename", "i", "j", "--mv", "6", "7"]);
        assert_eq!(
            m.get_many(RENAME)
                .unwrap()
                .cloned()
                .collect::<Vec<String>>(),
            vec!["i", "j"]
        );
        assert_eq!(
            m.get_many(MOVE).unwrap().cloned().collect::<Vec<String>>(),
            vec!["6", "7"]
        );
    }

    #[test]
    fn recognizes_clap32_list_all_four_modes_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-l", "-n", "-L", "-N"]);
        assert!(m.get_flag(LIST_FILES));
        assert!(m.get_flag(LIST_FILES_NUMBERED));
        assert!(m.get_flag(LIST_CONTENTS));
        assert!(m.get_flag(LIST_CONTENTS_NUMBERED));
    }

    #[test]
    fn recognizes_clap32_rev_expire_path_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--rev", "--expire", "0", "--path", "3"]);
        assert!(m.get_flag(REVERSE));
        assert_eq!(m.get_one::<String>(EXPIRE).map(|s| s.as_str()), Some("0"));
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("3"));
    }

    #[test]
    fn recognizes_clap32_edit_dup_output_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-e", "4", "-x", "5", "-o", "6"]);
        assert_eq!(m.get_one::<String>(EDIT).map(|s| s.as_str()), Some("4"));
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("5"));
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("6"));
    }

    #[test]
    fn recognizes_clap32_positional_dollar_paren() {
        let m = parse_opts().get_matches_from(vec!["tp", "$(cmd).out"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("$(cmd).out")
        );
    }

    #[test]
    fn recognizes_clap32_program_name_temprs_with_shift() {
        let m = parse_opts().get_matches_from(vec!["temprs", "--shift"]);
        assert!(m.get_flag(SHIFT));
    }

    #[test]
    fn recognizes_clap32_tag_long_zero_width_joiner() {
        let m = parse_opts().get_matches_from(vec!["tp", "--name", "x\u{200c}y"]);
        assert_eq!(
            m.get_one::<String>(TAG).map(|s| s.as_str()),
            Some("x\u{200c}y")
        );
    }

    #[test]
    fn recognizes_clap32_pop_clear_quiet_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-p", "-c", "-q"]);
        assert!(m.get_flag(POP));
        assert!(m.get_flag(CLEAR));
        assert!(m.get_flag(SILENT));
    }

    #[test]
    fn recognizes_clap32_info_cat_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-I", "0", "-C", "0", "1"]);
        assert_eq!(m.get_one::<String>(INFO).map(|s| s.as_str()), Some("0"));
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(v, vec!["0", "1"]);
    }

    #[test]
    fn recognizes_clap32_diff_only_short_pair() {
        let m = parse_opts().get_matches_from(vec!["tp", "-D", "one", "two"]);
        assert_eq!(
            m.get_many(DIFF).unwrap().cloned().collect::<Vec<String>>(),
            vec!["one", "two"]
        );
    }

    #[test]
    fn recognizes_clap32_positional_mqtt_uri() {
        let m = parse_opts().get_matches_from(vec!["tp", "mqtt://broker.example:1883/topic"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("mqtt://broker.example:1883/topic")
        );
    }

    #[test]
    fn recognizes_clap32_master_count_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-m", "-k"]);
        assert!(m.get_flag(MASTER));
        assert!(m.get_flag(COUNT));
    }

    // ── clap coverage round 33 ─────────────────────────────

    #[test]
    fn recognizes_clap33_positional_nntp_uri() {
        let m = parse_opts().get_matches_from(vec!["tp", "nntp://news.example/group"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("nntp://news.example/group")
        );
    }

    #[test]
    fn recognizes_clap33_positional_ldaps_uri() {
        let m = parse_opts().get_matches_from(vec!["tp", "ldaps://ldap.example/dc=foo"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("ldaps://ldap.example/dc=foo")
        );
    }

    #[test]
    fn recognizes_clap33_output_long_equals_plus_one() {
        let m = parse_opts().get_matches_from(vec!["tp", "--output=+1"]);
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("+1"));
    }

    #[test]
    fn recognizes_clap33_input_long_equals_double_hyphen() {
        let m = parse_opts().get_matches_from(vec!["tp", "--input=--"]);
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("--"));
    }

    #[test]
    fn recognizes_clap33_cat_twenty_eight_indices_long() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--cat", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13",
            "14", "15", "16", "17", "18", "19", "20", "21", "22", "23", "24", "25", "26", "27",
            "28",
        ]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(
            v,
            vec![
                "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15",
                "16", "17", "18", "19", "20", "21", "22", "23", "24", "25", "26", "27", "28"
            ]
        );
    }

    #[test]
    fn recognizes_clap33_grep_branch_reset() {
        let m = parse_opts().get_matches_from(vec!["tp", "-g", "(?|a|b)"]);
        assert_eq!(
            m.get_one::<String>(GREP).map(|s| s.as_str()),
            Some("(?|a|b)")
        );
    }

    #[test]
    fn recognizes_clap33_expire_comma_decimal() {
        let m = parse_opts().get_matches_from(vec!["tp", "--expire", "3,14"]);
        assert_eq!(
            m.get_one::<String>(EXPIRE).map(|s| s.as_str()),
            Some("3,14")
        );
    }

    #[test]
    fn recognizes_clap33_sort_size_positional_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort", "size", "big.bin"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("size"));
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("big.bin")
        );
    }

    #[test]
    fn recognizes_clap33_pop_unshift_clear_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-p", "-u", "-c"]);
        assert!(m.get_flag(POP));
        assert!(m.get_flag(UNSHIFT));
        assert!(m.get_flag(CLEAR));
    }

    #[test]
    fn recognizes_clap33_add_output_remove_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-a", "1", "-o", "2", "-r", "3"]);
        assert_eq!(m.get_one::<String>(ADD).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(REMOVE).map(|s| s.as_str()), Some("3"));
    }

    #[test]
    fn recognizes_clap33_replace_three_args_form_feed() {
        let m = parse_opts().get_matches_from(vec!["tp", "--replace", "0", "\u{000c}", ""]);
        let v: Vec<String> = m.get_many(REPLACE).unwrap().cloned().collect();
        assert_eq!(v, vec!["0", "\u{000c}", ""]);
    }

    #[test]
    fn recognizes_clap33_diff_swap_move_long_chain() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--diff", "p", "q", "--swap", "r", "s", "--mv", "t", "u",
        ]);
        assert_eq!(
            m.get_many(DIFF).unwrap().cloned().collect::<Vec<String>>(),
            vec!["p", "q"]
        );
        assert_eq!(
            m.get_many(SWAP).unwrap().cloned().collect::<Vec<String>>(),
            vec!["r", "s"]
        );
        assert_eq!(
            m.get_many(MOVE).unwrap().cloned().collect::<Vec<String>>(),
            vec!["t", "u"]
        );
    }

    #[test]
    fn recognizes_clap33_verbose_thirty_two_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv"]);
        assert_eq!(m.get_count(VERBOSE), 32);
    }

    #[test]
    fn recognizes_clap33_double_dash_positional_tab_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "--", "\t"]);
        assert_eq!(m.get_one::<String>(ARGFILE).map(|s| s.as_str()), Some("\t"));
    }

    #[test]
    fn recognizes_clap33_grep_dup_append_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-g", "rx", "-x", "1", "-A", "2"]);
        assert_eq!(m.get_one::<String>(GREP).map(|s| s.as_str()), Some("rx"));
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(APPEND).map(|s| s.as_str()), Some("2"));
    }

    #[test]
    fn recognizes_clap33_path_head_size_long() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--path", "9", "--head", "9", "25", "--size", "9",
        ]);
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("9"));
        assert_eq!(
            m.get_many::<String>(HEAD)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["9", "25"]
        );
        assert_eq!(m.get_one::<String>(SIZE).map(|s| s.as_str()), Some("9"));
    }

    #[test]
    fn recognizes_clap33_tail_wc_only_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--tail", "3", "40", "--wc", "3"]);
        assert_eq!(
            m.get_many::<String>(TAIL)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["3", "40"]
        );
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("3"));
    }

    #[test]
    fn recognizes_clap33_cat_short_seventeen_indices() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "-C", "101", "102", "103", "104", "105", "106", "107", "108", "109", "110",
            "111", "112", "113", "114", "115", "116", "117",
        ]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(
            v,
            vec![
                "101", "102", "103", "104", "105", "106", "107", "108", "109", "110", "111", "112",
                "113", "114", "115", "116", "117"
            ]
        );
    }

    #[test]
    fn recognizes_clap33_sort_bare_long_then_positional() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort", "file.dat"]);
        assert_eq!(
            m.get_one::<String>(SORT).map(|s| s.as_str()),
            Some("file.dat")
        );
        assert_eq!(m.get_one::<String>(ARGFILE).map(|s| s.as_str()), None);
    }

    #[test]
    fn recognizes_clap33_dup_append_long_equals() {
        let m = parse_opts().get_matches_from(vec!["tp", "--dup=0", "--append=1"]);
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("0"));
        assert_eq!(m.get_one::<String>(APPEND).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_clap33_rename_diff_long_short_mix() {
        let m = parse_opts().get_matches_from(vec!["tp", "--rename", "a", "b", "-D", "0", "1"]);
        assert_eq!(
            m.get_many(RENAME)
                .unwrap()
                .cloned()
                .collect::<Vec<String>>(),
            vec!["a", "b"]
        );
        assert_eq!(
            m.get_many(DIFF).unwrap().cloned().collect::<Vec<String>>(),
            vec!["0", "1"]
        );
    }

    #[test]
    fn recognizes_clap33_master_list_contents_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--master", "--list-contents"]);
        assert!(m.get_flag(MASTER));
        assert!(m.get_flag(LIST_CONTENTS));
    }

    #[test]
    fn recognizes_clap33_rev_sort_expire_long() {
        let m =
            parse_opts().get_matches_from(vec!["tp", "--rev", "--sort", "mtime", "--expire", "48"]);
        assert!(m.get_flag(REVERSE));
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("mtime"));
        assert_eq!(m.get_one::<String>(EXPIRE).map(|s| s.as_str()), Some("48"));
    }

    #[test]
    fn recognizes_clap33_head_tail_negative_line_counts() {
        let m = parse_opts().get_matches_from(vec!["tp", "--head", "4", "-3", "--tail", "5", "-4"]);
        assert_eq!(
            m.get_many::<String>(HEAD)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["4", "-3"]
        );
        assert_eq!(
            m.get_many::<String>(TAIL)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["5", "-4"]
        );
    }

    #[test]
    fn recognizes_clap33_edit_append_info_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-e", "7", "-A", "8", "-I", "9"]);
        assert_eq!(m.get_one::<String>(EDIT).map(|s| s.as_str()), Some("7"));
        assert_eq!(m.get_one::<String>(APPEND).map(|s| s.as_str()), Some("8"));
        assert_eq!(m.get_one::<String>(INFO).map(|s| s.as_str()), Some("9"));
    }

    #[test]
    fn recognizes_clap33_positional_escaped_space() {
        let m = parse_opts().get_matches_from(vec!["tp", "my\\ file.txt"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("my\\ file.txt")
        );
    }

    #[test]
    fn recognizes_clap33_program_name_temprs_with_unshift() {
        let m = parse_opts().get_matches_from(vec!["temprs", "--unshift"]);
        assert!(m.get_flag(UNSHIFT));
    }

    #[test]
    fn recognizes_clap33_tag_short_equals_hash() {
        let m = parse_opts().get_matches_from(vec!["tp", "-w=#tag"]);
        assert_eq!(m.get_one::<String>(TAG).map(|s| s.as_str()), Some("#tag"));
    }

    #[test]
    fn recognizes_clap33_shift_quiet_pop_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-s", "-q", "-p"]);
        assert!(m.get_flag(SHIFT));
        assert!(m.get_flag(SILENT));
        assert!(m.get_flag(POP));
    }

    #[test]
    fn recognizes_clap33_count_verbose_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-k", "-vv"]);
        assert!(m.get_flag(COUNT));
        assert_eq!(m.get_count(VERBOSE), 2);
    }

    #[test]
    fn recognizes_clap33_swap_only_short_pair() {
        let m = parse_opts().get_matches_from(vec!["tp", "-S", "x", "y"]);
        assert_eq!(
            m.get_many(SWAP).unwrap().cloned().collect::<Vec<String>>(),
            vec!["x", "y"]
        );
    }

    #[test]
    fn recognizes_clap33_positional_coap_uri() {
        let m = parse_opts().get_matches_from(vec!["tp", "coap://sensor.local/status"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("coap://sensor.local/status")
        );
    }

    // ── clap coverage round 34 ─────────────────────────────

    #[test]
    fn recognizes_clap34_positional_rtsp_uri() {
        let m = parse_opts().get_matches_from(vec!["tp", "rtsp://camera.local/stream"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("rtsp://camera.local/stream")
        );
    }

    #[test]
    fn recognizes_clap34_positional_rtmp_uri() {
        let m = parse_opts().get_matches_from(vec!["tp", "rtmp://live.example/app/stream"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("rtmp://live.example/app/stream")
        );
    }

    #[test]
    fn recognizes_clap34_output_short_negative() {
        let m = parse_opts().get_matches_from(vec!["tp", "-o", "-3"]);
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("-3"));
    }

    #[test]
    fn recognizes_clap34_input_long_equals_zero() {
        let m = parse_opts().get_matches_from(vec!["tp", "--input=0"]);
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("0"));
    }

    #[test]
    fn recognizes_clap34_cat_twenty_nine_indices_long() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--cat", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13",
            "14", "15", "16", "17", "18", "19", "20", "21", "22", "23", "24", "25", "26", "27",
            "28", "29",
        ]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(
            v,
            vec![
                "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15",
                "16", "17", "18", "19", "20", "21", "22", "23", "24", "25", "26", "27", "28", "29"
            ]
        );
    }

    #[test]
    fn recognizes_clap34_grep_conditional_pattern() {
        let m = parse_opts().get_matches_from(vec!["tp", "-g", "(?(1)a|b)"]);
        assert_eq!(
            m.get_one::<String>(GREP).map(|s| s.as_str()),
            Some("(?(1)a|b)")
        );
    }

    #[test]
    fn recognizes_clap34_expire_slash_range() {
        let m = parse_opts().get_matches_from(vec!["tp", "--expire", "1/24"]);
        assert_eq!(
            m.get_one::<String>(EXPIRE).map(|s| s.as_str()),
            Some("1/24")
        );
    }

    #[test]
    fn recognizes_clap34_sort_name_rev_positional() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort", "name", "--rev", "z.txt"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("name"));
        assert!(m.get_flag(REVERSE));
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("z.txt")
        );
    }

    #[test]
    fn recognizes_clap34_unshift_shift_pop_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-u", "-s", "-p"]);
        assert!(m.get_flag(UNSHIFT));
        assert!(m.get_flag(SHIFT));
        assert!(m.get_flag(POP));
    }

    #[test]
    fn recognizes_clap34_remove_append_dup_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-r", "1", "-A", "2", "-x", "3"]);
        assert_eq!(m.get_one::<String>(REMOVE).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(APPEND).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("3"));
    }

    #[test]
    fn recognizes_clap34_replace_three_args_line_separator() {
        let m = parse_opts().get_matches_from(vec!["tp", "--replace", "0", "\u{2028}", "\u{2029}"]);
        let v: Vec<String> = m.get_many(REPLACE).unwrap().cloned().collect();
        assert_eq!(v, vec!["0", "\u{2028}", "\u{2029}"]);
    }

    #[test]
    fn recognizes_clap34_swap_diff_move_short_chain() {
        let m = parse_opts()
            .get_matches_from(vec!["tp", "-S", "a", "b", "-D", "c", "d", "-M", "e", "f"]);
        assert_eq!(
            m.get_many(SWAP).unwrap().cloned().collect::<Vec<String>>(),
            vec!["a", "b"]
        );
        assert_eq!(
            m.get_many(DIFF).unwrap().cloned().collect::<Vec<String>>(),
            vec!["c", "d"]
        );
        assert_eq!(
            m.get_many(MOVE).unwrap().cloned().collect::<Vec<String>>(),
            vec!["e", "f"]
        );
    }

    #[test]
    fn recognizes_clap34_verbose_thirty_three_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv"]);
        assert_eq!(m.get_count(VERBOSE), 33);
    }

    #[test]
    fn recognizes_clap34_double_dash_positional_semicolon() {
        let m = parse_opts().get_matches_from(vec!["tp", "--", ";"]);
        assert_eq!(m.get_one::<String>(ARGFILE).map(|s| s.as_str()), Some(";"));
    }

    #[test]
    fn recognizes_clap34_grep_list_files_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--grep", "pat", "--list-files"]);
        assert_eq!(m.get_one::<String>(GREP).map(|s| s.as_str()), Some("pat"));
        assert!(m.get_flag(LIST_FILES));
    }

    #[test]
    fn recognizes_clap34_size_wc_path_long() {
        let m =
            parse_opts().get_matches_from(vec!["tp", "--size", "2", "--wc", "2", "--path", "2"]);
        assert_eq!(m.get_one::<String>(SIZE).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("2"));
    }

    #[test]
    fn recognizes_clap34_head_tail_path_long() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--head", "1", "5", "--tail", "1", "6", "--path", "1",
        ]);
        assert_eq!(
            m.get_many::<String>(HEAD)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["1", "5"]
        );
        assert_eq!(
            m.get_many::<String>(TAIL)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["1", "6"]
        );
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_clap34_cat_short_eighteen_indices() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "-C", "118", "119", "120", "121", "122", "123", "124", "125", "126", "127",
            "128", "129", "130", "131", "132", "133", "134", "135",
        ]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(
            v,
            vec![
                "118", "119", "120", "121", "122", "123", "124", "125", "126", "127", "128", "129",
                "130", "131", "132", "133", "134", "135"
            ]
        );
    }

    #[test]
    fn recognizes_clap34_sort_equals_size_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort=size"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("size"));
    }

    #[test]
    fn recognizes_clap34_add_remove_long_chain() {
        let m = parse_opts().get_matches_from(vec!["tp", "--add", "20", "--remove", "21"]);
        assert_eq!(m.get_one::<String>(ADD).map(|s| s.as_str()), Some("20"));
        assert_eq!(m.get_one::<String>(REMOVE).map(|s| s.as_str()), Some("21"));
    }

    #[test]
    fn recognizes_clap34_rename_swap_long_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "--rename", "u", "v", "--swap", "8", "9"]);
        assert_eq!(
            m.get_many(RENAME)
                .unwrap()
                .cloned()
                .collect::<Vec<String>>(),
            vec!["u", "v"]
        );
        assert_eq!(
            m.get_many(SWAP).unwrap().cloned().collect::<Vec<String>>(),
            vec!["8", "9"]
        );
    }

    #[test]
    fn recognizes_clap34_directory_master_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-d", "-m"]);
        assert!(m.get_flag(DIRECTORY));
        assert!(m.get_flag(MASTER));
    }

    #[test]
    fn recognizes_clap34_expire_rev_path_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--expire", "12", "--rev", "--path", "4"]);
        assert_eq!(m.get_one::<String>(EXPIRE).map(|s| s.as_str()), Some("12"));
        assert!(m.get_flag(REVERSE));
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("4"));
    }

    #[test]
    fn recognizes_clap34_head_tail_negative_line_counts() {
        let m = parse_opts().get_matches_from(vec!["tp", "--head", "6", "-2", "--tail", "7", "-3"]);
        assert_eq!(
            m.get_many::<String>(HEAD)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["6", "-2"]
        );
        assert_eq!(
            m.get_many::<String>(TAIL)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["7", "-3"]
        );
    }

    #[test]
    fn recognizes_clap34_info_output_edit_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-I", "1", "-o", "2", "-e", "3"]);
        assert_eq!(m.get_one::<String>(INFO).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(EDIT).map(|s| s.as_str()), Some("3"));
    }

    #[test]
    fn recognizes_clap34_positional_ellipsis() {
        let m = parse_opts().get_matches_from(vec!["tp", "file..."]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("file...")
        );
    }

    #[test]
    fn recognizes_clap34_program_name_temprs_with_clear() {
        let m = parse_opts().get_matches_from(vec!["temprs", "--clear"]);
        assert!(m.get_flag(CLEAR));
    }

    #[test]
    fn recognizes_clap34_tag_long_variation_selector() {
        let m = parse_opts().get_matches_from(vec!["tp", "--name", "x\u{fe0f}"]);
        assert_eq!(
            m.get_one::<String>(TAG).map(|s| s.as_str()),
            Some("x\u{fe0f}")
        );
    }

    #[test]
    fn recognizes_clap34_count_quiet_rev_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-k", "-q", "--rev"]);
        assert!(m.get_flag(COUNT));
        assert!(m.get_flag(SILENT));
        assert!(m.get_flag(REVERSE));
    }

    #[test]
    fn recognizes_clap34_move_only_short_pair() {
        let m = parse_opts().get_matches_from(vec!["tp", "-M", "i", "j"]);
        assert_eq!(
            m.get_many(MOVE).unwrap().cloned().collect::<Vec<String>>(),
            vec!["i", "j"]
        );
    }

    #[test]
    fn recognizes_clap34_positional_stun_uri() {
        let m = parse_opts().get_matches_from(vec!["tp", "stun:stun.l.google.com:19302"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("stun:stun.l.google.com:19302")
        );
    }

    #[test]
    fn recognizes_clap34_positional_turn_uri() {
        let m = parse_opts().get_matches_from(vec!["tp", "turn:turn.example:3478?transport=udp"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("turn:turn.example:3478?transport=udp")
        );
    }

    // ── clap coverage round 35 ─────────────────────────────

    #[test]
    fn recognizes_clap35_positional_sip_uri() {
        let m = parse_opts().get_matches_from(vec!["tp", "sip:user@voip.example"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("sip:user@voip.example")
        );
    }

    #[test]
    fn recognizes_clap35_positional_tel_uri() {
        let m = parse_opts().get_matches_from(vec!["tp", "tel:+1-555-0100"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("tel:+1-555-0100")
        );
    }

    #[test]
    fn recognizes_clap35_output_long_equals_positive() {
        let m = parse_opts().get_matches_from(vec!["tp", "--output=100"]);
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("100"));
    }

    #[test]
    fn recognizes_clap35_input_short_positive() {
        let m = parse_opts().get_matches_from(vec!["tp", "-i", "7"]);
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("7"));
    }

    #[test]
    fn recognizes_clap35_cat_thirty_indices_long() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--cat", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13",
            "14", "15", "16", "17", "18", "19", "20", "21", "22", "23", "24", "25", "26", "27",
            "28", "29", "30",
        ]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(
            v,
            vec![
                "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15",
                "16", "17", "18", "19", "20", "21", "22", "23", "24", "25", "26", "27", "28", "29",
                "30"
            ]
        );
    }

    #[test]
    fn recognizes_clap35_grep_backreference() {
        let m = parse_opts().get_matches_from(vec!["tp", "-g", r"(\w)\1"]);
        assert_eq!(
            m.get_one::<String>(GREP).map(|s| s.as_str()),
            Some(r"(\w)\1")
        );
    }

    #[test]
    fn recognizes_clap35_expire_pipe_list() {
        let m = parse_opts().get_matches_from(vec!["tp", "--expire", "a|b|c"]);
        assert_eq!(
            m.get_one::<String>(EXPIRE).map(|s| s.as_str()),
            Some("a|b|c")
        );
    }

    #[test]
    fn recognizes_clap35_sort_mtime_positional_rev() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort", "mtime", "--rev", "old.log"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("mtime"));
        assert!(m.get_flag(REVERSE));
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("old.log")
        );
    }

    #[test]
    fn recognizes_clap35_pop_shift_unshift_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-p", "-s", "-u"]);
        assert!(m.get_flag(POP));
        assert!(m.get_flag(SHIFT));
        assert!(m.get_flag(UNSHIFT));
    }

    #[test]
    fn recognizes_clap35_add_dup_append_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-a", "1", "-x", "2", "-A", "3"]);
        assert_eq!(m.get_one::<String>(ADD).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(APPEND).map(|s| s.as_str()), Some("3"));
    }

    #[test]
    fn recognizes_clap35_replace_three_args_ellipsis() {
        let m = parse_opts().get_matches_from(vec!["tp", "--replace", "0", "…", "..."]);
        let v: Vec<String> = m.get_many(REPLACE).unwrap().cloned().collect();
        assert_eq!(v, vec!["0", "…", "..."]);
    }

    #[test]
    fn recognizes_clap35_diff_move_swap_long_chain() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--diff", "1", "2", "--mv", "3", "4", "--swap", "5", "6",
        ]);
        assert_eq!(
            m.get_many(DIFF).unwrap().cloned().collect::<Vec<String>>(),
            vec!["1", "2"]
        );
        assert_eq!(
            m.get_many(MOVE).unwrap().cloned().collect::<Vec<String>>(),
            vec!["3", "4"]
        );
        assert_eq!(
            m.get_many(SWAP).unwrap().cloned().collect::<Vec<String>>(),
            vec!["5", "6"]
        );
    }

    #[test]
    fn recognizes_clap35_verbose_thirty_four_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv"]);
        assert_eq!(m.get_count(VERBOSE), 34);
    }

    #[test]
    fn recognizes_clap35_double_dash_positional_ampersand() {
        let m = parse_opts().get_matches_from(vec!["tp", "--", "&"]);
        assert_eq!(m.get_one::<String>(ARGFILE).map(|s| s.as_str()), Some("&"));
    }

    #[test]
    fn recognizes_clap35_grep_path_wc_short_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "-g", "re", "--path", "0", "--wc", "0"]);
        assert_eq!(m.get_one::<String>(GREP).map(|s| s.as_str()), Some("re"));
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("0"));
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("0"));
    }

    #[test]
    fn recognizes_clap35_tail_head_size_long() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--tail", "3", "50", "--head", "3", "51", "--size", "3",
        ]);
        assert_eq!(
            m.get_many::<String>(TAIL)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["3", "50"]
        );
        assert_eq!(
            m.get_many::<String>(HEAD)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["3", "51"]
        );
        assert_eq!(m.get_one::<String>(SIZE).map(|s| s.as_str()), Some("3"));
    }

    #[test]
    fn recognizes_clap35_cat_short_nineteen_indices() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "-C", "136", "137", "138", "139", "140", "141", "142", "143", "144", "145",
            "146", "147", "148", "149", "150", "151", "152", "153", "154",
        ]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(
            v,
            vec![
                "136", "137", "138", "139", "140", "141", "142", "143", "144", "145", "146", "147",
                "148", "149", "150", "151", "152", "153", "154"
            ]
        );
    }

    #[test]
    fn recognizes_clap35_sort_bare_long_default_name() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("name"));
    }

    #[test]
    fn recognizes_clap35_remove_output_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-r", "5", "-o", "6"]);
        assert_eq!(m.get_one::<String>(REMOVE).map(|s| s.as_str()), Some("5"));
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("6"));
    }

    #[test]
    fn recognizes_clap35_rename_diff_long_only() {
        let m =
            parse_opts().get_matches_from(vec!["tp", "--rename", "src", "dst", "--diff", "a", "b"]);
        assert_eq!(
            m.get_many(RENAME)
                .unwrap()
                .cloned()
                .collect::<Vec<String>>(),
            vec!["src", "dst"]
        );
        assert_eq!(
            m.get_many(DIFF).unwrap().cloned().collect::<Vec<String>>(),
            vec!["a", "b"]
        );
    }

    #[test]
    fn recognizes_clap35_master_list_files_numbered_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--master", "--list-files-numbered"]);
        assert!(m.get_flag(MASTER));
        assert!(m.get_flag(LIST_FILES_NUMBERED));
    }

    #[test]
    fn recognizes_clap35_rev_sort_path_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--rev", "--sort", "size", "--path", "0"]);
        assert!(m.get_flag(REVERSE));
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("size"));
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("0"));
    }

    #[test]
    fn recognizes_clap35_head_tail_negative_line_counts() {
        let m = parse_opts().get_matches_from(vec!["tp", "--head", "8", "-4", "--tail", "9", "-5"]);
        assert_eq!(
            m.get_many::<String>(HEAD)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["8", "-4"]
        );
        assert_eq!(
            m.get_many::<String>(TAIL)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["9", "-5"]
        );
    }

    #[test]
    fn recognizes_clap35_cat_info_edit_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-I", "0", "-e", "1", "-C", "1", "2"]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(v, vec!["1", "2"]);
        assert_eq!(m.get_one::<String>(INFO).map(|s| s.as_str()), Some("0"));
        assert_eq!(m.get_one::<String>(EDIT).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_clap35_positional_bang_path() {
        let m = parse_opts().get_matches_from(vec!["tp", "!event.log"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("!event.log")
        );
    }

    #[test]
    fn recognizes_clap35_program_name_temprs_with_shift() {
        let m = parse_opts().get_matches_from(vec!["temprs", "--shift"]);
        assert!(m.get_flag(SHIFT));
    }

    #[test]
    fn recognizes_clap35_tag_short_equals_ampersand() {
        let m = parse_opts().get_matches_from(vec!["tp", "-w=a&b"]);
        assert_eq!(m.get_one::<String>(TAG).map(|s| s.as_str()), Some("a&b"));
    }

    #[test]
    fn recognizes_clap35_clear_count_quiet_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--clear", "--count", "--quiet"]);
        assert!(m.get_flag(CLEAR));
        assert!(m.get_flag(COUNT));
        assert!(m.get_flag(SILENT));
    }

    #[test]
    fn recognizes_clap35_swap_only_long_pair() {
        let m = parse_opts().get_matches_from(vec!["tp", "--swap", "left", "right"]);
        assert_eq!(
            m.get_many(SWAP).unwrap().cloned().collect::<Vec<String>>(),
            vec!["left", "right"]
        );
    }

    #[test]
    fn recognizes_clap35_positional_irc_uri() {
        let m = parse_opts().get_matches_from(vec!["tp", "irc://irc.libera.chat/#rust"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("irc://irc.libera.chat/#rust")
        );
    }

    #[test]
    fn recognizes_clap35_positional_xmpp_uri() {
        let m = parse_opts().get_matches_from(vec!["tp", "xmpp:user@jabber.example"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("xmpp:user@jabber.example")
        );
    }

    #[test]
    fn recognizes_clap35_directory_list_numbered_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-d", "-n"]);
        assert!(m.get_flag(DIRECTORY));
        assert!(m.get_flag(LIST_FILES_NUMBERED));
    }

    // ── clap coverage round 36 ─────────────────────────────

    #[test]
    fn recognizes_clap36_positional_mailto_uri() {
        let m = parse_opts().get_matches_from(vec!["tp", "mailto:user@example.org"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("mailto:user@example.org")
        );
    }

    #[test]
    fn recognizes_clap36_positional_magnet_uri() {
        let m = parse_opts().get_matches_from(vec![
            "tp",
            "magnet:?xt=urn:btih:0123456789abcdef0123456789abcdef01234567",
        ]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("magnet:?xt=urn:btih:0123456789abcdef0123456789abcdef01234567")
        );
    }

    #[test]
    fn recognizes_clap36_positional_sftp_uri() {
        let m = parse_opts().get_matches_from(vec!["tp", "sftp://backup.example/var/log"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("sftp://backup.example/var/log")
        );
    }

    #[test]
    fn recognizes_clap36_positional_file_uri() {
        let m = parse_opts().get_matches_from(vec!["tp", "file:///tmp/session.log"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("file:///tmp/session.log")
        );
    }

    #[test]
    fn recognizes_clap36_output_equals_negative() {
        let m = parse_opts().get_matches_from(vec!["tp", "--output=-9"]);
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("-9"));
    }

    #[test]
    fn recognizes_clap36_input_short_zero() {
        let m = parse_opts().get_matches_from(vec!["tp", "-i", "0"]);
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("0"));
    }

    #[test]
    fn recognizes_clap36_cat_thirty_one_indices_long() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--cat", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13",
            "14", "15", "16", "17", "18", "19", "20", "21", "22", "23", "24", "25", "26", "27",
            "28", "29", "30", "31",
        ]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(
            v,
            vec![
                "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15",
                "16", "17", "18", "19", "20", "21", "22", "23", "24", "25", "26", "27", "28", "29",
                "30", "31"
            ]
        );
    }

    #[test]
    fn recognizes_clap36_grep_positive_lookahead() {
        let m = parse_opts().get_matches_from(vec!["tp", "-g", r"(?=foo)bar"]);
        assert_eq!(
            m.get_one::<String>(GREP).map(|s| s.as_str()),
            Some(r"(?=foo)bar")
        );
    }

    #[test]
    fn recognizes_clap36_expire_decimal_hours() {
        let m = parse_opts().get_matches_from(vec!["tp", "--expire", "12.5"]);
        assert_eq!(
            m.get_one::<String>(EXPIRE).map(|s| s.as_str()),
            Some("12.5")
        );
    }

    #[test]
    fn recognizes_clap36_sort_name_positional_rev() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort", "name", "--rev", "notes.md"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("name"));
        assert!(m.get_flag(REVERSE));
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("notes.md")
        );
    }

    #[test]
    fn recognizes_clap36_pop_unshift_only_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-p", "-u"]);
        assert!(m.get_flag(POP));
        assert!(m.get_flag(UNSHIFT));
    }

    #[test]
    fn recognizes_clap36_replace_three_args_unicode_replacement() {
        let m = parse_opts().get_matches_from(vec!["tp", "--replace", "3", "x", "\u{2030}"]);
        let v: Vec<String> = m.get_many(REPLACE).unwrap().cloned().collect();
        assert_eq!(v, vec!["3", "x", "\u{2030}"]);
    }

    #[test]
    fn recognizes_clap36_diff_dup_long_chain() {
        let m = parse_opts().get_matches_from(vec!["tp", "--diff", "0", "1", "--dup", "-1"]);
        assert_eq!(
            m.get_many(DIFF).unwrap().cloned().collect::<Vec<String>>(),
            vec!["0", "1"]
        );
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("-1"));
    }

    #[test]
    fn recognizes_clap36_verbose_thirty_five_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv"]);
        assert_eq!(m.get_count(VERBOSE), 35);
    }

    #[test]
    fn recognizes_clap36_double_dash_positional_percent() {
        let m = parse_opts().get_matches_from(vec!["tp", "--", "%25encoded"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("%25encoded")
        );
    }

    #[test]
    fn recognizes_clap36_grep_size_path_long() {
        let m = parse_opts()
            .get_matches_from(vec!["tp", "--grep", r"\d+", "--size", "-2", "--path", "9"]);
        assert_eq!(m.get_one::<String>(GREP).map(|s| s.as_str()), Some(r"\d+"));
        assert_eq!(m.get_one::<String>(SIZE).map(|s| s.as_str()), Some("-2"));
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("9"));
    }

    #[test]
    fn recognizes_clap36_head_tail_single_line() {
        let m = parse_opts().get_matches_from(vec!["tp", "--head", "0", "1", "--tail", "0", "1"]);
        assert_eq!(
            m.get_many::<String>(HEAD)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["0", "1"]
        );
        assert_eq!(
            m.get_many::<String>(TAIL)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["0", "1"]
        );
    }

    #[test]
    fn recognizes_clap36_cat_short_twenty_indices() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "-C", "201", "202", "203", "204", "205", "206", "207", "208", "209", "210",
            "211", "212", "213", "214", "215", "216", "217", "218", "219", "220",
        ]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(
            v,
            vec![
                "201", "202", "203", "204", "205", "206", "207", "208", "209", "210", "211", "212",
                "213", "214", "215", "216", "217", "218", "219", "220"
            ]
        );
    }

    #[test]
    fn recognizes_clap36_sort_explicit_name_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort=name"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("name"));
    }

    #[test]
    fn recognizes_clap36_add_remove_output_short_chain() {
        let m = parse_opts().get_matches_from(vec!["tp", "-a", "8", "-r", "9", "-o", "10"]);
        assert_eq!(m.get_one::<String>(ADD).map(|s| s.as_str()), Some("8"));
        assert_eq!(m.get_one::<String>(REMOVE).map(|s| s.as_str()), Some("9"));
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("10"));
    }

    #[test]
    fn recognizes_clap36_rename_swap_long_chain() {
        let m =
            parse_opts().get_matches_from(vec!["tp", "--rename", "old", "new", "--swap", "4", "5"]);
        assert_eq!(
            m.get_many(RENAME)
                .unwrap()
                .cloned()
                .collect::<Vec<String>>(),
            vec!["old", "new"]
        );
        assert_eq!(
            m.get_many(SWAP).unwrap().cloned().collect::<Vec<String>>(),
            vec!["4", "5"]
        );
    }

    #[test]
    fn recognizes_clap36_master_list_contents_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--master", "--list-contents"]);
        assert!(m.get_flag(MASTER));
        assert!(m.get_flag(LIST_CONTENTS));
    }

    #[test]
    fn recognizes_clap36_rev_sort_mtime_path_long() {
        let m =
            parse_opts().get_matches_from(vec!["tp", "--rev", "--sort", "mtime", "--path", "-0"]);
        assert!(m.get_flag(REVERSE));
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("mtime"));
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("-0"));
    }

    #[test]
    fn recognizes_clap36_head_tail_zero_lines() {
        let m = parse_opts().get_matches_from(vec!["tp", "--head", "2", "0", "--tail", "3", "0"]);
        assert_eq!(
            m.get_many::<String>(HEAD)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["2", "0"]
        );
        assert_eq!(
            m.get_many::<String>(TAIL)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["3", "0"]
        );
    }

    #[test]
    fn recognizes_clap36_info_edit_grep_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-I", "2", "-e", "3", "-g", "^BEGIN"]);
        assert_eq!(m.get_one::<String>(INFO).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(EDIT).map(|s| s.as_str()), Some("3"));
        assert_eq!(
            m.get_one::<String>(GREP).map(|s| s.as_str()),
            Some("^BEGIN")
        );
    }

    #[test]
    fn recognizes_clap36_positional_ws_uri() {
        let m = parse_opts().get_matches_from(vec!["tp", "wss://echo.example/socket"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("wss://echo.example/socket")
        );
    }

    #[test]
    fn recognizes_clap36_program_name_temprs_with_unshift() {
        let m = parse_opts().get_matches_from(vec!["temprs", "--unshift"]);
        assert!(m.get_flag(UNSHIFT));
    }

    #[test]
    fn recognizes_clap36_tag_zwj_emoji_sequence() {
        let m = parse_opts().get_matches_from(vec!["tp", "--name", "a\u{200d}b"]);
        assert_eq!(
            m.get_one::<String>(TAG).map(|s| s.as_str()),
            Some("a\u{200d}b")
        );
    }

    #[test]
    fn recognizes_clap36_clear_list_files_quiet_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--clear", "--list-files", "--quiet"]);
        assert!(m.get_flag(CLEAR));
        assert!(m.get_flag(LIST_FILES));
        assert!(m.get_flag(SILENT));
    }

    #[test]
    fn recognizes_clap36_move_only_long_pair() {
        let m = parse_opts().get_matches_from(vec!["tp", "--mv", "first", "last"]);
        assert_eq!(
            m.get_many(MOVE).unwrap().cloned().collect::<Vec<String>>(),
            vec!["first", "last"]
        );
    }

    #[test]
    fn recognizes_clap36_positional_ftp_uri() {
        let m = parse_opts().get_matches_from(vec!["tp", "ftp://anonymous@ftp.example/pub/README"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("ftp://anonymous@ftp.example/pub/README")
        );
    }

    #[test]
    fn recognizes_clap36_directory_list_contents_numbered_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-d", "-N"]);
        assert!(m.get_flag(DIRECTORY));
        assert!(m.get_flag(LIST_CONTENTS_NUMBERED));
    }

    // ── clap coverage round 37 ─────────────────────────────

    #[test]
    fn recognizes_clap37_positional_ldap_uri() {
        let m =
            parse_opts().get_matches_from(vec!["tp", "ldap://directory.example:389/dc=example"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("ldap://directory.example:389/dc=example")
        );
    }

    #[test]
    fn recognizes_clap37_positional_nfs_uri() {
        let m = parse_opts().get_matches_from(vec!["tp", "nfs://fileserver/export/home"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("nfs://fileserver/export/home")
        );
    }

    #[test]
    fn recognizes_clap37_positional_git_uri() {
        let m = parse_opts().get_matches_from(vec!["tp", "git://github.com/rust-lang/rust.git"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("git://github.com/rust-lang/rust.git")
        );
    }

    #[test]
    fn recognizes_clap37_positional_ssh_uri() {
        let m = parse_opts().get_matches_from(vec!["tp", "ssh://deploy@build.example/runner"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("ssh://deploy@build.example/runner")
        );
    }

    #[test]
    fn recognizes_clap37_positional_data_uri() {
        let m = parse_opts().get_matches_from(vec!["tp", "data:text/plain;base64,Zm9v"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("data:text/plain;base64,Zm9v")
        );
    }

    #[test]
    fn recognizes_clap37_output_short_large_index() {
        let m = parse_opts().get_matches_from(vec!["tp", "-o", "4096"]);
        assert_eq!(
            m.get_one::<String>(OUTPUT).map(|s| s.as_str()),
            Some("4096")
        );
    }

    #[test]
    fn recognizes_clap37_input_long_equals() {
        let m = parse_opts().get_matches_from(vec!["tp", "--input=11"]);
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("11"));
    }

    #[test]
    fn recognizes_clap37_cat_thirty_two_indices_long() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--cat", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13",
            "14", "15", "16", "17", "18", "19", "20", "21", "22", "23", "24", "25", "26", "27",
            "28", "29", "30", "31", "32",
        ]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(
            v,
            vec![
                "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15",
                "16", "17", "18", "19", "20", "21", "22", "23", "24", "25", "26", "27", "28", "29",
                "30", "31", "32"
            ]
        );
    }

    #[test]
    fn recognizes_clap37_grep_named_capture() {
        let m = parse_opts().get_matches_from(vec!["tp", "-g", r"(?P<word>\w+)"]);
        assert_eq!(
            m.get_one::<String>(GREP).map(|s| s.as_str()),
            Some(r"(?P<word>\w+)")
        );
    }

    #[test]
    fn recognizes_clap37_expire_dash_range_hours() {
        let m = parse_opts().get_matches_from(vec!["tp", "--expire", "1-168"]);
        assert_eq!(
            m.get_one::<String>(EXPIRE).map(|s| s.as_str()),
            Some("1-168")
        );
    }

    #[test]
    fn recognizes_clap37_sort_size_positional_plain() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort", "size", "heap.bin"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("size"));
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("heap.bin")
        );
    }

    #[test]
    fn recognizes_clap37_shift_pop_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-s", "-p"]);
        assert!(m.get_flag(SHIFT));
        assert!(m.get_flag(POP));
    }

    #[test]
    fn recognizes_clap37_append_dup_output_short_chain() {
        let m = parse_opts().get_matches_from(vec!["tp", "-A", "0", "-x", "1", "-o", "2"]);
        assert_eq!(m.get_one::<String>(APPEND).map(|s| s.as_str()), Some("0"));
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("2"));
    }

    #[test]
    fn recognizes_clap37_replace_three_args_hyphen_indices() {
        let m = parse_opts().get_matches_from(vec!["tp", "--replace", "-1", "--old", "--new"]);
        let v: Vec<String> = m.get_many(REPLACE).unwrap().cloned().collect();
        assert_eq!(v, vec!["-1", "--old", "--new"]);
    }

    #[test]
    fn recognizes_clap37_swap_move_diff_long_chain() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--swap", "aa", "bb", "--mv", "cc", "dd", "--diff", "ee", "ff",
        ]);
        assert_eq!(
            m.get_many(SWAP).unwrap().cloned().collect::<Vec<String>>(),
            vec!["aa", "bb"]
        );
        assert_eq!(
            m.get_many(MOVE).unwrap().cloned().collect::<Vec<String>>(),
            vec!["cc", "dd"]
        );
        assert_eq!(
            m.get_many(DIFF).unwrap().cloned().collect::<Vec<String>>(),
            vec!["ee", "ff"]
        );
    }

    #[test]
    fn recognizes_clap37_verbose_thirty_six_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv"]);
        assert_eq!(m.get_count(VERBOSE), 36);
    }

    #[test]
    fn recognizes_clap37_double_dash_positional_colon_windows_path() {
        let m = parse_opts().get_matches_from(vec!["tp", "--", "C:\\\\Users\\\\Public\\\\log.txt"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("C:\\\\Users\\\\Public\\\\log.txt")
        );
    }

    #[test]
    fn recognizes_clap37_grep_wc_tail_long() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "-g", r"ERROR", "--wc", "0", "--tail", "1", "500",
        ]);
        assert_eq!(m.get_one::<String>(GREP).map(|s| s.as_str()), Some("ERROR"));
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("0"));
        assert_eq!(
            m.get_many::<String>(TAIL)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["1", "500"]
        );
    }

    #[test]
    fn recognizes_clap37_head_tail_large_line_counts() {
        let m = parse_opts()
            .get_matches_from(vec!["tp", "--head", "0", "999999", "--tail", "0", "888888"]);
        assert_eq!(
            m.get_many::<String>(HEAD)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["0", "999999"]
        );
        assert_eq!(
            m.get_many::<String>(TAIL)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["0", "888888"]
        );
    }

    #[test]
    fn recognizes_clap37_cat_short_twenty_one_indices() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "-C", "301", "302", "303", "304", "305", "306", "307", "308", "309", "310",
            "311", "312", "313", "314", "315", "316", "317", "318", "319", "320", "321",
        ]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(
            v,
            vec![
                "301", "302", "303", "304", "305", "306", "307", "308", "309", "310", "311", "312",
                "313", "314", "315", "316", "317", "318", "319", "320", "321"
            ]
        );
    }

    #[test]
    fn recognizes_clap37_sort_mtime_only_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort", "mtime"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("mtime"));
    }

    #[test]
    fn recognizes_clap37_rename_move_long_chain() {
        let m = parse_opts().get_matches_from(vec!["tp", "--rename", "x", "y", "--mv", "0", "9"]);
        assert_eq!(
            m.get_many(RENAME)
                .unwrap()
                .cloned()
                .collect::<Vec<String>>(),
            vec!["x", "y"]
        );
        assert_eq!(
            m.get_many(MOVE).unwrap().cloned().collect::<Vec<String>>(),
            vec!["0", "9"]
        );
    }

    #[test]
    fn recognizes_clap37_master_list_contents_numbered_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--master", "--list-contents-numbered"]);
        assert!(m.get_flag(MASTER));
        assert!(m.get_flag(LIST_CONTENTS_NUMBERED));
    }

    #[test]
    fn recognizes_clap37_rev_sort_size_path_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--rev", "--sort", "size", "--path", "7"]);
        assert!(m.get_flag(REVERSE));
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("size"));
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("7"));
    }

    #[test]
    fn recognizes_clap37_count_quiet_rev_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-k", "-q", "--rev"]);
        assert!(m.get_flag(COUNT));
        assert!(m.get_flag(SILENT));
        assert!(m.get_flag(REVERSE));
    }

    #[test]
    fn recognizes_clap37_info_edit_cat_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-I", "4", "-e", "5", "-C", "6", "7"]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(v, vec!["6", "7"]);
        assert_eq!(m.get_one::<String>(INFO).map(|s| s.as_str()), Some("4"));
        assert_eq!(m.get_one::<String>(EDIT).map(|s| s.as_str()), Some("5"));
    }

    #[test]
    fn recognizes_clap37_positional_rtsp_uri() {
        let m = parse_opts().get_matches_from(vec!["tp", "rtsp://camera.local:554/stream1"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("rtsp://camera.local:554/stream1")
        );
    }

    #[test]
    fn recognizes_clap37_program_name_temprs_with_pop() {
        let m = parse_opts().get_matches_from(vec!["temprs", "--pop"]);
        assert!(m.get_flag(POP));
    }

    #[test]
    fn recognizes_clap37_tag_combining_grave_accent() {
        let m = parse_opts().get_matches_from(vec!["tp", "-w", "e\u{0300}"]);
        assert_eq!(
            m.get_one::<String>(TAG).map(|s| s.as_str()),
            Some("e\u{0300}")
        );
    }

    #[test]
    fn recognizes_clap37_clear_list_contents_quiet_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--clear", "--list-contents", "--quiet"]);
        assert!(m.get_flag(CLEAR));
        assert!(m.get_flag(LIST_CONTENTS));
        assert!(m.get_flag(SILENT));
    }

    #[test]
    fn recognizes_clap37_directory_list_files_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-d", "-l"]);
        assert!(m.get_flag(DIRECTORY));
        assert!(m.get_flag(LIST_FILES));
    }

    #[test]
    fn recognizes_clap37_diff_only_short_pair() {
        let m = parse_opts().get_matches_from(vec!["tp", "-D", "left", "right"]);
        assert_eq!(
            m.get_many(DIFF).unwrap().cloned().collect::<Vec<String>>(),
            vec!["left", "right"]
        );
    }

    // ── clap coverage round 38 ─────────────────────────────

    #[test]
    fn recognizes_clap38_positional_postgres_uri() {
        let m = parse_opts().get_matches_from(vec!["tp", "postgres://db.example:5432/app"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("postgres://db.example:5432/app")
        );
    }

    #[test]
    fn recognizes_clap38_positional_mysql_uri() {
        let m =
            parse_opts().get_matches_from(vec!["tp", "mysql://user@db.internal:3306/inventory"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("mysql://user@db.internal:3306/inventory")
        );
    }

    #[test]
    fn recognizes_clap38_positional_redis_uri() {
        let m = parse_opts().get_matches_from(vec!["tp", "redis://cache.example:6379/0"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("redis://cache.example:6379/0")
        );
    }

    #[test]
    fn recognizes_clap38_positional_mongodb_uri() {
        let m =
            parse_opts().get_matches_from(vec!["tp", "mongodb://mongo.example:27017/telemetry"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("mongodb://mongo.example:27017/telemetry")
        );
    }

    #[test]
    fn recognizes_clap38_positional_imaps_uri() {
        let m = parse_opts().get_matches_from(vec!["tp", "imaps://mail.example:993/INBOX"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("imaps://mail.example:993/INBOX")
        );
    }

    #[test]
    fn recognizes_clap38_output_long_equals_negative_zero() {
        let m = parse_opts().get_matches_from(vec!["tp", "--output=-0"]);
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("-0"));
    }

    #[test]
    fn recognizes_clap38_input_long_equals_negative() {
        let m = parse_opts().get_matches_from(vec!["tp", "--input=-7"]);
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("-7"));
    }

    #[test]
    fn recognizes_clap38_cat_thirty_three_indices_long() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--cat", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13",
            "14", "15", "16", "17", "18", "19", "20", "21", "22", "23", "24", "25", "26", "27",
            "28", "29", "30", "31", "32", "33",
        ]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        let expected: Vec<String> = (1..=33).map(|n| n.to_string()).collect();
        let expected: Vec<&str> = expected.iter().map(String::as_str).collect();
        assert_eq!(v, expected);
    }

    #[test]
    fn recognizes_clap38_grep_atomic_group() {
        let m = parse_opts().get_matches_from(vec!["tp", "-g", r"(?>foo)bar"]);
        assert_eq!(
            m.get_one::<String>(GREP).map(|s| s.as_str()),
            Some(r"(?>foo)bar")
        );
    }

    #[test]
    fn recognizes_clap38_expire_plus_hours_suffix() {
        let m = parse_opts().get_matches_from(vec!["tp", "--expire", "+72h"]);
        assert_eq!(
            m.get_one::<String>(EXPIRE).map(|s| s.as_str()),
            Some("+72h")
        );
    }

    #[test]
    fn recognizes_clap38_sort_rev_positional_plain() {
        let m = parse_opts().get_matches_from(vec!["tp", "--rev", "stdin.txt"]);
        assert!(m.get_flag(REVERSE));
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("stdin.txt")
        );
    }

    #[test]
    fn recognizes_clap38_unshift_shift_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-u", "-s"]);
        assert!(m.get_flag(UNSHIFT));
        assert!(m.get_flag(SHIFT));
    }

    #[test]
    fn recognizes_clap38_add_append_pop_short_chain() {
        let m = parse_opts().get_matches_from(vec!["tp", "-a", "3", "-A", "4", "-p"]);
        assert_eq!(m.get_one::<String>(ADD).map(|s| s.as_str()), Some("3"));
        assert_eq!(m.get_one::<String>(APPEND).map(|s| s.as_str()), Some("4"));
        assert!(m.get_flag(POP));
    }

    #[test]
    fn recognizes_clap38_replace_three_args_newline_replacement() {
        let m = parse_opts().get_matches_from(vec!["tp", "--replace", "9", ",", "\n"]);
        let v: Vec<String> = m.get_many(REPLACE).unwrap().cloned().collect();
        assert_eq!(v, vec!["9", ",", "\n"]);
    }

    #[test]
    fn recognizes_clap38_verbose_thirty_seven_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv"]);
        assert_eq!(m.get_count(VERBOSE), 37);
    }

    #[test]
    fn recognizes_clap38_double_dash_positional_dot_hidden() {
        let m = parse_opts().get_matches_from(vec!["tp", "--", ".config/app.toml"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some(".config/app.toml")
        );
    }

    #[test]
    fn recognizes_clap38_grep_head_tail_long() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "-g", r"\S+", "--head", "0", "12", "--tail", "0", "34",
        ]);
        assert_eq!(m.get_one::<String>(GREP).map(|s| s.as_str()), Some(r"\S+"));
        assert_eq!(
            m.get_many::<String>(HEAD)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["0", "12"]
        );
        assert_eq!(
            m.get_many::<String>(TAIL)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["0", "34"]
        );
    }

    #[test]
    fn recognizes_clap38_head_tail_negative_line_counts() {
        let m = parse_opts().get_matches_from(vec!["tp", "--head", "1", "-9", "--tail", "2", "-8"]);
        assert_eq!(
            m.get_many::<String>(HEAD)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["1", "-9"]
        );
        assert_eq!(
            m.get_many::<String>(TAIL)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["2", "-8"]
        );
    }

    #[test]
    fn recognizes_clap38_cat_short_twenty_two_indices() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "-C", "401", "402", "403", "404", "405", "406", "407", "408", "409", "410",
            "411", "412", "413", "414", "415", "416", "417", "418", "419", "420", "421", "422",
        ]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        let expected: Vec<String> = (401..=422).map(|n| n.to_string()).collect();
        let expected: Vec<&str> = expected.iter().map(String::as_str).collect();
        assert_eq!(v, expected);
    }

    #[test]
    fn recognizes_clap38_sort_size_equals_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort=size"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("size"));
    }

    #[test]
    fn recognizes_clap38_dup_swap_long_chain() {
        let m = parse_opts().get_matches_from(vec!["tp", "--dup", "6", "--swap", "0", "1"]);
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("6"));
        assert_eq!(
            m.get_many(SWAP).unwrap().cloned().collect::<Vec<String>>(),
            vec!["0", "1"]
        );
    }

    #[test]
    fn recognizes_clap38_master_list_files_numbered_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-m", "-n"]);
        assert!(m.get_flag(MASTER));
        assert!(m.get_flag(LIST_FILES_NUMBERED));
    }

    #[test]
    fn recognizes_clap38_rev_sort_mtime_wc_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--rev", "--sort", "mtime", "--wc", "3"]);
        assert!(m.get_flag(REVERSE));
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("mtime"));
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("3"));
    }

    #[test]
    fn recognizes_clap38_count_list_files_quiet_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--count", "--list-files", "--quiet"]);
        assert!(m.get_flag(COUNT));
        assert!(m.get_flag(LIST_FILES));
        assert!(m.get_flag(SILENT));
    }

    #[test]
    fn recognizes_clap38_info_grep_edit_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-I", "8", "-g", "\t", "-e", "9"]);
        assert_eq!(m.get_one::<String>(INFO).map(|s| s.as_str()), Some("8"));
        assert_eq!(m.get_one::<String>(GREP).map(|s| s.as_str()), Some("\t"));
        assert_eq!(m.get_one::<String>(EDIT).map(|s| s.as_str()), Some("9"));
    }

    #[test]
    fn recognizes_clap38_positional_nats_uri() {
        let m = parse_opts().get_matches_from(vec!["tp", "nats://bus.internal:4222/events"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("nats://bus.internal:4222/events")
        );
    }

    #[test]
    fn recognizes_clap38_program_name_temprs_list_contents() {
        let m = parse_opts().get_matches_from(vec!["temprs", "--list-contents"]);
        assert!(m.get_flag(LIST_CONTENTS));
    }

    #[test]
    fn recognizes_clap38_tag_bidi_embedding() {
        let m = parse_opts().get_matches_from(vec!["tp", "--name", "x\u{202e}y"]);
        assert_eq!(
            m.get_one::<String>(TAG).map(|s| s.as_str()),
            Some("x\u{202e}y")
        );
    }

    #[test]
    fn recognizes_clap38_clear_directory_quiet_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--clear", "--dir", "--quiet"]);
        assert!(m.get_flag(CLEAR));
        assert!(m.get_flag(DIRECTORY));
        assert!(m.get_flag(SILENT));
    }

    #[test]
    fn recognizes_clap38_directory_list_contents_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-d", "-L"]);
        assert!(m.get_flag(DIRECTORY));
        assert!(m.get_flag(LIST_CONTENTS));
    }

    #[test]
    fn recognizes_clap38_diff_move_long_chain() {
        let m = parse_opts().get_matches_from(vec!["tp", "--diff", "p", "q", "--mv", "r", "s"]);
        assert_eq!(
            m.get_many(DIFF).unwrap().cloned().collect::<Vec<String>>(),
            vec!["p", "q"]
        );
        assert_eq!(
            m.get_many(MOVE).unwrap().cloned().collect::<Vec<String>>(),
            vec!["r", "s"]
        );
    }

    #[test]
    fn recognizes_clap38_swap_only_short_pair() {
        let m = parse_opts().get_matches_from(vec!["tp", "-S", "lo", "hi"]);
        assert_eq!(
            m.get_many(SWAP).unwrap().cloned().collect::<Vec<String>>(),
            vec!["lo", "hi"]
        );
    }

    // ── clap coverage round 39 ─────────────────────────────

    #[test]
    fn recognizes_clap39_positional_https_uri() {
        let m = parse_opts().get_matches_from(vec!["tp", "https://api.example/v2/resource"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("https://api.example/v2/resource")
        );
    }

    #[test]
    fn recognizes_clap39_positional_http_uri() {
        let m = parse_opts().get_matches_from(vec!["tp", "http://localhost:8080/health"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("http://localhost:8080/health")
        );
    }

    #[test]
    fn recognizes_clap39_positional_coap_uri() {
        let m = parse_opts().get_matches_from(vec!["tp", "coap://sensor.local/.well-known/core"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("coap://sensor.local/.well-known/core")
        );
    }

    #[test]
    fn recognizes_clap39_positional_smtp_uri() {
        let m = parse_opts().get_matches_from(vec!["tp", "smtp://relay.example:587"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("smtp://relay.example:587")
        );
    }

    #[test]
    fn recognizes_clap39_positional_pop3_uri() {
        let m = parse_opts().get_matches_from(vec!["tp", "pop3://mailbox.example:110"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("pop3://mailbox.example:110")
        );
    }

    #[test]
    fn recognizes_clap39_output_long_equals_double_hyphen() {
        let m = parse_opts().get_matches_from(vec!["tp", "--output=--"]);
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("--"));
    }

    #[test]
    fn recognizes_clap39_input_long_equals_double_hyphen() {
        let m = parse_opts().get_matches_from(vec!["tp", "--input=--"]);
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("--"));
    }

    #[test]
    fn recognizes_clap39_cat_thirty_four_indices_long() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--cat", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13",
            "14", "15", "16", "17", "18", "19", "20", "21", "22", "23", "24", "25", "26", "27",
            "28", "29", "30", "31", "32", "33", "34",
        ]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        let expected: Vec<String> = (1..=34).map(|n| n.to_string()).collect();
        let expected: Vec<&str> = expected.iter().map(String::as_str).collect();
        assert_eq!(v, expected);
    }

    #[test]
    fn recognizes_clap39_grep_branch_reset() {
        let m = parse_opts().get_matches_from(vec!["tp", "-g", r"(?|a|b)"]);
        assert_eq!(
            m.get_one::<String>(GREP).map(|s| s.as_str()),
            Some(r"(?|a|b)")
        );
    }

    #[test]
    fn recognizes_clap39_expire_iso8601_utc() {
        let m = parse_opts().get_matches_from(vec!["tp", "--expire", "2026-04-03T12:00:00Z"]);
        assert_eq!(
            m.get_one::<String>(EXPIRE).map(|s| s.as_str()),
            Some("2026-04-03T12:00:00Z")
        );
    }

    #[test]
    fn recognizes_clap39_sort_name_rev_positional() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort", "name", "--rev", "Cargo.toml"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("name"));
        assert!(m.get_flag(REVERSE));
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("Cargo.toml")
        );
    }

    #[test]
    fn recognizes_clap39_pop_unshift_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-p", "-u"]);
        assert!(m.get_flag(POP));
        assert!(m.get_flag(UNSHIFT));
    }

    #[test]
    fn recognizes_clap39_remove_append_dup_short_chain() {
        let m = parse_opts().get_matches_from(vec!["tp", "-r", "2", "-A", "3", "-x", "4"]);
        assert_eq!(m.get_one::<String>(REMOVE).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(APPEND).map(|s| s.as_str()), Some("3"));
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("4"));
    }

    #[test]
    fn recognizes_clap39_replace_three_args_ideographic_space() {
        let m = parse_opts().get_matches_from(vec!["tp", "--replace", "0", " ", "\u{3000}"]);
        let v: Vec<String> = m.get_many(REPLACE).unwrap().cloned().collect();
        assert_eq!(v, vec!["0", " ", "\u{3000}"]);
    }

    #[test]
    fn recognizes_clap39_verbose_thirty_eight_short() {
        let m =
            parse_opts().get_matches_from(vec!["tp", "-vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv"]);
        assert_eq!(m.get_count(VERBOSE), 38);
    }

    #[test]
    fn recognizes_clap39_double_dash_positional_tilde_path() {
        let m = parse_opts().get_matches_from(vec!["tp", "--", "~/Library/Logs/app.log"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("~/Library/Logs/app.log")
        );
    }

    #[test]
    fn recognizes_clap39_size_path_wc_long() {
        let m =
            parse_opts().get_matches_from(vec!["tp", "--size", "1", "--path", "2", "--wc", "3"]);
        assert_eq!(m.get_one::<String>(SIZE).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("3"));
    }

    #[test]
    fn recognizes_clap39_head_tail_zero_lines_both() {
        let m = parse_opts().get_matches_from(vec!["tp", "--head", "0", "0", "--tail", "1", "0"]);
        assert_eq!(
            m.get_many::<String>(HEAD)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["0", "0"]
        );
        assert_eq!(
            m.get_many::<String>(TAIL)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["1", "0"]
        );
    }

    #[test]
    fn recognizes_clap39_cat_short_twenty_three_indices() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "-C", "501", "502", "503", "504", "505", "506", "507", "508", "509", "510",
            "511", "512", "513", "514", "515", "516", "517", "518", "519", "520", "521", "522",
            "523",
        ]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        let expected: Vec<String> = (501..=523).map(|n| n.to_string()).collect();
        let expected: Vec<&str> = expected.iter().map(String::as_str).collect();
        assert_eq!(v, expected);
    }

    #[test]
    fn recognizes_clap39_sort_mtime_equals_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort=mtime"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("mtime"));
    }

    #[test]
    fn recognizes_clap39_move_diff_swap_long_chain() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--mv", "m0", "m1", "--diff", "d0", "d1", "--swap", "s0", "s1",
        ]);
        assert_eq!(
            m.get_many(MOVE).unwrap().cloned().collect::<Vec<String>>(),
            vec!["m0", "m1"]
        );
        assert_eq!(
            m.get_many(DIFF).unwrap().cloned().collect::<Vec<String>>(),
            vec!["d0", "d1"]
        );
        assert_eq!(
            m.get_many(SWAP).unwrap().cloned().collect::<Vec<String>>(),
            vec!["s0", "s1"]
        );
    }

    #[test]
    fn recognizes_clap39_master_list_contents_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-m", "-L"]);
        assert!(m.get_flag(MASTER));
        assert!(m.get_flag(LIST_CONTENTS));
    }

    #[test]
    fn recognizes_clap39_rev_sort_size_wc_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--rev", "--sort", "size", "--wc", "-1"]);
        assert!(m.get_flag(REVERSE));
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("size"));
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("-1"));
    }

    #[test]
    fn recognizes_clap39_count_list_numbered_quiet_long() {
        let m = parse_opts().get_matches_from(vec![
            "tp",
            "--count",
            "--list-files-numbered",
            "--quiet",
        ]);
        assert!(m.get_flag(COUNT));
        assert!(m.get_flag(LIST_FILES_NUMBERED));
        assert!(m.get_flag(SILENT));
    }

    #[test]
    fn recognizes_clap39_info_edit_cat_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-I", "6", "-e", "7", "-C", "8", "9"]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(v, vec!["8", "9"]);
        assert_eq!(m.get_one::<String>(INFO).map(|s| s.as_str()), Some("6"));
        assert_eq!(m.get_one::<String>(EDIT).map(|s| s.as_str()), Some("7"));
    }

    #[test]
    fn recognizes_clap39_positional_kafka_uri() {
        let m = parse_opts().get_matches_from(vec!["tp", "kafka://broker.internal:9092/topic"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("kafka://broker.internal:9092/topic")
        );
    }

    #[test]
    fn recognizes_clap39_program_name_temprs_list_numbered() {
        let m = parse_opts().get_matches_from(vec!["temprs", "--list-files-numbered"]);
        assert!(m.get_flag(LIST_FILES_NUMBERED));
    }

    #[test]
    fn recognizes_clap39_tag_emoji_variation_selector() {
        let m = parse_opts().get_matches_from(vec!["tp", "-w", "x\u{fe0f}"]);
        assert_eq!(
            m.get_one::<String>(TAG).map(|s| s.as_str()),
            Some("x\u{fe0f}")
        );
    }

    #[test]
    fn recognizes_clap39_clear_master_quiet_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--clear", "--master", "--quiet"]);
        assert!(m.get_flag(CLEAR));
        assert!(m.get_flag(MASTER));
        assert!(m.get_flag(SILENT));
    }

    #[test]
    fn recognizes_clap39_directory_list_contents_numbered_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-d", "-N"]);
        assert!(m.get_flag(DIRECTORY));
        assert!(m.get_flag(LIST_CONTENTS_NUMBERED));
    }

    #[test]
    fn recognizes_clap39_grep_long_multiline_flag_syntax() {
        let m = parse_opts().get_matches_from(vec!["tp", "--grep", "(?s)^BEGIN.*END$"]);
        assert_eq!(
            m.get_one::<String>(GREP).map(|s| s.as_str()),
            Some("(?s)^BEGIN.*END$")
        );
    }

    #[test]
    fn recognizes_clap39_swap_move_short_chain() {
        let m = parse_opts().get_matches_from(vec!["tp", "-S", "u", "v", "-M", "w", "z"]);
        assert_eq!(
            m.get_many(SWAP).unwrap().cloned().collect::<Vec<String>>(),
            vec!["u", "v"]
        );
        assert_eq!(
            m.get_many(MOVE).unwrap().cloned().collect::<Vec<String>>(),
            vec!["w", "z"]
        );
    }

    // ── clap coverage round 40 ─────────────────────────────

    #[test]
    fn recognizes_clap40_positional_vscode_file_uri() {
        let m = parse_opts().get_matches_from(vec!["tp", "vscode://file/Users/dev/src/main.rs"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("vscode://file/Users/dev/src/main.rs")
        );
    }

    #[test]
    fn recognizes_clap40_positional_ipfs_uri() {
        let m = parse_opts().get_matches_from(vec![
            "tp",
            "ipfs://QmYwAPJzv5CZsnA625s3Xf2nemtYgPpHdWEz79ojWnPbdG/readme",
        ]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("ipfs://QmYwAPJzv5CZsnA625s3Xf2nemtYgPpHdWEz79ojWnPbdG/readme")
        );
    }

    #[test]
    fn recognizes_clap40_positional_ipns_uri() {
        let m = parse_opts().get_matches_from(vec!["tp", "ipns://example.com/app/index.html"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("ipns://example.com/app/index.html")
        );
    }

    #[test]
    fn recognizes_clap40_positional_gemini_uri() {
        let m = parse_opts().get_matches_from(vec!["tp", "gemini://gemini.circumlunar.space/"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("gemini://gemini.circumlunar.space/")
        );
    }

    #[test]
    fn recognizes_clap40_positional_gopher_uri() {
        let m = parse_opts().get_matches_from(vec!["tp", "gopher://gopher.floodgap.com:70/1"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("gopher://gopher.floodgap.com:70/1")
        );
    }

    #[test]
    fn recognizes_clap40_output_short_equals_form() {
        let m = parse_opts().get_matches_from(vec!["tp", "-o=42"]);
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("42"));
    }

    #[test]
    fn recognizes_clap40_input_short_equals_form() {
        let m = parse_opts().get_matches_from(vec!["tp", "-i=0"]);
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("0"));
    }

    #[test]
    fn recognizes_clap40_cat_thirty_five_indices_long() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--cat", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13",
            "14", "15", "16", "17", "18", "19", "20", "21", "22", "23", "24", "25", "26", "27",
            "28", "29", "30", "31", "32", "33", "34", "35",
        ]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        let expected: Vec<String> = (1..=35).map(|n| n.to_string()).collect();
        let expected: Vec<&str> = expected.iter().map(String::as_str).collect();
        assert_eq!(v, expected);
    }

    #[test]
    fn recognizes_clap40_grep_possessive_quantifier() {
        let m = parse_opts().get_matches_from(vec!["tp", "-g", r"a++b"]);
        assert_eq!(m.get_one::<String>(GREP).map(|s| s.as_str()), Some(r"a++b"));
    }

    #[test]
    fn recognizes_clap40_expire_cron_expression() {
        let m = parse_opts().get_matches_from(vec!["tp", "--expire", "0 */6 * * *"]);
        assert_eq!(
            m.get_one::<String>(EXPIRE).map(|s| s.as_str()),
            Some("0 */6 * * *")
        );
    }

    #[test]
    fn recognizes_clap40_sort_size_positional_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort", "size", "big.bin"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("size"));
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("big.bin")
        );
    }

    #[test]
    fn recognizes_clap40_shift_pop_unshift_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-s", "-p", "-u"]);
        assert!(m.get_flag(SHIFT));
        assert!(m.get_flag(POP));
        assert!(m.get_flag(UNSHIFT));
    }

    #[test]
    fn recognizes_clap40_add_remove_pop_short_chain() {
        let m = parse_opts().get_matches_from(vec!["tp", "-a", "5", "-r", "6", "-p"]);
        assert_eq!(m.get_one::<String>(ADD).map(|s| s.as_str()), Some("5"));
        assert_eq!(m.get_one::<String>(REMOVE).map(|s| s.as_str()), Some("6"));
        assert!(m.get_flag(POP));
    }

    #[test]
    fn recognizes_clap40_replace_three_args_empty_replacement() {
        let m = parse_opts().get_matches_from(vec!["tp", "--replace", "1", "needle", ""]);
        let v: Vec<String> = m.get_many(REPLACE).unwrap().cloned().collect();
        assert_eq!(v, vec!["1", "needle", ""]);
    }

    #[test]
    fn recognizes_clap40_verbose_thirty_nine_short() {
        let m =
            parse_opts().get_matches_from(vec!["tp", "-vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv"]);
        assert_eq!(m.get_count(VERBOSE), 39);
    }

    #[test]
    fn recognizes_clap40_double_dash_positional_dot_slash_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "--", "./"]);
        assert_eq!(m.get_one::<String>(ARGFILE).map(|s| s.as_str()), Some("./"));
    }

    #[test]
    fn recognizes_clap40_wc_tail_head_long() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--wc", "0", "--tail", "1", "7", "--head", "2", "8",
        ]);
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("0"));
        assert_eq!(
            m.get_many::<String>(TAIL)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["1", "7"]
        );
        assert_eq!(
            m.get_many::<String>(HEAD)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["2", "8"]
        );
    }

    #[test]
    fn recognizes_clap40_head_tail_same_index_different_counts() {
        let m = parse_opts().get_matches_from(vec!["tp", "--head", "0", "11", "--tail", "0", "22"]);
        assert_eq!(
            m.get_many::<String>(HEAD)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["0", "11"]
        );
        assert_eq!(
            m.get_many::<String>(TAIL)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["0", "22"]
        );
    }

    #[test]
    fn recognizes_clap40_cat_short_twenty_four_indices() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "-C", "601", "602", "603", "604", "605", "606", "607", "608", "609", "610",
            "611", "612", "613", "614", "615", "616", "617", "618", "619", "620", "621", "622",
            "623", "624",
        ]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        let expected: Vec<String> = (601..=624).map(|n| n.to_string()).collect();
        let expected: Vec<&str> = expected.iter().map(String::as_str).collect();
        assert_eq!(v, expected);
    }

    #[test]
    fn recognizes_clap40_rev_sort_size_equals_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--rev", "--sort=size"]);
        assert!(m.get_flag(REVERSE));
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("size"));
    }

    #[test]
    fn recognizes_clap40_dup_swap_diff_long_chain() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--dup", "9", "--swap", "1", "2", "--diff", "3", "4",
        ]);
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("9"));
        assert_eq!(
            m.get_many(SWAP).unwrap().cloned().collect::<Vec<String>>(),
            vec!["1", "2"]
        );
        assert_eq!(
            m.get_many(DIFF).unwrap().cloned().collect::<Vec<String>>(),
            vec!["3", "4"]
        );
    }

    #[test]
    fn recognizes_clap40_master_list_contents_numbered_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-m", "-N"]);
        assert!(m.get_flag(MASTER));
        assert!(m.get_flag(LIST_CONTENTS_NUMBERED));
    }

    #[test]
    fn recognizes_clap40_rev_sort_mtime_path_long() {
        let m =
            parse_opts().get_matches_from(vec!["tp", "--rev", "--sort", "mtime", "--path", "5"]);
        assert!(m.get_flag(REVERSE));
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("mtime"));
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("5"));
    }

    #[test]
    fn recognizes_clap40_count_list_contents_quiet_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-k", "-L", "-q"]);
        assert!(m.get_flag(COUNT));
        assert!(m.get_flag(LIST_CONTENTS));
        assert!(m.get_flag(SILENT));
    }

    #[test]
    fn recognizes_clap40_info_grep_cat_short() {
        let m =
            parse_opts().get_matches_from(vec!["tp", "-I", "1", "-g", "(?m)^#", "-C", "2", "3"]);
        assert_eq!(m.get_one::<String>(INFO).map(|s| s.as_str()), Some("1"));
        assert_eq!(
            m.get_one::<String>(GREP).map(|s| s.as_str()),
            Some("(?m)^#")
        );
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(v, vec!["2", "3"]);
    }

    #[test]
    fn recognizes_clap40_positional_slack_deep_link() {
        let m = parse_opts().get_matches_from(vec!["tp", "slack://channel?team=T123&id=C456"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("slack://channel?team=T123&id=C456")
        );
    }

    #[test]
    fn recognizes_clap40_program_name_temprs_list_contents_numbered() {
        let m = parse_opts().get_matches_from(vec!["temprs", "--list-contents-numbered"]);
        assert!(m.get_flag(LIST_CONTENTS_NUMBERED));
    }

    #[test]
    fn recognizes_clap40_tag_zero_width_joiner_sequence() {
        let m = parse_opts().get_matches_from(vec!["tp", "--name", "f\u{200c}i"]);
        assert_eq!(
            m.get_one::<String>(TAG).map(|s| s.as_str()),
            Some("f\u{200c}i")
        );
    }

    #[test]
    fn recognizes_clap40_clear_list_files_quiet_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--clear", "--list-files", "--quiet"]);
        assert!(m.get_flag(CLEAR));
        assert!(m.get_flag(LIST_FILES));
        assert!(m.get_flag(SILENT));
    }

    #[test]
    fn recognizes_clap40_directory_list_files_numbered_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-d", "-n"]);
        assert!(m.get_flag(DIRECTORY));
        assert!(m.get_flag(LIST_FILES_NUMBERED));
    }

    #[test]
    fn recognizes_clap40_rename_diff_long_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "--rename", "u", "v", "--diff", "i", "j"]);
        assert_eq!(
            m.get_many(RENAME)
                .unwrap()
                .cloned()
                .collect::<Vec<String>>(),
            vec!["u", "v"]
        );
        assert_eq!(
            m.get_many(DIFF).unwrap().cloned().collect::<Vec<String>>(),
            vec!["i", "j"]
        );
    }

    #[test]
    fn recognizes_clap40_move_only_long_pair() {
        let m = parse_opts().get_matches_from(vec!["tp", "--mv", "src", "dst"]);
        assert_eq!(
            m.get_many(MOVE).unwrap().cloned().collect::<Vec<String>>(),
            vec!["src", "dst"]
        );
    }

    // ── clap coverage round 41 ─────────────────────────────

    #[test]
    fn recognizes_clap41_positional_steam_uri() {
        let m = parse_opts().get_matches_from(vec!["tp", "steam://run/440"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("steam://run/440")
        );
    }

    #[test]
    fn recognizes_clap41_positional_obsidian_uri() {
        let m =
            parse_opts().get_matches_from(vec!["tp", "obsidian://open?vault=notes&file=daily.md"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("obsidian://open?vault=notes&file=daily.md")
        );
    }

    #[test]
    fn recognizes_clap41_positional_cursor_uri() {
        let m =
            parse_opts().get_matches_from(vec!["tp", "cursor://file//Users/dev/crate/src/lib.rs"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("cursor://file//Users/dev/crate/src/lib.rs")
        );
    }

    #[test]
    fn recognizes_clap41_positional_figma_uri() {
        let m = parse_opts().get_matches_from(vec!["tp", "figma://file/abc123def456"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("figma://file/abc123def456")
        );
    }

    #[test]
    fn recognizes_clap41_positional_notion_uri() {
        let m =
            parse_opts().get_matches_from(vec!["tp", "notion://www.notion.so/workspace/page-id"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("notion://www.notion.so/workspace/page-id")
        );
    }

    #[test]
    fn recognizes_clap41_output_long_hex_style() {
        let m = parse_opts().get_matches_from(vec!["tp", "--output=0xFF"]);
        assert_eq!(
            m.get_one::<String>(OUTPUT).map(|s| s.as_str()),
            Some("0xFF")
        );
    }

    #[test]
    fn recognizes_clap41_input_short_hex_style() {
        let m = parse_opts().get_matches_from(vec!["tp", "-i=0x10"]);
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("0x10"));
    }

    #[test]
    fn recognizes_clap41_cat_thirty_six_indices_long() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--cat", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13",
            "14", "15", "16", "17", "18", "19", "20", "21", "22", "23", "24", "25", "26", "27",
            "28", "29", "30", "31", "32", "33", "34", "35", "36",
        ]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        let expected: Vec<String> = (1..=36).map(|n| n.to_string()).collect();
        let expected: Vec<&str> = expected.iter().map(String::as_str).collect();
        assert_eq!(v, expected);
    }

    #[test]
    fn recognizes_clap41_grep_conditional_branch() {
        let m = parse_opts().get_matches_from(vec!["tp", "-g", r"(?(1)foo|bar)"]);
        assert_eq!(
            m.get_one::<String>(GREP).map(|s| s.as_str()),
            Some(r"(?(1)foo|bar)")
        );
    }

    #[test]
    fn recognizes_clap41_expire_compound_duration() {
        let m = parse_opts().get_matches_from(vec!["tp", "--expire", "2d+3h15m"]);
        assert_eq!(
            m.get_one::<String>(EXPIRE).map(|s| s.as_str()),
            Some("2d+3h15m")
        );
    }

    #[test]
    fn recognizes_clap41_sort_mtime_positional_plain() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort", "mtime", "archive.tar"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("mtime"));
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("archive.tar")
        );
    }

    #[test]
    fn recognizes_clap41_pop_shift_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-p", "-s"]);
        assert!(m.get_flag(POP));
        assert!(m.get_flag(SHIFT));
    }

    #[test]
    fn recognizes_clap41_add_dup_swap_short_chain() {
        let m = parse_opts().get_matches_from(vec!["tp", "-a", "1", "-x", "2", "-S", "3", "4"]);
        assert_eq!(m.get_one::<String>(ADD).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("2"));
        assert_eq!(
            m.get_many(SWAP).unwrap().cloned().collect::<Vec<String>>(),
            vec!["3", "4"]
        );
    }

    #[test]
    fn recognizes_clap41_replace_three_args_pipe_in_pattern() {
        let m = parse_opts().get_matches_from(vec!["tp", "--replace", "0", "a|b|c", "x"]);
        let v: Vec<String> = m.get_many(REPLACE).unwrap().cloned().collect();
        assert_eq!(v, vec!["0", "a|b|c", "x"]);
    }

    #[test]
    fn recognizes_clap41_verbose_forty_short() {
        let m =
            parse_opts().get_matches_from(vec!["tp", "-vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv"]);
        assert_eq!(m.get_count(VERBOSE), 40);
    }

    #[test]
    fn recognizes_clap41_double_dash_positional_unc_path() {
        let m = parse_opts().get_matches_from(vec!["tp", "--", r"\\fileserver\share\report.csv"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some(r"\\fileserver\share\report.csv")
        );
    }

    #[test]
    fn recognizes_clap41_path_size_wc_long() {
        let m =
            parse_opts().get_matches_from(vec!["tp", "--path", "4", "--size", "5", "--wc", "6"]);
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("4"));
        assert_eq!(m.get_one::<String>(SIZE).map(|s| s.as_str()), Some("5"));
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("6"));
    }

    #[test]
    fn recognizes_clap41_head_tail_max_line_counts() {
        let m = parse_opts().get_matches_from(vec![
            "tp",
            "--head",
            "0",
            "4294967295",
            "--tail",
            "1",
            "4294967294",
        ]);
        assert_eq!(
            m.get_many::<String>(HEAD)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["0", "4294967295"]
        );
        assert_eq!(
            m.get_many::<String>(TAIL)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["1", "4294967294"]
        );
    }

    #[test]
    fn recognizes_clap41_cat_short_twenty_five_indices() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "-C", "701", "702", "703", "704", "705", "706", "707", "708", "709", "710",
            "711", "712", "713", "714", "715", "716", "717", "718", "719", "720", "721", "722",
            "723", "724", "725",
        ]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        let expected: Vec<String> = (701..=725).map(|n| n.to_string()).collect();
        let expected: Vec<&str> = expected.iter().map(String::as_str).collect();
        assert_eq!(v, expected);
    }

    #[test]
    fn recognizes_clap41_sort_name_equals_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort=name"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("name"));
    }

    #[test]
    fn recognizes_clap41_rename_move_swap_long_chain() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--rename", "old", "new", "--mv", "a", "b", "--swap", "c", "d",
        ]);
        assert_eq!(
            m.get_many(RENAME)
                .unwrap()
                .cloned()
                .collect::<Vec<String>>(),
            vec!["old", "new"]
        );
        assert_eq!(
            m.get_many(MOVE).unwrap().cloned().collect::<Vec<String>>(),
            vec!["a", "b"]
        );
        assert_eq!(
            m.get_many(SWAP).unwrap().cloned().collect::<Vec<String>>(),
            vec!["c", "d"]
        );
    }

    #[test]
    fn recognizes_clap41_master_list_files_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-m", "-l"]);
        assert!(m.get_flag(MASTER));
        assert!(m.get_flag(LIST_FILES));
    }

    #[test]
    fn recognizes_clap41_rev_wc_size_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--rev", "--wc", "8", "--size", "9"]);
        assert!(m.get_flag(REVERSE));
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("8"));
        assert_eq!(m.get_one::<String>(SIZE).map(|s| s.as_str()), Some("9"));
    }

    #[test]
    fn recognizes_clap41_count_master_quiet_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-k", "-m", "-q"]);
        assert!(m.get_flag(COUNT));
        assert!(m.get_flag(MASTER));
        assert!(m.get_flag(SILENT));
    }

    #[test]
    fn recognizes_clap41_info_edit_cat_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-I", "0", "-e", "1", "-C", "2", "3"]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(v, vec!["2", "3"]);
        assert_eq!(m.get_one::<String>(INFO).map(|s| s.as_str()), Some("0"));
        assert_eq!(m.get_one::<String>(EDIT).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_clap41_positional_ms_teams_uri() {
        let m = parse_opts().get_matches_from(vec![
            "tp",
            "msteams://teams.microsoft.com/l/channel/19%3a...",
        ]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("msteams://teams.microsoft.com/l/channel/19%3a...")
        );
    }

    #[test]
    fn recognizes_clap41_program_name_temprs_list_contents_numbered_short() {
        let m = parse_opts().get_matches_from(vec!["temprs", "-N"]);
        assert!(m.get_flag(LIST_CONTENTS_NUMBERED));
    }

    #[test]
    fn recognizes_clap41_tag_superscript_two() {
        let m = parse_opts().get_matches_from(vec!["tp", "-w", "m\u{00b2}"]);
        assert_eq!(
            m.get_one::<String>(TAG).map(|s| s.as_str()),
            Some("m\u{00b2}")
        );
    }

    #[test]
    fn recognizes_clap41_clear_list_files_numbered_quiet_long() {
        let m = parse_opts().get_matches_from(vec![
            "tp",
            "--clear",
            "--list-files-numbered",
            "--quiet",
        ]);
        assert!(m.get_flag(CLEAR));
        assert!(m.get_flag(LIST_FILES_NUMBERED));
        assert!(m.get_flag(SILENT));
    }

    #[test]
    fn recognizes_clap41_directory_list_contents_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-d", "-L"]);
        assert!(m.get_flag(DIRECTORY));
        assert!(m.get_flag(LIST_CONTENTS));
    }

    #[test]
    fn recognizes_clap41_diff_swap_long_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "--diff", "u", "v", "--swap", "x", "y"]);
        assert_eq!(
            m.get_many(DIFF).unwrap().cloned().collect::<Vec<String>>(),
            vec!["u", "v"]
        );
        assert_eq!(
            m.get_many(SWAP).unwrap().cloned().collect::<Vec<String>>(),
            vec!["x", "y"]
        );
    }

    #[test]
    fn recognizes_clap41_grep_path_size_long() {
        let m = parse_opts()
            .get_matches_from(vec!["tp", "--grep", r"\S", "--path", "0", "--size", "1"]);
        assert_eq!(m.get_one::<String>(GREP).map(|s| s.as_str()), Some(r"\S"));
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("0"));
        assert_eq!(m.get_one::<String>(SIZE).map(|s| s.as_str()), Some("1"));
    }

    // ── clap coverage round 42 ─────────────────────────────

    #[test]
    fn recognizes_clap42_positional_zoommtg_uri() {
        let m = parse_opts().get_matches_from(vec!["tp", "zoommtg://zoom.us/join?conf=abc123"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("zoommtg://zoom.us/join?conf=abc123")
        );
    }

    #[test]
    fn recognizes_clap42_positional_matrix_uri() {
        let m = parse_opts().get_matches_from(vec!["tp", "matrix:r/user:example.org?action=chat"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("matrix:r/user:example.org?action=chat")
        );
    }

    #[test]
    fn recognizes_clap42_positional_discord_uri() {
        let m =
            parse_opts().get_matches_from(vec!["tp", "discord://discord.com/channels/111/222/333"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("discord://discord.com/channels/111/222/333")
        );
    }

    #[test]
    fn recognizes_clap42_positional_vscode_insiders_uri() {
        let m =
            parse_opts().get_matches_from(vec!["tp", "vscode-insiders://file/Users/dev/project"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("vscode-insiders://file/Users/dev/project")
        );
    }

    #[test]
    fn recognizes_clap42_positional_brave_uri() {
        let m = parse_opts().get_matches_from(vec!["tp", "brave://settings/shields"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("brave://settings/shields")
        );
    }

    #[test]
    fn recognizes_clap42_output_percent_encoded_plus() {
        let m = parse_opts().get_matches_from(vec!["tp", "--output=%2B"]);
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("%2B"));
    }

    #[test]
    fn recognizes_clap42_input_scientific_notation_string() {
        let m = parse_opts().get_matches_from(vec!["tp", "--input=1e+3"]);
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("1e+3"));
    }

    #[test]
    fn recognizes_clap42_cat_thirty_seven_indices_long() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--cat", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13",
            "14", "15", "16", "17", "18", "19", "20", "21", "22", "23", "24", "25", "26", "27",
            "28", "29", "30", "31", "32", "33", "34", "35", "36", "37",
        ]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        let expected: Vec<String> = (1..=37).map(|n| n.to_string()).collect();
        let expected: Vec<&str> = expected.iter().map(String::as_str).collect();
        assert_eq!(v, expected);
    }

    #[test]
    fn recognizes_clap42_grep_unicode_property_letter() {
        let m = parse_opts().get_matches_from(vec!["tp", "-g", r"\p{L}+"]);
        assert_eq!(
            m.get_one::<String>(GREP).map(|s| s.as_str()),
            Some(r"\p{L}+")
        );
    }

    #[test]
    fn recognizes_clap42_expire_never_keyword() {
        let m = parse_opts().get_matches_from(vec!["tp", "--expire", "never"]);
        assert_eq!(
            m.get_one::<String>(EXPIRE).map(|s| s.as_str()),
            Some("never")
        );
    }

    #[test]
    fn recognizes_clap42_sort_name_rev_positional_plain() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort", "name", "--rev", "readme.txt"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("name"));
        assert!(m.get_flag(REVERSE));
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("readme.txt")
        );
    }

    #[test]
    fn recognizes_clap42_unshift_pop_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-u", "-p"]);
        assert!(m.get_flag(UNSHIFT));
        assert!(m.get_flag(POP));
    }

    #[test]
    fn recognizes_clap42_remove_dup_move_short_chain() {
        let m = parse_opts().get_matches_from(vec!["tp", "-r", "3", "-x", "4", "-M", "5", "6"]);
        assert_eq!(m.get_one::<String>(REMOVE).map(|s| s.as_str()), Some("3"));
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("4"));
        assert_eq!(
            m.get_many(MOVE).unwrap().cloned().collect::<Vec<String>>(),
            vec!["5", "6"]
        );
    }

    #[test]
    fn recognizes_clap42_replace_three_args_dollar_signs() {
        let m = parse_opts().get_matches_from(vec!["tp", "--replace", "0", "$", "$$"]);
        let v: Vec<String> = m.get_many(REPLACE).unwrap().cloned().collect();
        assert_eq!(v, vec!["0", "$", "$$"]);
    }

    #[test]
    fn recognizes_clap42_verbose_forty_one_short() {
        let m =
            parse_opts().get_matches_from(vec!["tp", "-vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv"]);
        assert_eq!(m.get_count(VERBOSE), 41);
    }

    #[test]
    fn recognizes_clap42_double_dash_positional_query_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "--", "?foo=bar&baz=qux"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("?foo=bar&baz=qux")
        );
    }

    #[test]
    fn recognizes_clap42_tail_head_wc_long() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--tail", "0", "3", "--head", "1", "4", "--wc", "2",
        ]);
        assert_eq!(
            m.get_many::<String>(TAIL)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["0", "3"]
        );
        assert_eq!(
            m.get_many::<String>(HEAD)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["1", "4"]
        );
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("2"));
    }

    #[test]
    fn recognizes_clap42_head_tail_large_negative_line_counts() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--head", "0", "-999999", "--tail", "1", "-888888",
        ]);
        assert_eq!(
            m.get_many::<String>(HEAD)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["0", "-999999"]
        );
        assert_eq!(
            m.get_many::<String>(TAIL)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["1", "-888888"]
        );
    }

    #[test]
    fn recognizes_clap42_cat_short_twenty_six_indices() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "-C", "801", "802", "803", "804", "805", "806", "807", "808", "809", "810",
            "811", "812", "813", "814", "815", "816", "817", "818", "819", "820", "821", "822",
            "823", "824", "825", "826",
        ]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        let expected: Vec<String> = (801..=826).map(|n| n.to_string()).collect();
        let expected: Vec<&str> = expected.iter().map(String::as_str).collect();
        assert_eq!(v, expected);
    }

    #[test]
    fn recognizes_clap42_rev_sort_mtime_only_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--rev", "--sort", "mtime"]);
        assert!(m.get_flag(REVERSE));
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("mtime"));
    }

    #[test]
    fn recognizes_clap42_swap_diff_move_long_chain() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--swap", "s0", "s1", "--diff", "d0", "d1", "--mv", "m0", "m1",
        ]);
        assert_eq!(
            m.get_many(SWAP).unwrap().cloned().collect::<Vec<String>>(),
            vec!["s0", "s1"]
        );
        assert_eq!(
            m.get_many(DIFF).unwrap().cloned().collect::<Vec<String>>(),
            vec!["d0", "d1"]
        );
        assert_eq!(
            m.get_many(MOVE).unwrap().cloned().collect::<Vec<String>>(),
            vec!["m0", "m1"]
        );
    }

    #[test]
    fn recognizes_clap42_master_list_files_numbered_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-m", "-n"]);
        assert!(m.get_flag(MASTER));
        assert!(m.get_flag(LIST_FILES_NUMBERED));
    }

    #[test]
    fn recognizes_clap42_grep_wc_path_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "-g", "pat", "--wc", "0", "--path", "1"]);
        assert_eq!(m.get_one::<String>(GREP).map(|s| s.as_str()), Some("pat"));
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("0"));
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn recognizes_clap42_count_list_numbered_quiet_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-k", "-n", "-q"]);
        assert!(m.get_flag(COUNT));
        assert!(m.get_flag(LIST_FILES_NUMBERED));
        assert!(m.get_flag(SILENT));
    }

    #[test]
    fn recognizes_clap42_info_grep_edit_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-I", "8", "-g", "^BEGIN", "-e", "9"]);
        assert_eq!(m.get_one::<String>(INFO).map(|s| s.as_str()), Some("8"));
        assert_eq!(
            m.get_one::<String>(GREP).map(|s| s.as_str()),
            Some("^BEGIN")
        );
        assert_eq!(m.get_one::<String>(EDIT).map(|s| s.as_str()), Some("9"));
    }

    #[test]
    fn recognizes_clap42_positional_webex_uri() {
        let m =
            parse_opts().get_matches_from(vec!["tp", "webex://meet.example.com/meeting?key=xyz"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("webex://meet.example.com/meeting?key=xyz")
        );
    }

    #[test]
    fn recognizes_clap42_program_name_temprs_list_files() {
        let m = parse_opts().get_matches_from(vec!["temprs", "--list-files"]);
        assert!(m.get_flag(LIST_FILES));
    }

    #[test]
    fn recognizes_clap42_tag_unicode_subscript_two() {
        let m = parse_opts().get_matches_from(vec!["tp", "-w", "x\u{2082}"]);
        assert_eq!(
            m.get_one::<String>(TAG).map(|s| s.as_str()),
            Some("x\u{2082}")
        );
    }

    #[test]
    fn recognizes_clap42_clear_list_contents_quiet_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--clear", "--list-contents", "--quiet"]);
        assert!(m.get_flag(CLEAR));
        assert!(m.get_flag(LIST_CONTENTS));
        assert!(m.get_flag(SILENT));
    }

    #[test]
    fn recognizes_clap42_directory_list_files_numbered_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-d", "-n"]);
        assert!(m.get_flag(DIRECTORY));
        assert!(m.get_flag(LIST_FILES_NUMBERED));
    }

    #[test]
    fn recognizes_clap42_rename_swap_long_only() {
        let m =
            parse_opts().get_matches_from(vec!["tp", "--rename", "n0", "n1", "--swap", "a", "b"]);
        assert_eq!(
            m.get_many(RENAME)
                .unwrap()
                .cloned()
                .collect::<Vec<String>>(),
            vec!["n0", "n1"]
        );
        assert_eq!(
            m.get_many(SWAP).unwrap().cloned().collect::<Vec<String>>(),
            vec!["a", "b"]
        );
    }

    #[test]
    fn recognizes_clap42_diff_only_long_pair() {
        let m = parse_opts().get_matches_from(vec!["tp", "--diff", "L", "R"]);
        assert_eq!(
            m.get_many(DIFF).unwrap().cloned().collect::<Vec<String>>(),
            vec!["L", "R"]
        );
    }

    // ── clap coverage round 43 ─────────────────────────────

    #[test]
    fn recognizes_clap43_positional_spotify_track_uri() {
        let m = parse_opts().get_matches_from(vec!["tp", "spotify:track:6rqhFgbbKwnb9MLmUQDhG6"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("spotify:track:6rqhFgbbKwnb9MLmUQDhG6")
        );
    }

    #[test]
    fn recognizes_clap43_positional_itms_uri() {
        let m =
            parse_opts().get_matches_from(vec!["tp", "itms://itunes.apple.com/us/app/id123456789"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("itms://itunes.apple.com/us/app/id123456789")
        );
    }

    #[test]
    fn recognizes_clap43_positional_podcasts_uri() {
        let m = parse_opts().get_matches_from(vec![
            "tp",
            "podcasts://podcasts.apple.com/us/podcast/id1441470207",
        ]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("podcasts://podcasts.apple.com/us/podcast/id1441470207")
        );
    }

    #[test]
    fn recognizes_clap43_positional_smb_uri() {
        let m = parse_opts().get_matches_from(vec!["tp", "smb://nas.local/share/backup"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("smb://nas.local/share/backup")
        );
    }

    #[test]
    fn recognizes_clap43_positional_afp_uri() {
        let m = parse_opts()
            .get_matches_from(vec!["tp", "afp://time-machine._afpovertcp._tcp.local/Data"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("afp://time-machine._afpovertcp._tcp.local/Data")
        );
    }

    #[test]
    fn recognizes_clap43_output_long_equals_empty() {
        let m = parse_opts().get_matches_from(vec!["tp", "--output="]);
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some(""));
    }

    #[test]
    fn recognizes_clap43_input_long_equals_underscore_token() {
        let m = parse_opts().get_matches_from(vec!["tp", "--input=__proto__"]);
        assert_eq!(
            m.get_one::<String>(INPUT).map(|s| s.as_str()),
            Some("__proto__")
        );
    }

    #[test]
    fn recognizes_clap43_cat_thirty_eight_indices_long() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--cat", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13",
            "14", "15", "16", "17", "18", "19", "20", "21", "22", "23", "24", "25", "26", "27",
            "28", "29", "30", "31", "32", "33", "34", "35", "36", "37", "38",
        ]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        let expected: Vec<String> = (1..=38).map(|n| n.to_string()).collect();
        let expected: Vec<&str> = expected.iter().map(String::as_str).collect();
        assert_eq!(v, expected);
    }

    #[test]
    fn recognizes_clap43_grep_word_boundaries() {
        let m = parse_opts().get_matches_from(vec!["tp", "-g", r"\bfoo\b"]);
        assert_eq!(
            m.get_one::<String>(GREP).map(|s| s.as_str()),
            Some(r"\bfoo\b")
        );
    }

    #[test]
    fn recognizes_clap43_expire_relative_phrase() {
        let m = parse_opts().get_matches_from(vec!["tp", "--expire", "in 3 days"]);
        assert_eq!(
            m.get_one::<String>(EXPIRE).map(|s| s.as_str()),
            Some("in 3 days")
        );
    }

    #[test]
    fn recognizes_clap43_sort_size_rev_positional() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort", "size", "--rev", "disk.img"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("size"));
        assert!(m.get_flag(REVERSE));
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("disk.img")
        );
    }

    #[test]
    fn recognizes_clap43_shift_unshift_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-s", "-u"]);
        assert!(m.get_flag(SHIFT));
        assert!(m.get_flag(UNSHIFT));
    }

    #[test]
    fn recognizes_clap43_add_remove_swap_short_chain() {
        let m = parse_opts().get_matches_from(vec!["tp", "-a", "7", "-r", "8", "-S", "9", "10"]);
        assert_eq!(m.get_one::<String>(ADD).map(|s| s.as_str()), Some("7"));
        assert_eq!(m.get_one::<String>(REMOVE).map(|s| s.as_str()), Some("8"));
        assert_eq!(
            m.get_many(SWAP).unwrap().cloned().collect::<Vec<String>>(),
            vec!["9", "10"]
        );
    }

    #[test]
    fn recognizes_clap43_replace_three_args_backslashes() {
        let m = parse_opts().get_matches_from(vec!["tp", "--replace", "0", r"\", "/"]);
        let v: Vec<String> = m.get_many(REPLACE).unwrap().cloned().collect();
        assert_eq!(v, vec!["0", r"\", "/"]);
    }

    #[test]
    fn recognizes_clap43_verbose_forty_two_short() {
        let m = parse_opts()
            .get_matches_from(vec!["tp", "-vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv"]);
        assert_eq!(m.get_count(VERBOSE), 42);
    }

    #[test]
    fn recognizes_clap43_double_dash_positional_hash_fragment() {
        let m = parse_opts().get_matches_from(vec!["tp", "--", "#section-2"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("#section-2")
        );
    }

    #[test]
    fn recognizes_clap43_head_wc_tail_long() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--head", "0", "5", "--wc", "1", "--tail", "2", "6",
        ]);
        assert_eq!(
            m.get_many::<String>(HEAD)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["0", "5"]
        );
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("1"));
        assert_eq!(
            m.get_many::<String>(TAIL)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["2", "6"]
        );
    }

    #[test]
    fn recognizes_clap43_head_tail_u32_max_line_counts() {
        let m = parse_opts().get_matches_from(vec![
            "tp",
            "--head",
            "0",
            "4294967295",
            "--tail",
            "1",
            "4294967295",
        ]);
        assert_eq!(
            m.get_many::<String>(HEAD)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["0", "4294967295"]
        );
        assert_eq!(
            m.get_many::<String>(TAIL)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["1", "4294967295"]
        );
    }

    #[test]
    fn recognizes_clap43_cat_short_twenty_seven_indices() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "-C", "901", "902", "903", "904", "905", "906", "907", "908", "909", "910",
            "911", "912", "913", "914", "915", "916", "917", "918", "919", "920", "921", "922",
            "923", "924", "925", "926", "927",
        ]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        let expected: Vec<String> = (901..=927).map(|n| n.to_string()).collect();
        let expected: Vec<&str> = expected.iter().map(String::as_str).collect();
        assert_eq!(v, expected);
    }

    #[test]
    fn recognizes_clap43_rev_sort_size_only_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--rev", "--sort", "size"]);
        assert!(m.get_flag(REVERSE));
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("size"));
    }

    #[test]
    fn recognizes_clap43_dup_diff_move_long_chain() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--dup", "0", "--diff", "a", "b", "--mv", "c", "d",
        ]);
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("0"));
        assert_eq!(
            m.get_many(DIFF).unwrap().cloned().collect::<Vec<String>>(),
            vec!["a", "b"]
        );
        assert_eq!(
            m.get_many(MOVE).unwrap().cloned().collect::<Vec<String>>(),
            vec!["c", "d"]
        );
    }

    #[test]
    fn recognizes_clap43_master_list_contents_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-m", "-L"]);
        assert!(m.get_flag(MASTER));
        assert!(m.get_flag(LIST_CONTENTS));
    }

    #[test]
    fn recognizes_clap43_rev_path_wc_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--rev", "--path", "4", "--wc", "5"]);
        assert!(m.get_flag(REVERSE));
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("4"));
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("5"));
    }

    #[test]
    fn recognizes_clap43_count_directory_quiet_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-k", "-d", "-q"]);
        assert!(m.get_flag(COUNT));
        assert!(m.get_flag(DIRECTORY));
        assert!(m.get_flag(SILENT));
    }

    #[test]
    fn recognizes_clap43_info_edit_cat_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-I", "3", "-e", "4", "-C", "5", "6"]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(v, vec!["5", "6"]);
        assert_eq!(m.get_one::<String>(INFO).map(|s| s.as_str()), Some("3"));
        assert_eq!(m.get_one::<String>(EDIT).map(|s| s.as_str()), Some("4"));
    }

    #[test]
    fn recognizes_clap43_positional_tel_e164_dots() {
        let m = parse_opts().get_matches_from(vec!["tp", "tel:+1.415.555.2671"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("tel:+1.415.555.2671")
        );
    }

    #[test]
    fn recognizes_clap43_program_name_temprs_pop() {
        let m = parse_opts().get_matches_from(vec!["temprs", "-p"]);
        assert!(m.get_flag(POP));
    }

    #[test]
    fn recognizes_clap43_tag_nary_summation() {
        let m = parse_opts().get_matches_from(vec!["tp", "-w", "\u{2211}k"]);
        assert_eq!(
            m.get_one::<String>(TAG).map(|s| s.as_str()),
            Some("\u{2211}k")
        );
    }

    #[test]
    fn recognizes_clap43_clear_list_files_quiet_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-c", "-l", "-q"]);
        assert!(m.get_flag(CLEAR));
        assert!(m.get_flag(LIST_FILES));
        assert!(m.get_flag(SILENT));
    }

    #[test]
    fn recognizes_clap43_directory_list_contents_numbered_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-d", "-N"]);
        assert!(m.get_flag(DIRECTORY));
        assert!(m.get_flag(LIST_CONTENTS_NUMBERED));
    }

    #[test]
    fn recognizes_clap43_move_diff_long_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "--mv", "i", "j", "--diff", "k", "l"]);
        assert_eq!(
            m.get_many(MOVE).unwrap().cloned().collect::<Vec<String>>(),
            vec!["i", "j"]
        );
        assert_eq!(
            m.get_many(DIFF).unwrap().cloned().collect::<Vec<String>>(),
            vec!["k", "l"]
        );
    }

    #[test]
    fn recognizes_clap43_grep_tail_size_long() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--grep", "ERR", "--tail", "0", "9", "--size", "1",
        ]);
        assert_eq!(m.get_one::<String>(GREP).map(|s| s.as_str()), Some("ERR"));
        assert_eq!(
            m.get_many::<String>(TAIL)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["0", "9"]
        );
        assert_eq!(m.get_one::<String>(SIZE).map(|s| s.as_str()), Some("1"));
    }

    // ── clap coverage round 44 ─────────────────────────────

    #[test]
    fn recognizes_clap44_positional_intent_uri() {
        let m = parse_opts().get_matches_from(vec![
            "tp",
            "intent://scan/#Intent;scheme=zxing;package=com.google.zxing.client.android;end",
        ]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("intent://scan/#Intent;scheme=zxing;package=com.google.zxing.client.android;end")
        );
    }

    #[test]
    fn recognizes_clap44_positional_market_uri() {
        let m = parse_opts().get_matches_from(vec!["tp", "market://details?id=com.example.app"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("market://details?id=com.example.app")
        );
    }

    #[test]
    fn recognizes_clap44_positional_sms_uri() {
        let m = parse_opts().get_matches_from(vec!["tp", "sms:+15550100?body=hello%20world"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("sms:+15550100?body=hello%20world")
        );
    }

    #[test]
    fn recognizes_clap44_positional_comgooglemaps_uri() {
        let m = parse_opts()
            .get_matches_from(vec!["tp", "comgooglemaps://?center=37.78,-122.42&zoom=14"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("comgooglemaps://?center=37.78,-122.42&zoom=14")
        );
    }

    #[test]
    fn recognizes_clap44_positional_geo_uri() {
        let m = parse_opts().get_matches_from(vec!["tp", "geo:37.78,-122.42?q=San+Francisco"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("geo:37.78,-122.42?q=San+Francisco")
        );
    }

    #[test]
    fn recognizes_clap44_output_long_equals_tab() {
        let m = parse_opts().get_matches_from(vec!["tp", "--output=\t"]);
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("\t"));
    }

    #[test]
    fn recognizes_clap44_input_long_equals_embedded_spaces() {
        let m = parse_opts().get_matches_from(vec!["tp", "--input=with space"]);
        assert_eq!(
            m.get_one::<String>(INPUT).map(|s| s.as_str()),
            Some("with space")
        );
    }

    #[test]
    fn recognizes_clap44_cat_thirty_nine_indices_long() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--cat", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13",
            "14", "15", "16", "17", "18", "19", "20", "21", "22", "23", "24", "25", "26", "27",
            "28", "29", "30", "31", "32", "33", "34", "35", "36", "37", "38", "39",
        ]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        let expected: Vec<String> = (1..=39).map(|n| n.to_string()).collect();
        let expected: Vec<&str> = expected.iter().map(String::as_str).collect();
        assert_eq!(v, expected);
    }

    #[test]
    fn recognizes_clap44_grep_lookbehind() {
        let m = parse_opts().get_matches_from(vec!["tp", "-g", r"(?<=prefix)foo"]);
        assert_eq!(
            m.get_one::<String>(GREP).map(|s| s.as_str()),
            Some(r"(?<=prefix)foo")
        );
    }

    #[test]
    fn recognizes_clap44_expire_iso8601_duration() {
        let m = parse_opts().get_matches_from(vec!["tp", "--expire", "P1DT12H"]);
        assert_eq!(
            m.get_one::<String>(EXPIRE).map(|s| s.as_str()),
            Some("P1DT12H")
        );
    }

    #[test]
    fn recognizes_clap44_sort_mtime_positional_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "--sort", "mtime", "backup.tgz"]);
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("mtime"));
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("backup.tgz")
        );
    }

    #[test]
    fn recognizes_clap44_pop_shift_unshift_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-p", "-s", "-u"]);
        assert!(m.get_flag(POP));
        assert!(m.get_flag(SHIFT));
        assert!(m.get_flag(UNSHIFT));
    }

    #[test]
    fn recognizes_clap44_append_remove_dup_short_chain() {
        let m = parse_opts().get_matches_from(vec!["tp", "-A", "0", "-r", "1", "-x", "2"]);
        assert_eq!(m.get_one::<String>(APPEND).map(|s| s.as_str()), Some("0"));
        assert_eq!(m.get_one::<String>(REMOVE).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(DUP).map(|s| s.as_str()), Some("2"));
    }

    #[test]
    fn recognizes_clap44_replace_three_args_cjk() {
        let m = parse_opts().get_matches_from(vec!["tp", "--replace", "0", "旧", "新"]);
        let v: Vec<String> = m.get_many(REPLACE).unwrap().cloned().collect();
        assert_eq!(v, vec!["0", "旧", "新"]);
    }

    #[test]
    fn recognizes_clap44_verbose_forty_three_short() {
        let m = parse_opts()
            .get_matches_from(vec!["tp", "-vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv"]);
        assert_eq!(m.get_count(VERBOSE), 43);
    }

    #[test]
    fn recognizes_clap44_double_dash_positional_at_mention() {
        let m = parse_opts().get_matches_from(vec!["tp", "--", "@team/alerts"]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("@team/alerts")
        );
    }

    #[test]
    fn recognizes_clap44_size_wc_path_long() {
        let m =
            parse_opts().get_matches_from(vec!["tp", "--size", "6", "--wc", "7", "--path", "8"]);
        assert_eq!(m.get_one::<String>(SIZE).map(|s| s.as_str()), Some("6"));
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("7"));
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("8"));
    }

    #[test]
    fn recognizes_clap44_tail_head_wc_long() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--tail", "0", "2", "--head", "1", "3", "--wc", "4",
        ]);
        assert_eq!(
            m.get_many::<String>(TAIL)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["0", "2"]
        );
        assert_eq!(
            m.get_many::<String>(HEAD)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["1", "3"]
        );
        assert_eq!(m.get_one::<String>(WC).map(|s| s.as_str()), Some("4"));
    }

    #[test]
    fn recognizes_clap44_cat_short_twenty_eight_indices() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "-C", "1001", "1002", "1003", "1004", "1005", "1006", "1007", "1008", "1009",
            "1010", "1011", "1012", "1013", "1014", "1015", "1016", "1017", "1018", "1019", "1020",
            "1021", "1022", "1023", "1024", "1025", "1026", "1027", "1028",
        ]);
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        let expected: Vec<String> = (1001..=1028).map(|n| n.to_string()).collect();
        let expected: Vec<&str> = expected.iter().map(String::as_str).collect();
        assert_eq!(v, expected);
    }

    #[test]
    fn recognizes_clap44_rev_sort_name_only_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--rev", "--sort", "name"]);
        assert!(m.get_flag(REVERSE));
        assert_eq!(m.get_one::<String>(SORT).map(|s| s.as_str()), Some("name"));
    }

    #[test]
    fn recognizes_clap44_swap_move_diff_long_chain() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "--swap", "a", "b", "--mv", "c", "d", "--diff", "e", "f",
        ]);
        assert_eq!(
            m.get_many(SWAP).unwrap().cloned().collect::<Vec<String>>(),
            vec!["a", "b"]
        );
        assert_eq!(
            m.get_many(MOVE).unwrap().cloned().collect::<Vec<String>>(),
            vec!["c", "d"]
        );
        assert_eq!(
            m.get_many(DIFF).unwrap().cloned().collect::<Vec<String>>(),
            vec!["e", "f"]
        );
    }

    #[test]
    fn recognizes_clap44_master_list_files_numbered_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-m", "-n"]);
        assert!(m.get_flag(MASTER));
        assert!(m.get_flag(LIST_FILES_NUMBERED));
    }

    #[test]
    fn recognizes_clap44_grep_path_tail_long() {
        let m = parse_opts()
            .get_matches_from(vec!["tp", "-g", "pat", "--path", "0", "--tail", "1", "50"]);
        assert_eq!(m.get_one::<String>(GREP).map(|s| s.as_str()), Some("pat"));
        assert_eq!(m.get_one::<String>(PATH).map(|s| s.as_str()), Some("0"));
        assert_eq!(
            m.get_many::<String>(TAIL)
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["1", "50"]
        );
    }

    #[test]
    fn recognizes_clap44_count_list_contents_quiet_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-k", "-L", "-q"]);
        assert!(m.get_flag(COUNT));
        assert!(m.get_flag(LIST_CONTENTS));
        assert!(m.get_flag(SILENT));
    }

    #[test]
    fn recognizes_clap44_info_grep_cat_short() {
        let m =
            parse_opts().get_matches_from(vec!["tp", "-I", "2", "-g", "(?m)^#", "-C", "3", "4"]);
        assert_eq!(m.get_one::<String>(INFO).map(|s| s.as_str()), Some("2"));
        assert_eq!(
            m.get_one::<String>(GREP).map(|s| s.as_str()),
            Some("(?m)^#")
        );
        let v: Vec<&str> = m
            .get_many::<String>(CAT)
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(v, vec!["3", "4"]);
    }

    #[test]
    fn recognizes_clap44_positional_long_https_url() {
        let m = parse_opts().get_matches_from(vec![
            "tp",
            "https://crates.io/crates/tokio/versions?page=2&sort=semver#all-features",
        ]);
        assert_eq!(
            m.get_one::<String>(ARGFILE).map(|s| s.as_str()),
            Some("https://crates.io/crates/tokio/versions?page=2&sort=semver#all-features")
        );
    }

    #[test]
    fn recognizes_clap44_program_name_temprs_unshift() {
        let m = parse_opts().get_matches_from(vec!["temprs", "--unshift"]);
        assert!(m.get_flag(UNSHIFT));
    }

    #[test]
    fn recognizes_clap44_tag_vulgar_fraction_one_half() {
        let m = parse_opts().get_matches_from(vec!["tp", "-w", "\u{00bd}"]);
        assert_eq!(
            m.get_one::<String>(TAG).map(|s| s.as_str()),
            Some("\u{00bd}")
        );
    }

    #[test]
    fn recognizes_clap44_clear_directory_list_files_long() {
        let m = parse_opts().get_matches_from(vec!["tp", "--clear", "--dir", "--list-files"]);
        assert!(m.get_flag(CLEAR));
        assert!(m.get_flag(DIRECTORY));
        assert!(m.get_flag(LIST_FILES));
    }

    #[test]
    fn recognizes_clap44_directory_list_contents_numbered_short() {
        let m = parse_opts().get_matches_from(vec!["tp", "-d", "-N"]);
        assert!(m.get_flag(DIRECTORY));
        assert!(m.get_flag(LIST_CONTENTS_NUMBERED));
    }

    #[test]
    fn recognizes_clap44_rename_move_long_only() {
        let m = parse_opts().get_matches_from(vec!["tp", "--rename", "p", "q", "--mv", "r", "s"]);
        assert_eq!(
            m.get_many(RENAME)
                .unwrap()
                .cloned()
                .collect::<Vec<String>>(),
            vec!["p", "q"]
        );
        assert_eq!(
            m.get_many(MOVE).unwrap().cloned().collect::<Vec<String>>(),
            vec!["r", "s"]
        );
    }

    #[test]
    fn recognizes_clap44_diff_swap_short_chain() {
        let m = parse_opts().get_matches_from(vec!["tp", "-D", "0", "1", "-S", "2", "3"]);
        assert_eq!(
            m.get_many(DIFF).unwrap().cloned().collect::<Vec<String>>(),
            vec!["0", "1"]
        );
        assert_eq!(
            m.get_many(SWAP).unwrap().cloned().collect::<Vec<String>>(),
            vec!["2", "3"]
        );
    }
}
