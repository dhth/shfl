mod common;
mod model;
mod update;
mod utils;
mod view;

use anyhow::Context;
use clap::Parser;
use common::UNEXPECTED_ERROR_MESSAGE;
use model::{Items, Model, RunningState, UserMessage};
use std::fs::File;
use update::{handle_event, update};
use utils::read_from_file;
use view::view;

/// shfl helps you easily rearrange lines in a file with simple keymaps
#[derive(Parser, Debug)]
#[command(about, long_about=None)]
struct Args {
    /// File path
    #[arg(value_name = "STRING")]
    path: String,
    /// If set, shfl will save the new order of lines on exit
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
    terminal.clear().context(UNEXPECTED_ERROR_MESSAGE)?;

    let mut model = Model {
        running_state: RunningState::Running,
        file_path: args.path,
        todo_list: Items::from(&lines),
        message: None,
        save_on_exit: args.save_on_exit,
    };

    while model.running_state != RunningState::Done {
        terminal
            .draw(|f| view(&mut model, f))
            .context(UNEXPECTED_ERROR_MESSAGE)?;
        let mut current_msg = handle_event(&model).context(UNEXPECTED_ERROR_MESSAGE)?;

        while current_msg.is_some() {
            current_msg = update(&mut model, current_msg.unwrap());
        }
    }

    ratatui::try_restore().context(UNEXPECTED_ERROR_MESSAGE)?;
    if let Some(UserMessage::Error(msg)) = &model.message {
        println!("error: {}", msg);
    }

    Ok(())
}
