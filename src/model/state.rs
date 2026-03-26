use std::path::PathBuf;

use crate::util::utils::util_path_to_string;

pub struct TempState {
    new_temp_file: PathBuf,
    master_record_file: PathBuf,
    temprs_dir: PathBuf,
    temp_file_stack: Vec<PathBuf>,
    arg_file: Option<PathBuf>,
    insert_idx: Option<String>,
    output_buffer: String,
    holding_buffer: String,
    input_temp_file: Option<String>,
    output_temp_file: Option<String>,
    silent: bool,
    verbose: u32,
}

impl TempState {

    #[cfg(test)]
    pub fn set_new_temp_file(&mut self, new_temp_file: PathBuf) {
        self.new_temp_file = new_temp_file;
    }

    #[cfg(test)]
    pub fn set_master_record_file(&mut self, master_record_file: PathBuf) {
        self.master_record_file = master_record_file;
    }

    #[cfg(test)]
    pub fn set_temprs_dir(&mut self, temprs_dir: PathBuf) {
        self.temprs_dir = temprs_dir;
    }

    #[cfg(test)]
    pub fn set_temp_file_stack(&mut self, temp_file_stack: Vec<PathBuf>) {
        self.temp_file_stack = temp_file_stack;
    }

    pub fn set_arg_file(&mut self, arg_file: Option<PathBuf>) {
        self.arg_file = arg_file;
    }

    pub fn set_insert_idx(&mut self, insert_idx: Option<String>) {
        self.insert_idx = insert_idx;
    }

    pub fn set_holding_buffer(&mut self, holding_buffer: String) {
        self.holding_buffer = holding_buffer;
    }

    pub fn set_output_buffer(&mut self, output_buffer: String) {
        self.output_buffer = output_buffer;
    }

    pub fn set_input_temp_file(&mut self, input_temp_file: Option<String>) {
        self.input_temp_file = input_temp_file;
    }

    pub fn set_output_temp_file(&mut self, output_temp_file: Option<String>) {
        self.output_temp_file = output_temp_file;
    }

    pub fn set_silent(&mut self, silent: bool) {
        self.silent = silent;
    }

    pub fn set_verbose(&mut self, verbose: u32) {
        self.verbose = verbose;
    }
}

impl TempState {

    pub fn new_temp_file(&self) -> &PathBuf {
        &self.new_temp_file
    }

    pub fn master_record_file(&self) -> &PathBuf {
        &self.master_record_file
    }

    pub fn temprs_dir(&self) -> &PathBuf {
        &self.temprs_dir
    }

    pub fn temp_file_stack(&self) -> &Vec<PathBuf> {
        &self.temp_file_stack
    }

    pub fn arg_file(&self) -> &Option<PathBuf> {
        &self.arg_file
    }

    pub fn insert_idx(&self) -> &Option<String> {
        &self.insert_idx
    }

    pub fn holding_buffer(&self) -> &str {
        &self.holding_buffer
    }

    pub fn output_buffer(&self) -> &str {
        &self.output_buffer
    }

    pub fn input_temp_file(&self) -> &Option<String> {
        &self.input_temp_file
    }

    pub fn output_temp_file(&self) -> &Option<String> {
        &self.output_temp_file
    }

    #[cfg(test)]
    pub fn silent(&self) -> bool {
        self.silent
    }

    pub fn verbose(&self) -> u32 {
        self.verbose
    }
}

impl TempState {

    pub fn new(
        out_file: PathBuf,
        master_record_file: PathBuf,
        home_dir: PathBuf,
        temp_file_stack: Vec<PathBuf>,
        arg_file: Option<PathBuf>,
        output_buffer: String,
    ) -> Self {
        Self {
            new_temp_file: out_file,
            master_record_file,
            temprs_dir: home_dir,
            temp_file_stack,
            arg_file,
            insert_idx: None,
            output_buffer,
            holding_buffer: String::new(),
            input_temp_file: None,
            output_temp_file: None,
            silent: false,
            verbose: 0,
        }
    }
}

