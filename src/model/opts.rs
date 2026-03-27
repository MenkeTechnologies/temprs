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
 "\x1b[36m │ STATUS: ONLINE  // SIGNAL: ████████░░ // v", env!("CARGO_PKG_VERSION"), "\x1b[36m   │\x1b[0m\n",
 "\x1b[36m └──────────────────────────────────────────────────────┘\x1b[0m\n",
 "\x1b[35m  >> TEMPORARY FILE STACK MANAGER // FULL SPECTRUM <<\x1b[0m");

const AFTER: &str = concat!(
"\x1b[36m  ── SYSTEM ─────────────────────────────────────────\x1b[0m\n",
"\x1b[35m  v", env!("CARGO_PKG_VERSION"), " \x1b[0m// \x1b[33m(c) Jacob Menke and contributors\x1b[0m\n",
"\x1b[35m  The stack is vast and infinite.\x1b[0m\n",
"\x1b[33m  >>> JACK IN. PUSH YOUR DATA. OWN YOUR TEMP FILES. <<<\x1b[0m\n",
"\x1b[36m ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░\x1b[0m");

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
            .help("\x1b[32m//\x1b[0m Push to bottom of stack"))
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
        assert_eq!(m.get_one::<String>(ARGFILE).map(|s| s.as_str()), Some("myfile.txt"));
    }

    #[test]
    fn positional_argfile_with_path() {
        let m = parse_opts().get_matches_from(vec!["tp", "/tmp/data.txt"]);
        assert_eq!(m.get_one::<String>(ARGFILE).map(|s| s.as_str()), Some("/tmp/data.txt"));
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
        let arg_ids: Vec<&str> = app.get_arguments()
            .map(|a| a.get_id().as_str())
            .collect();
        for expected in &[
            INPUT, OUTPUT, ADD, REMOVE, POP, UNSHIFT, SHIFT,
            ARGFILE, DIRECTORY, MASTER, LIST_FILES, LIST_FILES_NUMBERED,
            LIST_CONTENTS, LIST_CONTENTS_NUMBERED, SILENT, CLEAR, VERBOSE,
        ] {
            assert!(arg_ids.contains(expected), "missing arg: {}", expected);
        }
    }

    #[test]
    fn command_has_17_custom_args() {
        let app = parse_opts();
        // clap adds --help and --version automatically
        let custom_count = app.get_arguments()
            .filter(|a| a.get_id() != "help" && a.get_id() != "version")
            .count();
        assert_eq!(custom_count, 29);
    }

    // ── flag mutual independence ────────────────────────

    #[test]
    fn all_boolean_flags_independent() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "-p", "-u", "-s", "-d", "-m", "-l", "-n",
            "-L", "-N", "-q", "-c", "-v",
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
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("-999"));
    }

    #[test]
    fn add_accepts_negative_large() {
        let m = parse_opts().get_matches_from(vec!["tp", "-a", "-100"]);
        assert_eq!(m.get_one::<String>(ADD).map(|s| s.as_str()), Some("-100"));
    }

    #[test]
    fn remove_accepts_large_value() {
        let m = parse_opts().get_matches_from(vec!["tp", "-r", "99999"]);
        assert_eq!(m.get_one::<String>(REMOVE).map(|s| s.as_str()), Some("99999"));
    }

    // ── positional with flags combined ──────────────────

    #[test]
    fn positional_with_pop() {
        let m = parse_opts().get_matches_from(vec!["tp", "-p", "somefile"]);
        assert!(m.get_flag(POP));
        assert_eq!(m.get_one::<String>(ARGFILE).map(|s| s.as_str()), Some("somefile"));
    }

    #[test]
    fn positional_with_verbose() {
        let m = parse_opts().get_matches_from(vec!["tp", "-v", "myfile"]);
        assert!(m.get_count(VERBOSE) > 0);
        assert_eq!(m.get_one::<String>(ARGFILE).map(|s| s.as_str()), Some("myfile"));
    }

    #[test]
    fn positional_with_quiet() {
        let m = parse_opts().get_matches_from(vec!["tp", "-q", "file.txt"]);
        assert!(m.get_flag(SILENT));
        assert_eq!(m.get_one::<String>(ARGFILE).map(|s| s.as_str()), Some("file.txt"));
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
        assert_eq!(m.get_one::<String>(ARGFILE).map(|s| s.as_str()), Some("--not-a-flag"));
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
        let m = parse_opts().get_matches_from(vec![
            "tp", "-i", "1", "-o", "2", "-a", "3", "-r", "4",
        ]);
        assert_eq!(m.get_one::<String>(INPUT).map(|s| s.as_str()), Some("1"));
        assert_eq!(m.get_one::<String>(OUTPUT).map(|s| s.as_str()), Some("2"));
        assert_eq!(m.get_one::<String>(ADD).map(|s| s.as_str()), Some("3"));
        assert_eq!(m.get_one::<String>(REMOVE).map(|s| s.as_str()), Some("4"));
    }

    #[test]
    fn value_args_with_boolean_flags() {
        let m = parse_opts().get_matches_from(vec![
            "tp", "-i", "1", "-q", "-v", "-d",
        ]);
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
        assert_eq!(m.get_one::<String>(ARGFILE).map(|s| s.as_str()), Some("my file.txt"));
    }

    #[test]
    fn positional_with_unicode() {
        let m = parse_opts().get_matches_from(vec!["tp", "日本語.txt"]);
        assert_eq!(m.get_one::<String>(ARGFILE).map(|s| s.as_str()), Some("日本語.txt"));
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
        assert_eq!(m.get_one::<String>(ARGFILE).map(|s| s.as_str()), Some(".hidden"));
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
}
