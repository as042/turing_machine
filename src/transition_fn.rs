use std::collections::HashMap;
use std::hash::{BuildHasherDefault, Hasher};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TransitionFn {
    map: HashMap<(u64, u64), (u64, u64, i64), PairingBuildHasher>,
}

impl TransitionFn {
    #[inline]
    pub fn new(state_table: &Vec<((u64, u64), (u64, u64, i64))>) -> Self {
        TransitionFn {
            map: HashMap::from_iter(state_table.iter().map(|x| *x)),
        }
    }

    #[inline]
    pub fn run(&self, state: u64, symbol: u64) -> Option<(u64, u64, i64)> {
        self.map.get(&(state, symbol)).copied()
    }
}

#[derive(Default)]
pub struct PairingHasher {
    state: u64,
}

impl Hasher for PairingHasher {
    fn finish(&self) -> u64 {
        self.state
    }

    fn write(&mut self, bytes: &[u8]) {
        todo!()
    }
}

pub type PairingBuildHasher = BuildHasherDefault<PairingHasher>;