use crate::constants;
use std::collections::HashMap;

pub struct GameHandler {
    functions: HashMap<String, fn(&GameHandler, &str)>,
    max_memory: i32,
    timeout_turn: i32,
}

impl GameHandler {
    pub fn new() -> GameHandler {
        let mut functions: HashMap<String, fn(&GameHandler, &str)> = HashMap::new();
        functions.insert(constants::ABOUT_COMMAND.to_string(), GameHandler::about);
        functions.insert(constants::END_COMMAND.to_string(), GameHandler::end);
        functions.insert(constants::INFO_COMMAND.to_string(), GameHandler::info);
        functions.insert(constants::START_COMMAND.to_string(), GameHandler::start);

        GameHandler {
            functions,
            max_memory: constants::DEFAULT_MAX_MEMORY,
            timeout_turn: constants::DEFAULT_TIMEOUT_TURN,
        }
    }

    pub fn handle_line(&self, line: String) -> bool {
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

    pub fn debug(&self, args: &str) {
        println!("{} {}", constants::DEBUG_RESPONSE, args);
    }

    pub fn error(&self, args: &str) {
        println!("{} {}", constants::ERROR_RESPONSE, args);
    }

    fn handle_command(&self, command: &str, args: &str) {
        match self.functions.get(command) {
            Some(function) => {
                function(self, args);
            }
            None => {
                println!("{}", constants::UNKNOWN_RESPONSE);
            }
        }
    }

    fn about(&self, _args: &str) {
        println!(
            "name=\"{}\", version=\"{}\", author=\"{}\", country=\"{}\"",
            constants::BRAIN_NAME,
            constants::BRAIN_VERSION,
            constants::BRAIN_AUTHOR,
            constants::BRAIN_COUNTRY
        );
    }

    fn end(&self, _args: &str) {}

    fn info(&self, args: &str) {
        self.debug(args);
    }

    fn start(&self, args: &str) {
        match args.parse::<i8>() {
            Ok(size) => {
                // TODO https://github.com/EdenComp/R-Type/issues/5
                if size != constants::DEFAULT_SIZE {
                    self.error("Invalid size")
                }
                println!("{}", constants::OK_RESPONSE)
            }
            Err(_) => self.error("Invalid size"),
        }
    }
}
