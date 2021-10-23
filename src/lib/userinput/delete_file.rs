use super::{Input, App, FileType};

use std::fs;

use std::io::ErrorKind;

pub struct FileDelete <'a> {
    pub msg: &'a str,
    pub input: String,
}

impl<'a> Default for FileDelete<'a> {
    fn default() -> Self {
        Self {
            msg: "Are you sure you want to delete this [y/n]: ",
            input: String::with_capacity(1),
        }
    }
}

impl <'a> Input for FileDelete<'a> {
    fn add_to_input(&mut self, ch: char, _: &mut App) {
        if self.input.is_empty() {
            self.input.push(ch);
        }
    }

    fn del(&mut self, app: &mut App) {
        if !self.input.is_empty() {
            self.input.pop();
        } else {
            app.end_input();
        }
    }

    fn on_enter(&mut self, app: &mut App) {
        match self.input.to_lowercase().as_str() {
            "y" => {
                let selected_file = app.selected_file().expect("Error getting selected file");
                match selected_file.ftype {
                    FileType::Directory => match fs::remove_dir_all(&selected_file.path) {
                        Ok(_) => {},
                        Err(e) => match e.kind() {
                            ErrorKind::PermissionDenied => app.err_msg.push_str("Permission Denied"),
                            _ => app.err_msg.push_str("Unexpected Error"),
                        }
                    },
                    _ => match fs::remove_file(&selected_file.path) {
                        Ok(_) => {},
                        Err(e) => match e.kind() {
                            ErrorKind::PermissionDenied => app.err_msg.push_str("Permission Denied"),
                            _ => app.err_msg.push_str("Unexpected Error"),
                        } 
                    },
                };
            },
             _ => app.end_input(),
        }
    }

    fn msg(&self) -> &'a str {
        self.msg
    }

    fn input(&self) -> &str {
        &self.input
    }

    fn clear(&mut self) {
        self.input.clear();
    } 
}
