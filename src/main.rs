use anyhow::Context;
use clap::Parser;
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, List, ListDirection, ListItem, ListState, Padding},
    Frame,
};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Write;
use std::time::Duration;

const TITLE_FG_COLOR: Color = Color::from_u32(0x282828);
const PRIMARY_COLOR: Color = Color::from_u32(0xd3869b);
const TITLE: &str = " shfl ";

#[derive(Parser, Debug)]
#[command(about, long_about=None)]
struct Args {
    /// File path
    #[arg(value_name = "STRING")]
    path: String,
    /// Save on exit
    #[arg(short = 's', long = "save-on-exit", value_name = "STRING")]
    save_on_exit: bool,
}

#[derive(Debug)]
enum UserMessage {
    Success(String),
    Error(String),
}

impl UserMessage {
    fn value(&self) -> String {
        match self {
            UserMessage::Success(v) => v.clone(),
            UserMessage::Error(v) => v.clone(),
        }
    }
}

#[derive(Debug)]
struct Model {
    running_state: RunningState,
    file_path: String,
    todo_list: Items,
    message: Option<UserMessage>,
    save_on_exit: bool,
}

#[derive(Debug)]
struct Items {
    items: Vec<Item>,
    state: ListState,
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

#[derive(Debug)]
struct Item {
    line: String,
    status: Selected,
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

#[derive(Debug)]
enum Selected {
    Yes,
    No,
}

#[derive(Debug, Default, PartialEq, Eq)]
enum RunningState {
    #[default]
    Running,
    Done,
}

#[derive(PartialEq)]
enum Message {
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

impl Model {
    fn move_to_index(&mut self, index: usize) {
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
    fn select_next(&mut self) {
        self.todo_list.state.select_next();
    }
    fn select_previous(&mut self) {
        self.todo_list.state.select_previous();
    }
    fn select_first(&mut self) {
        self.todo_list.state.select_first();
    }
    fn select_last(&mut self) {
        self.todo_list.state.select_last();
    }
    fn switch_with_next(&mut self) {
        let current = self.todo_list.state.selected();
        if let Some(i) = current {
            if i == self.todo_list.items.len() - 1 {
                return;
            }
            self.todo_list.items.swap(i, i + 1);
            self.todo_list.state.select_next();
        }
    }
    fn switch_with_previous(&mut self) {
        let current = self.todo_list.state.selected();
        if let Some(i) = current {
            if i == 0 {
                return;
            }
            self.todo_list.items.swap(i, i - 1);
            self.todo_list.state.select_previous();
        }
    }
    fn switch_with_first(&mut self) {
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

    fn toggle_current(&mut self) {
        let current = self.todo_list.state.selected();
        if let Some(i) = current {
            self.todo_list.items[i].toggle();
        }
    }

    fn save_selection(&mut self) {
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

    fn go_back_or_quit(&mut self) {
        if self.save_on_exit {
            self.save_selection();
        }
        self.running_state = RunningState::Done;
    }
}

fn update(model: &mut Model, msg: Message) -> Option<Message> {
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

fn view(model: &mut Model, frame: &mut Frame) {
    let items: Vec<ListItem> = model.todo_list.items.iter().map(ListItem::from).collect();

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

    frame.render_stateful_widget(list, frame.area(), &mut model.todo_list.state)
}

fn handle_event(_: &Model) -> anyhow::Result<Option<Message>> {
    if event::poll(Duration::from_millis(16))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                return Ok(handle_key(key));
            }
        }
    }
    Ok(None)
}

fn handle_key(key: event::KeyEvent) -> Option<Message> {
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
        KeyCode::Char('j') => Some(Message::GoToNextItem),
        KeyCode::Char('k') => Some(Message::GoToPreviousPreview),
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

fn read_from_file(file: &File) -> Result<Vec<String>, std::io::Error> {
    let reader = BufReader::new(file);
    let lines = reader
        .lines()
        .collect::<Result<Vec<String>, std::io::Error>>()?;

    Ok(lines)
}

fn write_to_file(data: Vec<&str>, file_path: &str) -> Result<(), std::io::Error> {
    let mut file = File::options().write(true).truncate(true).open(file_path)?;

    let content = data.join("\n") + "\n";
    file.write(content.as_bytes()).map(|_| ())
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let file = File::open(&args.path)
        .with_context(|| format!("couldn't open file at the provided path: {}", &args.path))?;

    let lines = read_from_file(&file).with_context(|| {
        format!(
            "couldn't read data from file at the provided path: {}",
            &args.path
        )
    })?;

    let mut terminal = ratatui::init();
    terminal.clear()?;

    let mut model = Model {
        running_state: RunningState::Running,
        file_path: args.path,
        todo_list: Items::from(&lines),
        message: None,
        save_on_exit: args.save_on_exit,
    };

    while model.running_state != RunningState::Done {
        terminal.draw(|f| view(&mut model, f))?;
        let mut current_msg = handle_event(&model)?;

        while current_msg.is_some() {
            current_msg = update(&mut model, current_msg.unwrap());
        }
    }

    ratatui::try_restore()?;
    if let Some(UserMessage::Error(msg)) = &model.message {
        println!("error: {}", msg);
    }

    Ok(())
}
