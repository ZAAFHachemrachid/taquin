# A* Algorithm for the Taquin Puzzle Solver

## Overview

The A* algorithm is a best-first search algorithm that uses both the actual cost
from the start node (g-cost) and a heuristic estimate to the goal (h-cost) to
find the optimal solution path. In this implementation, it's used to solve the
Taquin (sliding puzzle) problem.

## Implementation Details

### State Representation

The puzzle state is represented by the `State` struct which contains:

```rust
struct State {
    state: Vec<u8>,         // Current board configuration
    blank_pos: usize,       // Position of the blank tile
    size: usize,           // Size of the board (e.g., 3 for 3x3)
    path: Vec<Direction>,   // Path taken to reach this state
    cost: u32,             // g-cost: actual cost from start
    heuristic: u32,        // h-cost: estimated cost to goal
}
```

### Heuristic Function

The implementation uses the Manhattan distance heuristic, which is admissible
for the sliding puzzle problem. For each tile:

1. Calculate its current position (row, col)
2. Calculate its goal position (expected_row, expected_col)
3. Sum the absolute differences in rows and columns

```rust
fn manhattan_distance(state: &[u8], size: usize) -> u32 {
    let mut distance = 0;
    for pos in 0..state.len() {
        let value = state[pos];
        if value != 0 {  // Skip blank tile
            let current_row = pos / size;
            let current_col = pos % size;
            let expected_row = ((value - 1) / size as u8) as usize;
            let expected_col = ((value - 1) % size as u8) as usize;
            distance += (current_row.abs_diff(expected_row) 
                     + current_col.abs_diff(expected_col)) as u32;
        }
    }
    distance
}
```

### Priority Queue Ordering

States are ordered in the priority queue based on their total cost (f-cost =
g-cost + h-cost):

```rust
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for min-heap (lower cost = higher priority)
        other
            .total_cost()
            .cmp(&self.total_cost())
            .then_with(|| other.cost.cmp(&self.cost))
    }
}
```

## Algorithm Flow

1. **Initialization**:
   - Create initial state with the starting board configuration
   - Add initial state to priority queue (open set)
   - Initialize visited set to track explored states

2. **Main Loop**:
   ```rust
   while let Some(current_state) = open_set.pop() {
       // Check if goal state reached
       if current_state.is_goal() {
           return Some(SolutionInfo::new(current_state.path, None));
       }

       // Generate and evaluate successor states
       for direction in current_state.get_possible_moves() {
           if let Some(new_state) = current_state.make_move(direction) {
               if !visited.contains(&new_state.state) {
                   visited.insert(new_state.state.clone());
                   open_set.push(new_state);
               }
           }
       }
   }
   ```

3. **For Each State**:
   - Check if current state is goal state
   - If not, generate all possible moves (Up, Down, Left, Right)
   - For each valid move:
     - Create new state with updated position
     - Calculate new g-cost (path length) and h-cost (Manhattan distance)
     - Add to priority queue if not visited

4. **Solution**:
   - When goal state is found, return the path of moves taken
   - If queue empties without finding solution, return None

## Key Features

1. **Optimality**: A* guarantees the shortest solution path due to the
   admissible Manhattan distance heuristic.

2. **Efficiency**:
   - Uses binary heap for efficient priority queue operations
   - Maintains visited set to avoid exploring same states
   - Uses Manhattan distance heuristic to guide search toward goal

3. **Memory Management**:
   - States are cloned only when necessary
   - Visited states tracked to prevent cycles
   - Path stored incrementally in each state
