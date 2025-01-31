use serde_json::Value;
use std::fs;
use std::io::{self, Write};
use std::thread;
use std::time::{Duration, Instant};

use taquin::common::{print_side_by_side, write_results_to_file, Config, MethodState};
use taquin::{AStarSolver, BFSSolver, Board, DFSSolver, Solver};

fn load_board_from_file(path: &str) -> Board {
    let content = fs::read_to_string(path).expect("Failed to read config file");
    let json: Value = serde_json::from_str(&content).expect("Failed to parse JSON");
    let board_state: Vec<Vec<i32>> = json
        .get("board")
        .expect("Missing board field")
        .as_array()
        .expect("Invalid board format")
        .iter()
        .map(|row| {
            row.as_array()
                .expect("Invalid row format")
                .iter()
                .map(|x| x.as_i64().expect("Invalid number") as i32)
                .collect()
        })
        .collect();

    Board::new(board_state)
}

fn solve_puzzle(initial_board: Board, _goal_board: Board) -> io::Result<()> {
    let config = Config {
        iteration_delay: Duration::from_millis(200), // Fast speed
    };

    // Initialize solvers
    let dfs_solver = DFSSolver::new_with_goal(initial_board.clone());
    let bfs_solver = BFSSolver::new_with_goal(initial_board.clone());
    let astar_solver = AStarSolver::new_with_goal(initial_board.clone());

    // Solve with all methods and measure time
    let start = Instant::now();
    let dfs_solution = dfs_solver.solve(None);
    let dfs_time = start.elapsed();

    let start = Instant::now();
    let bfs_solution = bfs_solver.solve(None);
    let bfs_time = start.elapsed();

    let start = Instant::now();
    let astar_solution = astar_solver.solve(None);
    let astar_time = start.elapsed();

    // Create method states
    let method_states = vec![
        MethodState {
            board: initial_board.clone(),
            current_step: 0,
            solution: dfs_solution,
            time_taken: dfs_time,
        },
        MethodState {
            board: initial_board.clone(),
            current_step: 0,
            solution: bfs_solution,
            time_taken: bfs_time,
        },
        MethodState {
            board: initial_board,
            current_step: 0,
            solution: astar_solution,
            time_taken: astar_time,
        },
    ];

    // Find maximum number of steps
    let max_steps = method_states
        .iter()
        .filter_map(|state| state.solution.as_ref())
        .map(|sol| sol.moves.len())
        .max()
        .unwrap_or(0);

    // Display side by side visualization
    for step in 0..max_steps {
        print_side_by_side(&method_states.iter().collect::<Vec<_>>(), step);
        thread::sleep(config.iteration_delay);
    }

    // Write final results to file
    write_results_to_file(&method_states.iter().collect::<Vec<_>>())?;

    println!("\nResults have been written to results.md");
    Ok(())
}

fn main() -> io::Result<()> {
    // Load initial and goal states from config files
    let initial_board = load_board_from_file("src/configs/initial_state.json");
    let goal_board = load_board_from_file("src/configs/final_state.json");

    // Run solvers with side-by-side visualization
    solve_puzzle(initial_board, goal_board)
}
