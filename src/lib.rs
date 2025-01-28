pub mod a_star;
pub mod bfs;
pub mod common;
pub mod dfs;

pub use a_star::AStarSolver;
pub use bfs::BFSSolver;
pub use common::Board;
pub use common::Direction;
pub use dfs::DFSSolver;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MoveQuality {
    Best,   // Optimal solution (same as A*)
    Medium, // Within 50% more moves than optimal
    Poor,   // More than 50% more moves than optimal
}

#[derive(Debug, Clone)]
pub struct SolutionInfo {
    pub moves: Vec<Direction>,
    pub quality: MoveQuality,
}

impl SolutionInfo {
    pub fn new(moves: Vec<Direction>, optimal_length: Option<usize>) -> Self {
        let quality = if let Some(optimal) = optimal_length {
            if moves.len() == optimal {
                MoveQuality::Best
            } else if moves.len() <= optimal + (optimal / 2) {
                MoveQuality::Medium
            } else {
                MoveQuality::Poor
            }
        } else {
            MoveQuality::Best // If no optimal length provided, assume it's best
        };

        SolutionInfo { moves, quality }
    }
}

// Define solver trait for consistent interface
pub trait Solver {
    fn solve(&self, optimal_length: Option<usize>) -> Option<SolutionInfo>;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_board() -> Board {
        // Create a simple 3x3 puzzle that requires just a few moves to solve
        // 1 2 3
        // 4 5 0   (0 represents the empty space)
        // 7 8 6
        Board::new(vec![vec![1, 2, 3], vec![4, 5, 0], vec![7, 8, 6]])
    }

    #[test]
    fn test_bfs_solver() {
        let board = create_test_board();
        let solver = BFSSolver::new(board);
        let solution = solver.solve(None);
        assert!(solution.is_some());
    }

    #[test]
    fn test_dfs_solver() {
        let board = create_test_board();
        let solver = DFSSolver::new(board);
        let solution = solver.solve(None);
        assert!(solution.is_some());
    }

    #[test]
    fn test_astar_solver() {
        let board = create_test_board();
        let solver = AStarSolver::new(board);
        let solution = solver.solve(None);
        assert!(solution.is_some());
    }

    #[test]
    fn test_is_goal_state() {
        let goal_board = Board::new(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 0]]);
        assert!(goal_board.is_goal());

        let non_goal_board = create_test_board();
        assert!(!non_goal_board.is_goal());
    }
}
