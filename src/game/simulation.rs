use crate::game::types::{NestedSimulation, Simulation};
use crate::game::GameData;
use crate::threads::ThreadPool;
use std::collections::VecDeque;

impl GameData {
    pub fn simulate_next_move(&mut self, pool: &mut ThreadPool) -> (i8, i8) {
        let ai_positions = self.get_positions_to_test();
        let mut simulations: VecDeque<Simulation> = VecDeque::new();

        match self.make_early_decision(&ai_positions) {
            Some(pos) => return pos,
            None => (),
        }
        for (idx, ai_pos) in ai_positions.iter().enumerate() {
            let mut simulation_t0 = Simulation::new(*ai_pos);
            for (idx_2, enemy_pos) in ai_positions.iter().enumerate() {
                if idx != idx_2 {
                    simulation_t0.nested.push(NestedSimulation::new(*enemy_pos));
                }
            }
            simulations.push_back(simulation_t0);
        }

        let results = pool.launch_simulations(self, simulations);
        ai_positions[self.analyze_best_move(&results)]
    }

    fn make_early_decision(&mut self, positions: &Vec<(i8, i8)>) -> Option<(i8, i8)> {
        for pos in positions.iter() {
            self.table[pos.0 as usize][pos.1 as usize] = 2;
            if self.is_move_winning(pos) {
                return Some(*pos);
            }
            self.table[pos.0 as usize][pos.1 as usize] = 1;
            if self.is_move_winning(pos) {
                return Some(*pos);
            }
            self.table[pos.0 as usize][pos.1 as usize] = 0;
        }
        None
    }
}
