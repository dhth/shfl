use crate::model::{Message, Model};
use ratatui::crossterm::event::{self, Event, KeyCode};
use std::time::Duration;

pub(crate) fn update(model: &mut Model, msg: Message) -> Option<Message> {
    model.message = None;
    match msg {
        Message::MoveToIndex(i) => model.move_to_index(i),
        Message::GoToNextItem => model.select_next(),
        Message::GoToPreviousPreview => model.select_previous(),
        Message::GoToLastItem => model.select_last(),
        Message::GoToFirstItem => model.select_first(),
        Message::SwitchWithNextItem => model.switch_with_next(),
        Message::SwitchWithPreviousItem => model.switch_with_previous(),
        Message::SwitchWithFirstItem => model.switch_with_first(),
        Message::ToggleSelection => model.toggle_current(),
        Message::SaveSelection => model.save_selection(),
        Message::Quit => model.go_back_or_quit(),
    };
    None
}

pub(crate) fn handle_event(_: &Model) -> anyhow::Result<Option<Message>> {
    if event::poll(Duration::from_millis(16))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                return Ok(handle_key(key));
            }
        }
    }
    Ok(None)
}

pub(crate) fn handle_key(key: event::KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Char('1') => Some(Message::MoveToIndex(0)),
        KeyCode::Char('2') => Some(Message::MoveToIndex(1)),
        KeyCode::Char('3') => Some(Message::MoveToIndex(2)),
        KeyCode::Char('4') => Some(Message::MoveToIndex(3)),
        KeyCode::Char('5') => Some(Message::MoveToIndex(4)),
        KeyCode::Char('6') => Some(Message::MoveToIndex(5)),
        KeyCode::Char('7') => Some(Message::MoveToIndex(6)),
        KeyCode::Char('8') => Some(Message::MoveToIndex(7)),
        KeyCode::Char('9') => Some(Message::MoveToIndex(8)),
        KeyCode::Char('j') | KeyCode::Down => Some(Message::GoToNextItem),
        KeyCode::Char('k') | KeyCode::Up => Some(Message::GoToPreviousPreview),
        KeyCode::Char('g') => Some(Message::GoToFirstItem),
        KeyCode::Char('G') => Some(Message::GoToLastItem),
        KeyCode::Char('J') => Some(Message::SwitchWithNextItem),
        KeyCode::Char('K') => Some(Message::SwitchWithPreviousItem),
        KeyCode::Enter => Some(Message::SwitchWithFirstItem),
        KeyCode::Char('s') | KeyCode::Char(' ') => Some(Message::ToggleSelection),
        KeyCode::Esc | KeyCode::Char('q') => Some(Message::Quit),
        KeyCode::Char('w') => Some(Message::SaveSelection),
        _ => None,
    }
}
