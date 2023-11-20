use crate::random::Random;
use std::io::{stdin, BufRead};
use std::process::ExitCode;
use std::time::{SystemTime, UNIX_EPOCH};

mod brain;
mod constants;
mod game;
mod handler;
mod random;
mod threads;

fn main() -> ExitCode {
    let millis = SystemTime::now().duration_since(UNIX_EPOCH).expect("Error getting time");
    let random = Random::new(millis.as_millis());
    let mut handler = handler::GameHandler::new(random);
    let mut done = false;
    let mut code = 0;

    while !done {
        match stdin().lock().lines().next() {
            Some(Ok(line)) => {
                done = handler.handle_line(line);
            }
            Some(Err(e)) => {
                eprintln!("Error: {}", e);
                code = 84;
                done = true;
            }
            None => {
                done = true;
            }
        }
    }
    handler.thread_pool.stop_threads();
    ExitCode::from(code)
}
