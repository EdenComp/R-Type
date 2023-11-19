use crate::game::types::Simulation;
use crate::handler::GameHandler;

impl GameHandler {
    pub fn analyze_best_move(&mut self, vec_simulation: &[Simulation]) -> usize {
        let mut final_idx = 0;
        let mut max_ai = 0.0;

        for (index, simulation) in vec_simulation.iter().enumerate() {
            if simulation.self_simulation.percentages.0 > max_ai {
                max_ai = simulation.self_simulation.percentages.0;
                final_idx = index;
            }
        }
        final_idx
    }
}
