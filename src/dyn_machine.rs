////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Try a generic loader that reads the tape from the input
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::collections::HashMap;

use crate::machine::{Dir, TuringLogic};

pub struct DynLogic {
    pub transitions: Vec<HashMap<char, (usize, char, Dir)>>,
}

impl DynLogic {
    pub fn add_trans(&mut self, state: usize, input: char, result: (usize, char, Dir)) {
        self.transitions[state].insert(input, result);
    }

    fn get_trans(&self, state: usize, input: char) -> Option<&(usize, char, Dir)> {
        self.transitions[state].get(&input)
    }
}

impl TuringLogic<char, usize> for DynLogic {
    fn do_trans(state: &usize, input: &Option<char>) -> Option<(usize, Option<char>, Dir)> {
        None
    }

    fn is_final(state: &usize) -> bool {
        false
    }

    fn get_start() -> usize {
        0
    }

    fn is_valid(input: &char) -> bool {
        false
    }
}
