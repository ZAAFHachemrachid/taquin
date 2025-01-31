pub mod a_star;
pub mod bfs;
pub mod common;
pub mod dfs;

// Re-export common types that other modules will use
pub use common::{Board, ColoredText, Direction};

#[derive(Debug, Clone)]
pub enum SolutionQuality {
    Optimal,
    Good,
    Poor,
}

impl SolutionQuality {
    pub fn to_colored_string(&self) -> String {
        match self {
            SolutionQuality::Optimal => ColoredText::green("Optimal"),
            SolutionQuality::Good => ColoredText::yellow("Good"),
            SolutionQuality::Poor => ColoredText::red("Poor"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SolutionInfo {
    pub moves: Vec<Direction>,
    pub optimal_length: Option<usize>,
    pub quality: SolutionQuality,
}

impl SolutionInfo {
    pub fn new(moves: Vec<Direction>, optimal_length: Option<usize>) -> Self {
        let quality = if let Some(optimal) = optimal_length {
            match moves.len() {
                len if len == optimal => SolutionQuality::Optimal,
                len if len <= optimal + 5 => SolutionQuality::Good,
                _ => SolutionQuality::Poor,
            }
        } else {
            SolutionQuality::Good // Default when optimal length is unknown
        };

        SolutionInfo {
            moves,
            optimal_length,
            quality,
        }
    }

    pub fn display_solution(&self) -> String {
        let quality_str = self.quality.to_colored_string();
        let moves_len = self.moves.len();

        let mut result = format!("Found {} solution in {} moves", quality_str, moves_len);
        if let Some(optimal) = self.optimal_length {
            result.push_str(&format!(" (Optimal: {})", optimal));
        }
        result
    }
}

pub trait Solver {
    fn new(board: Board) -> Self;
    fn solve(&self, optimal_length: Option<usize>) -> Option<SolutionInfo>;
    fn new_with_goal(board: Board) -> Self;
}

// Re-export solvers
pub use a_star::AStarSolver;
pub use bfs::BFSSolver;
pub use dfs::DFSSolver;
