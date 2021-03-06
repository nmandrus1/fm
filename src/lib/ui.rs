use std::path::Path;

use super::app::{App, InputMode};
use super::workingdir::WorkingDir;
use super::file::File;
use super::filetype::FileType;
use super::userinput::Input;

use tui::Frame;
use tui::backend::Backend;
use tui::layout::{Constraint, Layout, Alignment, Rect};
use tui::text::{Text, Span};
use tui::style::{Style, Color, Modifier};
use tui::widgets::{
    Block, BorderType, Borders, List, 
    ListItem, Paragraph,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App, user_inp: &mut Box<dyn Input>) {

    let selected_file = if app.selected_file().is_none() {
        render_empty(f, app, user_inp);
        return 
    } else {
        app.selected_file().unwrap().to_owned()
    };

    let (chunks, middle_chunks) = gen_chunks(f);
    let files = list_from_files(&app.displayed_files);
    let list = gen_list(&files, &selected_file);

    match app.input_mode {
        InputMode::Normal => {
            let extra_chunks = nmode_extra_chunks(&chunks);
            f.render_widget(gen_cwd(app.wd.cwd()), chunks[0]);
            f.render_widget(gen_input(""), extra_chunks[4]);

            // Render an empty screen for an empty directory
            if app.wd.files().is_empty() {
                f.render_widget(invalid_prev("Empty Directory"), chunks[1])
            } else {
                f.render_stateful_widget(list, middle_chunks[0], &mut app.flist_state);
            }
            
            let (ex1, ex2, ex3) = gen_extras(
                    &selected_file, app.flist_state.selected().unwrap(), 
                    app.displayed_files.len(), 
            );

            f.render_widget(ex1, extra_chunks[1]);
            f.render_widget(ex2, extra_chunks[2]);
            f.render_widget(ex3, extra_chunks[3]);
        }, 

        InputMode::Editing => {
            f.render_widget(gen_cwd(app.wd.cwd()), chunks[0]);
            f.render_widget(gen_input(&user_inp.output()), chunks[2]);
            f.set_cursor(chunks[2].x + user_inp.output().len() as u16, chunks[2].y + 1);
            f.render_widget(list, middle_chunks[0]);
        },
        InputMode::Visual => {},
        InputMode::Error => {
            f.render_widget(gen_cwd(app.wd.cwd()), chunks[0]);
            f.render_widget(gen_err(&app.err_msg), chunks[2]);
            f.render_widget(list, middle_chunks[0]);
        }
    };

    match selected_file.ftype {
        FileType::Directory => {
            match gen_dir_preview(&selected_file) {
                Ok(list) => f.render_widget(list, middle_chunks[1]),
                Err(s) => f.render_widget(invalid_prev(s), middle_chunks[1])
            }
        },
        FileType::File => { 
            match gen_file_preview(&selected_file) {
                Ok(file) => f.render_widget(file, middle_chunks[1]),
                Err(s) => f.render_widget(invalid_prev(&s), middle_chunks[1]),
            }
        }
        _ => {}
    };
}

fn gen_file_preview<'a>(file: &File) -> anyhow::Result<Paragraph<'a>, String> {
    use std::io::{Read, ErrorKind};
    use std::fs;

    // check to see if the file needs to be cut off
    // for performance reasons 
    if file.size > 1000 {
        // create buffer
        let mut buf = [0; 1000];
        let mut f = fs::File::open(file.path()).unwrap();
        // check to make sure everything read into the buffer properly
        match f.read_exact(&mut buf) {
            Ok(_) => {},
            Err(e) => match e.kind() {
                // if it didnt return early with the error
                ErrorKind::Interrupted => return Err("Read Interrupted".to_string()),
                _ => return Err(e.to_string())
            }
        }
        // return result of trying to create a string from buf
        return match String::from_utf8(buf.to_vec()) {
            Ok(s) => Ok(Paragraph::new(Text::from(s)).block(prev_block())),
            Err(_) => Err("Invalid UTF-8".to_string())
        }
    }

    // if the file is less than 500 bytes just read_to_string
    match std::fs::read_to_string(&file.path()) {
        Ok(s) => {
            if s.is_empty() {
                Err("Empty File".to_string())
            } else {
                Ok(Paragraph::new(Text::from(s)).block(prev_block()))
            }
        },
        Err(_) => Err("Invalid UTF-8".to_string()),
    }
}

fn gen_input(input: &str) -> Paragraph {
    Paragraph::new(input).alignment(Alignment::Left)
        .block(Block::default().borders(Borders::TOP))
}

fn invalid_prev(msg: &str) -> Paragraph {
    Paragraph::new(Span::styled(msg, Style::default().fg(Color::Red)))
        .block(prev_block())
}

