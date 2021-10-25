use super::{App, FileType, Input};

use std::fs;

use std::io::ErrorKind;

pub struct FileDelete <'a> {
    msg: &'a str,
    input: String,
}

impl<'a> Default for FileDelete<'a> {
    fn default() -> Self {
        Self {
            msg: " Are you sure you want to delete this [y/n]: ",
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
            app.to_normal_mode();
        }
    }

    fn on_enter(&mut self, app: &mut App) {
        let selected_file = app.selected_file().expect("Error getting selected file");
        match self.input.to_lowercase().as_str() {
            "y" => {
                match selected_file.ftype {
                    FileType::Directory => match fs::remove_dir_all(&selected_file.path()) {
                        Ok(_) => {},
                        Err(e) => match e.kind() {
                            ErrorKind::PermissionDenied => { 
                                app.err("Permission Denied"); 
                                return 
                            },
                            _ => { 
                                app.err("Unexpected Error"); 
                                return 
                            },
                        }
                    },
                    _ => match fs::remove_file(&selected_file.path()) {
                        Ok(_) => {},
                        Err(e) => match e.kind() {
                            ErrorKind::PermissionDenied => { 
                                app.err("Permission Denied"); 
                                return
                            },
                            _ => { 
                                app.err("Unexpected Error"); 
                                return 
                            },
                        } 
                    },
                };
            },
            _ => {
                app.to_normal_mode();
                return
            }
        }
        
        if app.wd.files().is_empty() {
            app.wd_back();
            return
        }

        app.wd.update();
        app.displayed_files.remove(app.flist_state.selected().unwrap());
        if let Some(selected_idx) = app.flist_state.selected() {

            app.new_list_state();

            // Determine which file should be highlighted after deletion
            if selected_idx == 0 {
            } else if selected_idx == app.displayed_files.len() {
                app.flist_state.select(Some(selected_idx - 1));
            } else {
                app.flist_state.select(Some(selected_idx))
            }

            app.to_normal_mode();
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
