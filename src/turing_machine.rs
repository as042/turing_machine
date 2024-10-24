use std::time::{Duration, Instant};

use crate::prelude::Recording;
use crate::tape::Tape;
use crate::transition_fn::TransitionFn;

/// A simulation of a Turing machine, aka an "a-machine", 
/// a concept invented by Alan Turing in 1936.
/// This type is inherently mutable as it represents
/// an actual Turing machine moving around and changing states.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TuringMachine {
    transition_fn: TransitionFn,
    state: u64,
    head_loc: i64,
}

impl TuringMachine {
    /// Constructs a new Turing machine from the specified transition function.
    /// Initial state and head location are always 0.
    #[inline]
    pub fn new(transition_fn: TransitionFn) -> Self {
        TuringMachine {
            transition_fn,
            ..Default::default()
        }
    }

    /// Returns the transition function of `self`.
    #[inline]
    pub fn transition_fn(&self) -> &TransitionFn {
        &self.transition_fn
    }

    /// Returns the current state of `self`.
    #[inline]
    pub fn state(&self) -> u64 {
        self.state
    }

    /// Returns the current head location of `self`.
    #[inline]
    pub fn head_loc(&self) -> i64 {
        self.head_loc
    }

    /// Resets the state and head location of `self` to their initial values of 0.
    #[inline]
    pub fn reset(&mut self) {
        self.state = 0;
        self.head_loc = 0;
    }

    /// Runs `self`, changing its state and moving its head while writing to the specified tape.
    #[inline]
    pub fn run(&mut self, tape: &mut Tape) {
        let mut symbol;
        loop {
            symbol = tape.symbol_at_n(self.head_loc);
            if let Some(output) = self.transition_fn.run(self.state, symbol) {
                self.state = output.0;
                tape.write(self.head_loc, output.1);
                self.head_loc += output.2 as i64 * 2 - 1;
            }
            else {
                break;
            }
        }
    }

    /// Runs `self`, changing its state and moving its head while writing to the specified tape.
    /// Returns a `Recording` of the process that contains all steps and can be played back.
    #[inline]
    pub fn run_and_record(&mut self, tape: &mut Tape) -> Recording {
        let input = tape.clone();
        let init_state = self.state.clone();
        let init_head_loc = self.head_loc.clone();
        let mut steps = Vec::default();

        let mut symbol;
        loop {
            symbol = tape.symbol_at_n(self.head_loc);
            if let Some(output) = self.transition_fn.run(self.state, symbol) {
                self.state = output.0;
                tape.write(self.head_loc, output.1);
                self.head_loc += output.2 as i64 * 2 - 1;

                steps.push(output);
            }
            else {
                break;
            }
        }

        Recording {
            input,
            init_state,
            init_head_loc,
            steps,
        }
    }

    /// Runs `self`, changing its state and moving its head while writing to the specified tape.
    /// Takes in a `HaltSetting` that describes when the machine should be forcibly halted.
    #[inline]
    pub fn run_with_halt_setting(&mut self, tape: &mut Tape, halt_setting: HaltSetting) {
        if halt_setting == HaltSetting::NoForcedHalt {
            self.run(tape);
            return;
        }

        let start = Instant::now();
        let mut step_num = 0;

        let mut symbol;
        loop {
            if let HaltSetting::AfterSteps(max_steps) = halt_setting {
                if step_num >= max_steps {
                    break;
                }
                step_num += 1;
            }
            else if let HaltSetting::AfterDuration(max_duration) = halt_setting {
                if start.elapsed() >= max_duration {
                    break;
                }
            } 

            symbol = tape.symbol_at_n(self.head_loc);
            if let Some(output) = self.transition_fn.run(self.state, symbol) {
                self.state = output.0;
                tape.write(self.head_loc, output.1);
                self.head_loc += output.2 as i64 * 2 - 1;
            }
            else {
                break;
            }
        }
    }

    /// Runs `self`, changing its state and moving its head while writing to the specified tape.
    /// Takes in a `HaltSetting` that describes when the machine should be forcibly halted.
    /// Returns a `Recording` of the process that contains all steps and can be played back.
    #[inline]
    pub fn run_with_halt_setting_and_record(&mut self, tape: &mut Tape, halt_setting: HaltSetting) -> Recording {
        let input = tape.clone();
        let init_state = self.state.clone();
        let init_head_loc = self.head_loc.clone();
        let mut steps = Vec::default();

        if halt_setting == HaltSetting::NoForcedHalt {
            return self.run_and_record(tape);
        }

        let start = Instant::now();
        let mut step_num = 0;

        let mut symbol;
        loop {
            if let HaltSetting::AfterSteps(max_steps) = halt_setting {
                if step_num >= max_steps {
                    break;
                }
                step_num += 1;
            }
            else if let HaltSetting::AfterDuration(max_duration) = halt_setting {
                if start.elapsed() >= max_duration {
                    break;
                }
            } 

            symbol = tape.symbol_at_n(self.head_loc);
            if let Some(output) = self.transition_fn.run(self.state, symbol) {
                self.state = output.0;
                tape.write(self.head_loc, output.1);
                self.head_loc += output.2 as i64 * 2 - 1;

                steps.push(output);
            }
            else {
                break;
            }
        }

        Recording {
            input,
            init_state,
            init_head_loc,
            steps,
        }
    }

