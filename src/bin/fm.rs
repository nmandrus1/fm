// std Imports
use std::sync::mpsc;

use fm::userinput::{
    Input, Search, FileDelete,
    FileCreate, FileRename, FileCopy
};
// Lib Imports
use fm::filetype::FileType;
use fm::{app::{App, InputMode}, ui};

// Crossterm Imports
use crossterm::{
    execute, 
    terminal::{ EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode, disable_raw_mode},
    event::{read, poll, Event as CEvent, KeyEvent, KeyCode},
};

// Tui imports
use tui::{Terminal, backend::CrosstermBackend};

// Handles wether input is recieved
enum Event<I>{
    Input(I),
    Tick,
}

fn main() -> anyhow::Result<()> {
    
    // Enable Raw Mode
    enable_raw_mode()?;

    // Create channel for communicating across threads
    let (tx, rx) = mpsc::channel();
    let (tx1, rx1) = mpsc::channel();

    // Creates the input handling thread
    handle_input(tx, rx1);

    // Create Alternate Screen
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    // Create a crossterm backend and create a terminal to draw to
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Set panic behavior
    let default_panic = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        disable_raw_mode().unwrap();
        default_panic(info);
    }));

    render_loop(&mut terminal, rx, tx1)?;

    Ok(())
}

