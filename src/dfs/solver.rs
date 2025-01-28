use crate::common::{Board, Direction};
use crate::{SolutionInfo, Solver};
use std::collections::HashSet;

// State struct for DFS
#[derive(Clone)]
struct State {
    state: Vec<u8>,
    blank_pos: usize,
    size: usize,
    path: Vec<Direction>,
}

impl State {
    fn new(board: &Board) -> Self {
        let state = board
            .get_state()
            .into_iter()
            .flat_map(|row| row.into_iter())
            .collect();
        State {
            state,
            blank_pos: 0, // Will be set in initialization
            size: board.get_size(),
            path: Vec::new(),
        }
    }

    fn is_goal(&self) -> bool {
        let mut expected = 1u8;
        for i in 0..self.state.len() {
            if i == self.state.len() - 1 {
                if self.state[i] != 0 {
                    return false;
                }
            } else if self.state[i] != expected {
                return false;
            }
            expected += 1;
        }
        true
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
    pub fn new(initial: Board) -> Self {
        DFSSolver {
            initial_board: initial,
        }
    }

    fn find_blank_pos(state: &[u8]) -> usize {
        state.iter().position(|&x| x == 0).unwrap_or(0) // Should never happen with valid input
    }
}

impl Solver for DFSSolver {
    fn solve(&self, optimal_length: Option<usize>) -> Option<SolutionInfo> {
        let mut stack = Vec::new();
        let mut visited = HashSet::new();

        // Initialize start state
        let mut initial_state = State::new(&self.initial_board);
        initial_state.blank_pos = Self::find_blank_pos(&initial_state.state);
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
