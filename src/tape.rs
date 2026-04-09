////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Implement a "universal" tape for a turing machine
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::collections::{HashMap, hash_map::Values};

pub struct Tape<A>
where
    A: Clone,
{
    map: HashMap<i128, A>,
}

impl<A> Tape<A>
where
    A: Clone,
{
    pub fn new() -> Tape<A> {
        Tape {
            map: HashMap::new(),
        }
    }

    pub fn get(&self, key: &i128) -> Option<A>
    where
        A: Clone,
    {
        match self.map.get(key) {
            Some(c) => Some(c.clone()),
            None => None,
        }
    }

    pub fn set(&mut self, key: &i128, val: Option<A>)
    where
        A: Clone,
    {
        match val {
            Some(c) => {
                self.map.insert(key.clone(), c);
            }
            None => {
                self.map.remove(key);
            }
        }
    }

    pub fn bounds(&self) -> Option<(i128, i128)> {
        let mut bounds: Option<(i128, i128)> = None;

        for key in self.map.keys() {
            let k = key.clone();

            match bounds {
                None => {
                    bounds = Some((k, k));
                }
                Some((l, h)) => {
                    if k < l {
                        bounds = Some((k, h));
                    } else if k > h {
                        bounds = Some((l, k));
                    }
                }
            }
        }

        bounds
    }

    pub fn values(&self) -> Values<'_, i128, A> {
        self.map.values()
    }
}

pub fn from_vec<A>(buf: &Vec<u8>, conv: fn(&u8) -> A) -> Tape<A>
where
    A: Clone,
{
    let mut i: i128 = 0;
    let mut tape: Tape<A> = Tape {
        map: HashMap::new(),
    };

    buf.iter().for_each(|u| {
        tape.set(
            &i,
            if u.is_ascii_whitespace() {
                None
            } else {
                Some(conv(u))
            },
        );

        i = i + 1;
    });

    tape
}
