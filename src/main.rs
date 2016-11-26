extern crate i3ipc;

use std::{env, process};
use i3ipc::I3Connection;

enum Command {
    Prev,
    Next,
}

fn usage() -> ! {
    println!("Usage: {} <prev | next>", env::args().next().unwrap());
    process::exit(1)
}

fn main() {
    let mut args = env::args();
    let command = match args.nth(1) {
        Some(ref arg) if arg == "prev" => Command::Prev,
        Some(ref arg) if arg == "next" => Command::Next,
        _ => usage(),
    };

    let mut i3conn = I3Connection::connect().unwrap();

    let workspaces = i3conn.get_workspaces().unwrap().workspaces;
    let focused_ws = workspaces.iter().find(|ws| ws.focused).expect("No focused workspace!");

    let mut same_output_workspaces = workspaces.iter().filter(|ws| ws.output == focused_ws.output);

    match command {
        Command::Prev => {
            if same_output_workspaces.next().unwrap().name != focused_ws.name {
                i3conn.command("workspace prev_on_output").unwrap();
            }
        }
        Command::Next => {
            if same_output_workspaces.last().unwrap().name != focused_ws.name {
                i3conn.command("workspace next_on_output").unwrap();
            }
        }
    };
}
