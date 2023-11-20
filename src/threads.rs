use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use crate::game::types::Simulation;

mod init;
mod execution;

pub struct ThreadPool {
    global_arc: Arc<(Mutex<()>, Condvar)>,
    threads: Vec<ThreadData>,
}

pub struct ThreadData {
    pub local_arc: Arc<(Mutex<ThreadInfo>, Condvar)>,
    pub thread: thread::JoinHandle<()>,
}

pub struct ThreadInfo {
    pub id: u8,
    pub exit: bool,
    pub position: (i8, i8),
    pub simulation: Simulation,
}
