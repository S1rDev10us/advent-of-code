pub mod dbg;
pub mod grid;
pub mod input;
pub mod merge;
pub mod position;
pub mod results;
#[cfg(test)]
pub mod tests;
pub mod prelude {
    pub use crate::dbg::display;
    pub use crate::grid::*;
    pub use crate::input::{initialize, initialize_macro};
    pub use crate::merge::merge_dupes;
    pub use crate::position::*;
    pub use crate::results::check_result;
}
