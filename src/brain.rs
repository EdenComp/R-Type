use crate::handler::GameHandler;
use rand::{thread_rng, Rng};
use std::thread;
use std::time::Duration;

impl GameHandler {
    pub fn get_next_move(&self) -> (u8, u8) {
        let mut rng = thread_rng();
        let n1: u8 = rng.gen();
        let n2: u8 = rng.gen();

        thread::sleep(Duration::from_millis(1000));
        (n1 % 20, n2 % 20)
    }
}
