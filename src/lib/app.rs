use std::path::Path;

use super::workingdir::WorkingDir;
use super::file::File;

use tui::widgets::ListState;

// Handles the state of the App
pub enum InputMode {
    Normal,
    Editing,
    Error,
    Visual,
}

pub struct App {
    // Input mode
    pub input_mode: InputMode,
    // Contains the user input in Editing mode
    pub key_press: String,
    // Info and helper methods for the cwd
    pub wd: WorkingDir,
    // Currently displayed files
    pub displayed_files: Vec<File>,
    // Current List state
    pub flist_state: ListState,
    // msg contains error messages and keybinds
    pub err_msg: String,
    // Requesting for user input
    pub is_searching: bool,
    pub searching_for: String,
}

impl App {
    /// Clear the current error and push the incoming error
    /// message to self.err_msg
    pub fn err(&mut self, msg: &str) {
        self.err_msg.clear();
        self.err_msg.push_str(msg);
        self.to_error_mode()
    }

    /// Used when you want to end the input 
    /// and restore the context to default conditions
    pub fn end_input(&mut self) {
        self.displayed_files = self.wd.files().to_vec();
        self.to_normal_mode();
        self.new_ctx();
    }

    pub fn clear_selection(&mut self) {
        self.displayed_files.iter_mut().for_each(|f| f.is_selected = false);
    }

    /// Helper function to set the input mode to InputMode::Normal
    pub fn to_normal_mode(&mut self) {
        self.input_mode = InputMode::Normal
    }

    /// Helper function to set the input mode to InputMode::Editing
    pub fn to_editing_mode(&mut self) {
        self.input_mode = InputMode::Editing
    }

    /// Helper function to set the input mode to InputMode::Editing
    fn to_error_mode(&mut self) {
        self.input_mode = InputMode::Error
    }

    /// Update the displayed files for a search or if a 
    /// file was deleted or added, takes an optional needle 
    /// argument for searching otherwise it just displays 
    /// the working directory
    pub fn update_displayed_files(&mut self, needle: Option<&str>) {
        if let Some(needle) = needle {
            self.searching_for = needle.to_string();
            self.displayed_files = self.wd.files()
                .iter()
                .filter(|f| f.name.starts_with(&self.searching_for))
                .cloned()
                .collect();
        // } else {
        } else if !self.searching_for.is_empty() {
             self.displayed_files = self.wd.files()
                .iter()
                .filter(|f| f.name.starts_with(&self.searching_for))
                .cloned()
                .collect();

        } else {
            self.reset_displayed_files();
            // self.displayed_files.iter_mut().for_each(|f| f.update()); 
        }
    }

    pub fn select_file(&mut self, needle: &Path) {
        // get the parent directory
        if let Some(parent) = needle.parent() {
            // Check to see if we should switch directories or not
            if self.wd.cwd().ne(parent) {
                match self.wd.set_cwd(parent) {
                    Ok(_) => self.update_displayed_files(None),
                    Err(e) => self.err(&e.to_string()),
                }
            }
        }

        // Option containing the index of our file
        let selection = self.displayed_files
            .iter()
            .enumerate()
            .find(|f| f.1.path.eq(needle));

        if let Some(selection) = selection {
            self.flist_state.select(Some(selection.0))
        } else {
            self.flist_state.select(Some(0))
        }
    }

    pub fn reset_displayed_files(&mut self) {
        self.displayed_files = self.wd.files().to_vec();
    }
    
    /// Basic opereations for opening a new context
    pub fn new_ctx(&mut self) {
        self.new_list_state();
        self.is_searching = false;
        self.searching_for.clear();
    }

    // Shifts the context to the next directory 
    pub fn wd_forward(&mut self) {
        let selected_path = self.selected_file().unwrap().path().to_owned();
        self.wd.forward(&selected_path); 
        self.displayed_files = self.wd.files().to_vec();
        self.new_ctx();
    }

    /// Shifts the context back one directory
    pub fn wd_back(&mut self) {
        if self.wd.back() {
            self.displayed_files = self.wd.files().to_vec();
            self.new_ctx();
        }
    }
    
    /// Returns a mutable reference to the currently 
    /// selected file if there is a file selected 
    /// otherwise it returns None
    pub fn selected_file(&self) -> Option<&File> {
        if let Some(selected) = self.flist_state.selected() {
            Some(&self.displayed_files[selected])
        } else if !self.displayed_files.is_empty() {
            Some(&self.displayed_files[0])
        } else {
            None
        }
    }

    /// Returns a mutable reference to the currently 
    /// selected file if there is a file selected 
    /// otherwise it returns None
    pub fn selected_file_mut(&mut self) -> Option<&mut File> {
        if let Some(selected) = self.flist_state.selected() {
            Some(&mut self.displayed_files[selected])
        } else if !self.displayed_files.is_empty() {
            Some(&mut self.displayed_files[0])
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

    /// Creates a default new App
    pub fn new() -> Self {
        let input_mode = InputMode::Normal;
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
        let err_msg = String::with_capacity(15);
        let is_searching = false;
        let searching_for = String::new();

        Self {
            input_mode,
            key_press,
            wd,
            displayed_files,
            flist_state,
            err_msg,
            is_searching,
            searching_for,
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
