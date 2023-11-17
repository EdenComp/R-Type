use crate::game::GameEnd;
use crate::handler::GameHandler;

impl GameHandler {
    pub fn simulate_random_game(&mut self, ai_pos: (i8, i8), enemy_pos: (i8, i8), mut turn: bool) -> GameEnd {
        let ai_won = self.is_move_winning(ai_pos);
        if ai_won {
            return GameEnd::Victory;
        }
        let enemy_won = self.is_move_winning(enemy_pos);
        if enemy_won {
            return GameEnd::Defeat;
        }

        let mut winning = false;
        let mut pos: (i8, i8);
        let mut turns: i32 = self.turns;

        while !winning {
            if turns == self.max_turns {
                return GameEnd::Draw;
            }
            pos = self.get_random_move();
            self.table[pos.0 as usize][pos.1 as usize] = if turn { 1 } else { 2 };
            turns += 1;
            turn = !turn;
            winning = self.is_move_winning(pos);
        }
        self.restore_table();
        if turn {
            GameEnd::Defeat
        } else {
            GameEnd::Victory
        }
    }

    fn get_random_move(&mut self) -> (i8, i8) {
        let mut x;
        let mut y;

        loop {
            x = self.random.range_i8(0, self.size.0);
            y = self.random.range_i8(0, self.size.1);
            if self.table[x as usize][y as usize] == 0 {
                return (x, y);
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
