use crate::constants;
use crate::game::types::{NestedSimulation, Simulation};
use crate::handler::GameHandler;

impl GameHandler {
    pub fn simulate_next_move(&mut self) -> (i8, i8) {
        let ai_positions = self.get_positions_to_test();
        let mut vec_simulation: Vec<Simulation> = Vec::new();

        for ai_pos in ai_positions.iter() {
            let mut simulation_t0 = Simulation::new(*ai_pos);
            self.table[ai_pos.0 as usize][ai_pos.1 as usize] = 1;
            if self.is_move_winning(ai_pos) {
                return *ai_pos;
            }

            let enemy_positions = self.get_positions_to_test();
            for enemy_pos in enemy_positions.iter() {
                self.table[enemy_pos.0 as usize][enemy_pos.1 as usize] = 2;
                if self.is_move_winning(enemy_pos) {
                    return *enemy_pos;
                }

                let mut simulation_t1 = NestedSimulation::new(*enemy_pos);
                self.simulate_games(&mut simulation_t1, ai_pos);
                simulation_t1.calculate_percentages();
                simulation_t0.nested.push(simulation_t1);
            }

            self.combine_results(&mut simulation_t0);
            simulation_t0.self_simulation.calculate_percentages();
            vec_simulation.push(simulation_t0);
        }

        ai_positions[self.analyze_best_move(&vec_simulation)]
    }

    fn simulate_games(&mut self, simulation_t1: &mut NestedSimulation, ai_pos: &(i8, i8)) {
        for _ in 0..constants::SIMULATIONS_AMOUNT {
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
