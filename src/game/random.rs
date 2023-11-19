use crate::game::GameEnd;
use crate::handler::GameHandler;
// use std::os::windows::thread;
use std::{thread, time::Duration};

impl GameHandler {

    // pub table: [[i8; 20]; 20],
    pub fn display_table(&self, table: [[i8; 20]; 20], size: (i8, i8)) {
        for x in 0..size.0 as usize {
            for y in 0..size.1 as usize {
                print!("{} ", table[x][y]);
            }
            println!();
        }
    }

    pub fn get_empty_pos_in_table(&mut self) {
        for x in 0..self.size.0 as usize {
            for y in 0..self.size.1 as usize {
                if self.table[x][y] == 0 {
                    self.vec_empty_pos.push((x as i8, y as i8));
                }
            }
        }
    }

    // pub fn display_empty_pos(&self) {
    //     for i in 0..self.vec_empty_pos.len() {
    //         println!("{:?}", self.vec_empty_pos[i]);
    //     }
    // }

    pub fn simulate_random_game(
        &mut self,
        ai_pos: &(i8, i8),
        enemy_pos: &(i8, i8),
        mut turn: bool,
    ) -> GameEnd {
        // let early_end = self.check_current_state(ai_pos, enemy_pos);
        // if early_end.is_some() {
        //     println!("early end");
        //     // thread::sleep(Duration::from_millis(100000));
        //     return early_end.unwrap();
        // }

        let mut winning = false;
        let mut pos;
        let mut index;
        self.get_empty_pos_in_table();

        while !winning {
            index = self.get_random_move();
            if index.is_err() {
                return GameEnd::Draw;
            }
            pos = self.vec_empty_pos[index.unwrap()];
            self.table[pos.0 as usize][pos.1 as usize] = if turn { 1 } else { 2 };
            turn = !turn;
            // self.display_table(self.table, self.size);
            winning = self.is_move_winning(pos);
            // thread::sleep(Duration::from_millis(1500));
            self.vec_empty_pos.remove(index.unwrap());
        }
        // println!("-------------------------1---------------------------1");
        // self.display_table(self.table, self.size);
        // println!("\n");
        self.restore_table();
        // print!("turn: {} ", turn);
        if turn {
            // println!("Defeat");
            // println!("-------------------------2---------------------------2");
            return GameEnd::Defeat
        } else {
            // println!("Victory");
            // println!("-------------------------2---------------------------2");
            return GameEnd::Victory
        }

    }

    fn check_current_state(&self, ai_pos: &(i8, i8), enemy_pos: &(i8, i8)) -> Option<GameEnd> {
        let ai_won = self.is_move_winning(*ai_pos);
        if ai_won {
            return Some(GameEnd::Victory);
        }
        let enemy_won = self.is_move_winning(*enemy_pos);
        if enemy_won {
            return Some(GameEnd::Defeat);
        }
        None
    }

    fn get_random_move(&mut self) -> Result<usize, ()> {
        let mut index = self.random.random_in_empty_pos(&self.vec_empty_pos);

        match index {
            Ok(index) => Ok(index),
            Err(_) => Err(()),
        }
    }

    fn restore_table(&mut self) {
        for x in 0..self.size.0 as usize {
            for y in 0..self.size.1 as usize {
                self.table[x][y] = self.state[x][y];
            }
        }
    }
}
