

mod ui;
mod desc;
use std::process::Command;
use cursive::CursiveExt;
use ui::UI;


use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    path: Option<String>,
}

fn main() {
    match Command::new("gh").output() {
        Ok(..) => {
            let args = Args::parse();
            let mut ui = UI::new();
            if let Some(path) = args.path {
                UI::open_repo_ui(&mut ui.base, &path, true);
                ui.base.run();
            } else {
                ui.run();
            }
        }

        Err(error) => {
            if let std::io::ErrorKind::NotFound = error.kind() {
                println!("I think the Github CLI is not installed! Install it here: https://cli.github.com/\nIf already installed, try updating your PATH, or refreshing your terminal's environment");
            } else {
                println!("A fatal error occured while trying to access the Github CLI! Error: {error}");
            }
        }
    }
}
