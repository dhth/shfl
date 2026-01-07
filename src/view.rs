use crate::common::{PRIMARY_COLOR, TITLE, TITLE_FG_COLOR, View};
use crate::model::Model;
use ratatui::{
    Frame,
    layout::Alignment,
    style::Style,
    text::Line,
    widgets::{Block, List, ListDirection, ListItem, Padding, Paragraph},
};

const HELP_CONTENTS: &str = include_str!("static/help.txt");

pub(crate) fn view(model: &mut Model, frame: &mut Frame) {
    match model.view {
        View::List => render_list_view(model, frame),
        View::Help => render_help_view(frame),
    }
}

fn render_list_view(model: &mut Model, frame: &mut Frame) {
    let items: Vec<ListItem> = model.lines.items.iter().map(ListItem::from).collect();

    let title = model
        .message
        .as_ref()
        .map(|m| format!(" {}", m.value()))
        .unwrap_or(TITLE.to_string());

    let base_title_style = Style::new().bold();
    let title_style = match model.message {
        Some(_) => base_title_style,
        None => base_title_style.bg(PRIMARY_COLOR).fg(TITLE_FG_COLOR),
    };

    let block = Block::default()
        .title_bottom(title)
        .title_style(title_style);

    let list = List::new(items)
        .block(block)
        .style(Style::new().white())
        .repeat_highlight_symbol(true)
        .highlight_symbol(">> ")
        .highlight_style(Style::new().fg(PRIMARY_COLOR))
        .direction(ListDirection::TopToBottom);

    frame.render_stateful_widget(list, frame.area(), &mut model.lines.state)
}

fn render_help_view(frame: &mut Frame) {
    let title_style = Style::new().bold().bg(PRIMARY_COLOR).fg(TITLE_FG_COLOR);

    let block = Block::default()
        .title_bottom(TITLE)
        .padding(Padding::left(1))
        .title_style(title_style);

    let lines: Vec<Line<'_>> = HELP_CONTENTS.lines().map(Line::from).collect();

    let p = Paragraph::new(lines)
        .block(block)
        .style(Style::new().white())
        .alignment(Alignment::Left);

    frame.render_widget(p, frame.area())
}
