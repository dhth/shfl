use crate::common::View;
use crate::message::Message;
use crate::model::{Model, RunningState, UserMessage};
use crate::utils::write_to_file;
use ratatui::crossterm::event::{self, Event, KeyCode};
use std::time::Duration;

pub(crate) fn handle_event(model: &Model) -> anyhow::Result<Option<Message>> {
    if event::poll(Duration::from_millis(16))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                return Ok(handle_key(model, key));
            }
        }
    }
    Ok(None)
}

fn handle_key(model: &Model, key: event::KeyEvent) -> Option<Message> {
    match model.view {
        View::List => match key.code {
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
            KeyCode::Char('?') => Some(Message::ShowView(View::Help)),
            KeyCode::Char('w') => Some(Message::SaveSelection),
            _ => None,
        },
        View::Help => match key.code {
            KeyCode::Esc | KeyCode::Char('q') => Some(Message::Quit),
            KeyCode::Char('?') => Some(Message::ShowView(View::Help)),
            _ => None,
        },
    }
}

pub(crate) fn update(model: &mut Model, msg: Message) -> Option<Message> {
    model.message = None;
    match msg {
        Message::MoveToIndex(i) => move_to_index(model, i),
        Message::GoToNextItem => select_next(model),
        Message::GoToPreviousPreview => select_previous(model),
        Message::GoToLastItem => select_last(model),
        Message::GoToFirstItem => select_first(model),
        Message::SwitchWithNextItem => switch_with_next(model),
        Message::SwitchWithPreviousItem => switch_with_previous(model),
        Message::SwitchWithFirstItem => switch_with_first(model),
        Message::ToggleSelection => toggle_current(model),
        Message::SaveSelection => save_selection(model),
        Message::ShowView(v) => show_view(model, v),
        Message::Quit => go_back_or_quit(model),
    }
}

fn move_to_index(model: &mut Model, index: usize) -> Option<Message> {
    if index > model.lines.items.len() - 1 {
        model.message = Some(UserMessage::Error("index is out of range".to_string()));
        return None;
    }

    let current = model.lines.state.selected();
    if let Some(i) = current {
        if i == index {
            return None;
        }
        let removed = model.lines.items.remove(i);
        model.lines.items.insert(index, removed);
        model.lines.state.select(Some(index));
    }

    None
}

fn select_next(model: &mut Model) -> Option<Message> {
    model.lines.state.select_next();
    None
}
fn select_previous(model: &mut Model) -> Option<Message> {
    model.lines.state.select_previous();
    None
}
fn select_first(model: &mut Model) -> Option<Message> {
    model.lines.state.select_first();
    None
}
fn select_last(model: &mut Model) -> Option<Message> {
    model.lines.state.select_last();
    None
}
fn switch_with_next(model: &mut Model) -> Option<Message> {
    let current = model.lines.state.selected();
    if let Some(i) = current {
        if i == model.lines.items.len() - 1 {
            return None;
        }
        model.lines.items.swap(i, i + 1);
        model.lines.state.select_next();
    }
    None
}
fn switch_with_previous(model: &mut Model) -> Option<Message> {
    let current = model.lines.state.selected();
    if let Some(i) = current {
        if i == 0 {
            return None;
        }
        model.lines.items.swap(i, i - 1);
        model.lines.state.select_previous();
    }
    None
}
fn switch_with_first(model: &mut Model) -> Option<Message> {
    let current = model.lines.state.selected();
    if let Some(i) = current {
        match i {
            0 => (),
            1 => model.lines.items.swap(0, 1),
            _ => {
                model.lines.items[0..i + 1].rotate_right(1);
            }
        };
        model.lines.state.select_first();
    }
    None
}

fn toggle_current(model: &mut Model) -> Option<Message> {
    let current = model.lines.state.selected();
    if let Some(i) = current {
        model.lines.items[i].toggle();
    }
    None
}

fn save_selection(model: &mut Model) -> Option<Message> {
    let items: Vec<&str> = model
        .lines
        .items
        .iter()
        .map(|item| item.content.as_str())
        .collect();

    let write_result = write_to_file(items, model.file_path.as_str());
    match write_result {
        Ok(_) => model.message = Some(UserMessage::Success("written to file".to_string())),
        Err(e) => {
            model.message = Some(UserMessage::Error(format!(
                "couldn't write to file; error: {}",
                e
            )))
        }
    }
    None
}

fn show_view(model: &mut Model, view: View) -> Option<Message> {
    model.view = match model.view {
        View::Help => View::List,
        _ => view,
    };
    None
}

fn go_back_or_quit(model: &mut Model) -> Option<Message> {
    match model.view {
        View::List => {
            if model.save_on_exit {
                let _ = save_selection(model);
            }
            model.running_state = RunningState::Done;
        }
        View::Help => model.view = View::List,
    };

    None
}
