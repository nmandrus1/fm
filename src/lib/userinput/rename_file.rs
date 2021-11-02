use super::{Input, App, File};
use std::fs;
use std::path::PathBuf;
use std::io::ErrorKind;

pub struct FileRename<'a> {
    msg: &'a str,
    input: String,
}

impl<'a> FileRename<'a> {
    pub fn file(mut self, f: &File) -> Self {
        self.input.push_str(f.path().to_str().unwrap());
        self
    }
}

impl<'a> Default for FileRename<'a> {
    fn default() -> Self {
        Self{ msg: " Rename file: ", input: String::with_capacity(30) }
    }
}

impl<'a> Input for FileRename<'a> {
    fn on_enter(&mut self, app: &mut App) {
        if app.selected_file().is_none() {
            return app.err("No File selected");
        }

        let mut new_file = PathBuf::from(self.input());

        if new_file.eq(app.selected_file().unwrap().path()) {
            return app.to_normal_mode()
        }

        let file = new_file.clone();

        match fs::rename(app.selected_file().unwrap().path(), &new_file){
            Ok(_) => { 
                //TODO Working on getting fm to 
                // automatically move to the new file location
                new_file.pop();
                app.wd.set_cwd(&new_file)
                    .unwrap_or_else(|e| app.err(&e.to_string()));
                app.update_displayed_files(None);
                app.select_file(&file);
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
