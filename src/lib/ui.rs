use std::path::Path;

use super::app::App;
use super::workingdir::WorkingDir;
use super::file::File;
use super::filetype::FileType;

use tui::Frame;
use tui::backend::Backend;
use tui::layout::{Constraint, Layout, Alignment};
use tui::text::{Text, Span};
use tui::style::{Style, Color, Modifier};
use tui::widgets::{
    Block, BorderType, Borders, List, 
    ListItem, ListState, Paragraph, 
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    // Create Layout for entire window
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

    let selected_file = &app.selected_file().to_owned();

    f.render_widget(gen_cwd_widget(app.wd.cwd()), chunks[0]);

    let list = gen_files(&app.wd, selected_file);

    f.render_stateful_widget(list, middle_chunks[0], &mut app.flist_state);

    match selected_file.ftype {
        FileType::Directory => {
            let prev_files = WorkingDir::get_files(app.selected_file().path())
                .expect("Occured trying to get preview files for a directory");
            f.render_widget(gen_dir_preview(&prev_files), middle_chunks[1])
        },
        FileType::File => { 
            f.render_widget(gen_file_preview(selected_file), middle_chunks[1])
        }
        _ => {}
    };

    f.render_widget(gen_extras(selected_file), chunks[2]);
}

fn gen_file_preview<'a>(file: &File) -> Paragraph<'a> {
    use std::io::ErrorKind;
    match std::fs::read_to_string(&file.path()) {
        Ok(s) => Paragraph::new(Text::from(s)).block(prev_block()),
        Err(e) => match e.kind() {
            ErrorKind::Interrupted => invalid_prev("Read Interrupted"),
            ErrorKind::InvalidData => invalid_prev("Invalid UTF-8"),
            _ => invalid_prev("Unexpected Error"),
        }
    }
}

fn invalid_prev(msg: &str) -> Paragraph {
    Paragraph::new(Span::styled(msg, Style::default().fg(Color::Red)))
        .block(prev_block())
}

fn gen_dir_preview(files: &[File]) -> List {
    let preview_list = list_from_files(files);
    List::new(preview_list)
        .block(prev_block())
}

fn gen_files<'a>(wd: &'a WorkingDir, selected_file: &File) -> List<'a> {
    let list_block = Block::default()
        .borders(Borders::RIGHT | Borders::LEFT)
        .style(Style::default().fg(Color::White))
        .border_type(BorderType::Plain);

    //TODO Create a new function to Render and Empty Directory

    let file_list = list_from_files(wd.files());

    let list = List::new(file_list).block(list_block)
        .highlight_style(
            Style::default()
                .bg(selected_file.color())
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD)
        );

    list
}

fn gen_extras<'a>(file: &File) -> Paragraph<'a> {
    let mut color = Color::White;

    if !file.perms.is_valid() { color = Color::Red }
    
    Paragraph::new(format!(" {}", file.perms.to_string()))
        .style(Style::default().fg(color))
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .borders(Borders::TOP)
                .style(Style::default().fg(Color::White))
                .border_type(BorderType::Plain),
    )
}

fn gen_keypress(input: &str) -> Paragraph {
    Paragraph::new(Span::from(input))
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::TOP)
                .style(Style::default().fg(Color::White))
                .border_type(BorderType::Plain),
    ) 
}

fn gen_cwd_widget<'a>(cwd: &Path) -> Paragraph<'a> {
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

fn list_from_files(files: &[File]) -> Vec<ListItem<'_>> {
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


