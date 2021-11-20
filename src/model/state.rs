#![allow(dead_code)]
#![allow(unused_must_use)]

use std::path::PathBuf;

use crate::util::utils::util_path_to_string;

pub struct TempState {
    new_temp_file: PathBuf,
    master_record_file: PathBuf,
    temp_file_stack: Vec<PathBuf>,
    arg_file: Option<PathBuf>,
    insert_idx: Option<String>,
    output_buffer: String,
    input_temp_file: Option<String>,
    output_temp_file: Option<String>,
    silent: bool,
}

impl TempState {
    pub fn set_new_temp_file(&mut self, new_temp_file: PathBuf) {
        self.new_temp_file = new_temp_file;
    }
    pub fn set_master_record_file(&mut self, master_record_file: PathBuf) {
        self.master_record_file = master_record_file;
    }
    pub fn set_temp_file_stack(&mut self, temp_file_stack: Vec<PathBuf>) {
        self.temp_file_stack = temp_file_stack;
    }
    pub fn set_arg_file(&mut self, arg_file: Option<PathBuf>) {
        self.arg_file = arg_file;
    }
    pub fn set_insert_idx(&mut self, insert_idx: Option<String>) {
        self.insert_idx = insert_idx;
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
}

impl TempState {
    pub fn new_temp_file(&self) -> &PathBuf {
        &self.new_temp_file
    }
    pub fn master_record_file(&self) -> &PathBuf {
        &self.master_record_file
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
    pub fn output_buffer(&self) -> &str {
        &self.output_buffer
    }
    pub fn input_temp_file(&self) -> &Option<String> {
        &self.input_temp_file
    }
    pub fn output_temp_file(&self) -> &Option<String> {
        &self.output_temp_file
    }
    pub fn silent(&self) -> bool {
        self.silent
    }
}

impl TempState {
    pub fn new(
        out_file: PathBuf,
        master_record_file: PathBuf,
        temp_file_stack: Vec<PathBuf>,
        arg_file: Option<PathBuf>,
        output_buffer: String,
    ) -> Self {
        TempState {
            new_temp_file: out_file,
            master_record_file,
            temp_file_stack,
            arg_file,
            insert_idx: None,
            output_buffer,
            input_temp_file: None,
            output_temp_file: None,
            silent: false,
        }
    }
}

impl TempState {
    pub fn out_file_path_str(&self) -> String {
        util_path_to_string(self.new_temp_file())
    }

    pub fn master_file_path_str(&self) -> String {
        util_path_to_string(self.master_record_file())
    }
}
