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

#[cfg(test)]
mod tests {
    use crate::game::GameData;
    use crate::random::Random;

    #[test]
    fn test_get_first_move() {
        let rand = Random::new(0);
        let game_data = GameData::new(rand);

        assert_eq!(game_data.get_first_move(), (10, 10));
    }
}
