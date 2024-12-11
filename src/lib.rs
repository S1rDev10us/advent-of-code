pub mod dbg;
pub mod grid;
pub mod input;
pub mod results;
pub mod prelude {
    pub use crate::dbg::display;
    pub use crate::grid::*;
    pub use crate::input::{initialize, initialize_macro};
    pub use crate::results::check_result;
}
