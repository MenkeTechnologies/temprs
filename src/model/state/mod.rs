use std::path::PathBuf;

use crate::util::utils::{util_path_to_string, util_paths_and_names_to_file};

#[cfg(test)]
mod tests;

/// `TempState` — see fields for the structure layout.
pub struct TempState {
    new_temp_file: PathBuf,
    master_record_file: PathBuf,
    temprs_dir: PathBuf,
    temp_file_stack: Vec<PathBuf>,
    temp_file_names: Vec<Option<String>>,
    arg_file: Option<PathBuf>,
    insert_idx: Option<String>,
    output_buffer: String,
    holding_buffer: String,
    input_temp_file: Option<String>,
    output_temp_file: Option<String>,
    append_temp_file: Option<String>,
    name: Option<String>,
    silent: bool,
    verbose: u32,
}

impl TempState {
    #[cfg(test)]
    /// `set_new_temp_file` — see implementation for the contract.
    pub fn set_new_temp_file(&mut self, new_temp_file: PathBuf) {
        self.new_temp_file = new_temp_file;
    }

    #[cfg(test)]
    /// `set_master_record_file` — see implementation for the contract.
    pub fn set_master_record_file(&mut self, master_record_file: PathBuf) {
        self.master_record_file = master_record_file;
    }

    #[cfg(test)]
    /// `set_temprs_dir` — see implementation for the contract.
    pub fn set_temprs_dir(&mut self, temprs_dir: PathBuf) {
        self.temprs_dir = temprs_dir;
    }

    #[cfg(test)]
    /// `set_temp_file_stack` — see implementation for the contract.
    pub fn set_temp_file_stack(&mut self, temp_file_stack: Vec<PathBuf>) {
        self.temp_file_stack = temp_file_stack;
    }

    /// `set_arg_file` — see implementation for the contract.
    pub fn set_arg_file(&mut self, arg_file: Option<PathBuf>) {
        self.arg_file = arg_file;
    }

    /// `set_insert_idx` — see implementation for the contract.
    pub fn set_insert_idx(&mut self, insert_idx: Option<String>) {
        self.insert_idx = insert_idx;
    }

    /// `set_holding_buffer` — see implementation for the contract.
    pub fn set_holding_buffer(&mut self, holding_buffer: String) {
        self.holding_buffer = holding_buffer;
    }

    /// `set_output_buffer` — see implementation for the contract.
    pub fn set_output_buffer(&mut self, output_buffer: String) {
        self.output_buffer = output_buffer;
    }

    /// `set_input_temp_file` — see implementation for the contract.
    pub fn set_input_temp_file(&mut self, input_temp_file: Option<String>) {
        self.input_temp_file = input_temp_file;
    }

    /// `set_output_temp_file` — see implementation for the contract.
    pub fn set_output_temp_file(&mut self, output_temp_file: Option<String>) {
        self.output_temp_file = output_temp_file;
    }

    /// `set_append_temp_file` — see implementation for the contract.
    pub fn set_append_temp_file(&mut self, append_temp_file: Option<String>) {
        self.append_temp_file = append_temp_file;
    }

    /// `set_silent` — see implementation for the contract.
    pub fn set_silent(&mut self, silent: bool) {
        self.silent = silent;
    }

    /// `set_verbose` — see implementation for the contract.
    pub fn set_verbose(&mut self, verbose: u32) {
        self.verbose = verbose;
    }

    /// `set_name` — see implementation for the contract.
    pub fn set_name(&mut self, name: Option<String>) {
        self.name = name;
    }

    /// `set_temp_file_names` — see implementation for the contract.
    pub fn set_temp_file_names(&mut self, names: Vec<Option<String>>) {
        self.temp_file_names = names;
    }
}

impl TempState {
    /// `new_temp_file` — see implementation for the contract.
    pub fn new_temp_file(&self) -> &PathBuf {
        &self.new_temp_file
    }

