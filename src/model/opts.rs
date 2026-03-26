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

#[cfg(test)]
mod tests {
    use super::*;

    // ── app construction ─────────────────────────────────

    #[test]
    fn parse_opts_returns_app() {
        let app = parse_opts();
        assert_eq!(app.p.meta.name, NAME);
    }

    // ── flag recognition ─────────────────────────────────

    #[test]
    fn recognizes_input_short() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "-i", "3"]);
        assert_eq!(m.value_of(INPUT), Some("3"));
    }

    #[test]
    fn recognizes_input_long() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "--input", "5"]);
        assert_eq!(m.value_of(INPUT), Some("5"));
    }

    #[test]
    fn recognizes_output_short() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "-o", "2"]);
        assert_eq!(m.value_of(OUTPUT), Some("2"));
    }

    #[test]
    fn recognizes_output_long() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "--output", "7"]);
        assert_eq!(m.value_of(OUTPUT), Some("7"));
    }

    #[test]
    fn recognizes_output_negative_index() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "-o", "-1"]);
        assert_eq!(m.value_of(OUTPUT), Some("-1"));
    }

    #[test]
    fn recognizes_add_short() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "-a", "4"]);
        assert_eq!(m.value_of(ADD), Some("4"));
    }

    #[test]
    fn recognizes_add_long() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "--add", "4"]);
        assert_eq!(m.value_of(ADD), Some("4"));
    }

    #[test]
    fn recognizes_add_negative_index() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "-a", "-2"]);
        assert_eq!(m.value_of(ADD), Some("-2"));
    }

    #[test]
    fn recognizes_remove_short() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "-r", "1"]);
        assert_eq!(m.value_of(REMOVE), Some("1"));
    }

    #[test]
    fn recognizes_remove_long() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "--remove", "1"]);
        assert_eq!(m.value_of(REMOVE), Some("1"));
    }

    #[test]
    fn recognizes_remove_negative_index() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "-r", "-3"]);
        assert_eq!(m.value_of(REMOVE), Some("-3"));
    }

    // ── boolean flags ────────────────────────────────────

    #[test]
    fn recognizes_pop_short() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "-p"]);
        assert!(m.is_present(POP));
    }

    #[test]
    fn recognizes_pop_long() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "--pop"]);
        assert!(m.is_present(POP));
    }

    #[test]
    fn recognizes_unshift_short() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "-u"]);
        assert!(m.is_present(UNSHIFT));
    }

    #[test]
    fn recognizes_unshift_long() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "--unshift"]);
        assert!(m.is_present(UNSHIFT));
    }

    #[test]
    fn recognizes_shift_short() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "-s"]);
        assert!(m.is_present(SHIFT));
    }

    #[test]
    fn recognizes_shift_long() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "--shift"]);
        assert!(m.is_present(SHIFT));
    }

    #[test]
    fn recognizes_dir_short() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "-d"]);
        assert!(m.is_present(DIRECTORY));
    }

    #[test]
    fn recognizes_dir_long() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "--dir"]);
        assert!(m.is_present(DIRECTORY));
    }

    #[test]
    fn recognizes_master_short() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "-m"]);
        assert!(m.is_present(MASTER));
    }

    #[test]
    fn recognizes_master_long() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "--master"]);
        assert!(m.is_present(MASTER));
    }

    #[test]
    fn recognizes_list_files_short() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "-l"]);
        assert!(m.is_present(LIST_FILES));
    }

    #[test]
    fn recognizes_list_files_long() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "--list-files"]);
        assert!(m.is_present(LIST_FILES));
    }

    #[test]
    fn recognizes_list_files_numbered_short() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "-n"]);
        assert!(m.is_present(LIST_FILES_NUMBERED));
    }

    #[test]
    fn recognizes_list_files_numbered_long() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "--list-files-numbered"]);
        assert!(m.is_present(LIST_FILES_NUMBERED));
    }

    #[test]
    fn recognizes_list_contents_short() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "-L"]);
        assert!(m.is_present(LIST_CONTENTS));
    }

    #[test]
    fn recognizes_list_contents_long() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "--list-contents"]);
        assert!(m.is_present(LIST_CONTENTS));
    }

    #[test]
    fn recognizes_list_contents_numbered_short() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "-N"]);
        assert!(m.is_present(LIST_CONTENTS_NUMBERED));
    }

    #[test]
    fn recognizes_list_contents_numbered_long() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "--list-contents-numbered"]);
        assert!(m.is_present(LIST_CONTENTS_NUMBERED));
    }

    #[test]
    fn recognizes_quiet_short() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "-q"]);
        assert!(m.is_present(SILENT));
    }

    #[test]
    fn recognizes_quiet_long() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "--quiet"]);
        assert!(m.is_present(SILENT));
    }

    #[test]
    fn recognizes_clear_short() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "-c"]);
        assert!(m.is_present(CLEAR));
    }

    #[test]
    fn recognizes_clear_long() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "--clear"]);
        assert!(m.is_present(CLEAR));
    }

    #[test]
    fn recognizes_verbose_short() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "-v"]);
        assert!(m.is_present(VERBOSE));
    }

    #[test]
    fn recognizes_verbose_long() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "--verbose"]);
        assert!(m.is_present(VERBOSE));
    }

    // ── defaults (no args) ──────────────────────────────

    #[test]
    fn no_args_all_absent() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp"]);
        assert!(!m.is_present(POP));
        assert!(!m.is_present(SHIFT));
        assert!(!m.is_present(UNSHIFT));
        assert!(!m.is_present(DIRECTORY));
        assert!(!m.is_present(MASTER));
        assert!(!m.is_present(LIST_FILES));
        assert!(!m.is_present(LIST_FILES_NUMBERED));
        assert!(!m.is_present(LIST_CONTENTS));
        assert!(!m.is_present(LIST_CONTENTS_NUMBERED));
        assert!(!m.is_present(SILENT));
        assert!(!m.is_present(CLEAR));
        assert!(!m.is_present(VERBOSE));
        assert!(m.value_of(INPUT).is_none());
        assert!(m.value_of(OUTPUT).is_none());
        assert!(m.value_of(ADD).is_none());
        assert!(m.value_of(REMOVE).is_none());
        assert!(m.value_of(ARGFILE).is_none());
    }

    // ── positional argument ─────────────────────────────

    #[test]
    fn recognizes_positional_argfile() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "myfile.txt"]);
        assert_eq!(m.value_of(ARGFILE), Some("myfile.txt"));
    }

    #[test]
    fn positional_argfile_with_path() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "/tmp/data.txt"]);
        assert_eq!(m.value_of(ARGFILE), Some("/tmp/data.txt"));
    }

    // ── combined flags ──────────────────────────────────

    #[test]
    fn combined_verbose_and_quiet() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "-v", "-q"]);
        assert!(m.is_present(VERBOSE));
        assert!(m.is_present(SILENT));
    }

    #[test]
    fn input_with_quiet() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "-i", "1", "-q"]);
        assert_eq!(m.value_of(INPUT), Some("1"));
        assert!(m.is_present(SILENT));
    }

    #[test]
    fn output_with_verbose() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "-o", "2", "-v"]);
        assert_eq!(m.value_of(OUTPUT), Some("2"));
        assert!(m.is_present(VERBOSE));
    }

    // ── invalid args ────────────────────────────────────

    #[test]
    fn unknown_flag_errors() {
        let app = parse_opts();
        let result = app.get_matches_from_safe(vec!["tp", "--nonexistent"]);
        assert!(result.is_err());
    }

    #[test]
    fn input_without_value_errors() {
        let app = parse_opts();
        let result = app.get_matches_from_safe(vec!["tp", "-i"]);
        assert!(result.is_err());
    }

    #[test]
    fn output_without_value_errors() {
        let app = parse_opts();
        let result = app.get_matches_from_safe(vec!["tp", "-o"]);
        assert!(result.is_err());
    }

    #[test]
    fn add_without_value_errors() {
        let app = parse_opts();
        let result = app.get_matches_from_safe(vec!["tp", "-a"]);
        assert!(result.is_err());
    }

    #[test]
    fn remove_without_value_errors() {
        let app = parse_opts();
        let result = app.get_matches_from_safe(vec!["tp", "-r"]);
        assert!(result.is_err());
    }

    // ── verbose multiple ────────────────────────────────

    #[test]
    fn verbose_counts_occurrences() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "-vvv"]);
        assert_eq!(m.occurrences_of(VERBOSE), 3);
    }

    #[test]
    fn verbose_single_occurrence() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "-v"]);
        assert_eq!(m.occurrences_of(VERBOSE), 1);
    }

    // ── value-taking args accept various strings ────────

    #[test]
    fn input_accepts_large_index() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "-i", "999"]);
        assert_eq!(m.value_of(INPUT), Some("999"));
    }

    #[test]
    fn output_accepts_negative() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "-o", "-10"]);
        assert_eq!(m.value_of(OUTPUT), Some("-10"));
    }

    #[test]
    fn remove_accepts_negative() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "-r", "-1"]);
        assert_eq!(m.value_of(REMOVE), Some("-1"));
    }

    // ── flag combination tests ─────────────────────────────

    #[test]
    fn all_boolean_flags_together() {
        let app = parse_opts();
        let m = app.get_matches_from(vec![
            "tp", "-p", "-u", "-s", "-d", "-m", "-l", "-n", "-L", "-N", "-q", "-c", "-v",
        ]);
        assert!(m.is_present(POP));
        assert!(m.is_present(UNSHIFT));
        assert!(m.is_present(SHIFT));
        assert!(m.is_present(DIRECTORY));
        assert!(m.is_present(MASTER));
        assert!(m.is_present(LIST_FILES));
        assert!(m.is_present(LIST_FILES_NUMBERED));
        assert!(m.is_present(LIST_CONTENTS));
        assert!(m.is_present(LIST_CONTENTS_NUMBERED));
        assert!(m.is_present(SILENT));
        assert!(m.is_present(CLEAR));
        assert!(m.is_present(VERBOSE));
    }

    #[test]
    fn pop_with_list() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "-p", "-l"]);
        assert!(m.is_present(POP));
        assert!(m.is_present(LIST_FILES));
    }

    #[test]
    fn shift_with_list_numbered() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "-s", "-n"]);
        assert!(m.is_present(SHIFT));
        assert!(m.is_present(LIST_FILES_NUMBERED));
    }

    #[test]
    fn unshift_with_verbose() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "-u", "-v"]);
        assert!(m.is_present(UNSHIFT));
        assert!(m.is_present(VERBOSE));
    }

    #[test]
    fn clear_with_quiet() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "-c", "-q"]);
        assert!(m.is_present(CLEAR));
        assert!(m.is_present(SILENT));
    }

    #[test]
    fn dir_and_master_together() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "-d", "-m"]);
        assert!(m.is_present(DIRECTORY));
        assert!(m.is_present(MASTER));
    }

    #[test]
    fn list_files_and_list_contents() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "-l", "-L"]);
        assert!(m.is_present(LIST_FILES));
        assert!(m.is_present(LIST_CONTENTS));
    }

    #[test]
    fn list_numbered_both() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "-n", "-N"]);
        assert!(m.is_present(LIST_FILES_NUMBERED));
        assert!(m.is_present(LIST_CONTENTS_NUMBERED));
    }

    // ── value args with boolean flags ──────────────────────

    #[test]
    fn input_with_pop() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "-i", "3", "-p"]);
        assert_eq!(m.value_of(INPUT), Some("3"));
        assert!(m.is_present(POP));
    }

    #[test]
    fn output_with_shift() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "-o", "2", "-s"]);
        assert_eq!(m.value_of(OUTPUT), Some("2"));
        assert!(m.is_present(SHIFT));
    }

    #[test]
    fn add_with_unshift() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "-a", "1", "-u"]);
        assert_eq!(m.value_of(ADD), Some("1"));
        assert!(m.is_present(UNSHIFT));
    }

    #[test]
    fn remove_with_verbose() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "-r", "5", "-v"]);
        assert_eq!(m.value_of(REMOVE), Some("5"));
        assert!(m.is_present(VERBOSE));
    }

    #[test]
    fn input_output_together() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "-i", "1", "-o", "2"]);
        assert_eq!(m.value_of(INPUT), Some("1"));
        assert_eq!(m.value_of(OUTPUT), Some("2"));
    }

    #[test]
    fn add_remove_together() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "-a", "3", "-r", "4"]);
        assert_eq!(m.value_of(ADD), Some("3"));
        assert_eq!(m.value_of(REMOVE), Some("4"));
    }

    #[test]
    fn all_value_args_together() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "-i", "1", "-o", "2", "-a", "3", "-r", "4"]);
        assert_eq!(m.value_of(INPUT), Some("1"));
        assert_eq!(m.value_of(OUTPUT), Some("2"));
        assert_eq!(m.value_of(ADD), Some("3"));
        assert_eq!(m.value_of(REMOVE), Some("4"));
    }

    // ── positional with flags ──────────────────────────────

    #[test]
    fn argfile_with_verbose() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "myfile.txt", "-v"]);
        assert_eq!(m.value_of(ARGFILE), Some("myfile.txt"));
        assert!(m.is_present(VERBOSE));
    }

    #[test]
    fn argfile_with_quiet() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "-q", "myfile.txt"]);
        assert!(m.is_present(SILENT));
        assert_eq!(m.value_of(ARGFILE), Some("myfile.txt"));
    }

    #[test]
    fn argfile_with_input() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "-i", "1", "myfile.txt"]);
        assert_eq!(m.value_of(INPUT), Some("1"));
        assert_eq!(m.value_of(ARGFILE), Some("myfile.txt"));
    }

    #[test]
    fn argfile_with_output_and_verbose() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "-o", "3", "-v", "myfile.txt"]);
        assert_eq!(m.value_of(OUTPUT), Some("3"));
        assert!(m.is_present(VERBOSE));
        assert_eq!(m.value_of(ARGFILE), Some("myfile.txt"));
    }

    // ── edge case values ───────────────────────────────────

    #[test]
    fn input_accepts_zero() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "-i", "0"]);
        assert_eq!(m.value_of(INPUT), Some("0"));
    }

    #[test]
    fn output_accepts_zero() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "-o", "0"]);
        assert_eq!(m.value_of(OUTPUT), Some("0"));
    }

    #[test]
    fn add_accepts_zero() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "-a", "0"]);
        assert_eq!(m.value_of(ADD), Some("0"));
    }

    #[test]
    fn remove_accepts_zero() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "-r", "0"]);
        assert_eq!(m.value_of(REMOVE), Some("0"));
    }

    #[test]
    fn add_accepts_large_negative() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "-a", "-100"]);
        assert_eq!(m.value_of(ADD), Some("-100"));
    }

    // ── long flag forms with values ────────────────────────

    #[test]
    fn input_long_with_equals() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "--input=7"]);
        assert_eq!(m.value_of(INPUT), Some("7"));
    }

    #[test]
    fn output_long_with_equals() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "--output=3"]);
        assert_eq!(m.value_of(OUTPUT), Some("3"));
    }

    #[test]
    fn add_long_with_value() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "--add", "10"]);
        assert_eq!(m.value_of(ADD), Some("10"));
    }

    #[test]
    fn remove_long_with_value() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "--remove", "8"]);
        assert_eq!(m.value_of(REMOVE), Some("8"));
    }

    // ── multiple verbose levels ────────────────────────────

    #[test]
    fn verbose_four_times() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "-vvvv"]);
        assert_eq!(m.occurrences_of(VERBOSE), 4);
    }

    #[test]
    fn verbose_separated() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "-v", "-v", "-v"]);
        assert_eq!(m.occurrences_of(VERBOSE), 3);
    }

    #[test]
    fn verbose_mixed_short_long() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "-v", "--verbose"]);
        assert_eq!(m.occurrences_of(VERBOSE), 2);
    }

    // ── invalid args (additional) ──────────────────────────

    #[test]
    fn double_dash_stops_flags() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "--", "-v"]);
        assert_eq!(m.value_of(ARGFILE), Some("-v"));
        assert!(!m.is_present(VERBOSE));
    }

    #[test]
    fn extra_positional_errors() {
        let app = parse_opts();
        let result = app.get_matches_from_safe(vec!["tp", "file1", "file2"]);
        assert!(result.is_err());
    }

    // ── no args edge cases ─────────────────────────────────

    #[test]
    fn no_args_argfile_absent() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp"]);
        assert!(m.value_of(ARGFILE).is_none());
    }

    #[test]
    fn no_args_input_absent() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp"]);
        assert!(m.value_of(INPUT).is_none());
    }

    #[test]
    fn no_args_output_absent() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp"]);
        assert!(m.value_of(OUTPUT).is_none());
    }

    #[test]
    fn no_args_add_absent() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp"]);
        assert!(m.value_of(ADD).is_none());
    }

    #[test]
    fn no_args_remove_absent() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp"]);
        assert!(m.value_of(REMOVE).is_none());
    }

    // ── flag with argfile ordering ─────────────────────────

    #[test]
    fn flags_before_argfile() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "-v", "-q", "myfile.txt"]);
        assert!(m.is_present(VERBOSE));
        assert!(m.is_present(SILENT));
        assert_eq!(m.value_of(ARGFILE), Some("myfile.txt"));
    }

    #[test]
    fn flags_after_argfile() {
        let app = parse_opts();
        let m = app.get_matches_from(vec!["tp", "myfile.txt", "-v", "-q"]);
        assert!(m.is_present(VERBOSE));
        assert!(m.is_present(SILENT));
        assert_eq!(m.value_of(ARGFILE), Some("myfile.txt"));
    }
}
