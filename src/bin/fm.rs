// std Imports
use std::sync::mpsc;

// Lib Imports
use fm::{filetype::FileType, workingdir::WorkingDir};
use fm::helpers;

// Crossterm Imports
use crossterm::{
    execute, 
    terminal::{
        EnterAlternateScreen,
        LeaveAlternateScreen,
        enable_raw_mode,
        disable_raw_mode
    },
    event::{
        read,
        poll,
        Event as CEvent,
        KeyEvent,
        KeyCode,
    },
};

// Tui imports
use tui::{
    Terminal, 
    backend::CrosstermBackend, 
    layout::{
        Constraint, 
        Direction, 
        Layout,
    },
    widgets::ListState,
};

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

    // Creates the input handling thread
    handle_input(tx);

    // Create Alternate Screen
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    // Create a crossterm backend and create a terminal to draw to
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    render_loop(&mut terminal, rx)?;

    Ok(())
}

fn render_loop(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>, 
    rx: mpsc::Receiver<Event<KeyEvent>>
    ) -> anyhow::Result<()> 
{
    let mut working_dir = WorkingDir::new(None)?;

    terminal.hide_cursor()?;

    let mut file_list_state = ListState::default();
    file_list_state.select(Some(0));

    loop {
        terminal.draw(|rect| {
            let size = rect.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(0)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Min(3),
                        Constraint::Length(2),
                    ].as_ref()
                )
                .split(size);            

        
            // helper function in lib/helpers.rs
            let cwd = helpers::gen_cwd_widget(working_dir.cwd());
            rect.render_widget(cwd, chunks[0]);

            // helper function in lib/helpers.rs
            let extras = helpers::gen_extras(&file_list_state, &working_dir);
            rect.render_widget(extras, chunks[2]);

            let middle_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
                .split(chunks[1]);

            let list = helpers::gen_files(&file_list_state, &working_dir);
            rect.render_stateful_widget(list, middle_chunks[0], &mut file_list_state);

            let selected = file_list_state.selected().unwrap_or_else(|| 0);
            let selected_file = &working_dir.files()[selected];

            match selected_file.ftype {
                FileType::File => {
                    let contents = std::fs::read_to_string(selected_file.path());
                    match contents {
                        Ok(s) => { 
                            let preview = helpers::gen_file_preview(&s);
                            rect.render_widget(preview, middle_chunks[1]); 
                        },
                        Err(_) => {
                            let preview = helpers::gen_file_preview_invalid();
                            rect.render_widget(preview, middle_chunks[1]); 
                        }
                    }
                }
                FileType::Directory => {
                    let contents = WorkingDir::get_files(selected_file.path());
                    match contents {
                        Ok(c) => match c.len() {
                            0 => {
                                let preview = helpers::gen_dir_preview_invalid("Empty Directory");
                                rect.render_widget(preview, middle_chunks[1]) 
                            },
                            _ => {
                                let preview = helpers::gen_dir_preview(&c);
                                rect.render_widget(preview, middle_chunks[1]) 
                            }
                        },

                        Err(e) => match e {
                            std::io::Error {..} => {
                                let preview = helpers::gen_dir_preview_invalid("Permission Denied");
                                rect.render_widget(preview, middle_chunks[1]) 
                            },

                            _ => panic!("Unhandled io Error")
                        }
                    }
                    
                    
                }
                _ => {}
            }

        })?;

        // Handle input send from other thread
        match rx.recv()? {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    // Call shutdown method
                    shutdown(terminal.backend_mut())?;
                    break;
                },
                // Goes down the list and wraps up to the top
                KeyCode::Char('j') => {
                    if let Some(selected) = file_list_state.selected() {
                        let num_files = working_dir.files().len();
                        if selected >= num_files -1 {
                            file_list_state.select(Some(0))
                        } else {
                            file_list_state.select(Some(selected + 1))
                        }
                    }
                },
                // Goes up the list
                KeyCode::Char('k') => {
                    if let Some(selected) = file_list_state.selected() {
                        if selected > 0 {
                            file_list_state.select(Some(selected - 1))
                        } else {
                            file_list_state.select(Some(0))
                        }
                    } 
                },
                // Going back
                KeyCode::Char('h') => {
                    working_dir.back(1);
                    // Reset selection to start at the top of the next directory 
                    file_list_state.select(Some(0));
                },
                // Going forward
                KeyCode::Char('l') => {
                    // Checks to see if the directory is valid
                    if let Some(selected) = file_list_state.selected() {
                        if working_dir.files()[selected].ftype == FileType::Directory
                        && WorkingDir::get_files(working_dir.files()[selected].path()).unwrap().len() != 0 {
                            let new_folder = working_dir.files()[selected].name.to_owned();
                            working_dir.forward(new_folder);
                            // Reset selection to start at the top of the next directory
                            file_list_state.select(Some(0))
                        }                    
                    }
                }
                // Keymap to jump to the last element
                KeyCode::Char('G') => {
                    let num_files = working_dir.files().len();
                    file_list_state.select(Some(num_files - 1))
                }
                _ => {}
            },
            Event::Tick => {}
        }
    }

    Ok(())
}


// Input Handling Thread
// Takes a transmitter and a tickrate and listens for input
fn handle_input(tx: mpsc::Sender<Event<KeyEvent>>) {
    use std::thread;
    use std::time::{Duration, Instant};

    let tick_rate = Duration::from_millis(200);

    thread::spawn(move || -> anyhow::Result<()> {
        let mut last_tick = Instant::now();
        loop {
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
            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now()
                }
            }
        }
    });
}

fn shutdown(backend: &mut CrosstermBackend<std::io::Stdout>) -> anyhow::Result<()> {
    execute!(backend, LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
