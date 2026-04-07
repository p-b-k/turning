////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Try a generic loader that reads the tape from the input
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////


use std::collections::HashMap;

use crate::machine::{Dir, TuringLogic};

pub struct SymLogic {
    pub transitions: HashMap<String, HashMap<Option<char>, (String, Option<char>, Dir)>>,
    pub valid_input: Vec<char>,
    pub final_states: Vec<String>,
}

impl SymLogic {
    pub fn new() -> SymLogic {
        SymLogic {
            transitions: HashMap::new(),
            valid_input: Vec::new(),
            final_states: Vec::new(),
        }
    }

    pub fn add_input(&mut self, c: char) {
        self.valid_input.push(c);
    }

    pub fn add_final(&mut self, s: String) {
        self.final_states.push(s);
    }

    pub fn add_trans(
        &mut self,
        state: String,
        input: &Option<char>,
        result: (String, Option<char>, Dir),
    ) {
        match self.transitions.get_mut(&state) {
            Some (m) => {
                m.insert(input.clone(), result);
            }
            None => {
                let mut m = HashMap::new();
                m.insert(input.clone(), result);
                self.transitions.insert(state, m);
            }
        }
    }

    pub fn get_trans(
        &self,
        state: String,
        input: &Option<char>,
    ) -> Option<&(String, Option<char>, Dir)> {
        match self.transitions.get(&state) {
            Some(m) => {m.get(&input)}
            None => None
        }
    }
}

impl TuringLogic<char, String> for SymLogic {
    fn do_trans(&self, state: &String, input: &Option<char>) -> Option<(String, Option<char>, Dir)> {
        match self.get_trans(state.clone(), input) {
            Some((s, o, d)) => Some((s.clone(), o.clone(), d.clone())),
            None => None,
        }
    }

    fn is_final(&self, state: &String) -> bool {
        let mut result = false;

        self.final_states.iter().for_each(|x| {
            if state == x {
                result = true;
            }
        });

        return result;
    }

    fn get_start(&self) -> String {
        "q0".to_string()
    }

    fn is_valid(&self, input: &char) -> bool {
        let mut result = false;

        self.valid_input.iter().for_each(|x| {
            if input == x {
                result = true;
            }
        });

        return result;
    }
}
