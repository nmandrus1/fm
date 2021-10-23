use super::{Input, InputMode, App};

pub struct Search <'a> {
    pub msg: &'a str,
    pub input: String,
}

impl<'a> Default for Search<'a> {
    fn default() -> Self {
        Self {
            msg: "/", 
            input: String::with_capacity(15),
        }
    }
}

impl<'a> Input for Search<'a> {
    fn msg(&self) -> &'a str {
        self.msg
    }

    fn input(&self) -> &str {
        &self.input
    }

    fn on_enter(&mut self, app: &mut App) {
        if app.displayed_files.is_empty() {
            self.clear();
            app.end_input();
        } else {
            app.input_mode = InputMode::Normal;
        }
    }

    fn add_to_input(&mut self, ch: char, app: &mut App) {
        self.input.push(ch);
        app.update_displayed_files(self.input())
    }

    fn del(&mut self, app: &mut App) {
        if !self.input.is_empty() {
            self.input.pop();
            app.update_displayed_files(self.input())
        } else {
            app.end_input();
        }
    }

    fn clear(&mut self) {
        self.input.clear();
    }
}