    #[inline]
    pub fn chaitin_approx(num_states: usize, num_symbols: usize, halt_setting: HaltSetting) -> (f64, f64) {
        let trans_fns = TransitionFn::enumerate(num_states, num_symbols);
        let mut halted = 0;
        let mut undecided = 0;

        for t in 0..trans_fns.len() {
            let mut tm = TuringMachine::new(trans_fns[t].clone());

            tm.run_with_halt_setting(&mut Tape::default(), halt_setting);
            
            if tm.state == num_states as u64 {
                halted += 1;
            }
            else {
                undecided += 1;
            }
        }

        (halted as f64 / trans_fns.len() as f64, undecided as f64 / trans_fns.len() as f64)
    }
}

/// A parameter type that describes when a Turing machine should be forcibly halted.
/// The `NoForcedHalt` variant simply states that the machine should not be forcibly halted.
/// The `AfterSteps(usize)` variant states that it should be halted after `usize` number of steps;
/// i.e., the machine has written to the tape `usize` number of times.
/// The `AfterDuration(Duration)` variant states the machine should be halted after a `Duration` has elapsed.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum HaltSetting {
    #[default]
    NoForcedHalt,
    AfterSteps(usize),
    AfterDuration(Duration),
}

#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::prelude::*;

    #[test]
    fn test_reset() {
        let trans_fn = TransitionFn::new(
            &vec![
                ((0, 0), (1, 312, true)),
                ((1, 0), (2, 0, false)),
                ((2, 0), (7, 4, true)),
                ((3, 0), (4, 0, false)),
                ((4, 0), (5, 2, false)),
                ((2, 312), (3, 999, false)),
                ((0, 999), (6, 999, false)),
                ((1, 2), (2, 2, false)),
                ((3, 2), (4, 4, false)),
                ((5, 999), (8, 73, false)),
                ((6, 0), (1, 0, false)),
            ]
        );

        let mut machine = TuringMachine::new(trans_fn);
        let mut tape = Tape::default();

        machine.run(&mut tape);

        assert_eq!(tape.symbols(), [2, 0, 999]);

        machine.reset(); // resets head location and state
        machine.run(&mut tape);

        assert_eq!(tape.symbols(), [4, 2, 0, 999]);
    }

    #[test]
    fn test_run() {
        let trans_fn = TransitionFn::new(
            &vec![
                ((0, 0), (10, 10, false)),
                ((10, 0), (9, 9, false)),
                ((9, 0), (8, 8, false)),
                ((8, 0), (7, 7, false)),
                ((7, 0), (6, 6, false)),
                ((6, 0), (5, 5, false)),
                ((5, 0), (4, 4, false)),
                ((4, 0), (3, 3, false)),
                ((3, 0), (2, 2, false)),
                ((2, 0), (1, 1, false)),
            ]
        );

        let mut machine = TuringMachine::new(trans_fn);
        let mut tape = Tape::default();

        machine.run(&mut tape);

        assert_eq!(tape.symbols(), [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn test_run_and_record() {
        let trans_fn = TransitionFn::new(
            &vec![
                ((0, 0), (1, 1, true)),
                ((1, 0), (1, 4, false)),
                ((1, 1), (2, 1, false)),
                ((2, 0), (3, 3, true)),
            ]
        );

        let mut machine = TuringMachine::new(trans_fn);
        let tape = Tape::new(vec![0, 0, 1, 5, 9]);
        let mut tape2 = tape.clone();

        let record = machine.run_and_record(&mut tape2);

        assert_eq!(tape2.symbols(), vec![3, 1, 4, 1, 5, 9]);
        assert_eq!(record.input, tape);
        assert_eq!(record.steps, [(1, 1, true), (1, 4, false), (2, 1, false), (3, 3, true)]);
    }

    #[test]
    fn test_run_with_halt_setting() {
        let trans_fn = TransitionFn::new(
            &vec![
                ((0, 0), (1, 1, true)),
                ((1, 0), (2, 2, true)),
                ((2, 0), (0, 3, true)),
            ]
        );

        let mut machine = TuringMachine::new(trans_fn);
        let mut tape = Tape::default();
        machine.run_with_halt_setting(&mut tape, HaltSetting::AfterSteps(7));
        assert_eq!(tape.symbols(), [1, 2, 3, 1, 2, 3, 1]);

        machine.reset();
        tape = Tape::default();
        machine.run_with_halt_setting(&mut tape, HaltSetting::AfterSteps(1));
        assert_eq!(tape.symbols(), [1]);

        machine.reset();
        tape = Tape::default();
        machine.run_with_halt_setting(&mut tape, HaltSetting::AfterSteps(0));
        assert_eq!(tape.symbols(), []);

        machine.reset();
        tape = Tape::default();
        machine.run_with_halt_setting(&mut tape, HaltSetting::AfterDuration(Duration::from_micros(1000)));
    }

    #[test]
    fn test_run_with_halt_setting_and_record() {
        let trans_fn = TransitionFn::new(
            &vec![
                ((0, 0), (1, 1, true)),
                ((1, 0), (0, 1, false)),
                ((0, 1), (1, 2, true)),
                ((1, 1), (0, 2, false)),
                ((0, 2), (1, 3, true)),
                ((1, 2), (0, 3, false)),
                ((0, 3), (1, 1, true)),
                ((1, 3), (0, 1, false)),
            ]
        );

        let mut machine = TuringMachine::new(trans_fn);
        let tape = Tape::default();
        let mut tape2 = tape.clone();

        let record = machine.run_with_halt_setting_and_record(&mut tape2, HaltSetting::AfterSteps(5));

        assert_eq!(tape2.symbols(), vec![3, 2]);
        assert_eq!(record.input, tape);
        assert_eq!(record.steps, [(1, 1, true), (0, 1, false), (1, 2, true), (0, 2, false), (1, 3, true)]);
    }
}