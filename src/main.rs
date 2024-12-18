mod common;
mod model;
mod update;
mod utils;
mod view;

use std::fs::File;
use update::update;
use view::view;

use anyhow::Context;
use clap::Parser;
use model::{Items, Model, RunningState, UserMessage};
use update::handle_event;
use utils::read_from_file;

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
