use crate::constants;
use std::collections::HashMap;

// TODO Fixed size
// https://github.com/EdenComp/R-Type/issues/5
pub struct GameHandler {
    pub table: [[i8; 20]; 20],
    pub size_x: i8,
    pub size_y: i8,
    functions: HashMap<String, fn(&mut GameHandler, &str)>,
    board: bool,
    max_memory: i32,
    timeout_turn: i32,
}

impl GameHandler {
    pub fn new() -> GameHandler {
        let mut functions: HashMap<String, fn(&mut GameHandler, &str)> = HashMap::new();
        functions.insert(constants::ABOUT_COMMAND.to_string(), GameHandler::about);
        functions.insert(constants::BEGIN_COMMAND.to_string(), GameHandler::begin);
        functions.insert(constants::BOARD_COMMAND.to_string(), GameHandler::board);
        functions.insert(constants::END_COMMAND.to_string(), GameHandler::end);
        functions.insert(constants::INFO_COMMAND.to_string(), GameHandler::info);
        functions.insert(constants::START_COMMAND.to_string(), GameHandler::start);
        functions.insert(constants::TURN_COMMAND.to_string(), GameHandler::turn);

        GameHandler {
            functions,
            max_memory: constants::DEFAULT_MAX_MEMORY,
            timeout_turn: constants::DEFAULT_TIMEOUT_TURN,
            // TODO Fixed size
            // https://github.com/EdenComp/R-Type/issues/5
            size_x: 20,
            size_y: 20,
            table: [[0i8; 20]; 20],
            board: false,
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
        match parse_position(line) {
            Some(pos) => {
                self.register_turn(pos, false);
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

    pub fn board(&mut self, _args: &str) {
        self.table.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|cell| {
                *cell = 0;
            })
        });
        self.board = true;
    }

    fn broadcast_turn(&self, pos: (i8, i8)) {
        println!("{},{}", pos.0, pos.1);
    }

    fn register_turn(&mut self, pos: (i8, i8), me: bool) {
        self.table[pos.0 as usize][pos.1 as usize] = if me { 1 } else { 2 };
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
        let new_move = self.get_first_move();

        self.register_turn(new_move, true);
        self.broadcast_turn(new_move);
    }

    fn end(&mut self, _args: &str) {}

    fn info(&mut self, args: &str) {
        self.debug(args);
    }

    fn start(&mut self, args: &str) {
        match args.parse::<i8>() {
            Ok(size) => {
                // TODO Fixed size
                // https://github.com/EdenComp/R-Type/issues/5
                if size != constants::DEFAULT_SIZE {
                    self.error("Invalid size")
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
                (Ok(x), Ok(y)) => Some((x, y)),
                _ => None,
            }
        }
        None => None,
    }
}
