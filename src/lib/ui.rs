use std::path::Path;

use super::userinput::Input;

use super::app::{App, InputMode};
use super::workingdir::WorkingDir;
use super::file::File;
use super::filetype::FileType;
// use super::userinput::Input;

use tui::Frame;
use tui::backend::Backend;
use tui::layout::{Constraint, Layout, Alignment, Rect};
use tui::text::{Text, Span};
use tui::style::{Style, Color, Modifier};
use tui::widgets::{
    Block, BorderType, Borders, List, 
    ListItem, Paragraph,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {

    let selected_file = if app.selected_file().is_none() {
        render_empty(f, app);
        return ()
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
            f.render_widget(gen_input(""), extra_chunks[3]);
            f.render_stateful_widget(list, middle_chunks[0], &mut app.flist_state);
            
            let (ex1, ex2, ex3) = gen_extras(
                    &selected_file, app.flist_state.selected().unwrap(), 
                    app.displayed_files.len(), 
            );

            f.render_widget(ex1, extra_chunks[0]);
            f.render_widget(ex2, extra_chunks[1]);
            f.render_widget(ex3, extra_chunks[2]);
        }, 

        InputMode::Editing => {
            f.render_widget(gen_cwd(app.wd.cwd()), chunks[0]);
            f.render_widget(gen_input(&app.user_inp.output()), chunks[2]);
            f.set_cursor(chunks[2].x + app.user_inp.output().len() as u16, chunks[2].y + 1);
            f.render_widget(list, middle_chunks[0]);
        },
        InputMode::Visual => {}
    };

    match selected_file.ftype {
        FileType::Directory => {
            match gen_dir_preview(&selected_file) {
                Ok(list) => f.render_widget(list, middle_chunks[1]),
                Err(s) => f.render_widget(invalid_prev(&s), middle_chunks[1])
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
    use std::io::ErrorKind;
    match std::fs::read_to_string(&file.path()) {
        Ok(s) => Ok(Paragraph::new(Text::from(s)).block(prev_block())),
        Err(e) => match e.kind() {
            ErrorKind::Interrupted => Err("Read Interrupted".to_string()),
            ErrorKind::InvalidData => Err("Invalid UTF-8".to_string()),
            _ => Err(format!("{:?}", e)),
        }
    }
}

fn gen_input<'a>(input: &'a str) -> Paragraph<'a> {
    Paragraph::new(input).alignment(Alignment::Left)
        .block(Block::default().borders(Borders::TOP))
}

fn invalid_prev(msg: &str) -> Paragraph {
    Paragraph::new(Span::styled(msg, Style::default().fg(Color::Red)))
        .block(prev_block())
}

fn gen_dir_preview(file: &File) -> anyhow::Result<List, String> {
    match WorkingDir::get_files(file.path()) {
        Ok(files) => Ok(List::new(list_from_files(&files)).block(prev_block())),
        Err(e) => match e.kind() {
            std::io::ErrorKind::PermissionDenied => Err("Permission Denied".to_string()),
            _ => Err(format!("{:?}", e)),
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

fn render_empty<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let (chunks, _) = gen_chunks(f);
    f.render_widget(gen_cwd(app.wd.cwd()), chunks[0]);
    let err_msg = format!("Pattern not found: {}", &app.user_inp.output()[1..]);
    let err = invalid_prev(&err_msg)
        .block(Block::default().borders(Borders::LEFT | Borders::RIGHT));
    f.render_widget(err, chunks[1]);
    f.render_widget(gen_input(&app.user_inp.output()), chunks[2]);
    f.set_cursor(chunks[2].x + app.user_inp.output().len() as u16, chunks[2].y + 1);
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
                 Constraint::Percentage(13),
                 Constraint::Percentage(13),
                 Constraint::Percentage(14),
                 Constraint::Percentage(60),
            ].as_ref()
        )
        .split(chunks[2])
}
