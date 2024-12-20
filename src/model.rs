use crate::common::{View, SELECTED_COLOR};
use ratatui::{
    style::Style,
    text::Line,
    widgets::{ListItem, ListState},
};

#[derive(Debug)]
pub(crate) struct Model {
    pub(crate) view: View,
    pub(crate) running_state: RunningState,
    pub(crate) file_path: String,
    pub(crate) lines: Lines,
    pub(crate) selected_count: usize,
    pub(crate) message: Option<UserMessage>,
    pub(crate) save_on_exit: bool,
}

impl Model {
    pub(crate) fn default(file_path: String, lines: &Vec<String>, save_on_exit: bool) -> Self {
        Self {
            view: View::List,
            running_state: RunningState::Running,
            file_path,
            lines: Lines::from(lines),
            selected_count: 0,
            message: None,
            save_on_exit,
        }
    }
}

#[derive(Debug)]
pub(crate) struct Lines {
    pub(crate) items: Vec<LineItem>,
    pub(crate) state: ListState,
}

#[derive(Debug, Clone)]
pub(crate) struct LineItem {
    pub(crate) content: String,
    pub(crate) status: bool,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) enum RunningState {
    #[default]
    Running,
    Done,
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

impl From<&Vec<String>> for Lines {
    fn from(value: &Vec<String>) -> Self {
        let items = value
            .iter()
            .map(|line| LineItem::new(line, false))
            .collect();
        let state = ListState::default().with_selected(Some(0));

        Self { items, state }
    }
}

impl LineItem {
    fn new(line: &str, status: bool) -> Self {
        Self {
            content: line.to_string(),
            status,
        }
    }

    pub(crate) fn toggle(&mut self) -> bool {
        match self.status {
            true => {
                self.status = false;
                false
            }
            false => {
                self.status = true;
                true
            }
        }
    }
}

impl From<&LineItem> for ListItem<'_> {
    fn from(value: &LineItem) -> Self {
        let line = match value.status {
            false => Line::from(value.content.clone()),
            true => Line::styled(
                format!("> {}", value.content.clone()),
                Style::new().fg(SELECTED_COLOR),
            ),
        };
        ListItem::new(line)
    }
}
