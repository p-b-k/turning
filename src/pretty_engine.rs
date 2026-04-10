////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// a pretty printing turing engine
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
    thread::sleep,
    time::Duration,
};

use crate::{
    machine::{TuringEngine, TuringLogic, TuringMachine},
    tape::Tape,
};

const BLANK_CODE: u32 = 0x26ac;

const BLANK_CHAR: char = char::from_u32(BLANK_CODE).unwrap();

use ansi_term::{
    Color::{Blue, Green, Red, Yellow},
    Style,
};

pub struct PrettyEngine<L, S>
where
    S: Clone,
    S: PartialEq,
    S: Debug,
    S: Display,
    L: TuringLogic<char, S>,
{
    phantom_l: PhantomData<L>,
    pub sleep_time: u64,
    pub last_state: S,
}

impl<L, S> PrettyEngine<L, S>
where
    S: Clone,
    S: PartialEq,
    S: Debug,
    S: Display,
    L: TuringLogic<char, S>,
{
    pub fn new(last_state: S) -> PrettyEngine<L, S> {
        PrettyEngine {
            phantom_l: PhantomData {},
            sleep_time: 100,
            last_state,
        }
    }
}

impl<L, S> TuringEngine<char, S, L> for PrettyEngine<L, S>
where
    S: Clone,
    S: PartialEq,
    S: Debug,
    S: Display,
    L: TuringLogic<char, S>,
{
    fn init(&mut self, machine: &TuringMachine<char, S, L>, tape: &Tape<char>) {
        self.last_state = machine.logic.get_start();
        print_state(self, machine, tape, None);
        sleep(Duration::from_millis(self.sleep_time));
    }

    fn new_state(
        &mut self,
        machine: &TuringMachine<char, S, L>,
        tape: &Tape<char>,
        alt: Option<i128>,
    ) {
        print_state(self, machine, tape, alt);
        self.last_state = machine.state.clone();
        sleep(Duration::from_millis(self.sleep_time));
    }
}

fn print_state<L, S>(
    engine: &PrettyEngine<L, S>,
    machine: &TuringMachine<char, S, L>,
    tape: &Tape<char>,
    alt: Option<i128>,
) where
    S: Clone,
    S: PartialEq,
    S: Debug,
    S: Display,
    L: TuringLogic<char, S>,
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

    let (l, h) = tape.bounds().unwrap();

    for i in (l - 1)..(h + 2) {
        let is_pos = i == machine.position;
        let is_alt = match alt {
            Some(x) => x == i,
            None => false,
        };
        let char_to_print = match tape.get(&i) {
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
            if is_alt {
                print!(
                    "{}",
                    Style::new()
                        .reverse()
                        .paint(format!("{}", Red.paint(format!("{}", char_to_print))))
                );
            } else {
                print!("{}", Blue.paint(format!("{}", char_to_print)));
            }
        }
    }

    println!("");
}
