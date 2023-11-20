use std::sync::{Arc, Condvar, Mutex};
use crate::threads::ThreadInfo;

pub fn thread_function(local_arc: Arc<(Mutex<ThreadInfo>, Condvar)>, global_arc: Arc<(Mutex<()>, Condvar)>) {
    let (lock, cvar) = &*local_arc;
    let mut a ;

    loop {
        a = lock.lock().expect("Error locking mutex");
        a = cvar.wait(a).expect("Error waiting for condition variable");
        if a.exit {
            return;
        }

        drop(a);
    }
}