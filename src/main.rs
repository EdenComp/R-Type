use crate::random::Random;
use std::io::{stdin, BufRead};
use std::process::ExitCode;
use std::time::{SystemTime, UNIX_EPOCH};

mod brain;
mod constants;
mod game;
mod handler;
mod random;

fn main() -> ExitCode {
    // eprintln!("There is {} cores", std::thread::available_parallelism().unwrap().get());
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Error getting time");
    let random = Random::new(millis.as_millis());
    let mut handler = handler::GameHandler::new(random);
    let mut done = false;

    while !done {
        match stdin().lock().lines().next() {
            Some(Ok(line)) => {
                done = handler.handle_line(line);
            }
            Some(Err(e)) => {
                // eprintln!("Error: {}", e);
                return ExitCode::from(84);
            }
            None => {
                done = true;
            }
        }
    }
    ExitCode::from(0)
}
