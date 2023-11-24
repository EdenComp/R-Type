use crate::random::Random;

mod analyse;
mod random;
mod scan;
mod simulation;
pub mod types;
mod win;

#[derive(Clone)]
pub struct GameData {
    pub table: [[i8; 20]; 20],
    pub state: [[i8; 20]; 20],
    pub size: (i8, i8),
    pub turns: i32,
    pub max_turns: i32,
    pub remaining_turns: usize,
    pub random: Random,
    pub empty_positions: Vec<(i8, i8)>,
}

impl GameData {
    pub fn new(random: Random) -> GameData {
        let mut empty_positions: Vec<(i8, i8)> = Vec::new();
        for x in 0..20 {
            for y in 0..20 {
                empty_positions.push((x, y));
            }
        }

        GameData {
            size: (20, 20),
            table: [[0i8; 20]; 20],
            state: [[0i8; 20]; 20],
            turns: 0,
            remaining_turns: 400,
            max_turns: 400,
            random,
            empty_positions,
        }
    }
}

pub enum GameEnd {
    Victory,
    Defeat,
    Draw,
}

#[cfg(test)]
mod tests {
    use crate::game::GameData;
    use crate::random::Random;

    #[test]
    fn test_new() {
        let rand = Random::new(0);
        let game_data = GameData::new(rand);

        assert_eq!(game_data.size, (20, 20));
        assert_eq!(game_data.table, [[0i8; 20]; 20]);
        assert_eq!(game_data.state, [[0i8; 20]; 20]);
        assert_eq!(game_data.turns, 0);
        assert_eq!(game_data.remaining_turns, 400);
        assert_eq!(game_data.max_turns, 400);
        assert_eq!(game_data.empty_positions.len(), 400);
    }
}
