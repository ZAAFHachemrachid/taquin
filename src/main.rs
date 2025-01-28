use colored::*;
use ctrlc;
use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering as AtomicOrdering};
use std::sync::Arc;
use taquin::{
    AStarSolver, BFSSolver, Board, DFSSolver, Direction, MoveQuality, SolutionInfo, Solver,
};

fn clear_screen() {
    // Use Windows-compatible clear command
    if cfg!(windows) {
        std::process::Command::new("cmd")
            .args(["/c", "cls"])
            .status()
            .unwrap();
    } else {
        // ANSI escape codes for Unix-like systems
        print!("\x1B[2J\x1B[1;1H");
        io::stdout().flush().unwrap();
    }
}

fn print_solution(board: &Board, solution: Option<SolutionInfo>) {
    println!("\nInitial state:");
    println!("{}", board);

    match solution {
        Some(solution_info) => {
            let color = match solution_info.quality {
                MoveQuality::Best => "green",
                MoveQuality::Medium => "yellow",
                MoveQuality::Poor => "red",
            };

            println!(
                "\nSolution found! {} moves required.",
                solution_info.moves.len().to_string().color(color)
            );
            println!(
                "\nSolution quality: {}",
                match solution_info.quality {
                    MoveQuality::Best => "OPTIMAL (Best)".green(),
                    MoveQuality::Medium => "ACCEPTABLE (Medium)".yellow(),
                    MoveQuality::Poor => "SUB-OPTIMAL (Poor)".red(),
                }
            );

            println!("\nSolution path:");
            let mut current_board = board.clone();
            for (i, direction) in solution_info.moves.iter().enumerate() {
                println!("\nStep {}:", i + 1);
                current_board.make_move(direction.clone()).unwrap();
                println!("{}", current_board);
                let move_str = match direction {
                    Direction::Up => "Move: Up",
                    Direction::Down => "Move: Down",
                    Direction::Left => "Move: Left",
                    Direction::Right => "Move: Right",
                };
                println!("{}", move_str.color(color));
                io::stdout().flush().unwrap();
            }
        }
        None => println!("{}", "No solution found!".red()),
    }
}

fn print_menu(board: &Board) {
    println!(
        "{}",
        "\n╔═════════════════════════════════════╗".cyan().bold()
    );
    println!(
        "{}",
        "║        Taquin Puzzle Solver         ║".cyan().bold()
    );
    println!(
        "{}",
        "╚═════════════════════════════════════╝".cyan().bold()
    );
    println!("\nInitial puzzle state:");
    println!("{}", board);
    println!("\nChoose solving method:");
    println!("{}", "─────────────────────────────────────".cyan());
    println!("1) {}", "Breadth-First Search (BFS)".green());
    println!("2) {}", "Depth-First Search (DFS)".yellow());
    println!("3) {}", "A* Search".blue());
    println!("4) {}", "Exit".red());
    println!("{}", "─────────────────────────────────────".cyan());
    print!("Enter your choice (1-4): ");
}

fn solve_with_status<T: Solver>(solver: T, board: &Board, method: &str) {
    println!("\nSolving with {}...", method.bold());
    println!("{}", "This may take a moment, please wait...".yellow());
    println!("{}", "(Press Ctrl+C to cancel)".yellow());
    io::stdout().flush().unwrap();

    print_solution(board, solver.solve(None));
}

fn main() -> io::Result<()> {
    // Set up Ctrl+C handler
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, AtomicOrdering::SeqCst);
        println!("\n{}", "Operation cancelled by user.".red());
    })
    .expect("Error setting Ctrl-C handler");

    // Create a complex puzzle state that requires many moves to solve
    let initial_board = Board::new(vec![vec![2, 3, 6], vec![1, 5, 0], vec![4, 7, 8]]);

    while running.load(AtomicOrdering::SeqCst) {
        clear_screen();
        print_menu(&initial_board);
        io::stdout().flush()?;

        let mut choice = String::new();
        match io::stdin().read_line(&mut choice) {
            Ok(_) => match choice.trim() {
                "1" => {
                    let solver = BFSSolver::new(initial_board.clone());
                    solve_with_status(solver, &initial_board, "BFS");
                }
                "2" => {
                    let solver = DFSSolver::new(initial_board.clone());
                    solve_with_status(solver, &initial_board, "DFS");
                }
                "3" => {
                    let solver = AStarSolver::new(initial_board.clone());
                    solve_with_status(solver, &initial_board, "A*");
                }
                "4" => {
                    clear_screen();
                    println!("\nThank you for using Taquin Puzzle Solver!");
                    println!("Goodbye!\n");
                    break;
                }
                _ => println!(
                    "{}",
                    "\nInvalid choice! Please enter a number between 1 and 4.".red()
                ),
            },
            Err(error) => println!("Error reading input: {}", error),
        }

        if running.load(AtomicOrdering::SeqCst) {
            println!("\nPress Enter to continue...");
            io::stdout().flush()?;
            let mut temp = String::new();
            io::stdin().read_line(&mut temp)?;
        }
    }

    Ok(())
}
