use crate::handler::GameHandler;

impl GameHandler {
    pub fn is_move_winning(&self, pos: (i8, i8)) -> bool {
        let mut array: [i8; 9] = [0; 9];

        for i in 0i8..9 {
            if i == 4 {
                continue;
            }
            array[i as usize] = self.check_line(pos, (i % 3 - 1, i / 3 - 1), 1);
        }
        for i in 0..4 {
            if array[i] + array[8 - i] >= 4 {
                return true;
            }
        }
        false
    }

    fn check_line(&self, pos: (i8, i8), vec: (i8, i8), coef: i8) -> i8 {
        let position = (pos.0 + vec.0 * coef, pos.1 + vec.1 * coef);

        if position.0 < 0
            || position.1 < 0
            || position.0 >= self.size.0
            || position.1 >= self.size.1
        {
            return 0;
        }
        if self.table[position.0 as usize][position.1 as usize]
            != self.table[pos.0 as usize][pos.1 as usize]
        {
            return 0;
        }
        1 + self.check_line(pos, vec, coef + 1)
    }
}

#[cfg(test)]
mod tests {
    use crate::handler::GameHandler;
    use crate::random::Random;

    #[test]
    fn test_basic() {
        let mut rand = Random::new(0);
        let mut game = GameHandler::new(rand);

        game.table[0][0] = 1;
        assert_eq!(game.is_move_winning((0, 0)), false);
    }

    #[test]
    fn test_winning() {
        let rand = Random::new(0);
        let mut game = GameHandler::new(rand);

        game.table[0][0] = 1;
        game.table[1][0] = 1;
        game.table[2][0] = 1;
        game.table[3][0] = 1;
        game.table[4][0] = 1;
        assert_eq!(game.is_move_winning((2, 0)), true);
    }

    #[test]
    fn test_winning_diagonal_end() {
        let rand = Random::new(0);
        let mut game = GameHandler::new(rand);

        game.table[0][0] = 1;
        game.table[1][1] = 1;
        game.table[2][2] = 1;
        game.table[3][3] = 1;
        game.table[4][4] = 1;
        assert_eq!(game.is_move_winning((4, 4)), true);
    }

    #[test]
    fn test_other_player() {
        let rand = Random::new(0);
        let mut game = GameHandler::new(rand);

        game.table[0][0] = 1;
        game.table[1][0] = 1;
        game.table[2][0] = 2;
        game.table[3][0] = 1;
        game.table[4][0] = 1;
        assert_eq!(game.is_move_winning((3, 0)), false);
    }

    #[test]
    fn test_four_pieces() {
        let rand = Random::new(0);
        let mut game = GameHandler::new(rand);

        game.table[0][0] = 1;
        game.table[1][0] = 1;
        game.table[2][0] = 1;
        game.table[3][0] = 1;
        assert_eq!(game.is_move_winning((3, 0)), false);
    }
}
