use std::path::PathBuf;

pub struct TempState {
    out_file: PathBuf,
    master_file: PathBuf,
    temp_file_stack: Vec<PathBuf>,
    subcommand: String,
    buffer: String,
}

impl TempState {
    pub fn new(out_file: PathBuf, master_file: PathBuf, temp_file_stack: Vec<PathBuf>, subcommand: String, buffer: String) -> Self {
        TempState { out_file, master_file, temp_file_stack, subcommand, buffer }
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
    pub fn set_subcommand(&mut self, subcommand: String) {
        self.subcommand = subcommand;
    }
    pub fn set_buffer(&mut self, buffer: String) {
        self.buffer = buffer;
    }
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
    pub fn subcommand(&self) -> &str {
        &self.subcommand
    }
    pub fn buffer(&self) -> &str {
        &self.buffer
    }
}

impl TempState {
    pub fn out_file_string(&self) -> String {
        self.out_file
            .clone()
            .into_os_string()
            .into_string()
            .unwrap()
    }
    pub fn temp_list_file(&self) -> &PathBuf {
        &self.master_file
    }

    pub fn file_list_string(&self) -> String {
        self.master_file
            .clone()
            .into_os_string()
            .into_string()
            .unwrap()
    }
}