    /// `master_record_file` — see implementation for the contract.
    pub fn master_record_file(&self) -> &PathBuf {
        &self.master_record_file
    }

    /// `temprs_dir` — see implementation for the contract.
    pub fn temprs_dir(&self) -> &PathBuf {
        &self.temprs_dir
    }

    /// `temp_file_stack` — see implementation for the contract.
    pub fn temp_file_stack(&self) -> &Vec<PathBuf> {
        &self.temp_file_stack
    }

    /// `temp_file_stack_mut` — see implementation for the contract.
    pub fn temp_file_stack_mut(&mut self) -> &mut Vec<PathBuf> {
        &mut self.temp_file_stack
    }

    /// `arg_file` — see implementation for the contract.
    pub fn arg_file(&self) -> &Option<PathBuf> {
        &self.arg_file
    }

    /// `insert_idx` — see implementation for the contract.
    pub fn insert_idx(&self) -> &Option<String> {
        &self.insert_idx
    }

    /// `holding_buffer` — see implementation for the contract.
    pub fn holding_buffer(&self) -> &str {
        &self.holding_buffer
    }

    /// `holding_buffer_mut` — see implementation for the contract.
    pub fn holding_buffer_mut(&mut self) -> &mut String {
        &mut self.holding_buffer
    }

    /// `output_buffer` — see implementation for the contract.
    pub fn output_buffer(&self) -> &str {
        &self.output_buffer
    }

    /// `input_temp_file` — see implementation for the contract.
    pub fn input_temp_file(&self) -> &Option<String> {
        &self.input_temp_file
    }

    /// `output_temp_file` — see implementation for the contract.
    pub fn output_temp_file(&self) -> &Option<String> {
        &self.output_temp_file
    }

    /// `append_temp_file` — see implementation for the contract.
    pub fn append_temp_file(&self) -> &Option<String> {
        &self.append_temp_file
    }

    #[cfg(test)]
    /// `silent` — see implementation for the contract.
    pub fn silent(&self) -> bool {
        self.silent
    }

    /// `verbose` — see implementation for the contract.
    pub fn verbose(&self) -> u32 {
        self.verbose
    }

    /// `name` — see implementation for the contract.
    pub fn name(&self) -> &Option<String> {
        &self.name
    }

    /// `temp_file_names` — see implementation for the contract.
    pub fn temp_file_names(&self) -> &Vec<Option<String>> {
        &self.temp_file_names
    }

    /// `temp_file_names_mut` — see implementation for the contract.
    pub fn temp_file_names_mut(&mut self) -> &mut Vec<Option<String>> {
        &mut self.temp_file_names
    }
}

impl TempState {
    /// `new` — see implementation for the contract.
    pub fn new(
        out_file: PathBuf,
        master_record_file: PathBuf,
        home_dir: PathBuf,
        temp_file_stack: Vec<PathBuf>,
        temp_file_names: Vec<Option<String>>,
        arg_file: Option<PathBuf>,
        output_buffer: String,
    ) -> Self {
        Self {
            new_temp_file: out_file,
            master_record_file,
            temprs_dir: home_dir,
            temp_file_stack,
            temp_file_names,
            arg_file,
            insert_idx: None,
            output_buffer,
            holding_buffer: String::new(),
            input_temp_file: None,
            output_temp_file: None,
            append_temp_file: None,
            name: None,
            silent: false,
            verbose: 0,
        }
    }
}

impl TempState {
    /// `out_file_path_str` — see implementation for the contract.
    pub fn out_file_path_str(&self) -> String {
        util_path_to_string(self.new_temp_file())
    }

    #[cfg(test)]
    /// `master_file_path_str` — see implementation for the contract.
    pub fn master_file_path_str(&self) -> String {
        util_path_to_string(self.master_record_file())
    }

    /// `write_master` — see implementation for the contract.
    pub fn write_master(&self) {
        util_paths_and_names_to_file(
            &self.temp_file_stack,
            &self.temp_file_names,
            &self.master_record_file,
        );
    }
}
