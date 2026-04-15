////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Try a generic loader that reads the tape from the input
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::env;

use turing::machine::{TuringEngine, TuringMachine};
use turing::pretty_engine::PrettyEngine;
use turing::sym_machine::SymLogic;
use turing::tape::Tape;
use turing::tm_parser::read_transistion_file;

const LOGIC_ROOT: &str = "logic";
const DEFAULT_LOGIC: &str = "adder";

struct AppConfig {
    pub file: String,
    pub delay: u64,
    pub operands: Vec<u64>,
    pub show_vm: bool,
}

impl AppConfig {
    pub fn new() -> AppConfig {
        let file = format!("{LOGIC_ROOT}/{DEFAULT_LOGIC}.tm");
        AppConfig {
            file: String::from(file),
            delay: 100,
            operands: Vec::new(),
            show_vm: false,
        }
    }
}

struct NullEngine {}

impl TuringEngine<char, String, SymLogic> for NullEngine {}

fn main() {
    env_logger::init();
    // Create the application data structure
    let mut cfg = AppConfig::new();

    process_args(&mut cfg);

    match cfg.operands.len() {
        0 => panic!("Two operands are required"),
        1 => panic!("Exactly two operatnds are required"),
        2 => {}
        _ => panic!("Too many operands, exactly two (numeric) operands required"),
    }

    // Create the tape and read the operands in unarry format
    let mut tape: Tape<char> = Tape::new();
    let mut first = true;
    let mut base: usize = 0;

    for u in cfg.operands.iter() {
        if first {
            first = false;
        } else {
            tape.set(&(base as i128), Some('0'));
            base = base + 1;
        }

        for i in 0..u.clone() {
            tape.set(&((base + i as usize) as i128), Some('1'));
        }

        base = base + (u.clone() as usize);
    }

    // Now, create the Dynamic Logic ...
    let mut logic = SymLogic::new();

    // ... and read it from the file.
    read_transistion_file(cfg.file.as_str(), &mut logic);

    let max_state_size = logic.max_state_size();

    // Create the turning machine object
    let mut machine: TuringMachine<char, String, SymLogic> = TuringMachine::new(0, logic);

    if cfg.show_vm {
        let mut engine = PrettyEngine::new(max_state_size);
        engine.sleep_time = cfg.delay;

        machine.run(&mut tape, &mut engine);
    } else {
        let mut engine = NullEngine {};

        machine.run(&mut tape, &mut engine);
    }

    let start = machine.position.clone();
    let mut cnt = 0;
    loop {
        match tape.get(&(start + cnt)) {
            Some(c) => {
                if c == '1' {
                    cnt = cnt + 1;
                } else {
                    break;
                }
            }
            None => break,
        }
    }

    println!(
        "With logic from {} on ({}, {}), the result is '{cnt}' (in {:?} steps)",
        cfg.file, cfg.operands[0], cfg.operands[1], machine.step_cnt
    );
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
        } else if args[i] == "--show" {
            cfg.show_vm = true;
        } else {
            // panic!("Unknown argument '{}'", args[i]);
            let num: u64 = args[i].parse().unwrap();
            cfg.operands.push(num);
        }
        i = i + 1;
    }
}
