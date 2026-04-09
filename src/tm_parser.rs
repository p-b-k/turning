////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Parse Turing Machine definition
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::{collections::HashMap, fs::read_to_string};

use crate::{machine::Dir, sym_machine::SymLogic};

use log::debug;

struct Trans {
    pub trans: String,
    pub dir: Dir,
    pub cell_in: Option<char>,
    pub cell_out: Option<char>,
}

#[derive(Debug)]
enum PState {
    Top,           // Top level, outside all others
    InFromState,   // Reading the from state name
    BeforeBlock,   // Looking for state name
    BeforeBrace,   // Opening brace
    InToState,     // Reading the to state name
    BeforeTrans,   // Looking for value, will also accespt < or >
    BeforeDir,     // Looking for direction sign, < or >
    BeforeToState, // Looking for the transition to state
    BeforeOut,     // Looking for the transition output, will also accept , or }
    BeforeComma,   // Looking for , or }
}

pub fn read_transistion_file(file: &str, _logic: &mut SymLogic) {
    let file_data = read_to_string(file).unwrap();
    let mut state = PState::Top;
    let mut data: HashMap<String, (bool, Vec<Trans>)> = HashMap::new();
    let mut from_state = String::new();
    let mut to_state = String::new();
    let mut is_left = false;
    let mut in_char: Option<char> = None;

    file_data.chars().for_each(|next| {
        debug!("Next char = {next:?} : State = {state:?}");
        match state {
        PState::Top => {
            if next.is_whitespace() {
                // Just keep going
            } else if next.is_alphabetic() {
                from_state.push(next);
                state = PState::InFromState;
            } else {
                panic!("Expecting start of state name, got ({next})");
            }
        }
        PState::InFromState => {
            if next.is_alphanumeric() {
                from_state.push(next);
            } else if next.is_whitespace() || next == '{' {
                data.insert(from_state.clone(), (false, Vec::new()));
                from_state = String::new();
                state = PState::BeforeBrace;
            } else if next == '!' {
                data.insert(from_state.clone(), (true, Vec::new()));
                from_state = String::new();
                state = PState::BeforeBrace;
            } else {
                panic!("Expecting state name character, got ({next})");
            }
        }
        PState::BeforeBlock => {}
        PState::BeforeBrace => {
            if next.is_whitespace() {
                // Just keep going
            } else if next == '{' {
                state = PState::BeforeTrans;
            } else if next.is_alphabetic() {
                // No transitions out of this state, so start a new set
                state = PState::InFromState;
            } else {
                panic!("Expecting state name character, got ({next})");
            }
        }
        PState::BeforeTrans => {
            if next.is_whitespace() {
                // Just keep going
            } else if next == '<' {
                in_char = None;
                is_left = true;
                state = PState::BeforeToState;
            } else if next == '>' {
                in_char = None;
                is_left = false;
                state = PState::BeforeToState;
            } else if next.is_ascii_graphic() {
                in_char = Some(next);
                state = PState::BeforeDir;
            } else {
                panic!("Expecting state name character, got ({next})");
            }
        }
        PState::InToState => {
            if next.is_alphanumeric() {
                to_state.push(next);
            } else if next.is_whitespace() {
                state = PState::BeforeDir;
            } else {
                panic!("Expecting state name character, got ({next})");
            }
        }
        PState::BeforeDir => {
            if next.is_whitespace() {
                // Do Nothing
            } else if next == '<' {
                is_left = true;
                state = PState::BeforeToState;
            } else if next == '>' {
                is_left = false;
                state = PState::BeforeToState;
            } else {
                panic!("Expecting state name character, got ({next})");
            }
        }
        PState::BeforeToState => {}
        PState::BeforeOut => {}
        PState::BeforeComma => {}
    }});
}
