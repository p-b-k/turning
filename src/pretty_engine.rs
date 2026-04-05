////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// a pretty printing turing engine
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::{marker::PhantomData, thread::sleep, time::Duration};

use crate::machine::{TuringEngine, TuringLogic, TuringMachine};

const BLANK_CHAR: char = ' ';

use ansi_term::{
    Color::{Blue, Green, Red, Yellow},
    Style,
};

pub struct PrettyEngine<AL>
where
    AL: TuringLogic<char, usize>,
{
    phantom: PhantomData<AL>,
    pub sleep_time: u64,
    pub last_state: usize,
}

impl<AL> PrettyEngine<AL>
where
    AL: TuringLogic<char, usize>,
{
    pub fn new() -> PrettyEngine<AL> {
        PrettyEngine {
            sleep_time: 100,
            phantom: PhantomData {},
            last_state: 0,
        }
    }
}

impl<AL> TuringEngine<char, usize, AL> for PrettyEngine<AL>
where
    AL: TuringLogic<char, usize>,
{
    fn init(&mut self, machine: &TuringMachine<char, usize, AL>, tape: &Vec<Option<char>>) {
        self.last_state = machine.logic.get_start();
        print_state(self, machine, tape);
        sleep(Duration::from_millis(self.sleep_time));
    }

    fn new_state(&mut self, machine: &TuringMachine<char, usize, AL>, tape: &Vec<Option<char>>) {
        print_state(self, machine, tape);
        self.last_state = machine.state.clone();
        sleep(Duration::from_millis(self.sleep_time));
    }
}

fn print_state<AL>(
    engine: &PrettyEngine<AL>,
    machine: &TuringMachine<char, usize, AL>,
    tape: &Vec<Option<char>>,
) where
    AL: TuringLogic<char, usize>,
{
    let state_id = format!("{}", machine.state);

    let new_state = &engine.last_state.clone() != &machine.state.clone();

    if machine.logic.is_final(&machine.state) {
        if new_state {
            print!(
                "{}",
                Style::new()
                    .bold()
                    .paint(format!("{} ", Green.paint(state_id)))
            );
        } else {
            print!("{} ", Green.paint(state_id));
        }
    } else {
        if new_state {
            print!(
                "{}",
                Style::new()
                    .bold()
                    .paint(format!("{} ", Red.paint(state_id)))
            );
        } else {
            print!("{} ", Red.paint(state_id));
        }
    }

    for i in 0..tape.len() {
        let is_pos = i == machine.position;
        let char_to_print = match &tape[i] {
            Some(c) => c.clone(),
            None => BLANK_CHAR,
        };

        if is_pos {
            print!(
                "{}",
                Style::new()
                    .reverse()
                    .paint(format!("{}", Yellow.paint(format!("{}", char_to_print))))
            );
        } else {
            print!("{}", Blue.paint(format!("{}", char_to_print)));
        }
    }

    println!("");
}