fn render_loop(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>, 
    rx: mpsc::Receiver<Event<KeyEvent>>,
    tx1: mpsc::Sender<()>,
    ) -> anyhow::Result<()> 
{
    terminal.hide_cursor()?;
    let mut app = App::new();
    let mut user_inp: Box<dyn Input> = Box::new(Search::default());

    // let mut user_inp = Search::default();

    loop {
        terminal.draw(|rect| ui::draw(rect, &mut app, &mut user_inp))?;

        // Handle input send from other thread
        match rx.recv()? {
            Event::Input(event) => match app.input_mode {
                InputMode::Normal => match event.code {
                    KeyCode::Char('q') => {
                    // Call shutdown method
                    shutdown(terminal.backend_mut())?;
                    break;
                    },
                    // Goes down the list and wraps up to the top
                    KeyCode::Char('j') => {
                        if let Some(selected) = app.flist_state.selected() {
                            let num_files = app.displayed_files.len();
                            if selected >= num_files -1 {
                                app.flist_state.select(Some(0))
                            } else {
                                app.flist_state.select(Some(selected + 1))
                            }
                        }
                    },
                    // Goes up the list
                    KeyCode::Char('k') => {
                        if let Some(selected) = app.flist_state.selected() {
                            if selected > 0 {
                                app.flist_state.select(Some(selected - 1))
                            } else {
                                app.flist_state.select(Some(0))
                            }
                        } 
                    },
                    // Going back
                    KeyCode::Char('h') => {
                        app.wd_back();
                        user_inp.clear();
                    },
                    // Going forward
                    KeyCode::Char('l') => {
                        // Checks to see if the directory is valid
                        if app.selected_file().is_some()
                        && app.selected_file().unwrap().ftype == FileType::Directory
                        && std::fs::read_dir(app.selected_file().unwrap().path()).is_ok() {
                            app.wd_forward();
                            user_inp.clear();
                        }                    
                    },
                    KeyCode::Enter => {
                        if let Some(selected_file) = app.selected_file() {
                            if selected_file.ftype == FileType::File {
                                tx1.send(())?;
                                execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
                                std::process::Command::new("nvim")
                                    .arg(selected_file.path())
                                    .spawn()
                                    .unwrap()
                                    .wait()
                                    .unwrap();
                                execute!(terminal.backend_mut(), EnterAlternateScreen)?;
                                terminal.clear()?;
                                tx1.send(())?;
                            }
                        }
                        app.selected_file_mut().unwrap().update_size();
                    },
                    KeyCode::Char('g') => {
                        if &app.key_press == "g" {
                            app.flist_state.select(Some(0));
                            app.key_press.clear()
                        } else {
                            app.key_press = "g".to_string();
                        }
                    },
                    // Keymap to jump to the last element
                    KeyCode::Char('G') => {
                        let num_files = app.wd.files().len();
                        app.flist_state.select(Some(num_files - 1))
                    },
                    KeyCode::Char('d') => { 
                        app.to_editing_mode();
                        user_inp = Box::new(FileDelete::default());
                    },
                    KeyCode::Char('a')=> {
                        app.to_editing_mode();
                        user_inp = Box::new(FileCreate::default())
                    }
                    KeyCode::Char('A') => {
                        app.to_editing_mode();
                        user_inp = Box::new(FileCreate::default().dir())
                    }
                    KeyCode::Char('r') => {
                        app.to_editing_mode();
                        if let Some(file) = app.selected_file() {
                            user_inp = Box::new(FileRename::default().file(file))
                        }
                    }
                    KeyCode::Char('p') => {
                        println!("name: {} path: {:?}", app.selected_file().unwrap().name, app.selected_file().unwrap().path())
                    }
                    KeyCode::Char('/') => { 
                        if app.is_searching {
                            app.input_mode = InputMode::Editing
                        } else {
                            user_inp = Box::new(Search::default());
                            app.input_mode = InputMode::Editing;
                            app.is_searching = true;
                        }
                    },

                    KeyCode::Char('c') => {
                        app.to_editing_mode();
                        if let Some(file) = app.selected_file() {
                            user_inp = Box::new(FileCopy::default().file(file));
                        }
                    }

                    KeyCode::Char('v') => {
                        if let Some(file) = app.selected_file_mut() {
                            file.is_selected = !file.is_selected;
                        }
                    },

                    KeyCode::Esc => {
                        if app.is_searching {
                            app.end_input()
                        } else {
                            app.clear_selection();
                            app.to_normal_mode()
                        }
                    }
                    _ => {}
                },
                InputMode::Editing => match event.code {
                    KeyCode::Esc => { 
                        app.to_normal_mode();
                        app.end_input();
                        user_inp.clear();
                        app.is_searching = false;
                    },
                    KeyCode::Enter => { 
                        user_inp.on_enter(&mut app)
                    }
                    KeyCode::Char(c) => {
                        user_inp.add_to_input(c, &mut app)
                    }, 
                    KeyCode::Backspace => {
                        user_inp.del(&mut app);
                    }
                    _ => {} 
                }
                InputMode::Visual => match event.code {
                    _ => {}
                },
                InputMode::Error => { app.to_normal_mode() }
            },
            Event::Tick => {}
        }
    }

    Ok(())
}


// Input Handling Thread
// Takes a transmitter and a tickrate and listens for input
fn handle_input(tx: mpsc::Sender<Event<KeyEvent>>, rx: mpsc::Receiver<()>) {
    use std::thread;
    use std::time::{Duration, Instant};

    let tick_rate = Duration::from_millis(200);

    thread::spawn(move || -> anyhow::Result<()> {
        let mut last_tick = Instant::now();

        loop {
            if rx.recv_timeout(Duration::from_millis(10)).is_err() {
                // Time before we want to time out
                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or_else(|| Duration::from_secs(0));

                // If an event is available, send it to the rendering thread 
                if poll(timeout)? {
                    if let CEvent::Key(key) = read()? {
                        tx.send(Event::Input(key))?
                    }
                }

                // If a timeout has occured, let the rendering thread know
                // it was just a normal tick, and nothing is changing
                if last_tick.elapsed() >= tick_rate && tx.send(Event::Tick).is_ok() {
                    last_tick = Instant::now()
                } 

            } else {
                while rx.recv_timeout(tick_rate).is_err() {}
            }
        }
    });
}

fn shutdown(backend: &mut CrosstermBackend<std::io::Stdout>) -> anyhow::Result<()> {
    execute!(backend, LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
