use crate::constants;
use crate::random::Random;
use std::collections::HashMap;

pub struct GameHandler {
    pub table: [[i8; 20]; 20],
    pub state: [[i8; 20]; 20],
    pub size: (i8, i8),
    pub turns: i32,
    pub max_turns: i32,
    pub remaining_turns: usize,
    pub random: Random,
    pub empty_positions: Vec<(i8, i8)>,
    functions: HashMap<String, fn(&mut GameHandler, &str)>,
    board: bool,
    max_memory: i32,
    timeout_turn: i32,
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

        let mut empty_positions: Vec<(i8, i8)> = Vec::new();
        for x in 0..20 {
            for y in 0..20 {
                empty_positions.push((x, y));
            }
        }

        GameHandler {
            functions,
            max_memory: constants::DEFAULT_MAX_MEMORY,
            timeout_turn: constants::DEFAULT_TIMEOUT_TURN,
            size: (20, 20),
            table: [[0i8; 20]; 20],
            state: [[0i8; 20]; 20],
            turns: 0,
            remaining_turns: 400,
            max_turns: 400,
            board: false,
            random,
            empty_positions,
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
        self.table[pos.0 as usize][pos.1 as usize] = if me { 1 } else { 2 };
        self.state[pos.0 as usize][pos.1 as usize] = if me { 1 } else { 2 };
        self.empty_positions.push(pos);
        self.remaining_turns -= 1;
        self.turns += 1;
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
        let new_move = self.get_next_move();

        self.register_turn(new_move, true);
        self.broadcast_turn(new_move);
    }

    fn board(&mut self, _args: &str) {
        self.table.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|cell| {
                *cell = 0;
            })
        });
        self.turns = 0;
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
        let new_move = self.get_next_move();

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
                    if x < 0 || x > 19 || y < 0 || y > 19 {
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
            if x < 0 || x > 19 || y < 0 || y > 19 {
                return None;
            }
            Some(((x, y), p == 1))
        }
        _ => None,
    }
}
