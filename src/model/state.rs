use std::path::PathBuf;

use crate::util::utils::path_to_string;

pub struct TempState {
    out_file: PathBuf,
    master_file: PathBuf,
    temp_file_stack: Vec<PathBuf>,
    arg_file: String,
    buffer: String,
    input_temp_file: String,
    output_temp_file: String,
}

impl TempState {
    pub fn out_file(&self) -> &PathBuf {
        &self.out_file
    }
    pub fn master_file(&self) -> &PathBuf {
        &self.master_file
    }
    pub fn temp_file_stack(&self) -> &Vec<PathBuf> {
        &self.temp_file_stack
    }
    pub fn arg_file(&self) -> &str {
        &self.arg_file
    }
    pub fn buffer(&self) -> &str {
        &self.buffer
    }
    pub fn input_temp_file(&self) -> &str {
        &self.input_temp_file
    }
    pub fn output_temp_file(&self) -> &str {
        &self.output_temp_file
    }
}

impl TempState {
    pub fn set_out_file(&mut self, out_file: PathBuf) {
        self.out_file = out_file;
    }
    pub fn set_master_file(&mut self, master_file: PathBuf) {
        self.master_file = master_file;
    }
    pub fn set_temp_file_stack(&mut self, temp_file_stack: Vec<PathBuf>) {
        self.temp_file_stack = temp_file_stack;
    }
    pub fn set_arg_file(&mut self, arg_file: String) {
        self.arg_file = arg_file;
    }
    pub fn set_buffer(&mut self, buffer: String) {
        self.buffer = buffer;
    }
    pub fn set_input_temp_file(&mut self, input_temp_file: String) {
        self.input_temp_file = input_temp_file;
    }
    pub fn set_output_temp_file(&mut self, output_temp_file: String) {
        self.output_temp_file = output_temp_file;
    }
}

impl TempState {
    pub fn new(out_file: PathBuf, master_file: PathBuf, temp_file_stack: Vec<PathBuf>, arg_file: String, buffer: String) -> Self {
        TempState { out_file, master_file, temp_file_stack, arg_file, buffer, input_temp_file: String::new(), output_temp_file: String::new() }
    }
}


impl TempState {
    pub fn out_file_contents(&self) -> String {
        path_to_string(self.out_file())
    }

    pub fn temp_list_file(&self) -> &PathBuf {
        &self.master_file
    }

    pub fn master_file_contents(&self) -> String {
        path_to_string(self.master_file())
    }
}
