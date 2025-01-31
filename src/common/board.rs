use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Board {
    state: Vec<u8>,
    goal_state: Vec<u8>,
    blank_pos: usize,
    size: usize,
}

impl Board {
    pub fn new(initial_state: Vec<Vec<i32>>) -> Self {
        let size = initial_state.len();
        let mut state = Vec::with_capacity(size * size);
        let mut blank_pos = 0;

        // Convert 2D state to 1D and find blank position
        for i in 0..size {
            for j in 0..size {
                let value = initial_state[i][j] as u8;
                if value == 0 {
                    blank_pos = i * size + j;
                }
                state.push(value);
            }
        }

        // Default goal state: 1,2,3,4,5,6,7,8,0
        let mut goal_state = (1..=(size * size) as u8).collect::<Vec<u8>>();
        goal_state[(size * size) - 1] = 0;

        Board {
            state,
            goal_state,
            blank_pos,
            size,
        }
    }

    pub fn new_with_goal(initial_state: Vec<Vec<i32>>, goal_state: Vec<Vec<i32>>) -> Self {
        let size = initial_state.len();
        let mut state = Vec::with_capacity(size * size);
        let mut goal = Vec::with_capacity(size * size);
        let mut blank_pos = 0;

        // Convert 2D states to 1D
        for i in 0..size {
            for j in 0..size {
                let value = initial_state[i][j] as u8;
                if value == 0 {
                    blank_pos = i * size + j;
                }
                state.push(value);
                goal.push(goal_state[i][j] as u8);
            }
        }

        Board {
            state,
            goal_state: goal,
            blank_pos,
            size,
        }
    }

    pub fn get_state(&self) -> Vec<Vec<u8>> {
        let mut result = Vec::with_capacity(self.size);
        for i in 0..self.size {
            let mut row = Vec::with_capacity(self.size);
            for j in 0..self.size {
                row.push(self.state[i * self.size + j]);
            }
            result.push(row);
        }
        result
    }

    pub fn get_goal_state(&self) -> Vec<u8> {
        self.goal_state.clone()
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn is_goal(&self) -> bool {
        self.state == self.goal_state
    }

    pub fn get_possible_moves(&self) -> Vec<Direction> {
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

    pub fn make_move(&mut self, dir: Direction) -> Result<(), String> {
        let row = self.blank_pos / self.size;
        let col = self.blank_pos % self.size;

        let new_pos = match dir {
            Direction::Up if row > 0 => self.blank_pos - self.size,
            Direction::Down if row < self.size - 1 => self.blank_pos + self.size,
            Direction::Left if col > 0 => self.blank_pos - 1,
            Direction::Right if col < self.size - 1 => self.blank_pos + 1,
            _ => return Err("Invalid move".to_string()),
        };

        self.state.swap(self.blank_pos, new_pos);
        self.blank_pos = new_pos;

        Ok(())
    }

    pub fn manhattan_distance(&self) -> u32 {
        let mut distance = 0;
        for pos in 0..self.state.len() {
            let value = self.state[pos];
            if value != 0 {
                // Find position of this value in goal state
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
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Top border
        write!(f, "┌")?;
        for i in 0..self.size {
            write!(f, "───")?;
            if i < self.size - 1 {
                write!(f, "┬")?;
            }
        }
        writeln!(f, "┐")?;

        // Board content
        for row in 0..self.size {
            write!(f, "│")?;
            for col in 0..self.size {
                let num = self.state[row * self.size + col];
                if num == 0 {
                    write!(f, " _ │")?;
                } else {
                    write!(f, " {} │", num)?;
                }
            }
            writeln!(f)?;

            // Middle borders
            if row < self.size - 1 {
                write!(f, "├")?;
                for i in 0..self.size {
                    write!(f, "───")?;
                    if i < self.size - 1 {
                        write!(f, "┼")?;
                    }
                }
                writeln!(f, "┤")?;
            }
        }

        // Bottom border
        write!(f, "└")?;
        for i in 0..self.size {
            write!(f, "───")?;
            if i < self.size - 1 {
                write!(f, "┴")?;
            }
        }
        writeln!(f, "┘")
    }
}
