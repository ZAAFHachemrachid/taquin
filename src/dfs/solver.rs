use crate::common::{Board, Direction};
use crate::{SolutionInfo, Solver};
use std::collections::HashSet;

// State struct for DFS
#[derive(Clone)]
struct State {
    state: Vec<u8>,
    goal_state: Vec<u8>,
    blank_pos: usize,
    size: usize,
    path: Vec<Direction>,
}

impl State {
    fn new(board: &Board) -> Self {
        let state: Vec<u8> = board
            .get_state()
            .into_iter()
            .flat_map(|row| row.into_iter())
            .collect();
        let goal_state = board.get_goal_state();
        State {
            state,
            goal_state,
            blank_pos: 0, // Will be set in initialization
            size: board.get_size(),
            path: Vec::new(),
        }
    }

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

        Some(State {
            state: new_state,
            goal_state: self.goal_state.clone(),
            blank_pos: new_pos,
            size: self.size,
            path: new_path,
        })
    }
}

pub struct DFSSolver {
    initial_board: Board,
}

impl DFSSolver {
    fn find_blank_pos(state: &[u8]) -> usize {
        state.iter().position(|&x| x == 0).unwrap_or(0) // Should never happen with valid input
    }
}

impl Solver for DFSSolver {
    fn new(initial: Board) -> Self {
        DFSSolver {
            initial_board: initial,
        }
    }

    fn new_with_goal(initial: Board) -> Self {
        DFSSolver {
            initial_board: initial,
        }
    }

    fn solve(&self, optimal_length: Option<usize>) -> Option<SolutionInfo> {
        let mut stack = Vec::new();
        let mut visited = HashSet::new();

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
            goal_state,
            blank_pos: Self::find_blank_pos(&state),
            size: self.initial_board.get_size(),
            path: Vec::new(),
        };

        stack.push(initial_state);
        visited.insert(
            self.initial_board
                .get_state()
                .into_iter()
                .flatten()
                .collect::<Vec<u8>>(),
        );

        let max_depth = 20; // Maximum search depth

        while let Some(current_state) = stack.pop() {
            if current_state.is_goal() {
                return Some(SolutionInfo::new(current_state.path, optimal_length));
            }

            if current_state.path.len() >= max_depth {
                continue; // Skip if path is too long
            }

            // Collect all possible next states
            let mut next_states = Vec::new();
            for direction in current_state.get_possible_moves() {
                if let Some(new_state) = current_state.make_move(direction) {
                    if !visited.contains(&new_state.state) {
                        visited.insert(new_state.state.clone());
                        next_states.push(new_state);
                    }
                }
            }

            // Sort by path length in descending order since we pop from the end
            next_states.sort_by(|a, b| b.path.len().cmp(&a.path.len()));
            stack.extend(next_states);
        }

        None
    }
}
