////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Try a generic loader that reads the tape from the input
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::fs::read_to_string;
use std::io::{Read, stdin};

use turing::dyn_machine::DynLogic;
use turing::machine::{Dir, TuringMachine};
use turing::pretty_engine::PrettyEngine;

const LOGIC_ROOT: &str = "logic";
const DEFAULT_LOGIC: &str = "adder";

struct AppData {
    pub file: String,
    pub delay: u32,
}

impl AppData {
    pub fn new() -> AppData {
        let file = format!("{LOGIC_ROOT}/{DEFAULT_LOGIC}.tm");
        AppData {
            file: String::from(file),
            delay: 1000,
        }
    }
}

fn main() {
    // Create the application data structure
    let data = AppData::new();

    // Create the imput tape vector ...
    let mut tape: Vec<Option<char>> = Vec::new();

    // ... and set it up reading from stdin
    let mut buf: Vec<u8> = Vec::new();
    stdin().read_to_end(&mut buf).unwrap();
    buf.iter().for_each(|u| {
        tape.push(if u.is_ascii_whitespace() {
            None
        } else {
            Some(u.clone() as char)
        })
    });

    // Now, create the Dynamic Logic ...
    let mut logic = DynLogic::new();
    logic.add_input('0');
    logic.add_input('1');
    logic.add_final(4);

    // ... and read it from the file.
    read_trans_from_file(&data, &mut logic);

    // Create the turning machine object
    let mut machine: TuringMachine<char, usize, DynLogic> = TuringMachine::new(1, logic);
    let mut engine = PrettyEngine::new();
    machine.position = 1;

    machine.run(&mut tape, &mut engine);
}

#[derive(Debug)]
enum PState {
    Start,
    Size,
    Top,
    From,
    In,
    Dir,
    To,
    Out,
}

fn read_trans_from_file(app_data: &AppData, logic: &mut DynLogic) {
    let file_data = read_to_string(app_data.file.as_str()).unwrap();
    let mut state = PState::Start;

    let mut size_str = String::new();
    let mut from_in: Option<char> = None;
    let mut to_out: Option<char> = None;
    let mut from_str = String::new();
    let mut to_str = String::new();
    let mut left_trans = false;

    file_data.chars().for_each(|next| {
        match state {
            PState::Start => {
                if next.is_whitespace() {
                } else if next.is_digit(10) {
                    size_str.push(next);
                    state = PState::Size;
                } else {
                    panic!("Expected state count, found {next}");
                }
            }
            PState::Size => {
                if next.is_digit(10) {
                    size_str.push(next);
                } else if next.is_whitespace() {
                    state = PState::Top;
                    let size: usize = size_str.parse().unwrap();
                    for _ in 0..size {
                        logic.add_state();
                    }
                }
            }
            PState::Top => {
                if next.is_whitespace() {
                } else if next.is_digit(10) {
                    from_str.push(next);
                    state = PState::From;
                } else {
                    panic!("Expecting state id, found {next:?}")
                }
            }
            PState::From => {
                if next.is_digit(10) {
                    from_str.push(next);
                } else if next == '.' {
                    state = PState::In;
                } else if next == '<' {
                    left_trans = true;
                    from_in = None;
                    state = PState::To;
                } else if next == '>' {
                    left_trans = false;
                    from_in = None;
                    state = PState::To;
                } else {
                    panic!("Expected either a dot or an angle bracket, not {next}")
                }
            }
            PState::In => {
                from_in = Some(next);
                state = PState::Dir;
            }
            PState::Dir => {
                if next == '<' {
                    left_trans = true;
                } else if next == '>' {
                    left_trans = false;
                } else {
                    panic!("Expected either '<' or '>', got {next}");
                }
                state = PState::To;
            }
            PState::To => {
                if next.is_digit(10) {
                    to_str.push(next);
                } else if next == '.' {
                    state = PState::Out;
                } else if next.is_whitespace() {
                    to_out = None;
                    state = PState::Top;

                    // Finalize, add and reset
                    let from_state: usize = from_str.parse().unwrap();
                    let to_state: usize = to_str.parse().unwrap();
                    let dir = if left_trans { Dir::Left } else { Dir::Right };
                    logic.add_trans(from_state, &from_in, (to_state, to_out, dir));
                    from_str = String::new();
                    from_in = None;
                    to_str = String::new();
                    to_out = None;
                    left_trans = false;
                } else {
                    panic!("Expected either a dot or whitespace");
                }
            }
            PState::Out => {
                to_out = Some(next);
                state = PState::Top;
                // Finalize and reset
                let from_state: usize = from_str.parse().unwrap();
                let to_state: usize = to_str.parse().unwrap();
                let dir = if left_trans { Dir::Left } else { Dir::Right };
                logic.add_trans(from_state, &from_in, (to_state, to_out, dir));
                from_str = String::new();
                from_in = None;
                to_str = String::new();
                to_out = None;
                left_trans = false;
            }
        }
    });

    match state {
        PState::Top => {}
        PState::To => {
            let from_state: usize = from_str.parse().unwrap();
            let to_state: usize = to_str.parse().unwrap();
            let dir = if left_trans { Dir::Left } else { Dir::Right };
            logic.add_trans(from_state, &from_in, (to_state, to_out, dir));
        }
        _ => {
            panic!("Unexpected end state: {state:?}")
        }
    }
}
