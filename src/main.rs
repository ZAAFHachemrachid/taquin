use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering as AtomicOrdering};
use std::sync::Arc;
use std::thread;
use taquin::common::{ColoredText, Config};
use taquin::{AStarSolver, BFSSolver, Board, DFSSolver, Direction, SolutionInfo, Solver};

fn clear_screen() {
    // Use Windows-compatible clear command
    if cfg!(windows) {
        std::process::Command::new("cmd")
            .args(["/c", "cls"])
            .status()
            .unwrap();
    } else {
        // ANSI escape codes for Unix-like systems
        print!("[2J[1;1H");
        io::stdout().flush().unwrap();
    }
}

fn get_speed_setting() -> Config {
    println!("\n{}", ColoredText::cyan("Choose visualization speed:"));
    println!("1) Fast (0.2 seconds)");
    println!("2) Medium (0.5 seconds)");
    println!("3) Slow (1.0 seconds)");
    print!("\nEnter your choice (1-3): ");
    io::stdout().flush().unwrap();

    loop {
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let delay = match choice.trim() {
            "1" => std::time::Duration::from_millis(200),
            "2" => std::time::Duration::from_millis(500),
            "3" => std::time::Duration::from_secs(1),
            _ => {
                print!(
                    "{}\nEnter your choice (1-3): ",
                    ColoredText::red("Invalid choice!")
                );
                io::stdout().flush().unwrap();
                continue;
            }
        };
        return Config {
            iteration_delay: delay,
        };
    }
}

fn print_solution(board: &Board, solution: Option<SolutionInfo>) {
    println!("\nInitial state:");
    println!("{}", board);

    match solution {
        Some(solution_info) => {
            println!("\n{}", solution_info.display_solution());
            println!("\nSolution path:");
            let mut current_board = board.clone();

            // Get visualization speed
            let config = get_speed_setting();

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
                println!("{}", move_str);
                io::stdout().flush().unwrap();
                thread::sleep(config.iteration_delay);
            }
        }
        None => println!("{}", ColoredText::red("No solution found!")),
    }
}

fn print_menu(initial_board: &Board, goal_board: &Board) {
    println!(
        "{}",
        ColoredText::bold_cyan("\n╔═════════════════════════════════════╗")
    );
    println!(
        "{}",
        ColoredText::bold_cyan("║        Taquin Puzzle Solver         ║")
    );
    println!(
        "{}",
        ColoredText::bold_cyan("╚═════════════════════════════════════╝")
    );
    println!("\nInitial puzzle state:");
    println!("{}", initial_board);
    println!("\nGoal state (solving to reach this state):");
    println!("{}", goal_board);
    println!("\nChoose solving method:");
    println!(
        "{}",
        ColoredText::cyan("─────────────────────────────────────")
    );
    println!("1) {}", ColoredText::green("Breadth-First Search (BFS)"));
    println!("2) {}", ColoredText::yellow("Depth-First Search (DFS)"));
    println!("3) {}", ColoredText::blue("A* Search"));
    println!("4) {}", ColoredText::red("Exit"));
    println!(
        "{}",
        ColoredText::cyan("─────────────────────────────────────")
    );
    print!("Enter your choice (1-4): ");
}

fn solve_with_status<T: Solver>(solver: T, board: &Board, method: &str) {
    println!("\nSolving with {}...", method);
    println!(
        "{}",
        ColoredText::yellow("This may take a moment, please wait...")
    );
    println!("{}", ColoredText::yellow("(Press Ctrl+C to cancel)"));
    io::stdout().flush().unwrap();

    print_solution(board, solver.solve(None));
}

fn main() -> io::Result<()> {
    // Set up Ctrl+C handler
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, AtomicOrdering::SeqCst);
        println!("\n{}", ColoredText::red("Operation cancelled by user."));
    })
    .expect("Error setting Ctrl-C handler");

    // Get the initial state from the user
    println!(
        "\n{}",
        ColoredText::cyan("Enter the initial state (row by row):")
    );
    println!("Use numbers 0-8 separated by spaces, where 0 represents the empty tile");
    println!("Example: 2 8 3");
    println!("         1 6 4");
    println!("         7 0 5\n");

    let mut initial_state = Vec::new();
    for i in 0..3 {
        print!("Enter row {}: ", i + 1);
        io::stdout().flush()?;
        let mut row = String::new();
        io::stdin().read_line(&mut row)?;
        let numbers: Vec<i32> = row
            .split_whitespace()
            .filter_map(|num| num.parse().ok())
            .collect();
        if numbers.len() != 3 {
            println!(
                "{}",
                ColoredText::red("Each row must contain exactly 3 numbers!")
            );
            return Ok(());
        }
        initial_state.push(numbers);
    }

    // Get the goal state from the user
    println!(
        "\n{}",
        ColoredText::cyan("Enter the goal state (row by row):")
    );
    println!("Use numbers 0-8 separated by spaces, where 0 represents the empty tile\n");

    let mut goal_state = Vec::new();
    for i in 0..3 {
        print!("Enter row {}: ", i + 1);
        io::stdout().flush()?;
        let mut row = String::new();
        io::stdin().read_line(&mut row)?;
        let numbers: Vec<i32> = row
            .split_whitespace()
            .filter_map(|num| num.parse().ok())
            .collect();
        if numbers.len() != 3 {
            println!(
                "{}",
                ColoredText::red("Each row must contain exactly 3 numbers!")
            );
            return Ok(());
        }
        goal_state.push(numbers);
    }

    // Validate the input states
    let valid_numbers: Vec<i32> = (0..9).collect();
    let mut initial_numbers: Vec<i32> = initial_state.iter().flatten().cloned().collect();
    initial_numbers.sort();
    let mut goal_numbers: Vec<i32> = goal_state.iter().flatten().cloned().collect();
    goal_numbers.sort();

    if initial_numbers != valid_numbers || goal_numbers != valid_numbers {
        println!(
            "{}",
            ColoredText::red("Invalid input! Use numbers 0-8 exactly once in each state.")
        );
        return Ok(());
    }

    let initial_board = Board::new_with_goal(initial_state, goal_state.clone());
    let goal_board = Board::new_with_goal(goal_state.clone(), goal_state);

    while running.load(AtomicOrdering::SeqCst) {
        clear_screen();
        print_menu(&initial_board, &goal_board);
        io::stdout().flush()?;

        let mut choice = String::new();
        match io::stdin().read_line(&mut choice) {
            Ok(_) => match choice.trim() {
                "1" => {
                    let solver = BFSSolver::new_with_goal(initial_board.clone());
                    solve_with_status(solver, &initial_board, "BFS");
                }
                "2" => {
                    let solver = DFSSolver::new_with_goal(initial_board.clone());
                    solve_with_status(solver, &initial_board, "DFS");
                }
                "3" => {
                    let solver = AStarSolver::new_with_goal(initial_board.clone());
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
                    ColoredText::red("\nInvalid choice! Please enter a number between 1 and 4.")
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
