use crate::game::GameEnd;
use crate::handler::GameHandler;

impl GameHandler {
    pub fn simulate_random_game(&mut self, mut turn: bool) -> GameEnd {
        let mut winning = false;
        let mut position;
        let mut simulation_turns = self.turns;

        while !winning && simulation_turns < self.max_turns {
            position = self.get_next_random_pos();
            self.table[position.0 as usize][position.1 as usize] = if turn { 1 } else { 2 };
            turn = !turn;
            simulation_turns += 1;
            winning = self.is_move_winning(&position);
        }
        self.table.copy_from_slice(&self.state);

        if simulation_turns == self.max_turns {
            return GameEnd::Draw;
        }
        if turn {
            GameEnd::Defeat
        } else {
            GameEnd::Victory
        }
    }

    fn get_next_random_pos(&mut self) -> (i8, i8) {
        let mut index = self.random.range(0, self.remaining_turns);
        let mut pos = self.empty_positions[index];

        while self.table[pos.0 as usize][pos.1 as usize] != 0 {
            index = self.random.range(0, self.remaining_turns);
            pos = self.empty_positions[index];
        }
        pos
    }
}
