use crate::constants;
use crate::game::types::{NestedSimulation, Simulation};
use crate::game::GameData;
use crate::threads::{SharedData, ThreadPool};
use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn thread_function(local_arc: Arc<(Mutex<SharedData>, Condvar, Condvar)>) {
    let (lock, cvar, _) = &*local_arc;
    let mut info;
    let mut current_turn;

    loop {
        info = lock.lock().expect("Error locking mutex");
        info = cvar
            .wait(info)
            .expect("Error waiting for condition variable");
        current_turn = info.game.turns;
        drop(info);
        if !retrieve_simulations(&local_arc, current_turn) {
            return;
        }
    }
}

fn retrieve_simulations(local_arc: &Arc<(Mutex<SharedData>, Condvar, Condvar)>, turn: i32) -> bool {
    let (lock, _, cvar_main) = &**local_arc;
    let mut info = lock.lock().expect("Error locking mutex");

    if info.exit {
        return false;
    }
    let mut game = info.game.clone();
    game.random.refresh();
    match info.queue.pop_front() {
        None => {
            cvar_main.notify_one();
            true
        }
        Some(mut simulation) => {
            drop(info);
            execute_simulations(&mut simulation, &mut game);
            info = lock.lock().expect("Error locking mutex");
            if turn != info.game.turns {
                println!("NOT IN CURRENT TURN");
                cvar_main.notify_one();
                return true;
            }
            info.results.push(simulation);
            drop(info);
            retrieve_simulations(local_arc, turn)
        }
    }
}

fn execute_simulations(simulation: &mut Simulation, game: &mut GameData) {
    simulation.nested.iter_mut().for_each(|simulation_t1| {
        game.simulate_games(simulation_t1, &simulation.self_simulation.next_move);
        simulation_t1.calculate_percentages();
    });
    game.combine_results(simulation);
    simulation.self_simulation.calculate_percentages();
}

impl ThreadPool {
    pub fn launch_simulations(
        &mut self,
        game: &GameData,
        simulations: VecDeque<Simulation>,
    ) -> Option<Vec<Simulation>> {
        let mut info = self.arc.0.lock().expect("Error locking mutex");
        let size = simulations.len();
        let millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Error getting time")
            .as_millis() as u64;
        let mut delay = constants::MAX_SIMULATIONS_MILLIS;

        info.game = game.clone();
        info.results = Vec::new();
        info.queue = simulations;
        drop(info);
        self.arc.1.notify_all();

        for _ in 1..self.threads.len() {
            let lock = self.arc.0.lock().expect("Error locking mutex");
            let info = self
                .arc
                .2
                .wait_timeout(lock, Duration::from_millis(delay))
                .expect("Error waiting for condition variable");

            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Error getting time");
            delay = constants::MAX_SIMULATIONS_MILLIS - (now.as_millis() as u64 - millis);
            if info.1.timed_out() {
                return None;
            }
            if info.0.results.len() == size {
                break;
            }
        }

        let results = Vec::new();
        Some(std::mem::replace(
            &mut self.arc.0.lock().expect("Error locking mutex").results,
            results,
        ))
    }
}

impl GameData {
    fn simulate_games(&mut self, simulation_t1: &mut NestedSimulation, ai_pos: &(i8, i8)) {
        for _ in 0..simulation_t1.simulations {
            self.table[ai_pos.0 as usize][ai_pos.1 as usize] = 1;
            self.table[simulation_t1.next_move.0 as usize][simulation_t1.next_move.1 as usize] = 2;
            simulation_t1.add_game(self.simulate_random_game(true));
        }
    }

    fn combine_results(&mut self, simulation_t0: &mut Simulation) {
        for i in 0..simulation_t0.nested.len() {
            simulation_t0.self_simulation.games.0 += simulation_t0.nested[i].games.0;
            simulation_t0.self_simulation.games.1 += simulation_t0.nested[i].games.1;
            simulation_t0.self_simulation.games.2 += simulation_t0.nested[i].games.2;
        }
    }
}
