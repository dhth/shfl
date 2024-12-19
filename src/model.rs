use crate::common::PRIMARY_COLOR;
use ratatui::{
    text::Line,
    widgets::{ListItem, ListState},
};

#[derive(Debug)]
pub(crate) struct Model {
    pub(crate) view: View,
    pub(crate) running_state: RunningState,
    pub(crate) file_path: String,
    pub(crate) lines: Lines,
    pub(crate) message: Option<UserMessage>,
    pub(crate) save_on_exit: bool,
}

#[derive(Debug)]
pub(crate) struct Lines {
    pub(crate) items: Vec<LineItem>,
    pub(crate) state: ListState,
}

#[derive(Debug)]
pub(crate) struct LineItem {
    pub(crate) content: String,
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

#[derive(Debug)]
pub(crate) enum UserMessage {
    Success(String),
    Error(String),
}

#[derive(PartialEq, Debug)]
pub(crate) enum View {
    List,
    Help,
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
            .map(|line| LineItem::new(line, Selected::No))
            .collect();
        let state = ListState::default().with_selected(Some(0));

        Self { items, state }
    }
}

impl LineItem {
    fn new(line: &str, status: Selected) -> Self {
        Self {
            content: line.to_string(),
            status,
        }
    }

    pub(crate) fn toggle(&mut self) {
        match self.status {
            Selected::Yes => self.status = Selected::No,
            Selected::No => self.status = Selected::Yes,
        }
    }
}

impl From<&LineItem> for ListItem<'_> {
    fn from(value: &LineItem) -> Self {
        let line = match value.status {
            Selected::No => Line::from(value.content.clone()),
            Selected::Yes => Line::styled(value.content.clone(), PRIMARY_COLOR),
        };
        ListItem::new(line)
    }
}
