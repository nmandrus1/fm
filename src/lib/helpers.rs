use std::{clone, path::PathBuf};

use super::workingdir::WorkingDir;

use tui::{
    text::Span,
    layout::{
        Alignment,
    },
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

pub fn gen_files<'a>(list_state: &ListState, working_dir: &WorkingDir) -> List<'a> {
    let list_block = Block::default()
        .borders(Borders::RIGHT | Borders::LEFT)
        .style(Style::default().fg(Color::White))
        .border_type(BorderType::Plain);

    //TODO Create a new function to Render and Empty Directory

    let file_list = match working_dir.len() {
        0 => vec![ListItem::new(Span::raw("Empty Directory"))],
        _ => {
        working_dir
            .files()
            .iter()
            .map(|f| {
                let color = f.color();
                ListItem::new(
                    Span::styled(f.name.clone(),
                    Style::default().fg(color))
                )
            })
            .collect::<Vec<_>>() 
        }
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

pub fn gen_extras<'a>(extras: &'a str) -> Paragraph<'a> {
     Paragraph::new(extras)
        .style(Style::default().fg(Color::White))
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
