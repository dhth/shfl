use ratatui::{
    text::Line,
    widgets::{ListItem, ListState},
};

use crate::{common::PRIMARY_COLOR, utils::write_to_file};

#[derive(Debug)]
pub(crate) struct Model {
    pub(crate) running_state: RunningState,
    pub(crate) file_path: String,
    pub(crate) todo_list: Items,
    pub(crate) message: Option<UserMessage>,
    pub(crate) save_on_exit: bool,
}

#[derive(Debug)]
pub(crate) struct Items {
    pub(crate) items: Vec<Item>,
    pub(crate) state: ListState,
}

#[derive(Debug)]
pub(crate) struct Item {
    pub(crate) line: String,
    pub(crate) status: Selected,
}

#[derive(Debug)]
pub(crate) enum Selected {
    Yes,
    No,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) enum RunningState {
    #[default]
    Running,
    Done,
}

#[derive(PartialEq)]
pub(crate) enum Message {
    MoveToIndex(usize),
    GoToNextItem,
    GoToPreviousPreview,
    GoToFirstItem,
    GoToLastItem,
    SwitchWithNextItem,
    SwitchWithPreviousItem,
    SwitchWithFirstItem,
    ToggleSelection,
    SaveSelection,
    Quit,
}

#[derive(Debug)]
pub(crate) enum UserMessage {
    Success(String),
    Error(String),
}

impl UserMessage {
    pub(crate) fn value(&self) -> String {
        match self {
            UserMessage::Success(v) => v.clone(),
            UserMessage::Error(v) => v.clone(),
        }
    }
}

impl From<&Vec<String>> for Items {
    fn from(value: &Vec<String>) -> Self {
        let items = value
            .iter()
            .map(|line| Item::new(line, Selected::No))
            .collect();
        let state = ListState::default().with_selected(Some(0));

        Self { items, state }
    }
}

impl Item {
    fn new(line: &str, status: Selected) -> Self {
        Self {
            line: line.to_string(),
            status,
        }
    }

    fn toggle(&mut self) {
        match self.status {
            Selected::Yes => self.status = Selected::No,
            Selected::No => self.status = Selected::Yes,
        }
    }
}

impl From<&Item> for ListItem<'_> {
    fn from(value: &Item) -> Self {
        let line = match value.status {
            Selected::No => Line::from(value.line.clone()),
            Selected::Yes => Line::styled(value.line.clone(), PRIMARY_COLOR),
        };
        ListItem::new(line)
    }
}

impl Model {
    pub(crate) fn move_to_index(&mut self, index: usize) {
        if index > self.todo_list.items.len() - 1 {
            self.message = Some(UserMessage::Error("index is out of range".to_string()));
            return;
        }

        let current = self.todo_list.state.selected();
        if let Some(i) = current {
            if i == index {
                return;
            }
            let removed = self.todo_list.items.remove(i);
            self.todo_list.items.insert(index, removed);
            self.todo_list.state.select(Some(index));
        }
    }
    pub(crate) fn select_next(&mut self) {
        self.todo_list.state.select_next();
    }
    pub(crate) fn select_previous(&mut self) {
        self.todo_list.state.select_previous();
    }
    pub(crate) fn select_first(&mut self) {
        self.todo_list.state.select_first();
    }
    pub(crate) fn select_last(&mut self) {
        self.todo_list.state.select_last();
    }
    pub(crate) fn switch_with_next(&mut self) {
        let current = self.todo_list.state.selected();
        if let Some(i) = current {
            if i == self.todo_list.items.len() - 1 {
                return;
            }
            self.todo_list.items.swap(i, i + 1);
            self.todo_list.state.select_next();
        }
    }
    pub(crate) fn switch_with_previous(&mut self) {
        let current = self.todo_list.state.selected();
        if let Some(i) = current {
            if i == 0 {
                return;
            }
            self.todo_list.items.swap(i, i - 1);
            self.todo_list.state.select_previous();
        }
    }
    pub(crate) fn switch_with_first(&mut self) {
        let current = self.todo_list.state.selected();
        if let Some(i) = current {
            match i {
                0 => (),
                1 => self.todo_list.items.swap(0, 1),
                _ => {
                    self.todo_list.items[0..i + 1].rotate_right(1);
                }
            };
            self.todo_list.state.select_first();
        }
    }

    pub(crate) fn toggle_current(&mut self) {
        let current = self.todo_list.state.selected();
        if let Some(i) = current {
            self.todo_list.items[i].toggle();
        }
    }

    pub(crate) fn save_selection(&mut self) {
        let items: Vec<&str> = self
            .todo_list
            .items
            .iter()
            .map(|item| item.line.as_str())
            .collect();

        let write_result = write_to_file(items, self.file_path.as_str());
        match write_result {
            Ok(_) => self.message = Some(UserMessage::Success("written to file".to_string())),
            Err(e) => {
                self.message = Some(UserMessage::Error(format!(
                    "couldn't write to file; error: {}",
                    e
                )))
            }
        }
    }

    pub(crate) fn go_back_or_quit(&mut self) {
        if self.save_on_exit {
            self.save_selection();
        }
        self.running_state = RunningState::Done;
    }
}
