use crate::game::types::Simulation;
use crate::game::GameData;
use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

mod execution;
mod init;

pub struct ThreadPool {
    arc: Arc<(Mutex<SharedData>, Condvar, Condvar)>,
    threads: Vec<thread::JoinHandle<()>>,
}

pub struct SharedData {
    pub exit: bool,
    pub game: GameData,
    pub queue: VecDeque<Simulation>,
    pub results: Vec<Simulation>,
}
