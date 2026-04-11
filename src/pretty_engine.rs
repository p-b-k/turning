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
    machine::{AltType, TuringEngine, TuringLogic, TuringMachine},
    tape::Tape,
};

use ansi_term::{
    Color::{Blue, Green, Red, Yellow},
    Style,
};

const BLANK_CODE: u32 = 0x26ac;

const BLANK_CHAR: char = char::from_u32(BLANK_CODE).unwrap();

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
    pub last_state: Option<S>,
    pub max_state_len: usize,
}

impl<L, S> PrettyEngine<L, S>
where
    S: Clone,
    S: PartialEq,
    S: Debug,
    S: Display,
    L: TuringLogic<char, S>,
{
    pub fn new(max_state_len: usize) -> PrettyEngine<L, S> {
        PrettyEngine {
            phantom_l: PhantomData {},
            sleep_time: 100,
            last_state: None,
            max_state_len: max_state_len,
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
        self.last_state = Some(machine.state.clone());
        print_state(self, machine, tape, &AltType::None);
        sleep(Duration::from_millis(self.sleep_time));
    }

    fn new_state(&mut self, machine: &TuringMachine<char, S, L>, tape: &Tape<char>, alt: AltType) {
        print_state(self, machine, tape, &alt);
        self.last_state = Some(machine.state.clone());
        sleep(Duration::from_millis(self.sleep_time));
    }
}

fn print_state<L, S>(
    engine: &PrettyEngine<L, S>,
    machine: &TuringMachine<char, S, L>,
    tape: &Tape<char>,
    alt: &AltType,
) where
    S: Clone,
    S: PartialEq,
    S: Debug,
    S: Display,
    L: TuringLogic<char, S>,
{
    let state_id = format!("{}", machine.state);

    let new_state = match &engine.last_state {
        Some(c) => c != &machine.state.clone(),
        None => false,
    };

    // let width: usize = 40 + engine.max_state_len;
    let width: usize = 8;
    if machine.logic.is_final(&machine.state) {
        if new_state {
            print!(
                "{} ",
                Style::new()
                    .bold()
                    .paint(format!("{:width$}", Green.paint(state_id)))
            );
        } else {
            print!("{:8} ", Green.paint(state_id));
        }
    } else {
        if new_state {
            print!(
                "{:8} ",
                Style::new()
                    .bold()
                    .paint(format!("{}", Red.paint(state_id)))
            );
        } else {
            print!("{:8} ", Red.paint(state_id));
        }
    }

    let (lb, ub) = match tape.bounds {
        Some(x) => x,
        None => (0, 0),
    };

    let l = if machine.position < lb {
        machine.position
    } else {
        lb
    };

    let h = if machine.position > ub {
        machine.position
    } else {
        ub
    };

    for i in (l - 1)..(h + 2) {
        let is_pos = i == machine.position;
        let char_to_print = match tape.get(&i) {
            Some(c) => c.clone(),
            None => BLANK_CHAR,
        };

        if is_pos {
            print!(
                "{}",
                Style::new()
                    .underline()
                    .paint(format!("{}", Yellow.paint(format!("{}", char_to_print))))
            );
        } else {
            print!("{}", get_cell_fmt(alt, i, char_to_print));
        }
    }

    println!("");
}

fn get_cell_fmt<'a>(
    alt: &AltType,
    pos: i128,
    char_to_print: char,
) -> ansi_term::ANSIGenericString<'a, str> {
    let defstyle = Blue.paint(format!("{}", char_to_print));

    match alt {
        AltType::None => defstyle,
        AltType::Add(i) => {
            if i.clone() == pos {
                Style::new()
                    .bold()
                    .paint(format!("{}", Green.paint(char_to_print.to_string())))
            } else {
                defstyle
            }
        }
        AltType::Clear(i) => {
            if i.clone() == pos {
                Style::new()
                    .bold()
                    .paint(format!("{}", Green.paint(char_to_print.to_string())))
            } else {
                defstyle
            }
        }
        AltType::Alter(i) => {
            if i.clone() == pos {
                Style::new()
                    .bold()
                    .paint(format!("{}", Red.paint(char_to_print.to_string())))
            } else {
                defstyle
            }
        }
    }
}
