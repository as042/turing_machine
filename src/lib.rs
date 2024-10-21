pub mod tape;
pub mod transition_fn;
pub mod turing_machine;

pub mod prelude {
    pub use crate::tape::*;
    pub use crate::transition_fn::*;
    pub use crate::turing_machine::*;
}