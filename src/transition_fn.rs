use std::collections::{HashMap, HashSet};
use std::hash::{BuildHasherDefault, Hasher};

/// A representation of a turing machine's transition function.
/// It takes a state and a symbol and returns a new state, new symbol, 
/// and whether to move left or right.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TransitionFn {
    map: HashMap<(u64, u64), (u64, u64, bool), PairingBuildHasher>,
}

impl TransitionFn {
    /// Constructs a new `TransitionFn` from the specified state table.
    /// The state table is expressed in the form:
    /// `Key: (state, symbol) -> Value: (new state, symbol to write, head movement)` 
    /// where head movement is a `bool` (`true` for right, `false` for left).
    /// 
    /// # Panics
    /// Panics if the state table contains identical keys because this is a
    /// deterministic function.
    /// ```should_panic
    /// use turing_machine::transition_fn::TransitionFn;
    /// 
    /// let v = vec![((2, 1), (30, 12, false)), ((2, 1), (35, 3, true))];
    /// 
    /// let trans_fn = TransitionFn::new(&v);
    /// ```
    /// 
    /// # Examples
    /// ```
    /// # use turing_machine::transition_fn::TransitionFn;
    /// let v = vec![((1, 19), (30, 12, false)), ((22, 79), (35, 3, true))];
    /// 
    /// let trans_fn = TransitionFn::new(&v);
    /// ```
    #[inline]
    pub fn new(state_table: &Vec<((u64, u64), (u64, u64, bool))>) -> Self {
        TransitionFn {
            map: HashMap::from_iter(state_table
                    .iter()
                    .map(|x| *x)
                    .scan(HashSet::new(), |state: &mut HashSet<(u64, u64)>, x| {
                        if state.contains(&x.0) {
                            panic!()
                        }

                        state.insert(x.0);

                        Some(x)
                    }
                )
            ),
        }
    }

    /// Returns the state table of `self` in no particular order.
    #[inline]
    pub fn state_table(&self) -> Vec<((u64, u64), (u64, u64, bool))> {
        self.map.iter().map(|x| (*x.0, *x.1)).collect()
    }

    /// Runs `self` with the specified state and symbol and returns `Some((u64, u64, bool))`
    /// only if a match is found within the state table, otherwise it returns `None`.
    /// # Examples
    /// ```
    /// use turing_machine::transition_fn::TransitionFn;
    /// 
    /// let v = vec![((0, 0), (1, 2, true)), ((1, 0), (0, 1, false))];
    /// 
    /// let trans_fn = TransitionFn::new(&v);
    /// 
    /// assert_eq!(trans_fn.run(0, 0), Some((1, 2, true)));
    /// assert_eq!(trans_fn.run(1, 0), Some((0, 1, false)));
    /// assert_eq!(trans_fn.run(0, 1), None);
    /// assert_eq!(trans_fn.run(1, 1), None);
    /// assert_eq!(trans_fn.run(123, 97412), None);
    /// ```
    #[inline]
    pub fn run(&self, state: u64, symbol: u64) -> Option<(u64, u64, bool)> {
        self.map.get(&(state, symbol)).copied()
    }
}

#[derive(Default)]
pub(super) struct PairingHasher {
    hash: u64,
    n: u64,
}

impl Hasher for PairingHasher {
    #[inline]
    fn finish(&self) -> u64 {
        self.hash
    }

    #[inline]
    fn write(&mut self, _: &[u8]) {
        panic!("This hasher only accepts u64");
    }

    #[inline]
    fn write_u64(&mut self, i: u64) {
        // if n = 0, hash = i, else hash = cantor_pairing_function(hash, i)
        self.hash = i + self.n * (self.hash + i) * (self.hash + i + 1) / 2;
        self.n = 1;
    }
}

pub(super) type PairingBuildHasher = BuildHasherDefault<PairingHasher>;

#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::prelude::*;

    #[test]
    fn test_new() {
        let v = vec![((1, 2), (3, 2, false)), ((6, 7), (7, 8, true))];
        let trans_fn = TransitionFn::new(&v);
    
        let mut map = HashMap::with_hasher(PairingBuildHasher::default());
        map.insert((1, 2), (3, 2, false));
        map.insert((6, 7), (7, 8, true));
    
        assert_eq!(trans_fn, TransitionFn { map });
    }

    #[test]
    #[should_panic]
    fn panic_test_new() {
        TransitionFn::new(
            &vec![
                ((1, 23), (0, 15, true)),
                ((1, 23), (1, 72, false)),
            ]
        );
    }
    
    #[test]
    fn test_state_table() {
        let v = vec![((5, 19), (30, 12, true)), ((26, 90), (74, 1, false))];
        let trans_fn = TransitionFn::new(&v);
    
        assert_eq!(v, trans_fn.state_table().iter().rev().map(|x| (x.0, x.1)).collect::<Vec<((u64, u64), (u64, u64, bool))>>());
    }
    
    #[test]
    fn test_run() {
        let trans_fn = TransitionFn::new(
            &vec![
                ((300, 23), (0, 15, true)),
                ((1, 23), (1, 72, true)),
                ((4, 2), (2, 49, true)),
                ((66, 64), (3, 19, false)),
                ((123, 5), (4, 1, false)),
                ((523, 533), (5, 1, true)),
                ((12, 111), (6, 87, true)),
                ((53, 352), (7, 12, true)),
                ((53, 23), (8, 0, false))
            ]
        );
    
        assert_eq!(trans_fn.run(4, 2).unwrap(), (2, 49, true));
        assert_eq!(trans_fn.run(66, 64).unwrap(), (3, 19, false));
        assert_eq!(trans_fn.run(523, 533).unwrap(), (5, 1, true));
        assert_eq!(trans_fn.run(12, 111).unwrap(), (6, 87, true));
        assert_eq!(trans_fn.run(53, 23).unwrap(), (8, 0, false));
    }
}