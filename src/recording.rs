use std::{thread::sleep, time::Duration};

use crate::tape::Tape;

/// A log of the movements and operations of a specific `TuringMachine`.
#[derive(Clone, Default, Debug, PartialEq)]
pub struct Recording {
    pub(crate) input: Tape,
    pub(crate) init_state: u64,
    pub(crate) init_head_loc: i64,
    pub(crate) steps: Vec<(u64, u64, bool)>,
}

impl Recording {
    /// Plays back a "movie" of the Turing machine in the terminal.
    #[inline]
    pub fn play_in_console(&self, step_delay: Duration, cls: bool) {
        let mut tape = self.input.clone();

        let mut head_loc = self.init_head_loc;
        let mut state = self.init_state;

        recording_print(&tape, head_loc, state, 0);
        sleep(step_delay);
        if cls { print!("{}[2J", 27 as char) };

        for s in self.steps.clone() {
            state = s.0;
            tape.write(head_loc, s.1);

            let head_move = s.2 as i64 * 2 - 1;

            if cls { print!("{}[2J", 27 as char) };
            recording_print(&tape, head_loc, state, head_move);
            sleep(step_delay);

            head_loc += head_move;

            if cls { print!("{}[2J", 27 as char) };
            recording_print(&tape, head_loc, state, 0);
            sleep(step_delay);
        }
    }
}

#[inline]
fn recording_print(tape: &Tape, head_loc: i64, state: u64, head_move: i64) {
    let mut move_left = "    ";
    let mut move_right = "";
    if head_move == -1 {
        move_left = "<-- ";
    }
    else if head_move == 1 {
        move_right = " -->";
    }

    println!("
                                                       {}H({}){}
Tape:      {}         {}         {}         {}         {}         {}         {}         {}         {}         {}         {}

index: {: ^9} {: ^9} {: ^9} {: ^9} {: ^9} {: ^9} {: ^9} {: ^9} {: ^9} {: ^9} {: ^9}", 
    move_left, state, move_right, tape.symbol_at_n(head_loc - 5), tape.symbol_at_n(head_loc - 4), tape.symbol_at_n(head_loc - 3), 
    tape.symbol_at_n(head_loc - 2), tape.symbol_at_n(head_loc - 1), tape.symbol_at_n(head_loc), 
    tape.symbol_at_n(head_loc + 1), tape.symbol_at_n(head_loc + 2), tape.symbol_at_n(head_loc + 3), 
    tape.symbol_at_n(head_loc + 4), tape.symbol_at_n(head_loc + 5),
    head_loc - 5, head_loc - 4, head_loc - 3, head_loc - 2, head_loc - 1, head_loc,
    head_loc + 1, head_loc + 2, head_loc + 3, head_loc + 4, head_loc + 5
    );
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn recording_test() {
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
        let mut tape = Tape::default();
    
        let record = machine.run_with_halt_setting_and_record(&mut tape, HaltSetting::AfterSteps(20));
    
        record.play_in_console(std::time::Duration::from_micros(1), false);
    }
}
