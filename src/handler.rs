use crate::constants;
use crate::game::GameData;
use crate::random::Random;
use crate::threads::ThreadPool;
use std::collections::HashMap;

pub struct GameHandler {
    pub thread_pool: ThreadPool,
    game_data: GameData,
    board: bool,
    functions: HashMap<String, fn(&mut GameHandler, &str)>,
}

impl GameHandler {
    pub fn new(random: Random) -> GameHandler {
        let mut functions: HashMap<String, fn(&mut GameHandler, &str)> = HashMap::new();
        functions.insert(constants::ABOUT_COMMAND.to_string(), GameHandler::about);
        functions.insert(constants::BEGIN_COMMAND.to_string(), GameHandler::begin);
        functions.insert(constants::BOARD_COMMAND.to_string(), GameHandler::board);
        functions.insert(constants::END_COMMAND.to_string(), GameHandler::end);
        functions.insert(constants::INFO_COMMAND.to_string(), GameHandler::info);
        functions.insert(constants::START_COMMAND.to_string(), GameHandler::start);
        functions.insert(constants::TURN_COMMAND.to_string(), GameHandler::turn);

        GameHandler {
            thread_pool: ThreadPool::new(),
            game_data: GameData::new(random),
            board: false,
            functions,
        }
    }

    pub fn handle_line(&mut self, line: String) -> bool {
        if self.board {
            return self.handle_board(&line);
        }
        match line.split_once(' ') {
            Some((command, args)) => {
                self.handle_command(command, args);
            }
            None => {
                self.handle_command(&line, "");
            }
        }
        line == constants::END_COMMAND
    }

    fn handle_board(&mut self, line: &str) -> bool {
        if line == constants::BOARD_END {
            self.board = false;
            self.begin(line);
            return false;
        }
        match parse_board_position(line) {
            Some(pos) => {
                self.register_turn(pos.0, pos.1);
            }
            None => {
                self.error("Invalid position");
            }
        }
        line == constants::END_COMMAND
    }

    pub fn debug(&self, args: &str) {
        println!("{} {}", constants::DEBUG_RESPONSE, args);
    }

    pub fn error(&self, args: &str) {
        println!("{} {}", constants::ERROR_RESPONSE, args);
    }

    fn handle_command(&mut self, command: &str, args: &str) {
        match self.functions.get(command) {
            Some(function) => {
                function(self, args);
            }
            None => {
                println!("{}", constants::UNKNOWN_RESPONSE);
            }
        }
    }

    fn broadcast_turn(&self, pos: (i8, i8)) {
        println!("{},{}", pos.0, pos.1);
    }

    fn register_turn(&mut self, pos: (i8, i8), me: bool) {
        self.game_data.table[pos.0 as usize][pos.1 as usize] = if me { 1 } else { 2 };
        self.game_data.state[pos.0 as usize][pos.1 as usize] = if me { 1 } else { 2 };
        self.game_data.empty_positions.retain(|&x| x != pos);
        self.game_data.remaining_turns -= 1;
        self.game_data.turns += 1;
    }

    fn about(&mut self, _args: &str) {
        println!(
            "name=\"{}\", version=\"{}\", author=\"{}\", country=\"{}\"",
            constants::BRAIN_NAME,
            constants::BRAIN_VERSION,
            constants::BRAIN_AUTHOR,
            constants::BRAIN_COUNTRY
        );
    }

    fn begin(&mut self, _args: &str) {
        let new_move = self.game_data.get_next_move(&mut self.thread_pool);

        self.register_turn(new_move, true);
        self.broadcast_turn(new_move);
    }

