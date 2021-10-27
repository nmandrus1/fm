use super::{Input, App};
use std::fs;
use std::path::PathBuf;
use std::io::ErrorKind;

pub struct FileRename<'a> {
    msg: &'a str,
    input: String,
}

impl<'a> Default for FileRename<'a> {
    fn default() -> Self {
        Self{ msg: " Rename file: ", input: String::with_capacity(15) }
    }
}

impl<'a> Input for FileRename<'a> {
    fn on_enter(&mut self, app: &mut App) {
        if app.selected_file().is_none() {
            return app.err("No File selected");
        }
        match fs::rename(app.selected_file().unwrap().path(), PathBuf::from(&self.input())){
            Ok(_) => { 
                app.wd.update();
                app.update_displayed_files(None);
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
