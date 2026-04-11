////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Try a generic loader that reads the tape from the input
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::env;
use std::io::{Read, stdin};

use turing::machine::TuringMachine;
use turing::pretty_engine::PrettyEngine;
use turing::sym_machine::SymLogic;
use turing::tape::{Tape, from_vec};
use turing::tm_parser::read_transistion_file;

const LOGIC_ROOT: &str = "logic";
const DEFAULT_LOGIC: &str = "adder";

struct AppConfig {
    pub file: String,
    pub delay: u64,
}

impl AppConfig {
    pub fn new() -> AppConfig {
        let file = format!("{LOGIC_ROOT}/{DEFAULT_LOGIC}.tm");
        AppConfig {
            file: String::from(file),
            delay: 1000,
        }
    }
}

fn main() {
    env_logger::init();
    // Create the application data structure
    let mut cfg = AppConfig::new();

    process_args(&mut cfg);

    // Read the file into a vector and convert it into a tape
    let mut buf: Vec<u8> = Vec::new();
    stdin().read_to_end(&mut buf).unwrap();
    let mut tape: Tape<char> = from_vec(&buf, |u| u.clone() as char);

    // Now, create the Dynamic Logic ...
    let mut logic = SymLogic::new();

    // ... and read it from the file.
    read_transistion_file(cfg.file.as_str(), &mut logic);

    let max_state_size = logic.max_state_size();

    // Create the turning machine object
    let mut machine: TuringMachine<char, String, SymLogic> = TuringMachine::new(0, logic);

    let mut engine = PrettyEngine::new(max_state_size);
    engine.sleep_time = cfg.delay;

    machine.run(&mut tape, &mut engine);
}

fn process_args(cfg: &mut AppConfig) {
    let args: Vec<String> = env::args().collect();

    let mut i = 1;
    while i < args.len() {
        if args[i] == "--logic" {
            i = i + 1;
            cfg.file = format!("{LOGIC_ROOT}/{}.tm", args[i]);
        } else if args[i] == "--delay" {
            i = i + 1;
            cfg.delay = args[i].parse().unwrap();
        } else {
            panic!("Unknown argument '{}'", args[i]);
        }
        i = i + 1;
    }
}
