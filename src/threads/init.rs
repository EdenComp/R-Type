use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use crate::game::types::Simulation;
use crate::threads::{ThreadData, ThreadInfo, ThreadPool};
use crate::threads::execution::thread_function;

impl ThreadPool {
    pub fn new() -> ThreadPool {
        let cores = match thread::available_parallelism() {
            Ok(cores) => cores.get(),
            Err(_) => 1
        }.max(1);

        let global_arc = Arc::new((Mutex::new(()), Condvar::new()));
        let mut threads = Vec::with_capacity(cores);

        for i in 0..cores {
            threads.push(ThreadData::new(i as u8, &global_arc));
        }

        ThreadPool {
            threads,
            global_arc,
        }
    }

    pub fn stop_threads(&mut self) {
        for thread in self.threads.iter() {
            thread.local_arc.0.lock().expect("Error locking mutex").exit = true;
            thread.local_arc.1.notify_one();
        }
        for thread in self.threads.drain(..) {
            thread.thread.join().expect("Error joining thread");
        }
    }
}

impl ThreadData {
    pub fn new(id: u8, global_arc: &Arc<(Mutex<()>, Condvar)>) -> ThreadData {
        let local_arc = Arc::new((Mutex::new(ThreadInfo::new(id)), Condvar::new()));
        let local_clone = Arc::clone(&local_arc);
        let global_clone = Arc::clone(global_arc);

        ThreadData {
            local_arc,
            thread: thread::spawn(move || {
                thread_function(local_clone, global_clone);
            }),
        }
    }
}

impl ThreadInfo {
    pub fn new(id: u8) -> ThreadInfo {
        ThreadInfo {
            id,
            exit: false,
            position: (0, 0),
            simulation: Simulation::new((0, 0)),
        }
    }
}