fn gen_dir_preview(file: &File) -> anyhow::Result<List, &str> {
    match WorkingDir::get_files(file.path()) {
        Ok(files) => {
            if files.is_empty() {
                return Err("Empty Directory")
            }
            Ok(List::new(list_from_files(&files)).block(prev_block()))
        },
        Err(e) => match e.kind() {
            std::io::ErrorKind::PermissionDenied => Err("Permission Denied"),
            _ => Err("Unexpected Error")
        }
    }
}

fn gen_list<'a>(files: &'a [ListItem], selected_file: &File) -> List<'a> {
    let list_block = Block::default()
        .borders(Borders::RIGHT | Borders::LEFT)
        .style(Style::default().fg(Color::White))
        .border_type(BorderType::Plain);

    //TODO Create a new function to Render and Empty Directory

    let list = List::new(files)
        .block(list_block)
        .highlight_style(
            Style::default()
                .bg(selected_file.color())
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD)
        );

    list
}

fn gen_extras<'a>(
    file: &File, 
    selected: usize, 
    total: usize)
    -> (Paragraph<'a>, Paragraph<'a>, Paragraph<'a>) 
{
    let mut color = Color::White;
    if !file.perms.is_valid() { color = Color::Red }

    let block = Block::default()
                .borders(Borders::TOP)
                .style(Style::default().fg(Color::White))
                .border_type(BorderType::Plain);

    let p1 = Paragraph::new(file.perms.to_string())
        .style(Style::default().fg(color)).block(block.clone())
        .alignment(Alignment::Center);

    let p2 = Paragraph::new(file.size_to_readable())
        .style(Style::default().fg(color)).block(block.clone())
        .alignment(Alignment::Center);

    let p3 = Paragraph::new(format!("{}/{}", selected + 1, total))
        .style(Style::default().fg(color)).block(block)
        .alignment(Alignment::Center);
        
    (p1, p2, p3)
}

fn gen_cwd<'a>(cwd: &Path) -> Paragraph<'a> {
    Paragraph::new(Span::raw(cwd.display().to_string()))
        .style(Style::default().add_modifier(Modifier::BOLD).fg(Color::LightBlue))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .border_type(BorderType::Plain),            
    )
}

fn gen_err(msg: &str) -> Paragraph {
    Paragraph::new(msg)
        .style(Style::default().fg(Color::Red)
               .add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(
            Block::default()
            .borders(Borders::TOP)
            .style(Style::default().fg(Color::White))
            .border_type(BorderType::Plain))
}

fn prev_block() -> Block<'static> {
Block::default()
    .borders(Borders::RIGHT)
    .style(Style::default().fg(Color::White))
    .border_type(BorderType::Plain)
}

fn list_from_files<'a>(files: &[File]) -> Vec<ListItem<'a>> {
  files
    .iter()
    .map(|f| {
        ListItem::new(
            Span::styled(f.name.clone(),
            Style::default().fg(f.color()))
        )
    })
    .collect::<Vec<_>>()
}

fn render_empty<B: Backend>(f: &mut Frame<B>, app: &mut App, user_inp: &mut Box<dyn Input>) {
    let (chunks, _) = gen_chunks(f);
    f.render_widget(gen_cwd(app.wd.cwd()), chunks[0]);

    match app.input_mode {
        InputMode::Normal => {
            f.render_widget(gen_err("Empty Directory"), chunks[1]);
            f.render_widget(gen_input(""), chunks[2])
        },
        InputMode::Editing => {
            let msg = if app.is_searching {
                format!("Pattern not found: {}", user_inp.input())
            } else { String::new() };

            f.render_widget(gen_input(&user_inp.output()), chunks[2]);
            f.set_cursor(chunks[2].x + user_inp.output().len() as u16, chunks[2].y + 1);
            f.render_widget(gen_err(&msg), chunks[1]);
        },
        InputMode::Error => {
            f.render_widget(gen_err("Empty Directory"), chunks[1]);
            f.render_widget(gen_err(&app.err_msg), chunks[2])
        },
        _ => {},
    }
}

fn gen_chunks<B: Backend>(f: &mut Frame<B>) -> (Vec<Rect>, Vec<Rect>) {
    let chunks = Layout::default()
        .direction(tui::layout::Direction::Vertical)
        .margin(0)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(3),
                Constraint::Length(2),
            ].as_ref()
        ).split(f.size());

    let middle_chunks = Layout::default()
        .direction(tui::layout::Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(chunks[1]); 

    

    (chunks, middle_chunks)
}

fn nmode_extra_chunks(chunks: &[Rect]) -> Vec<Rect> {
     Layout::default()
        .direction(tui::layout::Direction::Horizontal)
        .constraints(
            [
                 Constraint::Percentage(1),
                 Constraint::Percentage(13),
                 Constraint::Percentage(13),
                 Constraint::Percentage(13),
                 Constraint::Percentage(60),
            ].as_ref()
        )
        .split(chunks[2])
}
