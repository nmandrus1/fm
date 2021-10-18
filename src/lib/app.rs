use super::workingdir::WorkingDir;
use super::file::File;

use tui::widgets::{List, ListItem, ListState, Paragraph};
use tui::text::Span;

// Handles the state of the App
pub enum InputMode {
    Normal,
    Editing,
}

pub struct App {
    pub input_mode: InputMode,
    pub search_bar: String,
    pub key_press: String,
    pub wd: WorkingDir,
    // pub prev: Paragraph<'a>,
    // pub flist: List<'a>,
    pub flist_state: ListState,
    // pub extras: Paragraph<'a>,
    // pub cwd: Paragraph<'a>
}

impl App {
    pub fn new() -> Self {
        let input_mode = InputMode::Normal;
        let search_bar = String::with_capacity(15);
        let key_press = String::with_capacity(2);
        let wd = match WorkingDir::new(None) {
            Ok(w) => w,
            Err(_) => {
                eprintln!("Error starting fm");
                std::process::exit(1)
            } 
        };

        // let prev = Paragraph::new(Span::raw(""));
        // let flist = List::new(vec![ListItem::new(Span::raw(""))]);
        let mut flist_state = ListState::default();
        flist_state.select(Some(0));
        // let extras = Paragraph::new(Span::raw(""));
        // let cwd = Paragraph::new(Span::raw(""));

        Self {
            input_mode,
            search_bar,
            key_press,
            wd,
            // prev,
            // flist,
            flist_state,
            // extras,
            // cwd,
        }
    }

    pub fn selected_file(&self) -> &File {
        if let Some(selected) =  self.flist_state.selected() {
            &self.wd.files()[selected]
        } else {
            &self.wd.files()[0]
        }
    }

    pub fn new_list_state(&mut self) {
        self.flist_state = ListState::default();
        self.flist_state.select(Some(0))
    }
}
