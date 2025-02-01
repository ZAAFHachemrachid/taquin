use colored::*;
use std::fs::File;
use std::io::{self, Error, Write};
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
    println!(
        "{}\n",
        "=== N-Puzzle Solver Visualization ===".blue().bold()
    );

    let methods = ["DFS", "BFS", "A*"];
    let separator = "║".bright_cyan();

    // Colorful header with borders
    println!(
        "{}",
        "╔═══════════════════╦═══════════════════╦═══════════════════╗".bright_cyan()
    );

    // Print method names
    print!("{}", separator);
    for (i, method) in methods.iter().enumerate() {
        print!("{:^19}", method.yellow());
        if i < methods.len() - 1 {
            print!("{}", separator);
        }
    }
    println!("{}", separator);

    // Top separator
    println!(
        "{} ",
        "╠═══════════════════╬═══════════════════╬═══════════════════╣".bright_cyan()
    );
    // Print puzzle state
    for i in 0..3 {
        print!("{}", separator);
        for (j, state) in states.iter().enumerate() {
            let board_str = if let Some(ref solution) = state.solution {
                if step < solution.moves.len() {
                    let mut board = state.board.clone();
                    for k in 0..=step {
                        board.make_move(solution.moves[k].clone()).unwrap();
                    }
                    let row = board.get_row(i);
                    let nums: Vec<String> = row
                        .iter()
                        .map(|&n| {
                            if n == 0 {
                                "_".red().to_string()
                            } else {
                                n.to_string().green().to_string()
                            }
                        })
                        .collect();
                    format!("    {}    {}    {}    ", nums[0], nums[1], nums[2])
                } else {
                    format!("{:^19}", "[Complete]".bright_blue())
                }
            } else {
                format!("{:^19}", "[No Solution]".bright_red())
            };
            print!("{}", board_str);
            if j < states.len() - 1 {
                print!("{}", separator);
            }
        }
        println!("{}", separator);
    }

    // Middle separator
    println!(
        "{} ",
        "╠═══════════════════╬═══════════════════╬═══════════════════╣".bright_cyan()
    );

    // Print current moves
    print!("{}", separator);
    for (j, state) in states.iter().enumerate() {
        let move_str = if let Some(ref solution) = state.solution {
            if step < solution.moves.len() {
                format!("Move: {:?}", solution.moves[step]).magenta()
            } else {
                format!("Total Steps: {}", solution.moves.len()).bright_blue()
            }
        } else {
            "No solution".bright_red()
        };
        print!("{:^19}", move_str);
        if j < states.len() - 1 {
            print!("{}", separator);
        }
    }
    println!("{}", separator);

    // Bottom separator
    println!(
        "{} ",
        "╠═══════════════════╬═══════════════════╬═══════════════════╣".bright_cyan()
    );
    // Print stats
    print!("{}", separator);
    for (j, state) in states.iter().enumerate() {
        let stats = if let Some(ref solution) = state.solution {
            format!("Time: {:?}", state.time_taken).yellow()
        } else {
            "Failed".bright_red()
        };
        print!("{:^19}", stats);
        if j < states.len() - 1 {
            print!("{}", separator);
        }
    }
    println!("{}", separator);

    // Final border
    println!(
        "{}",
        "╚═══════════════════╩═══════════════════╩═══════════════════╝".bright_cyan()
    );

    io::stdout().flush().unwrap();
}

pub fn write_results_to_file(states: &[&MethodState]) -> io::Result<()> {
    let mut file = File::create("results.md")?;
    let methods = ["DFS", "BFS", "A*"];

    // Write header with proper markdown table formatting
    writeln!(
        file,
        "| {:<18} | {:<18} | {:<18} |",
        methods[0], methods[1], methods[2]
    )?;
    writeln!(file, "|:{:-<18}:|:{:-<18}:|:{:-<18}:|", "", "", "")?;

    // Write initial state with better formatting
    writeln!(file, "\n### Initial State\n")?;
    writeln!(file, "|---|---|---|")?;
    for i in 0..3 {
        let mut row_str = String::new();
        for state in states.iter() {
            let row = state.board.get_row(i);
            let val0 = if row[0] == 0 {
                "_"
            } else {
                &row[0].to_string()
            };
            let val1 = if row[1] == 0 {
                "_"
            } else {
                &row[1].to_string()
            };
            let val2 = if row[2] == 0 {
                "_"
            } else {
                &row[2].to_string()
            };
            let cells = format!("| {} {} {} |", val0, val1, val2);
            row_str.push_str(&cells);
        }
        writeln!(file, "{}", row_str)?;
    }
    writeln!(file, "|---|---|---|")?;

    // Write solution steps with better formatting
    let max_steps = states
        .iter()
        .filter_map(|s| s.solution.as_ref())
        .map(|sol| sol.moves.len())
        .max()
        .unwrap_or(0);

    for step in 0..max_steps {
        writeln!(file, "\n### Step {}\n", step + 1)?;
        writeln!(file, "|---|---|---|")?;
        for i in 0..3 {
            let mut row_str = String::new();
            for state in states.iter() {
                if let Some(ref solution) = state.solution {
                    if step < solution.moves.len() {
                        let mut board = state.board.clone();
                        for k in 0..=step {
                            board.make_move(solution.moves[k].clone()).unwrap();
                        }
                        let row = board.get_row(i);
                        let val0 = if row[0] == 0 {
                            "_"
                        } else {
                            &row[0].to_string()
                        };
                        let val1 = if row[1] == 0 {
                            "_"
                        } else {
                            &row[1].to_string()
                        };
                        let val2 = if row[2] == 0 {
                            "_"
                        } else {
                            &row[2].to_string()
                        };
                        let cells = format!("| {} {} {} |", val0, val1, val2);
                        row_str.push_str(&cells);
                    } else {
                        row_str.push_str("| [Complete] |");
                    }
                } else {
                    row_str.push_str("| [No solution] |");
                }
            }
            writeln!(file, "{}", row_str)?;
        }
        writeln!(file, "|---|---|---|")?;

        // Write moves for this step
        let mut move_str = String::new();
        for state in states.iter() {
            let move_text = if let Some(ref solution) = state.solution {
                if step < solution.moves.len() {
                    format!("| Move: {:?} |", solution.moves[step])
                } else {
                    "| Complete |".to_string()
                }
            } else {
                "| No solution |".to_string()
            };
            move_str.push_str(&move_text);
        }
        writeln!(file, "{}", move_str)?;
    }

    // Write final stats with better formatting
    writeln!(file, "\n### Final Stats\n")?;
    writeln!(file, "| Metric | DFS | BFS | A* |")?;
    writeln!(file, "|:--|:--|:--|:--|")?;

    // Write steps
    let steps_str = format!(
        "| Steps | {} | {} | {} |",
        states[0]
            .solution
            .as_ref()
            .map_or("Failed".to_string(), |s| s.moves.len().to_string()),
        states[1]
            .solution
            .as_ref()
            .map_or("Failed".to_string(), |s| s.moves.len().to_string()),
        states[2]
            .solution
            .as_ref()
            .map_or("Failed".to_string(), |s| s.moves.len().to_string())
    );
    writeln!(file, "{}", steps_str)?;

    // Write time
    let time_str = format!(
        "| Time | {:?} | {:?} | {:?} |",
        states[0].time_taken, states[1].time_taken, states[2].time_taken
    );
    writeln!(file, "{}", time_str)?;

    Ok(())
}
