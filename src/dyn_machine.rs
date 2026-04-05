////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Try a generic loader that reads the tape from the input
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::collections::HashMap;

use crate::machine::{Dir, TuringLogic};

pub struct DynLogic {
    pub transitions: Vec<HashMap<Option<char>, (usize, Option<char>, Dir)>>,
    pub valid_input: Vec<char>,
    pub final_states: Vec<usize>,
}

impl DynLogic {
    pub fn new() -> DynLogic {
        DynLogic {
            transitions: Vec::new(),
            valid_input: Vec::new(),
            final_states: Vec::new(),
        }
    }

    pub fn add_state(&mut self) {
        self.transitions.push(HashMap::new());
    }

    pub fn add_input(&mut self, c: char) {
        self.valid_input.push(c);
    }

    pub fn add_final(&mut self, s: usize) {
        self.final_states.push(s);
    }

    pub fn add_trans(
        &mut self,
        state: usize,
        input: &Option<char>,
        result: (usize, Option<char>, Dir),
    ) {
        self.transitions[state].insert(input.clone(), result);
    }

    pub fn get_trans(
        &self,
        state: usize,
        input: &Option<char>,
    ) -> Option<&(usize, Option<char>, Dir)> {
        self.transitions[state].get(&input)
    }
}

impl TuringLogic<char, usize> for DynLogic {
    fn do_trans(&self, state: &usize, input: &Option<char>) -> Option<(usize, Option<char>, Dir)> {
        match self.get_trans(state.clone(), input) {
            Some((s, o, d)) => Some((s.clone(), o.clone(), d.clone())),
            None => None,
        }
    }

    fn is_final(&self, state: &usize) -> bool {
        let mut result = false;

        self.final_states.iter().for_each(|x| {
            if state == x {
                result = true;
            }
        });

        return result;
    }

    fn get_start(&self) -> usize {
        0
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
