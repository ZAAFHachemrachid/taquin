use crate::common::{Board, Direction};
use crate::{SolutionInfo, Solver};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

#[derive(Clone, Eq)]
struct State {
    state: Vec<u8>,
    goal_state: Vec<u8>,
    blank_pos: usize,
    size: usize,
    path: Vec<Direction>,
    g_cost: u32, // Cost from start to current node
    h_cost: u32, // Heuristic cost (Manhattan distance)
}

impl State {
    fn new(board: &Board) -> Self {
        let state: Vec<u8> = board
            .get_state()
            .into_iter()
            .flat_map(|row| row.into_iter())
            .collect();
        let goal_state = board.get_goal_state();
        let size = board.get_size();
        let h_cost = AStarSolver::manhattan_distance(&state, &goal_state, size);
        State {
            state,
            goal_state,
            blank_pos: 0, // Will be set in initialization
            size,
            path: Vec::new(),
            g_cost: 0,
            h_cost,
        }
    }

    fn f_cost(&self) -> u32 {
        self.g_cost + self.h_cost
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Lower f_cost = higher priority
        // If f_costs are equal, prefer higher g_cost (deeper nodes)
        other
            .f_cost()
            .cmp(&self.f_cost())
            .then_with(|| self.g_cost.cmp(&other.g_cost))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.f_cost() == other.f_cost() && self.g_cost == other.g_cost
    }
}

impl State {
    fn is_goal(&self) -> bool {
        self.state == self.goal_state
    }

    fn get_possible_moves(&self) -> Vec<Direction> {
        let mut moves = Vec::new();
        let row = self.blank_pos / self.size;
        let col = self.blank_pos % self.size;

        if row > 0 {
            moves.push(Direction::Up);
        }
        if row < self.size - 1 {
            moves.push(Direction::Down);
        }
        if col > 0 {
            moves.push(Direction::Left);
        }
        if col < self.size - 1 {
            moves.push(Direction::Right);
        }

        moves
    }

    fn make_move(&self, dir: Direction) -> Option<State> {
        let row = self.blank_pos / self.size;
        let col = self.blank_pos % self.size;

        let new_pos = match dir {
            Direction::Up if row > 0 => self.blank_pos - self.size,
            Direction::Down if row < self.size - 1 => self.blank_pos + self.size,
            Direction::Left if col > 0 => self.blank_pos - 1,
            Direction::Right if col < self.size - 1 => self.blank_pos + 1,
            _ => return None,
        };

        let mut new_state = self.state.clone();
        new_state.swap(self.blank_pos, new_pos);

        let mut new_path = self.path.clone();
        new_path.push(dir);

        let new_h_cost = AStarSolver::manhattan_distance(&new_state, &self.goal_state, self.size);

        Some(State {
            state: new_state,
            goal_state: self.goal_state.clone(),
            blank_pos: new_pos,
            size: self.size,
            path: new_path,
            g_cost: self.g_cost + 1, // Increment cost by 1 for each move
            h_cost: new_h_cost,
        })
    }
}

pub struct AStarSolver {
    initial_board: Board,
}

impl AStarSolver {
    fn find_blank_pos(state: &[u8]) -> usize {
        state.iter().position(|&x| x == 0).unwrap_or(0)
    }

    fn manhattan_distance(state: &[u8], goal_state: &[u8], size: usize) -> u32 {
        let mut distance = 0;
        for pos in 0..state.len() {
            let value = state[pos];
            if value != 0 {
                if let Some(goal_pos) = goal_state.iter().position(|&x| x == value) {
                    let current_row = pos / size;
                    let current_col = pos % size;
                    let goal_row = goal_pos / size;
                    let goal_col = goal_pos % size;
                    distance +=
                        (current_row.abs_diff(goal_row) + current_col.abs_diff(goal_col)) as u32;
                }
            }
        }
        distance
    }
}

impl Solver for AStarSolver {
    fn new(initial: Board) -> Self {
        AStarSolver {
            initial_board: initial,
        }
    }

    fn new_with_goal(initial: Board) -> Self {
        AStarSolver {
            initial_board: initial,
        }
    }

    fn solve(&self, _optimal_length: Option<usize>) -> Option<SolutionInfo> {
        // Initialize start state
        let state: Vec<u8> = self
            .initial_board
            .get_state()
            .into_iter()
            .flat_map(|row| row.into_iter())
            .collect();
        let goal_state = self.initial_board.get_goal_state();

        let mut initial_state = State {
            state: state.clone(),
            goal_state: goal_state.clone(),
            blank_pos: Self::find_blank_pos(&state),
            size: self.initial_board.get_size(),
            path: Vec::new(),
            g_cost: 0,
            h_cost: Self::manhattan_distance(&state, &goal_state, self.initial_board.get_size()),
        };

        let mut open_set = BinaryHeap::new();
        let mut closed_set = HashSet::new();

        open_set.push(initial_state);

        while let Some(current_state) = open_set.pop() {
            // Goal test
            if current_state.is_goal() {
                return Some(SolutionInfo::new(current_state.path, None));
            }

            // Add current state to closed set
            if !closed_set.insert(current_state.state.clone()) {
                continue; // Skip if we've already explored this state
            }

            // Generate and explore successors
            for direction in current_state.get_possible_moves() {
                if let Some(next_state) = current_state.make_move(direction) {
                    if !closed_set.contains(&next_state.state) {
                        open_set.push(next_state);
                    }
                }
            }
        }

        None // No solution found
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_simple_puzzle() {
        // Test a simple 2x2 puzzle
        let initial_state = vec![vec![1, 0], vec![3, 2]];
        let board = Board::new(initial_state);
        let solver = AStarSolver::new(board.clone());
        let solution = solver.solve(None);

        assert!(solution.is_some(), "Should find a solution");
        let steps = &solution.unwrap().moves;
        assert!(!steps.is_empty(), "Solution should have steps");
    }

    #[test]
    fn test_already_solved() {
        // Test an already solved 2x2 puzzle
        let initial_state = vec![vec![1, 2], vec![3, 0]];
        let board = Board::new(initial_state);
        let solver = AStarSolver::new(board.clone());
        let solution = solver.solve(None);

        assert!(solution.is_some(), "Should find a solution");
        let steps = &solution.unwrap().moves;
        assert!(
            steps.is_empty(),
            "Already solved puzzle should have no steps"
        );
    }

    #[test]
    fn test_3x3_puzzle() {
        // Test a 3x3 puzzle
        let initial_state = vec![vec![1, 2, 3], vec![4, 0, 6], vec![7, 5, 8]];
        let board = Board::new(initial_state);
        let solver = AStarSolver::new(board.clone());
        let solution = solver.solve(None);

        assert!(solution.is_some(), "Should find a solution");
        let steps = &solution.unwrap().moves;
        assert!(!steps.is_empty(), "Solution should have steps");
    }

    #[test]
    fn test_solution_validity() {
        // Test if solution reaches the goal state
        let initial_state = vec![vec![1, 0], vec![2, 3]];
        let board = Board::new(initial_state);
        let solver = AStarSolver::new(board.clone());
        let solution = solver.solve(None).unwrap();

        // Apply moves to reach final state
        let mut test_board = board;
        for move_dir in solution.moves {
            assert!(
                test_board.make_move(move_dir).is_ok(),
                "Move should be valid"
            );
        }

        // Verify final state is correct
        let final_state = test_board.get_state();
        let expected = vec![vec![1, 2], vec![3, 0]];
        assert_eq!(final_state, expected, "Final state should match goal state");
    }
}
