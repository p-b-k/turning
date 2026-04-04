////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Make a turing machine
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::marker::PhantomData;

pub enum Dir {
    Left,
    Right,
}

pub trait TuringLogic<A, S> {
    fn do_trans(state: &S, input: &Option<A>) -> Option<(S, Option<A>, Dir)>;
    fn is_final(state: &S) -> bool;
    fn get_start() -> S;
    fn is_valid(input: &A) -> bool;
}

pub struct TuringMachine<A, S, T>
where
    T: TuringLogic<A, S>,
{
    phantom_t: PhantomData<T>,
    phantom_a: PhantomData<A>,
    pub state: S,
    pub position: usize,
}

pub trait TuringEngine<A, S, T>
where
    T: TuringLogic<A, S>,
{
    fn init(&mut self, _machine: &TuringMachine<A, S, T>, _tape: &Vec<Option<A>>) {}
    fn new_state(&mut self, _machine: &TuringMachine<A, S, T>, _tape: &Vec<Option<A>>) {}
    fn finalize(&mut self, _machine: &TuringMachine<A, S, T>, _tape: &Vec<Option<A>>) {}
}

impl<A, S, T> TuringMachine<A, S, T>
where
    T: TuringLogic<A, S>,
{
    pub fn new(position: usize) -> TuringMachine<A, S, T> {
        TuringMachine {
            phantom_t: PhantomData {},
            phantom_a: PhantomData {},
            state: T::get_start(),
            position,
        }
    }

    fn advance(&mut self, tape: &mut Vec<Option<A>>) -> bool {
        let next = &tape[self.position];
        match T::do_trans(&self.state, next) {
            Some((s, c, d)) => {
                tape[self.position] = c;
                self.state = s;
                match d {
                    Dir::Left => {
                        if self.position == 0 {
                            panic!("The pointer has run off the left edge of the tape");
                        }
                        self.position = self.position - 1
                    }
                    Dir::Right => {
                        self.position = self.position + 1;
                        if self.position >= tape.len() {
                            panic!("THe pointer has run off the right edge of the tape")
                        }
                    }
                }
                true
            }
            None => false,
        }
    }

    fn run_to_end<E>(&mut self, tape: &mut Vec<Option<A>>, eng: &mut E)
    where
        E: TuringEngine<A, S, T>,
    {
        if self.advance(tape) {
            eng.new_state(self, tape);
            self.run_to_end(tape, eng);
        } else {
            eng.finalize(self, tape);
        }
    }

    pub fn run<E>(&mut self, tape: &mut Vec<Option<A>>, eng: &mut E)
    where
        E: TuringEngine<A, S, T>,
    {
        tape.iter().for_each(|x| match x {
            Some(c) => {
                if !T::is_valid(c) {
                    panic!("Invalid input on tape");
                }
            }
            None => {}
        });

        eng.init(self, tape);

        self.run_to_end(tape, eng);
    }
}
