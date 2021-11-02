use super::{App, Input, File};

use std::fs;
use std::path::PathBuf;
use std::io::ErrorKind;

pub struct FileCopy<'a> {
  msg: &'a str,
  input: String,
}

impl<'a> FileCopy<'a> {
    pub fn file(mut self, f: &File) -> Self {
        self.input.push_str(f.path().to_str().unwrap());
        self
    }
}

impl<'a> Default for FileCopy<'a> {
    fn default() -> Self {
        Self{ msg: " Copy file to: ", input: String::with_capacity(30) }
    }
}
 
impl<'a> Input for FileCopy<'a> {
    fn on_enter(&mut self, app: &mut App) {
        if app.selected_file().is_none() {
            return app.err("No File selected");
        }

        let new_file = PathBuf::from(self.input());

        if new_file.eq(app.selected_file().unwrap().path()) {
            return app.to_normal_mode()
        }


        match fs::copy(app.selected_file().unwrap().path(), &new_file){
            Ok(_) => { 
                app.wd.update();
                app.reset_displayed_files();
                app.select_file(&new_file);
                app.to_normal_mode();
            },
            Err(e) => match e.kind() {
                ErrorKind::AlreadyExists => { return app.err("Already Exists"); }
                _ => { return app.err(e.to_string().as_str()); }
            }
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
