use crate::handler::GameHandler;
use crate::simulation::Simulation;
use crate::game::GameEnd::{Defeat, Draw, Victory};
use crate::constants;
use rand::{thread_rng, Rng};
use std::cmp::{max, min};
use std::process::exit;
use std::thread;
use std::time::Duration;


impl GameHandler {
    pub fn get_next_move(&mut self) -> (i8, i8) {
        if self.turns == 0 {
            return self.get_first_move();
        }

        let mut rng = thread_rng();
        let mut positions = self.get_positions_to_test();
        let index = self.simule_next_move(&mut positions);
        // let index = rng.gen_range(0..positions.len());

        thread::sleep(Duration::from_millis(1000));
        positions[index]
    }

    fn display_map(&self, table: &[[i8; 20]; 20]) {
        for x in 0..self.size.0 {
            for y in 0..self.size.1 {
                print!("{}", table[x as usize][y as usize])
            }
            println!(" ")
        }
        println!(" ")
    }

    fn simule_win(&mut self, simulation_t1: &mut Simulation) {
        for _ in 0..constants::NBR_SIMULATION {
            let index = self.simulate_random_game(true);
            match index {
                Victory => simulation_t1.games.0 += 1,
                Defeat => simulation_t1.games.1 += 1,
                Draw => simulation_t1.games.2 += 1,
            }
        }
    }

    fn average_game(&mut self, simulation_t0: &mut Simulation) { 
        let mut game = (0, 0, 0);
        
        for i in 0 .. simulation_t0.nested.len() {
            game.0 += simulation_t0.nested[i].games.0;
            game.1 += simulation_t0.nested[i].games.1;
            game.2 += simulation_t0.nested[i].games.2;
        }
        simulation_t0.games = game;
    }

    fn average_percentage(&mut self, simulation_t0: &mut Simulation) {
        let mut percentage = (0.0, 0.0, 0.0);

        for i in 0 .. simulation_t0.nested.len() {
            percentage.0 += simulation_t0.nested[i].percentages.0;
            percentage.1 += simulation_t0.nested[i].percentages.1;
            percentage.2 += simulation_t0.nested[i].percentages.2;
        }
        simulation_t0.percentages.0 = percentage.0 / simulation_t0.nested.len() as f32;
        simulation_t0.percentages.1 = percentage.1 / simulation_t0.nested.len() as f32;
        simulation_t0.percentages.2 = percentage.2 / simulation_t0.nested.len() as f32;
    }

    fn display_vec_simulation(&self, vec_simulation: &Vec<Simulation>) {
        for i in 0..vec_simulation.len() {
            println!("next move: {:?}", vec_simulation[i].next_move);
            println!("games: {:?}", vec_simulation[i].games);
            println!("percentages: {:?}", vec_simulation[i].percentages);
            println!(" ")
        }
    }

    fn simule_next_move(&mut self, positions: &Vec <(i8, i8)>) -> usize {

        self.display_map(&self.table);
        let mut vec_simulation: Vec<Simulation> = Vec::new();

        for i in 0..positions.len() {
            let mut simulation_t0 = Simulation::new(positions[i]);
            self.table[positions[i].0 as usize][positions[i].1 as usize] = 1;
            let pos_first_complexity = self.get_positions_to_test();

            for k in 0..pos_first_complexity.len() {
                self.table[pos_first_complexity[k].0 as usize][pos_first_complexity[k].1 as usize] = 1;
                let mut simulation_t1 = Simulation::new(pos_first_complexity[k]);    
                self.simule_win(&mut simulation_t1);

                simulation_t1.percentages.0 = (simulation_t1.games.0 / constants::NBR_SIMULATION) as f32 * 100.0;
                simulation_t1.percentages.1 = (simulation_t1.games.1 / constants::NBR_SIMULATION) as f32 * 100.0;
                simulation_t1.percentages.2 = (simulation_t1.games.2 / constants::NBR_SIMULATION) as f32 * 100.0;

                simulation_t0.nested.push(simulation_t1);
            }
            self.average_game(&mut simulation_t0);
            self.average_percentage(&mut simulation_t0);
            vec_simulation.push(simulation_t0);
        }
        self.display_vec_simulation(&vec_simulation);
        0
    }

    fn get_first_move(&self) -> (i8, i8) {
        (self.size.0 / 2, self.size.1 / 2)
    }

    fn get_positions_to_test(&mut self) -> Vec<(i8, i8)> {
        let mut vec: Vec<(i8, i8)> = Vec::new();
        let mut data;

        for x in 0..self.size.0 {
            for y in 0..self.size.1 {
                data = self.table[x as usize][y as usize];
                if data == 1 || data == 2 {
                    self.append_positions_to_vec(&mut vec, (x, y));
                }
            }
        }

        vec.iter().for_each(|pos| {
            self.table[pos.0 as usize][pos.1 as usize] = 0;
        });
        vec
    }

    fn append_positions_to_vec(&mut self, vec: &mut Vec<(i8, i8)>, pos: (i8, i8)) {
        let min_x = max(pos.0 - 1, 0);
        let max_x = min(pos.0 + 1, self.size.0 - 1);
        let min_y = max(pos.1 - 1, 0);
        let max_y = min(pos.1 + 1, self.size.1 - 1);

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                if self.table[x as usize][y as usize] == 0 {
                    vec.push((x, y));
                    self.table[x as usize][y as usize] = 3;
                }
            }
        }
    }
}
