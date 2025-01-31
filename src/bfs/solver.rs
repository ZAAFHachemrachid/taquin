use crate::common::{Board, Direction};
use crate::{SolutionInfo, Solver};
use colored::*;
use std::collections::{HashSet, VecDeque};

// State struct for BFS
#[derive(Clone)]
struct State {
    state: Vec<u8>,
    goal_state: Vec<u8>,
    blank_pos: usize,
    size: usize,
    path: Vec<Direction>,
}

impl State {
    fn manhattan_distance(&self) -> u32 {
        let mut distance = 0;
        for pos in 0..self.state.len() {
            let value = self.state[pos];
            if value != 0 {
                if let Some(goal_pos) = self.goal_state.iter().position(|&x| x == value) {
                    let current_row = pos / self.size;
                    let current_col = pos % self.size;
                    let goal_row = goal_pos / self.size;
                    let goal_col = goal_pos % self.size;
                    distance +=
                        (current_row.abs_diff(goal_row) + current_col.abs_diff(goal_col)) as u32;
                }
            }
        }
        distance
    }

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

pub struct BFSSolver {
    initial_board: Board,
}

impl BFSSolver {
    fn find_blank_pos(state: &[u8]) -> usize {
        state.iter().position(|&x| x == 0).unwrap_or(0)
    }

    fn debug_print(
        current: &State,
        moves: &[(Direction, State, &str, String)],
        visited: &HashSet<Vec<u8>>,
        level: usize,
    ) {
        println!("\nExploring Level {}", level);
        // Print board state
        println!("{}", "┌───┬───┬───┐".cyan());
        for i in 0..current.size {
            print!("│");
            for j in 0..current.size {
                let value = current.state[i * current.size + j];
                if value == 0 {
                    print!(" _ │");
                } else {
                    print!(" {} │", value);
                }
            }
            println!();
            if i < current.size - 1 {
                println!("{}", "├───┼───┼───┤".cyan());
            }
        }
        println!("{}", "└───┴───┴───┘".cyan());

        println!("\nPossible moves at this level:");
        for (dir, _new_state, quality, reason) in moves {
            let move_str = match dir {
                Direction::Up => "Up   ",
                Direction::Down => "Down ",
                Direction::Left => "Left ",
                Direction::Right => "Right",
            };

            let colored_move = match *quality {
                "BEST" => format!("{} [{}]", move_str, quality).green(),
                "MID" => format!("{} [{}]", move_str, quality).yellow(),
                _ => format!("{} [{}]", move_str, quality).red(),
            };

            println!("- {} | {}", colored_move, reason);
        }
        println!("Total states visited: {}", visited.len());
    }

    fn evaluate_move(
        current: &State,
        next: &State,
        visited: &HashSet<Vec<u8>>,
    ) -> (&'static str, String) {
        let current_dist = current.manhattan_distance();
        let new_dist = next.manhattan_distance();

        if visited.contains(&next.state) {
            ("BAD", "Already visited".to_string())
        } else if new_dist < current_dist {
            (
                "BEST",
                format!("Manhattan distance: {} -> {}", current_dist, new_dist),
            )
        } else if new_dist == current_dist {
            (
                "MID",
                format!("Manhattan distance unchanged: {}", current_dist),
            )
        } else {
            (
                "BAD",
                format!("Manhattan distance: {} -> {}", current_dist, new_dist),
            )
        }
    }
}

impl Solver for BFSSolver {
    fn new(initial: Board) -> Self {
        BFSSolver {
            initial_board: initial,
        }
    }

    fn new_with_goal(initial: Board) -> Self {
        BFSSolver {
            initial_board: initial,
        }
    }

    fn solve(&self, optimal_length: Option<usize>) -> Option<SolutionInfo> {
        let mut queue = VecDeque::new();
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

        queue.push_back((initial_state, 0)); // (state, level)
        visited.insert(
            self.initial_board
                .get_state()
                .into_iter()
                .flatten()
                .collect::<Vec<u8>>(),
        );

        let max_depth = 30; // Increased max depth for BFS
        let mut nodes_visited = 0;
        let mut current_level = 0;

        while let Some((current_state, level)) = queue.pop_front() {
            nodes_visited += 1;

            // Show level transition
            if level > current_level {
                println!("\n{}", "Moving to next level...".blue());
                current_level = level;
            }

            // Evaluate and print all possible moves
            let mut possible_moves = Vec::new();
            for direction in current_state.get_possible_moves() {
                if let Some(new_state) = current_state.make_move(direction.clone()) {
                    let (quality, reason) =
                        Self::evaluate_move(&current_state, &new_state, &visited);
                    possible_moves.push((direction, new_state, quality, reason));
                }
            }

            let display_moves: Vec<(Direction, State, &str, String)> = possible_moves
                .iter()
                .map(|(d, s, q, r)| (d.clone(), s.clone(), *q, r.clone()))
                .collect();

            Self::debug_print(&current_state, &display_moves, &visited, level);

            if current_state.is_goal() {
                println!("\n{}", "🎉 GOAL STATE REACHED! 🎉".green());
                println!("BFS: Visited {} nodes", nodes_visited);
                return Some(SolutionInfo::new(current_state.path, optimal_length));
            }

            if level >= max_depth {
                println!("\n{}", "Max depth reached at this branch...".yellow());
                continue;
            }

            // Queue next states for BFS exploration
            for (_direction, new_state, quality, _) in possible_moves {
                if quality != "BAD" && !visited.contains(&new_state.state) {
                    visited.insert(new_state.state.clone());
                    queue.push_back((new_state, level + 1));
                }
            }
        }

        None
    }
}
