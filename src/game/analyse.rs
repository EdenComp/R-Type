use crate::game::types::Simulation;
use crate::handler::GameHandler;

impl GameHandler {

    pub fn display_simulation(&self, vec_simulation: &Vec<Simulation>) {
        for i in 0..vec_simulation.len() {
            println!("{}: {:?}", i, vec_simulation[i].self_simulation.next_move);
            println!("{}: {:?}", i, vec_simulation[i].self_simulation.games);
            println!()
        }
    }

    pub fn analyze_best_move(&mut self, vec_simulation: &Vec<Simulation>) -> usize {

        self.display_simulation(vec_simulation);

        let mut index = (0, 0);
        let mut max_ai = 0.0;
        let mut max_opponent = 0.0;

        for i in 0..vec_simulation.len() {
            if vec_simulation[i].self_simulation.percentages.0 > max_ai {
                max_ai = vec_simulation[i].self_simulation.percentages.0;
                index.0 = i;
            }
            if vec_simulation[i].self_simulation.percentages.1 > max_opponent {
                max_opponent = vec_simulation[i].self_simulation.percentages.1;
                index.1 = i;
            }
        }
        if max_opponent == 100.0 {
            return index.1 as usize;
        }
        if max_ai > max_opponent {
            return index.0 as usize;
        }
        index.0 as usize
    }
}
