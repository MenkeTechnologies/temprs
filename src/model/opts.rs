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
        .arg(Arg::new(VERBOSE)
            .short('v')
            .long("verbose")
            .action(ArgAction::Count)
            .help("\x1b[32m//\x1b[0m Increase verbosity level"))
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
}
