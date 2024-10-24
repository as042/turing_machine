use crate::prelude::*;

/// BB-2 shown in terminal.
pub fn busy_beaver_2_state() {
    let trans_fn = TransitionFn::new(
        &vec![
            ((0, 0), (1, 1, true)),
            ((1, 0), (0, 1, false)),
            ((0, 1), (1, 1, false)),
        ]
    );

    let mut machine = TuringMachine::new(trans_fn);
    let mut tape = Tape::default();

    let record = machine.run_with_halt_setting_and_record(&mut tape, HaltSetting::AfterSteps(100));

    record.play_in_console(std::time::Duration::from_millis(1000), true);
}

/// BB-3 shown in terminal.
pub fn busy_beaver_3_state() {
    let trans_fn = TransitionFn::new(
        &vec![
            ((0, 0), (1, 1, true)),
            ((0, 1), (2, 1, true)),
            ((1, 0), (2, 1, false)),
            ((1, 1), (2, 1, false)),
            ((2, 0), (3, 1, false)),
            ((2, 1), (1, 1, true)),
        ]
    );

    let mut machine = TuringMachine::new(trans_fn);
    let mut tape = Tape::default();

    let record = machine.run_with_halt_setting_and_record(&mut tape, HaltSetting::AfterSteps(100));

    record.play_in_console(std::time::Duration::from_millis(500), true);
}