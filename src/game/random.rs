use crate::game::GameEnd;
use crate::handler::GameHandler;
// use std::os::windows::thread;
use std::{thread, time::Duration};

impl GameHandler {

    // pub table: [[i8; 20]; 20],
    // pub fn display_table(&self, table: [[i8; 20]; 20], size: (i8, i8)) {
    //     for x in 0..size.0 as usize {
    //         for y in 0..size.1 as usize {
    //             print!("{} ", table[x][y]);
    //         }
    //         println!();
    //     }
    // }

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
        let early_end = self.check_current_state(ai_pos, enemy_pos);
        if early_end.is_some() {
            return early_end.unwrap();
        }

        let mut winning = false;
        let mut turns: i32 = self.turns;
        let mut pos;
        let mut index;
        self.get_empty_pos_in_table();

        while !winning {
            if turns == self.max_turns {
                return GameEnd::Draw;
            }
            index = self.get_random_move();
            pos = self.vec_empty_pos[index];
            self.table[pos.0 as usize][pos.1 as usize] = if turn { 1 } else { 2 };
            turns += 1;
            turn = !turn;
            winning = self.is_move_winning(pos);
            self.vec_empty_pos.remove(index);
        }
        self.restore_table();
        if turn {
            GameEnd::Defeat
        } else {
            GameEnd::Victory
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

    fn get_random_move(&mut self) -> usize {
        let mut index;

        loop {
            index = self.random.random_in_empty_pos(&self.vec_empty_pos);
            if index != 100000 {
                return index;
            }
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
