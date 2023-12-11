pub static BRAIN_NAME: &str = "R-Type";
pub static BRAIN_VERSION: &str = "0.1.0";
pub static BRAIN_AUTHOR: &str = "Mehdi Taha Tion";
pub static BRAIN_COUNTRY: &str = "France";

pub const _DEFAULT_MAX_MEMORY: i32 = 70000000;
pub const _DEFAULT_TIMEOUT_TURN: i32 = 5000;
pub const DEFAULT_SIZE: i8 = 20;

pub static ABOUT_COMMAND: &str = "ABOUT";
pub static BEGIN_COMMAND: &str = "BEGIN";
pub static BOARD_COMMAND: &str = "BOARD";
pub static END_COMMAND: &str = "END";
pub static INFO_COMMAND: &str = "INFO";
pub static START_COMMAND: &str = "START";
pub static TURN_COMMAND: &str = "TURN";

pub static BOARD_END: &str = "DONE";

pub static DEBUG_RESPONSE: &str = "DEBUG";
pub static ERROR_RESPONSE: &str = "ERROR";
pub static OK_RESPONSE: &str = "OK";
pub static UNKNOWN_RESPONSE: &str = "UNKNOWN";

pub static MAX_SIMULATIONS_PER_COMBINATION: usize = 6000;
pub static MAX_SIMULATIONS_PER_THREAD: usize = 180000;
pub static MAX_SIMULATIONS_MILLIS: u64 = 4900;