    fn board(&mut self, _args: &str) {
        self.game_data.table.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|cell| {
                *cell = 0;
            })
        });
        self.game_data.turns = 0;
        self.board = true;
    }

    fn end(&mut self, _args: &str) {}

    fn info(&mut self, args: &str) {
        self.debug(args);
    }

    fn start(&mut self, args: &str) {
        match args.parse::<i8>() {
            Ok(size) => {
                if size != constants::DEFAULT_SIZE {
                    self.error("Invalid size");
                    return;
                }
                println!("{}", constants::OK_RESPONSE)
            }
            Err(_) => self.error("Invalid size"),
        }
    }

    fn turn(&mut self, args: &str) {
        match parse_position(args) {
            Some(pos) => {
                self.register_turn(pos, false);
            }
            None => {
                self.error("Invalid position");
                return;
            }
        }
        let new_move = self.game_data.get_next_move(&mut self.thread_pool);

        self.register_turn(new_move, true);
        self.broadcast_turn(new_move);
    }
}

fn parse_position(pos: &str) -> Option<(i8, i8)> {
    match pos.split_once(',') {
        Some(str) => {
            let (x, y) = str;
            match (x.parse::<i8>(), y.parse::<i8>()) {
                (Ok(x), Ok(y)) => {
                    if !(0..20).contains(&x) || !(0..20).contains(&y) {
                        return None;
                    }
                    Some((x, y))
                }
                _ => None,
            }
        }
        None => None,
    }
}

fn parse_board_position(pos: &str) -> Option<((i8, i8), bool)> {
    let arr: Vec<&str> = pos.split(',').collect();

    if arr.len() != 3 {
        return None;
    }
    match (
        arr[0].parse::<i8>(),
        arr[1].parse::<i8>(),
        arr[2].parse::<i8>(),
    ) {
        (Ok(x), Ok(y), Ok(p)) => {
            if p != 1 && p != 2 {
                return None;
            }
            if !(0..20).contains(&x) || !(0..20).contains(&y) {
                return None;
            }
            Some(((x, y), p == 1))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::handler::GameHandler;
    use crate::random::Random;

    #[test]
    fn test_register_turn() {
        let rand = Random::new(0);
        let mut game_data = GameHandler::new(rand);

        game_data.register_turn((0, 0), true);
        assert_eq!(game_data.game_data.table[0][0], 1);
        assert_eq!(game_data.game_data.state[0][0], 1);
        assert_eq!(game_data.game_data.empty_positions.len(), 399);
        assert_eq!(game_data.game_data.remaining_turns, 399);
        assert_eq!(game_data.game_data.turns, 1);
    }

    #[test]
    fn test_register_turn_opponent() {
        let rand = Random::new(0);
        let mut game_data = GameHandler::new(rand);

        game_data.register_turn((0, 0), false);
        assert_eq!(game_data.game_data.table[0][0], 2);
        assert_eq!(game_data.game_data.state[0][0], 2);
        assert_eq!(game_data.game_data.empty_positions.len(), 399);
        assert_eq!(game_data.game_data.remaining_turns, 399);
        assert_eq!(game_data.game_data.turns, 1);
    }

    #[test]
    fn test_begin() {
        let rand = Random::new(0);
        let mut game_data = GameHandler::new(rand);

        game_data.begin("");
        assert_eq!(game_data.game_data.table[10][10], 1);
        assert_eq!(game_data.game_data.state[10][10], 1);
    }

    #[test]
    fn test_parse_board_position() {
        use super::parse_board_position;

        assert_eq!(
            parse_board_position("0,0,1"), Some(((0, 0), true))
        );
        assert_eq!(
            parse_board_position("0,0,2"), Some(((0, 0), false))
        );

        assert_eq!(parse_board_position("0,0,3"), None);
        assert_eq!(parse_board_position("0,0"), None);
    }

    #[test]
    fn test_parse_position() {
        use super::parse_position;

        assert_eq!(parse_position("0,0"), Some((0, 0)));
        assert_eq!(parse_position("1,0"), Some((1, 0)));
        assert_eq!(parse_position("0, 0"), None);
        assert_eq!(parse_position("0,0,0"), None);
        assert_eq!(parse_position(" 0,0"), None);
        assert_eq!(parse_position("0.0"), None);
        assert_eq!(parse_position("1000000,0"), None);
        assert_eq!(parse_position("0,1000000"), None);
    }
}