/// A tape containing infinite symbols, all initially blank. 
/// Can be modified by a turing machine.
#[derive(Clone, Debug, Default)]
pub struct Tape {
    raw_symbols: Vec<u64>,
}

impl Tape {
    /// Constructs a new `Tape` with the given input starting at location 0, counting up.
    /// 
    /// # Examples
    /// ```
    /// use turing_machine::tape::Tape;
    /// 
    /// let tape1 = Tape::new(vec![23, 1, 0, 49]);
    /// let tape2 = Tape::new(vec![23, 1, 0, 49, 0]);
    /// 
    /// // even though these tapes are constructed from different vectors,
    /// // they are mathematically identical
    /// assert_eq!(tape1, tape2);
    /// ```
    #[inline]
    pub fn new(input: Vec<u64>) -> Self {
        let mut tape = Tape::default();
        for s in 0..input.len() {
            tape.write(s as i64, input[s]);
        }

        tape
    }

    /// Constructs a new `Tape` with the given input starting at location 0, counting up.
    /// The internal vector has at least the specified capacity.
    /// 
    /// # Panics
    /// Panics if capacity is less than double input length minus one.
    /// ```should_panic
    /// # use turing_machine::tape::Tape;
    /// // panics because 6 < (2 * 4 - 1)
    /// let tape1 = Tape::with_capacity(vec![23, 1, 0, 49], 6);
    /// ```
    /// 
    /// # Examples
    /// ```
    /// use turing_machine::tape::Tape;
    /// 
    /// let tape1 = Tape::with_capacity(vec![23, 1, 0, 49], 20);
    /// let tape2 = Tape::new(vec![23, 1, 0, 49]);
    /// 
    /// assert!(tape1.raw_symbols().capacity() >= 20);
    /// assert!(tape2.raw_symbols().capacity() < 20);
    /// ```
    #[inline]
    pub fn with_capacity(input: Vec<u64>, capacity: usize) -> Self {
        if capacity < input.len() * 2 - 1 { panic!("Capacity must exceed double input len") };

        let mut tape = Tape::default();
        tape.raw_symbols.reserve(capacity);
        for s in 0..input.len() {
            tape.write(s as i64, input[s]);
        }
    
        tape
    }

    /// Returns the internal vector.
    #[inline]
    pub fn raw_symbols(&self) -> &Vec<u64> {
        &self.raw_symbols
    }

    /// Returns a `Vec` containing all meaningful symbols in `self`; 
    /// that is, a string of symbols containing all nonzero symbols and has no leading or trailing zeros.
    #[inline]
    pub fn symbols(&self) -> Vec<u64> {
        // order the symbols by their i64s
        let mut v1 = self.raw_symbols
            .iter()
            .enumerate()
            .map(|x| (idx_to_i64(x.0), *x.1))
            .collect::<Vec<(i64, u64)>>();
        v1.sort_unstable_by(|a, b| a.0.cmp(&b.0));

        // i64s are no longer needed
        let mut v2 = Vec::with_capacity(v1.capacity());
        for s in v1 {
            v2.push(s.1);
        }

        // remove leading and trailing zeros
        no_trailing_or_leading_zeros(&v2)
    }

    /// Returns the symbol at location n.
    #[inline]
    pub fn symbol_at_n(&self, n: i64) -> u64 {
        let idx = i64_to_idx(n);
        if idx >= self.raw_symbols.len() {
            0
        }
        else {
            self.raw_symbols[i64_to_idx(n)]
        }
    }

    /// Returns a vector containing all locations on `self` that have the specified symbol.
    #[inline]
    pub fn symbol(&self, symbol: u64) -> Vec<i64> {
        let mut vec = self.raw_symbols.iter()
            .enumerate()
            .filter(|&x| *x.1 == symbol)
            .map(|x| idx_to_i64(x.0))
            .collect::<Vec<i64>>();
        vec.sort();
        
        vec
    }

    /// Writes the specified symbol into `self` at location n.
    #[inline]
    pub(crate) fn write(&mut self, n: i64, symbol: u64) {
        let idx = i64_to_idx(n);
        if idx >= self.raw_symbols.len() {
            self.raw_symbols.resize(idx + 1, 0);
        }

        self.raw_symbols[idx] = symbol;
    }
}