impl TempState {

    pub fn out_file_path_str(&self) -> String {
        util_path_to_string(self.new_temp_file())
    }


    #[cfg(test)]
    pub fn master_file_path_str(&self) -> String {
        util_path_to_string(self.master_record_file())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_state() -> TempState {
        TempState::new(
            PathBuf::from("/tmp/temprs/tempfile123"),
            PathBuf::from("/tmp/temprs/temprs-stack"),
            PathBuf::from("/tmp/temprs"),
            vec![
                PathBuf::from("/tmp/temprs/f1"),
                PathBuf::from("/tmp/temprs/f2"),
            ],
            None,
            String::new(),
        )
    }

    // ── constructor defaults ───────────────────────────

    #[test]
    fn new_state_defaults() {
        let s = make_state();
        assert_eq!(s.verbose(), 0);
        assert!(!s.silent());
        assert!(s.holding_buffer().is_empty());
        assert!(s.output_buffer().is_empty());
        assert!(s.input_temp_file().is_none());
        assert!(s.output_temp_file().is_none());
        assert!(s.insert_idx().is_none());
        assert!(s.arg_file().is_none());
    }

    // ── getters ────────────────────────────────────────

    #[test]
    fn new_temp_file_getter() {
        let s = make_state();
        assert_eq!(s.new_temp_file(), &PathBuf::from("/tmp/temprs/tempfile123"));
    }

    #[test]
    fn master_record_file_getter() {
        let s = make_state();
        assert_eq!(
            s.master_record_file(),
            &PathBuf::from("/tmp/temprs/temprs-stack")
        );
    }

    #[test]
    fn temprs_dir_getter() {
        let s = make_state();
        assert_eq!(s.temprs_dir(), &PathBuf::from("/tmp/temprs"));
    }

    #[test]
    fn temp_file_stack_getter() {
        let s = make_state();
        assert_eq!(s.temp_file_stack().len(), 2);
        assert_eq!(s.temp_file_stack()[0], PathBuf::from("/tmp/temprs/f1"));
        assert_eq!(s.temp_file_stack()[1], PathBuf::from("/tmp/temprs/f2"));
    }

    // ── setters ────────────────────────────────────────

    #[test]
    fn set_verbose() {
        let mut s = make_state();
        s.set_verbose(3);
        assert_eq!(s.verbose(), 3);
    }

    #[test]
    fn set_silent() {
        let mut s = make_state();
        s.set_silent(true);
        assert!(s.silent());
    }

    #[test]
    fn set_holding_buffer() {
        let mut s = make_state();
        s.set_holding_buffer("data".to_string());
        assert_eq!(s.holding_buffer(), "data");
    }

    #[test]
    fn set_output_buffer() {
        let mut s = make_state();
        s.set_output_buffer("out".to_string());
        assert_eq!(s.output_buffer(), "out");
    }

    #[test]
    fn set_input_temp_file() {
        let mut s = make_state();
        s.set_input_temp_file(Some("1".to_string()));
        assert_eq!(s.input_temp_file(), &Some("1".to_string()));
    }

    #[test]
    fn set_output_temp_file() {
        let mut s = make_state();
        s.set_output_temp_file(Some("2".to_string()));
        assert_eq!(s.output_temp_file(), &Some("2".to_string()));
    }

    #[test]
    fn set_insert_idx() {
        let mut s = make_state();
        s.set_insert_idx(Some("3".to_string()));
        assert_eq!(s.insert_idx(), &Some("3".to_string()));
    }

    #[test]
    fn set_arg_file() {
        let mut s = make_state();
        s.set_arg_file(Some(PathBuf::from("/tmp/input.txt")));
        assert_eq!(s.arg_file(), &Some(PathBuf::from("/tmp/input.txt")));
    }

    #[test]
    fn set_new_temp_file() {
        let mut s = make_state();
        s.set_new_temp_file(PathBuf::from("/tmp/new"));
        assert_eq!(s.new_temp_file(), &PathBuf::from("/tmp/new"));
    }

    #[test]
    fn set_master_record_file() {
        let mut s = make_state();
        s.set_master_record_file(PathBuf::from("/tmp/master2"));
        assert_eq!(s.master_record_file(), &PathBuf::from("/tmp/master2"));
    }

    #[test]
    fn set_temprs_dir() {
        let mut s = make_state();
        s.set_temprs_dir(PathBuf::from("/tmp/other"));
        assert_eq!(s.temprs_dir(), &PathBuf::from("/tmp/other"));
    }

    #[test]
    fn set_temp_file_stack() {
        let mut s = make_state();
        s.set_temp_file_stack(vec![PathBuf::from("/tmp/x")]);
        assert_eq!(s.temp_file_stack().len(), 1);
    }

    // ── path string helpers ────────────────────────────

    #[test]
    fn out_file_path_str_matches() {
        let s = make_state();
        assert_eq!(s.out_file_path_str(), "/tmp/temprs/tempfile123");
    }

    #[test]
    fn master_file_path_str_matches() {
        let s = make_state();
        assert_eq!(s.master_file_path_str(), "/tmp/temprs/temprs-stack");
    }

    // ── constructor with non-default args ────────────────

    #[test]
    fn new_with_output_buffer() {
        let s = TempState::new(
            PathBuf::from("/tmp/out"),
            PathBuf::from("/tmp/master"),
            PathBuf::from("/tmp"),
            vec![],
            None,
            String::from("preset output"),
        );
        assert_eq!(s.output_buffer(), "preset output");
    }

    #[test]
    fn new_with_arg_file() {
        let s = TempState::new(
            PathBuf::from("/tmp/out"),
            PathBuf::from("/tmp/master"),
            PathBuf::from("/tmp"),
            vec![],
            Some(PathBuf::from("/tmp/input.txt")),
            String::new(),
        );
        assert_eq!(s.arg_file(), &Some(PathBuf::from("/tmp/input.txt")));
    }

    #[test]
    fn new_with_empty_stack() {
        let s = TempState::new(
            PathBuf::from("/tmp/out"),
            PathBuf::from("/tmp/master"),
            PathBuf::from("/tmp"),
            vec![],
            None,
            String::new(),
        );
        assert!(s.temp_file_stack().is_empty());
    }

    #[test]
    fn new_with_large_stack() {
        let stack: Vec<PathBuf> = (0..50)
            .map(|i| PathBuf::from(format!("/tmp/temprs/f{}", i)))
            .collect();
        let s = TempState::new(
            PathBuf::from("/tmp/out"),
            PathBuf::from("/tmp/master"),
            PathBuf::from("/tmp"),
            stack.clone(),
            None,
            String::new(),
        );
        assert_eq!(s.temp_file_stack().len(), 50);
        assert_eq!(s.temp_file_stack()[0], PathBuf::from("/tmp/temprs/f0"));
        assert_eq!(s.temp_file_stack()[49], PathBuf::from("/tmp/temprs/f49"));
    }

    // ── setter overwrites ────────────────────────────────

    #[test]
    fn set_verbose_multiple_times() {
        let mut s = make_state();
        s.set_verbose(1);
        assert_eq!(s.verbose(), 1);
        s.set_verbose(5);
        assert_eq!(s.verbose(), 5);
        s.set_verbose(0);
        assert_eq!(s.verbose(), 0);
    }

    #[test]
    fn set_silent_toggle() {
        let mut s = make_state();
        s.set_silent(true);
        assert!(s.silent());
        s.set_silent(false);
        assert!(!s.silent());
        s.set_silent(true);
        assert!(s.silent());
    }

    #[test]
    fn set_holding_buffer_overwrite() {
        let mut s = make_state();
        s.set_holding_buffer("first".to_string());
        assert_eq!(s.holding_buffer(), "first");
        s.set_holding_buffer("second".to_string());
        assert_eq!(s.holding_buffer(), "second");
    }

    #[test]
    fn set_output_buffer_overwrite() {
        let mut s = make_state();
        s.set_output_buffer("out1".to_string());
        s.set_output_buffer("out2".to_string());
        assert_eq!(s.output_buffer(), "out2");
    }

    #[test]
    fn set_insert_idx_to_none() {
        let mut s = make_state();
        s.set_insert_idx(Some("5".to_string()));
        assert!(s.insert_idx().is_some());
        s.set_insert_idx(None);
        assert!(s.insert_idx().is_none());
    }

    #[test]
    fn set_input_temp_file_to_none() {
        let mut s = make_state();
        s.set_input_temp_file(Some("1".to_string()));
        assert!(s.input_temp_file().is_some());
        s.set_input_temp_file(None);
        assert!(s.input_temp_file().is_none());
    }

    #[test]
    fn set_output_temp_file_to_none() {
        let mut s = make_state();
        s.set_output_temp_file(Some("2".to_string()));
        assert!(s.output_temp_file().is_some());
        s.set_output_temp_file(None);
        assert!(s.output_temp_file().is_none());
    }

    #[test]
    fn set_arg_file_to_none() {
        let mut s = make_state();
        s.set_arg_file(Some(PathBuf::from("/tmp/f")));
        assert!(s.arg_file().is_some());
        s.set_arg_file(None);
        assert!(s.arg_file().is_none());
    }

    #[test]
    fn set_temp_file_stack_empty() {
        let mut s = make_state();
        assert_eq!(s.temp_file_stack().len(), 2);
        s.set_temp_file_stack(vec![]);
        assert!(s.temp_file_stack().is_empty());
    }

    #[test]
    fn set_temp_file_stack_larger() {
        let mut s = make_state();
        let big: Vec<PathBuf> = (0..10).map(|i| PathBuf::from(format!("/f{}", i))).collect();
        s.set_temp_file_stack(big);
        assert_eq!(s.temp_file_stack().len(), 10);
    }

    // ── path string helpers with different paths ─────────

    #[test]
    fn out_file_path_str_with_spaces() {
        let s = TempState::new(
            PathBuf::from("/tmp/my dir/file"),
            PathBuf::from("/tmp/master"),
            PathBuf::from("/tmp"),
            vec![],
            None,
            String::new(),
        );
        assert_eq!(s.out_file_path_str(), "/tmp/my dir/file");
    }

    #[test]
    fn master_file_path_str_deep_path() {
        let s = TempState::new(
            PathBuf::from("/tmp/out"),
            PathBuf::from("/a/b/c/d/e/master"),
            PathBuf::from("/tmp"),
            vec![],
            None,
            String::new(),
        );
        assert_eq!(s.master_file_path_str(), "/a/b/c/d/e/master");
    }

    // ── holding_buffer and output_buffer with special content ─

    #[test]
    fn holding_buffer_unicode() {
        let mut s = make_state();
        s.set_holding_buffer("日本語 🎉".to_string());
        assert_eq!(s.holding_buffer(), "日本語 🎉");
    }

    #[test]
    fn output_buffer_multiline() {
        let mut s = make_state();
        s.set_output_buffer("line1\nline2\nline3".to_string());
        assert_eq!(s.output_buffer(), "line1\nline2\nline3");
    }

    #[test]
    fn holding_buffer_empty() {
        let mut s = make_state();
        s.set_holding_buffer(String::new());
        assert!(s.holding_buffer().is_empty());
    }

    // ── verbose boundary values ──────────────────────────

    #[test]
    fn verbose_max_value() {
        let mut s = make_state();
        s.set_verbose(u32::MAX);
        assert_eq!(s.verbose(), u32::MAX);
    }

    #[test]
    fn verbose_zero() {
        let mut s = make_state();
        s.set_verbose(0);
        assert_eq!(s.verbose(), 0);
    }

    #[test]
    fn verbose_one() {
        let mut s = make_state();
        s.set_verbose(1);
        assert_eq!(s.verbose(), 1);
    }

    // ── buffers with large content ───────────────────────

    #[test]
    fn holding_buffer_large() {
        let mut s = make_state();
        let large = "x".repeat(10_000);
        s.set_holding_buffer(large.clone());
        assert_eq!(s.holding_buffer(), large);
        assert_eq!(s.holding_buffer().len(), 10_000);
    }

    #[test]
    fn output_buffer_large() {
        let mut s = make_state();
        let large = "y".repeat(10_000);
        s.set_output_buffer(large.clone());
        assert_eq!(s.output_buffer(), large);
        assert_eq!(s.output_buffer().len(), 10_000);
    }

    // ── buffers with special characters ──────────────────

    #[test]
    fn holding_buffer_tabs_and_newlines() {
        let mut s = make_state();
        s.set_holding_buffer("col1\tcol2\nrow2col1\trow2col2".to_string());
        assert_eq!(s.holding_buffer(), "col1\tcol2\nrow2col1\trow2col2");
    }

    #[test]
    fn output_buffer_special_chars() {
        let mut s = make_state();
        s.set_output_buffer("quotes: \"hello\" 'world'\nbackslash: \\".to_string());
        assert_eq!(s.output_buffer(), "quotes: \"hello\" 'world'\nbackslash: \\");
    }

    #[test]
    fn output_buffer_empty_string() {
        let mut s = make_state();
        s.set_output_buffer("data".to_string());
        s.set_output_buffer(String::new());
        assert!(s.output_buffer().is_empty());
    }

    // ── input/output_temp_file with various values ───────

    #[test]
    fn input_temp_file_negative_index() {
        let mut s = make_state();
        s.set_input_temp_file(Some("-1".to_string()));
        assert_eq!(s.input_temp_file(), &Some("-1".to_string()));
    }

    #[test]
    fn output_temp_file_negative_index() {
        let mut s = make_state();
        s.set_output_temp_file(Some("-3".to_string()));
        assert_eq!(s.output_temp_file(), &Some("-3".to_string()));
    }

    #[test]
    fn input_temp_file_large_index() {
        let mut s = make_state();
        s.set_input_temp_file(Some("999".to_string()));
        assert_eq!(s.input_temp_file(), &Some("999".to_string()));
    }

    #[test]
    fn output_temp_file_large_index() {
        let mut s = make_state();
        s.set_output_temp_file(Some("1000".to_string()));
        assert_eq!(s.output_temp_file(), &Some("1000".to_string()));
    }

    // ── insert_idx values ────────────────────────────────

    #[test]
    fn insert_idx_negative() {
        let mut s = make_state();
        s.set_insert_idx(Some("-2".to_string()));
        assert_eq!(s.insert_idx(), &Some("-2".to_string()));
    }

    #[test]
    fn insert_idx_large() {
        let mut s = make_state();
        s.set_insert_idx(Some("100".to_string()));
        assert_eq!(s.insert_idx(), &Some("100".to_string()));
    }

    #[test]
    fn insert_idx_overwrite() {
        let mut s = make_state();
        s.set_insert_idx(Some("1".to_string()));
        s.set_insert_idx(Some("5".to_string()));
        assert_eq!(s.insert_idx(), &Some("5".to_string()));
    }

    // ── arg_file with various paths ──────────────────────

    #[test]
    fn arg_file_with_spaces() {
        let mut s = make_state();
        s.set_arg_file(Some(PathBuf::from("/tmp/my dir/file.txt")));
        assert_eq!(s.arg_file(), &Some(PathBuf::from("/tmp/my dir/file.txt")));
    }

    #[test]
    fn arg_file_deep_path() {
        let mut s = make_state();
        s.set_arg_file(Some(PathBuf::from("/a/b/c/d/e/f/g.txt")));
        assert_eq!(s.arg_file(), &Some(PathBuf::from("/a/b/c/d/e/f/g.txt")));
    }

    #[test]
    fn arg_file_overwrite() {
        let mut s = make_state();
        s.set_arg_file(Some(PathBuf::from("/first")));
        s.set_arg_file(Some(PathBuf::from("/second")));
        assert_eq!(s.arg_file(), &Some(PathBuf::from("/second")));
    }

    // ── temp_file_stack manipulation sequences ───────────

    #[test]
    fn stack_replace_then_check_elements() {
        let mut s = make_state();
        let stack = vec![
            PathBuf::from("/a"),
            PathBuf::from("/b"),
            PathBuf::from("/c"),
        ];
        s.set_temp_file_stack(stack);
        assert_eq!(s.temp_file_stack().len(), 3);
        assert_eq!(s.temp_file_stack()[0], PathBuf::from("/a"));
        assert_eq!(s.temp_file_stack()[1], PathBuf::from("/b"));
        assert_eq!(s.temp_file_stack()[2], PathBuf::from("/c"));
    }

    #[test]
    fn stack_replace_multiple_times() {
        let mut s = make_state();
        s.set_temp_file_stack(vec![PathBuf::from("/x")]);
        assert_eq!(s.temp_file_stack().len(), 1);
        s.set_temp_file_stack(vec![PathBuf::from("/y"), PathBuf::from("/z")]);
        assert_eq!(s.temp_file_stack().len(), 2);
        s.set_temp_file_stack(vec![]);
        assert!(s.temp_file_stack().is_empty());
    }

    // ── new_temp_file setter edge cases ──────────────────

    #[test]
    fn new_temp_file_with_unicode() {
        let mut s = make_state();
        s.set_new_temp_file(PathBuf::from("/tmp/日本語/tempfile"));
        assert_eq!(s.new_temp_file(), &PathBuf::from("/tmp/日本語/tempfile"));
    }

    #[test]
    fn new_temp_file_overwrite() {
        let mut s = make_state();
        s.set_new_temp_file(PathBuf::from("/tmp/first"));
        s.set_new_temp_file(PathBuf::from("/tmp/second"));
        assert_eq!(s.new_temp_file(), &PathBuf::from("/tmp/second"));
    }

    // ── master_record_file setter edge cases ─────────────

    #[test]
    fn master_record_file_overwrite() {
        let mut s = make_state();
        s.set_master_record_file(PathBuf::from("/first"));
        s.set_master_record_file(PathBuf::from("/second"));
        assert_eq!(s.master_record_file(), &PathBuf::from("/second"));
    }

    // ── temprs_dir setter edge cases ─────────────────────

    #[test]
    fn temprs_dir_overwrite() {
        let mut s = make_state();
        s.set_temprs_dir(PathBuf::from("/dir1"));
        s.set_temprs_dir(PathBuf::from("/dir2"));
        assert_eq!(s.temprs_dir(), &PathBuf::from("/dir2"));
    }

    #[test]
    fn temprs_dir_with_spaces() {
        let mut s = make_state();
        s.set_temprs_dir(PathBuf::from("/tmp/my dir"));
        assert_eq!(s.temprs_dir(), &PathBuf::from("/tmp/my dir"));
    }

    // ── path str helpers after mutation ──────────────────

    #[test]
    fn out_file_path_str_after_set() {
        let mut s = make_state();
        s.set_new_temp_file(PathBuf::from("/new/path/file"));
        assert_eq!(s.out_file_path_str(), "/new/path/file");
    }

    #[test]
    fn master_file_path_str_after_set() {
        let mut s = make_state();
        s.set_master_record_file(PathBuf::from("/new/master"));
        assert_eq!(s.master_file_path_str(), "/new/master");
    }

    // ── full state construction with all non-default args ─

    #[test]
    fn new_all_fields_non_default() {
        let s = TempState::new(
            PathBuf::from("/tmp/out"),
            PathBuf::from("/tmp/master"),
            PathBuf::from("/tmp/dir"),
            vec![PathBuf::from("/tmp/f1"), PathBuf::from("/tmp/f2"), PathBuf::from("/tmp/f3")],
            Some(PathBuf::from("/tmp/argfile")),
            String::from("initial output"),
        );
        assert_eq!(s.new_temp_file(), &PathBuf::from("/tmp/out"));
        assert_eq!(s.master_record_file(), &PathBuf::from("/tmp/master"));
        assert_eq!(s.temprs_dir(), &PathBuf::from("/tmp/dir"));
        assert_eq!(s.temp_file_stack().len(), 3);
        assert_eq!(s.arg_file(), &Some(PathBuf::from("/tmp/argfile")));
        assert_eq!(s.output_buffer(), "initial output");
        // defaults still hold
        assert!(s.holding_buffer().is_empty());
        assert!(s.insert_idx().is_none());
        assert!(s.input_temp_file().is_none());
        assert!(s.output_temp_file().is_none());
        assert!(!s.silent());
        assert_eq!(s.verbose(), 0);
    }

    // ── multiple fields set in sequence ──────────────────

    #[test]
    fn set_all_fields_then_verify() {
        let mut s = make_state();
        s.set_verbose(2);
        s.set_silent(true);
        s.set_holding_buffer("hold".to_string());
        s.set_output_buffer("out".to_string());
        s.set_input_temp_file(Some("3".to_string()));
        s.set_output_temp_file(Some("4".to_string()));
        s.set_insert_idx(Some("2".to_string()));
        s.set_arg_file(Some(PathBuf::from("/tmp/arg")));
        s.set_new_temp_file(PathBuf::from("/tmp/new"));
        s.set_master_record_file(PathBuf::from("/tmp/master2"));
        s.set_temprs_dir(PathBuf::from("/tmp/dir2"));
        s.set_temp_file_stack(vec![PathBuf::from("/tmp/only")]);

        assert_eq!(s.verbose(), 2);
        assert!(s.silent());
        assert_eq!(s.holding_buffer(), "hold");
        assert_eq!(s.output_buffer(), "out");
        assert_eq!(s.input_temp_file(), &Some("3".to_string()));
        assert_eq!(s.output_temp_file(), &Some("4".to_string()));
        assert_eq!(s.insert_idx(), &Some("2".to_string()));
        assert_eq!(s.arg_file(), &Some(PathBuf::from("/tmp/arg")));
        assert_eq!(s.new_temp_file(), &PathBuf::from("/tmp/new"));
        assert_eq!(s.master_record_file(), &PathBuf::from("/tmp/master2"));
        assert_eq!(s.temprs_dir(), &PathBuf::from("/tmp/dir2"));
        assert_eq!(s.temp_file_stack().len(), 1);
    }

    // ── setters don't affect other fields ────────────────

    #[test]
    fn set_verbose_doesnt_change_silent() {
        let mut s = make_state();
        s.set_silent(true);
        s.set_verbose(5);
        assert!(s.silent());
    }

    #[test]
    fn set_holding_doesnt_change_output() {
        let mut s = make_state();
        s.set_output_buffer("original".to_string());
        s.set_holding_buffer("changed".to_string());
        assert_eq!(s.output_buffer(), "original");
    }

    #[test]
    fn set_input_doesnt_change_output_temp() {
        let mut s = make_state();
        s.set_output_temp_file(Some("5".to_string()));
        s.set_input_temp_file(Some("3".to_string()));
        assert_eq!(s.output_temp_file(), &Some("5".to_string()));
    }

    #[test]
    fn set_stack_doesnt_change_dir() {
        let mut s = make_state();
        let orig_dir = s.temprs_dir().clone();
        s.set_temp_file_stack(vec![]);
        assert_eq!(s.temprs_dir(), &orig_dir);
    }

    #[test]
    fn set_arg_file_doesnt_change_new_temp_file() {
        let mut s = make_state();
        let orig = s.new_temp_file().clone();
        s.set_arg_file(Some(PathBuf::from("/tmp/other")));
        assert_eq!(s.new_temp_file(), &orig);
    }
}
