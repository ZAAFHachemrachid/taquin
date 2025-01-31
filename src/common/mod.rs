pub mod board;
pub mod utils;
pub mod visualization;

pub use board::{Board, Direction};
pub use utils::{ColoredText, Config};
pub use visualization::{print_side_by_side, write_results_to_file, MethodState};