impl PartialEq for Tape {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        no_trailing_or_leading_zeros(&self.raw_symbols) == no_trailing_or_leading_zeros(&other.raw_symbols)
    }
}

/// Converts an i64 location to an internal vector index.
#[inline]
fn i64_to_idx(int: i64) -> usize {
    ((4 * int.abs() + int.signum() - 1) / 2) as usize
}

// Converts an internal vector index to an i64 location.
#[inline]
fn idx_to_i64(idx: usize) -> i64 {
    (1 - 2 * (idx as i64 % 2)) * idx as i64 / 2 - idx as i64 % 2
}

/// Returns a modified vector containing no trailing or leading zeros
#[inline]
fn no_trailing_or_leading_zeros(vec: &Vec<u64>) -> Vec<u64> {
    if vec.iter().find(|&x| *x != 0).is_none() {
        return vec![];
    }

    vec[vec.iter().position(|x| *x != 0).unwrap()..vec.iter().rposition(|x| *x != 0).unwrap() + 1].to_vec()
}

#[test]
fn test_new() {
    assert_eq!(Tape { raw_symbols: vec![2, 0, 3, 0, 5] }, Tape::new(vec![2, 3, 5]));
}

#[test]
#[should_panic]
fn test_with_capacity() {
    Tape::with_capacity(vec![0, 1], 1);
}

#[test]
fn test_symbols() {
    let mut tape = Tape::new(vec![3, 34343, 1, 0, 25]);
    tape.write(-7, 946);

    assert_eq!(tape.raw_symbols(), &vec![3, 0, 34343, 0, 1, 0, 0, 0, 25, 0, 0, 0, 0, 946]);
    assert_eq!(tape.symbols(), vec![946, 0, 0, 0, 0, 0, 0, 3, 34343, 1, 0, 25]);
}

#[test]
fn test_symbol_at_n() {
    let mut tape = Tape::new(vec![2, 342,  6, 91]);
    tape.write(-1, 55991);

    assert_eq!(tape.symbol_at_n(0), 2);
    assert_eq!(tape.symbol_at_n(1), 342);
    assert_eq!(tape.symbol_at_n(3), 91);
    assert_eq!(tape.symbol_at_n(-1), 55991);
}

#[test]
fn test_symbol() {
    let mut tape = Tape::new(vec![0, 2, 2, 0, 2, 3, 2]);
    tape.write(-1, 0);
    tape.write(-2, 2);
    tape.write(-3, 2);
    tape.write(-4, 12);

    println!("{:?}", tape.raw_symbols);
    assert_eq!(tape.symbol(2), vec![-3, -2, 1, 2, 4, 6]);
}

#[test]
fn test_write() {
    let mut tape = Tape::new(vec![0, 22, 3]);
    tape.write(-2, 73);
    tape.write(59, 12);
    
    assert_eq!(tape.symbol_at_n(0), 0);
    assert_eq!(tape.symbol_at_n(-2), 73);
    assert_eq!(tape.symbol_at_n(-1), 0);
    assert_eq!(tape.symbol_at_n(-3), 0);
    assert_eq!(tape.symbol_at_n(2), 3);
    assert_eq!(tape.symbol_at_n(32193824), 0);
}

#[test]
fn test_i64_to_idx() {
    assert_eq!(i64_to_idx(-2), 3);
    assert_eq!(i64_to_idx(-1), 1);
    assert_eq!(i64_to_idx(0), 0);
    assert_eq!(i64_to_idx(1), 2);
    assert_eq!(i64_to_idx(2), 4);
}

#[test]
fn test_idx_to_i64() {
    assert_eq!(idx_to_i64(0), 0);
    assert_eq!(idx_to_i64(1), -1);
    assert_eq!(idx_to_i64(2), 1);
    assert_eq!(idx_to_i64(3), -2);
    assert_eq!(idx_to_i64(4), 2);
    assert_eq!(idx_to_i64(5), -3);
    assert_eq!(idx_to_i64(6), 3);
    assert_eq!(idx_to_i64(7), -4);
}

#[test]
fn test_no_trailing_or_leading_zeros() {
    let v = vec![0, 0, 3, 0, 4, 0];
    assert_eq!(no_trailing_or_leading_zeros(&v), vec![3, 0, 4]);
}