use std::path::Path;

use super::workingdir::WorkingDir;
use super::file::File;

use tui::widgets::ListState;

// Handles the state of the App
pub enum InputMode {
    Normal,
    Editing,
    Visual,
}

pub struct App {
    // Input mode
    pub input_mode: InputMode,
    // Contains the user input in Editing mode
    pub input: String,
    pub key_press: String,
    // Info and helper methods for the cwd
    pub wd: WorkingDir,
    // Currently displayed files
    pub displayed_files: Vec<File>,
    // Current List state
    pub flist_state: ListState,
    // msg contains error messages and keybinds
    pub msg: String,
    // Requesting for user input
    pub requesting_input: bool,
    // Struct that handles uesr input
}

impl App {

    pub fn add_to_input(&mut self, c: char) {
        self.input.push(c);
        self.update_displayed_files();
    }

    pub fn del_from_input(&mut self) {
        if self.input.len() > 1 {
            self.input.pop();
            self.update_displayed_files();
        } else { 
            self.input_mode = InputMode::Normal;
            self.requesting_input = false;
            self.input.clear();
        }
    }

    pub fn end_search(&mut self) {
        self.displayed_files = self.wd.files().to_vec();
        self.new_ctx();
    }

    fn update_displayed_files(&mut self) {
        self.displayed_files = self.wd.files()
            .iter()
            .filter(|f| f.name.starts_with(&self.input[1..]))
            .cloned()
            .collect();
        
        self.new_list_state();
    }

    // Shifts the context to the next directory 
    pub fn wd_forward(&mut self) {
        let selected_path = self.selected_file().unwrap().path().to_owned();
        self.wd.forward(&selected_path); 
        self.displayed_files = self.wd.files().to_vec();
        self.new_ctx();
    }

    pub fn new_ctx(&mut self) {
        self.new_list_state();
        self.requesting_input = false;
        self.input.clear();
    }

    pub fn wd_back(&mut self) {
        if self.wd.back() {
            self.new_ctx();
            self.displayed_files = self.wd.files().to_vec();
        }
    }
    /// Creates a default new App
    pub fn new() -> Self {
        let input_mode = InputMode::Normal;
        let input = String::with_capacity(15);
        let key_press = String::with_capacity(2);
        let wd = match WorkingDir::new(None) {
            Ok(w) => w,
            Err(_) => {
                eprintln!("Error starting fm");
                std::process::exit(1)
            } 
        };
        
        let displayed_files = wd.files().to_owned();

        let mut flist_state = ListState::default();
        flist_state.select(Some(0));
        let msg = String::with_capacity(15);
        let requesting_input = false;

        Self {
            input_mode,
            input,
            key_press,
            wd,
            displayed_files,
            flist_state,
            msg,
            requesting_input,
        }
    }

    /// Returns a reference to the Struct of the 
    /// Currently selected File
    pub fn selected_file(&self) -> Option<&File> {
        if let Some(selected) = self.flist_state.selected() {
            Some(&self.displayed_files[selected])
        } else if !self.displayed_files.is_empty() {
            Some(&self.displayed_files[0])
        } else {
            None
        }
    }

    /// ListState carries over data from the last 
    /// List that was rendered so call this method 
    /// Every time a new navigatable list of files
    /// Needs to be rendered
    pub fn new_list_state(&mut self) {
        self.flist_state = ListState::default();
        if self.displayed_files.is_empty() {
            self.flist_state.select(None)
        } else {
            self.flist_state.select(Some(0))
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
