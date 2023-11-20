use crate::game::types::Simulation;
use crate::game::GameData;

impl GameData {
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

#[cfg(test)]

mod tests {
    use crate::handler::GameHandler;
    use crate::random::Random;
    use crate::game::types::Simulation;

    #[test]
    fn analyze_best_move() {
        let rand = Random::new(0);
        let mut game = GameHandler::new(rand);
        let mut vec_simulation = Vec::new();
        let mut simulation = Simulation::new((0, 0));
        let mut simulation2 = Simulation::new((0, 0));

        simulation.self_simulation.percentages = (50.0, 50.0, 0.0);
        simulation2.self_simulation.percentages = (51.0, 49.0, 0.0);
        vec_simulation.push(simulation);
        vec_simulation.push(simulation2);
        assert_eq!(game.analyze_best_move(&vec_simulation), 1);
    }
}