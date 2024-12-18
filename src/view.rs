use crate::common::{PRIMARY_COLOR, TITLE, TITLE_FG_COLOR};
use crate::model::Model;
use ratatui::{
    style::{Style, Stylize},
    widgets::{Block, List, ListDirection, ListItem, Padding},
    Frame,
};

pub(crate) fn view(model: &mut Model, frame: &mut Frame) {
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
    let list = List::new(items)
        .block(
            Block::default()
                .title_bottom(title)
                .padding(Padding::bottom(1))
                .title_style(title_style),
        )
        .style(Style::new().white())
        .repeat_highlight_symbol(true)
        .highlight_symbol(">> ")
        .highlight_style(Style::new().fg(PRIMARY_COLOR))
        .direction(ListDirection::TopToBottom);

    frame.render_stateful_widget(list, frame.area(), &mut model.lines.state)
}
