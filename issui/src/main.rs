mod ui;
mod desc;
use std::process::Command;
use ui::UI;

fn main() {
    match Command::new("gh").output() {
        Ok(..) => {
            let mut ui = UI::new();
            ui.run();
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
