use crate::constants;
use crate::game::types::{NestedSimulation, Simulation};
use crate::game::GameData;
use crate::threads::ThreadPool;
use std::cmp::min;
use std::collections::VecDeque;

impl GameData {
    pub fn simulate_next_move(&mut self, pool: &mut ThreadPool) -> (i8, i8) {
        let ai_positions = self.get_positions_to_test();
        let mut simulations: VecDeque<Simulation> = VecDeque::new();
        let size = ai_positions.len();
        let total = self.get_simulations_per_combination(size, pool.cores);

        if let Some(pos) = self.make_early_decision(&ai_positions) {
            return pos;
        }
        for (idx, ai_pos) in ai_positions.iter().enumerate() {
            let mut simulation_t0 = Simulation::new(*ai_pos, (size - 1) * total);
            let mut enemy_positions = ai_positions.clone();
            enemy_positions.remove(idx);

            for enemy_pos in enemy_positions.iter() {
                simulation_t0
                    .nested
                    .push(NestedSimulation::new(*enemy_pos, total));
            }
            simulations.push_back(simulation_t0);
        }

        match pool.launch_simulations(self, simulations) {
            Some(results) => {
                let best_move = self.analyze_best_move(&results);
                results[best_move].self_simulation.next_move
            }
            None => ai_positions[self.random.range(0, size)],
        }
    }

    fn get_simulations_per_combination(&self, size: usize, cores: usize) -> usize {
        let total_combinations = size * (size - 1);
        let simulations_per_combination =
            constants::MAX_SIMULATIONS_PER_THREAD * cores / total_combinations;

        min(
            simulations_per_combination,
            constants::MAX_SIMULATIONS_PER_COMBINATION,
        )
    }

    fn make_early_decision(&mut self, positions: &[(i8, i8)]) -> Option<(i8, i8)> {
        let mut avoid_lose: Option<(i8, i8)> = None;

        for pos in positions.iter() {
            self.table[pos.0 as usize][pos.1 as usize] = 1;
            if self.is_move_winning(pos) {
                return Some(*pos);
            }
            self.table[pos.0 as usize][pos.1 as usize] = 2;
            if self.is_move_winning(pos) {
                avoid_lose = Some(*pos);
            }
            self.table[pos.0 as usize][pos.1 as usize] = 0;
        }
        avoid_lose
    }
}
