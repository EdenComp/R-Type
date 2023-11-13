use std::io::{stdin, BufRead};
use std::process::ExitCode;

mod brain;
mod constants;
mod game;
mod handler;

fn main() -> ExitCode {
    let mut handler = handler::GameHandler::new();
    let mut done = false;

    while !done {
        match stdin().lock().lines().next() {
            Some(Ok(line)) => {
                done = handler.handle_line(line);
            }
            Some(Err(e)) => {
                eprintln!("Error: {}", e);
                return ExitCode::from(84);
            }
            None => {
                done = true;
            }
        }
    }
    ExitCode::from(0)
}
