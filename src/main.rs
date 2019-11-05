use std::{env, process};

use anyhow::Context as _;
use i3ipc::I3Connection;

enum Command {
    Prev,
    Next,
}

fn usage() -> ! {
    println!("Usage: {} <prev | next>", env::args().next().unwrap());
    process::exit(1)
}

fn main() -> anyhow::Result<()> {
    let mut args = env::args();
    let command = match args.nth(1) {
        Some(ref arg) if arg == "prev" => Command::Prev,
        Some(ref arg) if arg == "next" => Command::Next,
        _ => usage(),
    };

    let mut i3conn = I3Connection::connect()?;

    let workspaces = i3conn.get_workspaces()?.workspaces;
    let focused_ws = workspaces.iter().find(|ws| ws.focused).context("No focused workspace!")?;

    let mut same_output_workspaces = workspaces.iter().filter(|ws| ws.output == focused_ws.output);

    match command {
        Command::Prev => {
            if same_output_workspaces.next().context("no workspaces")?.name != focused_ws.name {
                i3conn.run_command("workspace prev_on_output")?;
            }
        }
        Command::Next => {
            if same_output_workspaces.last().context("no workspaces")?.name != focused_ws.name {
                i3conn.run_command("workspace next_on_output")?;
            }
        }
    };

    Ok(())
}
