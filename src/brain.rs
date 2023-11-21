use crate::game::GameData;
use crate::threads::ThreadPool;

impl GameData {
    pub fn get_next_move(&mut self, pool: &mut ThreadPool) -> (i8, i8) {
        if self.turns == 0 {
            return self.get_first_move();
        }
        self.simulate_next_move(pool)
    }

    fn get_first_move(&self) -> (i8, i8) {
        (self.size.0 / 2, self.size.1 / 2)
    }
}
