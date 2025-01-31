use std::fs::File;
use std::io::{self, Write};
use std::time::Duration;

use crate::{Board, SolutionInfo};

pub struct MethodState {
    pub board: Board,
    pub current_step: usize,
    pub solution: Option<SolutionInfo>,
    pub time_taken: Duration,
}

pub fn clear_screen() {
    print!("[2J[1;1H");
    io::stdout().flush().unwrap();
}

pub fn print_side_by_side(states: &[&MethodState], step: usize) {
    clear_screen();

    let methods = ["DFS", "BFS", "A*"];
    let separator = "  |  ";

    // Print headers
    for (i, method) in methods.iter().enumerate() {
        print!("{:^20}", method);
        if i < methods.len() - 1 {
            print!("{}", separator);
        }
    }
    println!("\n{}", "─".repeat(70));

    // Print current state for each method
    for i in 0..3 {
        for (j, state) in states.iter().enumerate() {
            let board_str = if let Some(ref solution) = state.solution {
                if step < solution.moves.len() {
                    let mut board = state.board.clone();
                    for k in 0..=step {
                        board.make_move(solution.moves[k].clone()).unwrap();
                    }
                    let row = board.get_row(i);
                    // Format row with empty tile as '_'
                    format!(
                        "{:>2} {:>2} {:>2}",
                        if row[0] == 0 {
                            "_".to_string()
                        } else {
                            row[0].to_string()
                        },
                        if row[1] == 0 {
                            "_".to_string()
                        } else {
                            row[1].to_string()
                        },
                        if row[2] == 0 {
                            "_".to_string()
                        } else {
                            row[2].to_string()
                        }
                    )
                } else {
                    "  [Complete]  ".to_string()
                }
            } else {
                "  [No solution]  ".to_string()
            };

            print!("{:^20}", board_str);
            if j < states.len() - 1 {
                print!("{}", separator);
            }
        }
        println!();
    }

    // Print current move for each method
    for (j, state) in states.iter().enumerate() {
        let move_str = if let Some(ref solution) = state.solution {
            if step < solution.moves.len() {
                format!("Move: {:?}", solution.moves[step])
            } else {
                format!("Steps: {}", solution.moves.len())
            }
        } else {
            "No solution".to_string()
        };

        print!("{:^20}", move_str);
        if j < states.len() - 1 {
            print!("{}", separator);
        }
    }
    println!();

    // Print stats
    println!("\nStats:");
    for (j, state) in states.iter().enumerate() {
        let stats = if let Some(ref solution) = state.solution {
            format!("Time: {:?}", state.time_taken)
        } else {
            "Failed".to_string()
        };

        print!("{:^20}", stats);
        if j < states.len() - 1 {
            print!("{}", separator);
        }
    }
    println!();

    io::stdout().flush().unwrap();
}

pub fn write_results_to_file(states: &[&MethodState]) -> io::Result<()> {
    let mut file = File::create("results.txt")?;
    let methods = ["DFS", "BFS", "A*"];

    // Write header
    for (i, method) in methods.iter().enumerate() {
        write!(file, "{:^20}", method)?;
        if i < methods.len() - 1 {
            write!(file, "  |  ")?;
        }
    }
    writeln!(file)?;
    writeln!(file, "{}", "─".repeat(70))?;

    // Write initial state
    writeln!(file, "\nInitial State:")?;
    for i in 0..3 {
        for (j, state) in states.iter().enumerate() {
            let row = state.board.get_row(i);
            let row_str = format!(
                "{:>2} {:>2} {:>2}",
                if row[0] == 0 {
                    "_".to_string()
                } else {
                    row[0].to_string()
                },
                if row[1] == 0 {
                    "_".to_string()
                } else {
                    row[1].to_string()
                },
                if row[2] == 0 {
                    "_".to_string()
                } else {
                    row[2].to_string()
                }
            );
            write!(file, "{:^20}", row_str)?;
            if j < states.len() - 1 {
                write!(file, "  |  ")?;
            }
        }
        writeln!(file)?;
    }

    // Write solution steps
    let max_steps = states
        .iter()
        .filter_map(|s| s.solution.as_ref())
        .map(|sol| sol.moves.len())
        .max()
        .unwrap_or(0);

    for step in 0..max_steps {
        writeln!(file, "\nStep {}:", step + 1)?;
        for i in 0..3 {
            for (j, state) in states.iter().enumerate() {
                let board_str = if let Some(ref solution) = state.solution {
                    if step < solution.moves.len() {
                        let mut board = state.board.clone();
                        for k in 0..=step {
                            board.make_move(solution.moves[k].clone()).unwrap();
                        }
                        let row = board.get_row(i);
                        format!(
                            "{:>2} {:>2} {:>2}",
                            if row[0] == 0 {
                                "_".to_string()
                            } else {
                                row[0].to_string()
                            },
                            if row[1] == 0 {
                                "_".to_string()
                            } else {
                                row[1].to_string()
                            },
                            if row[2] == 0 {
                                "_".to_string()
                            } else {
                                row[2].to_string()
                            }
                        )
                    } else {
                        "[Complete]".to_string()
                    }
                } else {
                    "[No solution]".to_string()
                };
                write!(file, "{:^20}", board_str)?;
                if j < states.len() - 1 {
                    write!(file, "  |  ")?;
                }
            }
            writeln!(file)?;
        }

        // Write moves for this step
        for (j, state) in states.iter().enumerate() {
            let move_str = if let Some(ref solution) = state.solution {
                if step < solution.moves.len() {
                    format!("Move: {:?}", solution.moves[step])
                } else {
                    "Complete".to_string()
                }
            } else {
                "No solution".to_string()
            };
            write!(file, "{:^20}", move_str)?;
            if j < states.len() - 1 {
                write!(file, "  |  ")?;
            }
        }
        writeln!(file)?;
    }

    // Write final stats
    writeln!(file, "\nFinal Stats:")?;
    for (j, state) in states.iter().enumerate() {
        let stats = if let Some(ref solution) = state.solution {
            format!(
                "Steps: {}\nTime: {:?}",
                solution.moves.len(),
                state.time_taken
            )
        } else {
            "Failed".to_string()
        };
        write!(file, "{:^20}", stats)?;
        if j < states.len() - 1 {
            write!(file, "  |  ")?;
        }
    }
    writeln!(file)?;

    Ok(())
}
