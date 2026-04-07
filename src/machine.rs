////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Make a turing machine
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::{fmt::Debug, marker::PhantomData};

use crate::tape::Tape;

#[derive(Clone, Debug, PartialEq)]
pub enum Dir {
    Left,
    Right,
}

pub trait TuringLogic<A, S> {
    fn do_trans(&self, state: &S, input: &Option<A>) -> Option<(S, Option<A>, Dir)>;
    fn is_final(&self, state: &S) -> bool;
    fn get_start(&self) -> S;
    fn is_valid(&self, input: &A) -> bool;
}

pub struct TuringMachine<A, S, T>
where
    T: TuringLogic<A, S>,
    A: Clone,
    A: PartialEq,
{
    phantom_a: PhantomData<A>,
    pub logic: T,
    pub state: S,
    pub position: i128,
}

pub trait TuringEngine<A, S, T>
where
    T: TuringLogic<A, S>,
    A: Clone,
    A: PartialEq,
{
    fn init(&mut self, _machine: &TuringMachine<A, S, T>, _tape: &Tape<A>) {}
    fn new_state(&mut self, _machine: &TuringMachine<A, S, T>, _tape: &Tape<A>, _altered :Option<i128>) {}
    fn finalize(&mut self, _machine: &TuringMachine<A, S, T>, _tape: &Tape<A>) {}
}

impl<A, S, T> TuringMachine<A, S, T>
where
    T: TuringLogic<A, S>,
    A: Clone,
    A: PartialEq,
{
    pub fn new(position: i128, logic: T) -> TuringMachine<A, S, T> {
        TuringMachine {
            phantom_a: PhantomData {},
            state: logic.get_start(),
            logic,
            position,
        }
    }

    fn advance<E>(&mut self, tape: &mut Tape<A>, eng: &mut E) -> bool
    where
        A: Clone,
        E: TuringEngine<A, S, T>
    {
        let next = &tape.get(&self.position);
        match self.logic.do_trans(&self.state, next) {
            Some((s, c, d)) => {
                let pos = self.position;
                tape.set(&pos, c.clone());
                self.state = s;
                match d {
                    Dir::Left => self.position = self.position - 1,
                    Dir::Right => {
                        self.position = self.position + 1;
                    }
                }
                eng.new_state(self, tape, if next != &c { Some(pos) } else {None});
                true
            }
            None => false,
        }
    }

    fn run_to_end<E>(&mut self, tape: &mut Tape<A>, eng: &mut E)
    where
        E: TuringEngine<A, S, T>,
        A: Clone,
    {
        if self.advance(tape,eng) {
            self.run_to_end(tape, eng);
        } else {
            eng.finalize(self, tape);
        }
    }

    pub fn run<E>(&mut self, tape: &mut Tape<A>, eng: &mut E)
    where
        E: TuringEngine<A, S, T>,
        A: Debug,
    {
        // tape.iter().for_each(|x| match x {
        //     Some(c) => {
        //         if !self.logic.is_valid(c) {
        //             panic!("Invalid input on tape ({c:?})");
        //         }
        //     }
        //     None => {}
        // });
        for e in tape.values() {
            if !self.logic.is_valid(e) {
                panic!("Invalid input character ({e:?})");
            }
        }

        eng.init(self, tape);

        self.run_to_end(tape, eng);
    }
}
