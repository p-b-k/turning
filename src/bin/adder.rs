////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Implement an adder for the turing machine
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::{env, thread::sleep, time::Duration};

use ansi_term::{
    Color::{Blue, Green, Red, Yellow},
    Style,
};

use turing::machine::{Dir, TuringEngine, TuringLogic, TuringMachine};

pub struct AdderLogic {}

type AL = AdderLogic;

#[derive(Clone, Debug, PartialEq)]
pub enum AdderState {
    Q0,
    Q1,
    Q2,
    Q3,
    Q4,
}

type AS = AdderState;

#[derive(Clone, Debug, PartialEq)]
enum AdderAlphabet {
    A0,
    A1,
}

type AA = AdderAlphabet;

impl TuringLogic<AA, AS> for AL {
    fn do_trans(&self, state: &AS, input: &Option<AA>) -> Option<(AS, Option<AA>, Dir)> {
        match (state, input) {
            (AS::Q0, Some(AA::A1)) => Some((AS::Q0, Some(AA::A1), Dir::Right)),
            (AS::Q0, Some(AA::A0)) => Some((AS::Q1, Some(AA::A1), Dir::Right)),
            (AS::Q1, Some(AA::A1)) => Some((AS::Q1, Some(AA::A1), Dir::Right)),
            (AS::Q1, None) => Some((AS::Q2, None, Dir::Left)),
            (AS::Q2, Some(AA::A1)) => Some((AS::Q3, Some(AA::A0), Dir::Left)),
            (AS::Q3, Some(AA::A1)) => Some((AS::Q3, Some(AA::A1), Dir::Left)),
            (AS::Q3, None) => Some((AS::Q4, None, Dir::Right)),

            _ => None,
        }
    }

    fn is_final(&self, state: &AS) -> bool {
        state.clone() == AS::Q4
    }

    fn is_valid(&self, _: &AA) -> bool {
        true
    }

    fn get_start(&self) -> AS {
        AS::Q0
    }
}

type AdderMachine = TuringMachine<AA, AS, AL>;

struct AdderEngine {
    pub last_state: AS,
}

impl TuringEngine<AA, AS, AL> for AdderEngine {
    fn init(&mut self, machine: &AdderMachine, tape: &Vec<Option<AA>>) {
        self.last_state = machine.logic.get_start();
        print_state(self, machine, tape);
    }
    fn new_state(&mut self, machine: &AdderMachine, tape: &Vec<Option<AA>>) {
        print_state(self, machine, tape);
        self.last_state = machine.state.clone();
        sleep(Duration::from_millis(100));
    }
}

fn print_state(engine: &AdderEngine, machine: &AdderMachine, tape: &Vec<Option<AA>>) {
    let state_id = match machine.state {
        AS::Q0 => "q0",
        AS::Q1 => "q1",
        AS::Q2 => "q2",
        AS::Q3 => "q3",
        AS::Q4 => "q4",
    };

    // println!(
    //     "states: old = {:?}, now = {:?}",
    //     &engine.last_state, &machine.state
    // );

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
            Some(e) => match e {
                AA::A0 => "0",
                AA::A1 => "1",
            },
            None => " ",
        };

        if is_pos {
            print!(
                "{}",
                Style::new()
                    .reverse()
                    .paint(format!("{}", Yellow.paint(char_to_print)))
            );
        } else {
            print!("{}", Blue.paint(char_to_print));
        }
    }

    println!("");
}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let a_str = args[1].as_str();
    let b_str = args[2].as_str();
    println!("Adding {a_str} to {b_str}");

    let mut tape: Vec<Option<AA>> = Vec::new();

    let mut machine = AdderMachine::new(1, AdderLogic {});
    let mut engine = AdderEngine {
        last_state: machine.logic.get_start(),
    };

    initialize_tape(&mut tape, a_str.parse().unwrap(), b_str.parse().unwrap());

    println!("Initial Tape State");
    print_tape(&tape);

    machine.run(&mut tape, &mut engine);

    println!("Final Tape State");
    print_tape(&tape);

    if machine.logic.is_final(&machine.state) {
        println!("Success");
    } else {
        println!("Failure");
    }
}

fn print_tape(tape: &Vec<Option<AA>>) {
    tape.iter().for_each(|i| match i {
        Some(a) => match a {
            AA::A0 => print!("0"),
            AA::A1 => print!("1"),
        },
        None => {
            print!("#");
        }
    });

    println!("");
}

fn initialize_tape(tape: &mut Vec<Option<AA>>, a: usize, b: usize) {
    tape.push(None);
    for _ in 0..a {
        tape.push(Some(AA::A1));
    }
    tape.push(Some(AA::A0));
    for _ in 0..b {
        tape.push(Some(AA::A1));
    }
    tape.push(None);
}
