use crate::game::GameData;
use crate::random::Random;
use crate::threads::execution::thread_function;
use crate::threads::{SharedData, ThreadPool};
use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

impl ThreadPool {
    pub fn new() -> ThreadPool {
        let cores = match thread::available_parallelism() {
            Ok(cores) => cores.get(),
            Err(_) => 1,
        }
        .clamp(1, 8);

        let global_arc = Arc::new((
            Mutex::new(SharedData::new()),
            Condvar::new(),
            Condvar::new(),
        ));
        let mut threads = Vec::with_capacity(cores);

        for _ in 0..cores {
            let local_arc = Arc::clone(&global_arc);
            let thread = thread::spawn(move || {
                thread_function(local_arc);
            });

            threads.push(thread);
        }

        ThreadPool {
            arc: global_arc,
            threads,
            cores,
        }
    }

    pub fn stop_threads(&mut self) {
        self.arc.0.lock().expect("Error locking mutex").exit = true;
        self.arc.1.notify_all();

        for thread in self.threads.drain(..) {
            thread.join().expect("Error joining thread");
        }
    }
}

impl SharedData {
    pub fn new() -> SharedData {
        SharedData {
            exit: false,
            game: GameData::new(Random::new(0)),
            queue: VecDeque::new(),
            results: Vec::new(),
        }
    }
}
