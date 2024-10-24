use turing_machine::{prelude::*, tests::*};

fn main() {
    println!("{:?}", TuringMachine::chaitin_approx(3, 2, HaltSetting::AfterSteps(100)));
}