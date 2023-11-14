use crate::handler::GameHandler;
use rand::{thread_rng, Rng};
use std::cmp::{max, min};
use std::thread;
use std::time::Duration;

impl GameHandler {
    pub fn get_next_move(&mut self) -> (i8, i8) {
        if self.turns == 0 {
            return self.get_first_move();
        }

        let mut rng = thread_rng();
        let positions = self.get_positions_to_test();
        let index = rng.gen_range(0..positions.len());

        thread::sleep(Duration::from_millis(1000));
        positions[index]
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
