use super::{App, Input};

use std::fs;
use std::path::PathBuf;
use std::io::ErrorKind;

pub struct FileCreate<'a> {
  msg: &'a str,
  input: String,
  creating_dir: bool,
}

impl<'a> Default for FileCreate<'a> {
    fn default() -> Self {
        Self {
            msg: " Create new file: ",
            input: String::with_capacity(20),
            creating_dir: false,
        }
    }
}

impl<'a> FileCreate<'a> {
    pub fn dir(mut self) -> Self {
        self.creating_dir = true;
        self.msg = " Create new directory: ";
        self
    }
}


impl <'a> Input for FileCreate<'a> {
    fn on_enter(&mut self, app: &mut App) {
        if self.input.is_empty() {
            app.to_normal_mode();
            return
        }

        let mut new_file = app.wd.cwd().to_owned();
        new_file.push(PathBuf::from(self.input()));

        if !new_file.exists() {
            if self.creating_dir{
                match fs::create_dir(new_file) {
                    Ok(_) => {},
                    Err(e) => match e.kind() {
                        ErrorKind::PermissionDenied => app.err("Permission Denied"),
                        _ => app.err("Unexpected Error"),
                    } 
                }
            } else {
                match fs::File::create(new_file) {
                    Ok(_) => {},
                    Err(e) => match e.kind() {
                        ErrorKind::PermissionDenied => app.err("Permission Denied"),
                        _ => app.err("Unexpected Error"),
                    }
                }
            }

            app.wd.update();
            app.update_displayed_files(None);
            app.to_normal_mode();
        } else {
            app.err("Already Exists");
        }

    }
   
    fn add_to_input(&mut self, ch: char, _: &mut App) {
        self.input.push(ch);
    }

    fn del(&mut self, app: &mut App) {
        if !self.input.is_empty() {
            self.input.pop();
        } else {
            app.to_normal_mode()
        }
    }

    fn msg(&self) -> &str {
        self.msg
    }

    fn input(&self) -> &str {
        &self.input
    }

    fn clear(&mut self) {
        self.input.clear();
    }
}
