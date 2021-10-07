use std::{convert::TryFrom, iter::FromIterator, path::PathBuf};

use super::workingdir::WorkingDir;
use super::file::File;
use super::filetype::FileType;

use anyhow::Result;
use tui::{
    text::{
        Span,
        Spans, 
        Text,
    },
    layout::Alignment,
    widgets::{
        Paragraph,
        Block,
        Borders,
        BorderType,
        List,
        ListItem,
        ListState,
    },
    style::{
        Style,
        Color,
        Modifier,
    }
};

pub fn gen_file_preview<'a>(contents: &'a str) -> Paragraph<'a> {
    Paragraph::new(Text::from(contents))
}

pub fn gen_file_preview_invalid<'a>() -> Paragraph<'a> {
    Paragraph::new(Span::styled("Invalid UTF-8", Style::default().fg(Color::Red)))
}

pub fn gen_dir_preview<'a>(files: &'a [File]) -> List<'a> {
    let preview_list = list_from_files(&files);
    List::new(preview_list)
}

pub fn gen_files<'a>(list_state: &ListState, working_dir: &'a WorkingDir) -> List<'a> {
    let list_block = Block::default()
        .borders(Borders::RIGHT | Borders::LEFT)
        .style(Style::default().fg(Color::White))
        .border_type(BorderType::Plain);

    //TODO Create a new function to Render and Empty Directory

    let file_list = match working_dir.len() {
        0 => vec![ListItem::new(Span::raw("Empty Directory"))],
        _ => list_from_files(working_dir.files())
    };
        

    let selected_file = list_state.selected().expect("Inside file_list.get()");

    let list = List::new(file_list).block(list_block)
        .highlight_style(
            Style::default()
                .bg(working_dir.files()[selected_file].color())
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD)
        );

    list
}

pub fn gen_extras<'a>(list_state: &ListState, working_dir: &WorkingDir) -> Paragraph<'a> {
    let perms = working_dir.files()[list_state.selected().unwrap_or_else(|| 0)].perms;
    let mut color = Color::White;

    if !perms.is_valid() { color = Color::Red }
    
    Paragraph::new(format!(" {}", perms.to_string()))
        .style(Style::default().fg(color))
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .borders(Borders::TOP)
                .style(Style::default().fg(Color::White))
                .border_type(BorderType::Plain),
    )
}

pub fn gen_cwd_widget<'a>(cwd: &PathBuf) -> Paragraph<'a> {
    Paragraph::new(Span::raw(cwd.display().to_string()))
        .style(Style::default().add_modifier(Modifier::BOLD).fg(Color::LightBlue))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::BOTTOM)
                .style(Style::default().fg(Color::White))
                .border_type(BorderType::Plain),            
    )
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
