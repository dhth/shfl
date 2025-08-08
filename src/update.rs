use crate::common::View;
use crate::message::Message;
use crate::model::{LineItem, Model, RunningState, UserMessage};
use crate::utils::write_to_file;
use ratatui::crossterm::event::{self, Event, KeyCode};
use std::time::Duration;

pub(crate) fn handle_event(model: &Model) -> anyhow::Result<Option<Message>> {
    if event::poll(Duration::from_millis(16))?
        && let Event::Key(key) = event::read()?
        && key.kind == event::KeyEventKind::Press
    {
        return Ok(handle_key(model, key));
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
            KeyCode::Enter => Some(Message::MoveToTop),
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
        Message::MoveToTop => move_to_top(model),
        Message::ToggleSelection => toggle_current(model),
        Message::SaveSelection => save_selection(model),
        Message::ShowView(v) => show_view(model, v),
        Message::Quit => go_back_or_quit(model),
    }
}

fn move_to_index(model: &mut Model, index: usize) -> Option<Message> {
    if model.selected_count > 0 {
        model.message = Some(UserMessage::Error("remove selection first".to_string()));
        return None;
    }
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

fn move_selection_to_top(model: &mut Model) -> Option<Message> {
    if model.selected_count == model.lines.items.len() {
        model.lines.items.iter_mut().for_each(|i| {
            i.status = false;
        });
        model.selected_count = 0;
        return None;
    }

    let mut selected: Vec<LineItem> = Vec::with_capacity(model.selected_count);
    let mut unselected: Vec<LineItem> =
        Vec::with_capacity(model.lines.items.len() - model.selected_count);

    model.lines.items.iter().for_each(|i| match i.status {
        true => {
            let mut item = i.clone();
            item.status = false;
            selected.push(item);
        }
        false => unselected.push(i.clone()),
    });

    if selected.is_empty() {
        return None;
    }

    selected.extend(unselected);
    model.lines.items = selected;
    model.lines.state.select(Some(0));
    model.selected_count = 0;

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

fn move_to_top(model: &mut Model) -> Option<Message> {
    match model.selected_count {
        0 => move_item_to_top(model),
        _ => move_selection_to_top(model),
    }
}

fn move_item_to_top(model: &mut Model) -> Option<Message> {
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
        let selected = model.lines.items[i].toggle();
        if selected {
            model.selected_count += 1;
        } else {
            model.selected_count -= 1;
        };
    }
    select_next(model)
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
                "couldn't write to file; error: {e}"
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

fn unselect_selected_items(model: &mut Model) {
    model.lines.items.iter_mut().for_each(|i| {
        i.status = false;
    });
    model.selected_count = 0;
}

fn go_back_or_quit(model: &mut Model) -> Option<Message> {
    match model.view {
        View::List => match model.selected_count {
            0 => {
                if model.save_on_exit {
                    let _ = save_selection(model);
                }
                model.running_state = RunningState::Done;
            }
            _ => unselect_selected_items(model),
        },
        View::Help => model.view = View::List,
    };

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_item_to_top_works() {
        // GIVEN
        let lines: Vec<String> = (0..5).map(|n| n.to_string()).collect();
        let mut model = Model::default("file.txt".to_string(), &lines, false);

        model.lines.state.select(Some(2));

        // WHEN
        let message = move_item_to_top(&mut model);

        // THEN
        assert!(message.is_none());
        let content: Vec<&str> = model
            .lines
            .items
            .iter()
            .map(|i| i.content.as_str())
            .collect();

        assert_eq!(content, vec!["2", "0", "1", "3", "4"]);
    }

    #[test]
    fn move_to_index_works() {
        // GIVEN
        let lines: Vec<String> = (0..5).map(|n| n.to_string()).collect();
        let mut model = Model::default("file.txt".to_string(), &lines, false);

        model.lines.state.select(Some(4));

        // WHEN
        let message = move_to_index(&mut model, 1);

        // THEN
        assert!(message.is_none());
        let content: Vec<&str> = model
            .lines
            .items
            .iter()
            .map(|i| i.content.as_str())
            .collect();

        assert_eq!(content, vec!["0", "4", "1", "2", "3"]);
    }

    #[test]
    fn move_to_index_works_handles_an_index_out_of_bounds() {
        // GIVEN
        let lines: Vec<String> = (0..5).map(|n| n.to_string()).collect();
        let mut model = Model::default("file.txt".to_string(), &lines, false);

        model.lines.state.select(Some(4));

        // WHEN
        let message = move_to_index(&mut model, 6);

        // THEN
        assert!(message.is_none());
        assert!(model.message.is_some());
        let content: Vec<&str> = model
            .lines
            .items
            .iter()
            .map(|i| i.content.as_str())
            .collect();

        assert_eq!(content, vec!["0", "1", "2", "3", "4"]);
    }

    #[test]
    fn move_selection_to_top_works() {
        // GIVEN
        let lines: Vec<String> = (0..10).map(|n| n.to_string()).collect();
        let mut model = Model::default("file.txt".to_string(), &lines, false);
        model.lines.items[1..3]
            .iter_mut()
            .for_each(|i| i.status = true);
        model.lines.items[5].status = true;
        model.lines.items[7..9]
            .iter_mut()
            .for_each(|i| i.status = true);

        // WHEN
        let message = move_selection_to_top(&mut model);

        // THEN
        assert!(message.is_none());
        let content: Vec<&str> = model
            .lines
            .items
            .iter()
            .map(|i| i.content.as_str())
            .collect();

        assert_eq!(
            content,
            vec!["1", "2", "5", "7", "8", "0", "3", "4", "6", "9"]
        );
    }

    #[test]
    fn switch_with_next_works() {
        // GIVEN
        let lines: Vec<String> = (0..5).map(|n| n.to_string()).collect();
        let mut model = Model::default("file.txt".to_string(), &lines, false);
        model.lines.state.select(Some(2));

        // WHEN
        let message = switch_with_next(&mut model);

        // THEN
        assert!(message.is_none());
        let content: Vec<&str> = model
            .lines
            .items
            .iter()
            .map(|i| i.content.as_str())
            .collect();

        assert_eq!(content, vec!["0", "1", "3", "2", "4"]);
    }

    #[test]
    fn switch_with_previous_works() {
        // GIVEN
        let lines: Vec<String> = (0..5).map(|n| n.to_string()).collect();
        let mut model = Model::default("file.txt".to_string(), &lines, false);
        model.lines.state.select(Some(2));

        // WHEN
        let message = switch_with_previous(&mut model);

        // THEN
        assert!(message.is_none());
        let content: Vec<&str> = model
            .lines
            .items
            .iter()
            .map(|i| i.content.as_str())
            .collect();

        assert_eq!(content, vec!["0", "2", "1", "3", "4"]);
    }

    #[test]
    fn toggle_current_works() {
        // GIVEN
        let lines: Vec<String> = (0..5).map(|n| n.to_string()).collect();
        let mut model = Model::default("file.txt".to_string(), &lines, false);
        model.lines.state.select(Some(1));

        // WHEN
        let message_one = toggle_current(&mut model);
        let message_two = toggle_current(&mut model);
        let message_three = toggle_current(&mut model);

        // THEN
        assert!(message_one.is_none());
        assert!(message_two.is_none());
        assert!(message_three.is_none());
        let content: Vec<&str> = model
            .lines
            .items
            .iter()
            .map(|i| i.content.as_str())
            .collect();
        let statuses: Vec<bool> = model.lines.items.iter().map(|i| i.status).collect();

        assert_eq!(content, vec!["0", "1", "2", "3", "4"]);
        assert_eq!(statuses, vec![false, true, true, true, false]);
        assert_eq!(model.selected_count, 3);
        assert_eq!(model.lines.state.selected(), Some(4));
    }

    #[test]
    fn selection_can_be_reset() {
        // GIVEN
        let lines: Vec<String> = (0..5).map(|n| n.to_string()).collect();
        let mut model = Model::default("file.txt".to_string(), &lines, false);
        model.lines.state.select(Some(1));

        // WHEN
        let _ = toggle_current(&mut model);
        let _ = toggle_current(&mut model);
        let _ = toggle_current(&mut model);
        let message = go_back_or_quit(&mut model);

        // THEN
        assert!(message.is_none());
        let statuses: Vec<bool> = model.lines.items.iter().map(|i| i.status).collect();

        assert_eq!(statuses, vec![false, false, false, false, false]);
        assert_eq!(model.selected_count, 0);
        assert_eq!(model.lines.state.selected(), Some(4));
    }
}
