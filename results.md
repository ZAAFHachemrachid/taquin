| DFS                | BFS                | A*                 |
|:------------------:|:------------------:|:------------------:|

### Initial State

|---|---|---|
| 1 2 3 || 1 2 3 || 1 2 3 |
| 4 _ 6 || 4 _ 6 || 4 _ 6 |
| 7 5 8 || 7 5 8 || 7 5 8 |
|---|---|---|

### Step 1

|---|---|---|
| 1 2 3 || 1 2 3 || 1 2 3 |
| 4 6 _ || 4 5 6 || 4 5 6 |
| 7 5 8 || 7 _ 8 || 7 _ 8 |
|---|---|---|
| Move: Right || Move: Down || Move: Down |

### Step 2

|---|---|---|
| 1 2 3 || 1 2 3 || 1 2 3 |
| 4 6 8 || 4 5 6 || 4 5 6 |
| 7 5 _ || 7 8 _ || 7 8 _ |
|---|---|---|
| Move: Down || Move: Right || Move: Right |

### Step 3

|---|---|---|
| 1 2 3 || [Complete] || [Complete] |
| 4 6 8 || [Complete] || [Complete] |
| 7 _ 5 || [Complete] || [Complete] |
|---|---|---|
| Move: Left || Complete || Complete |

### Step 4

|---|---|---|
| 1 2 3 || [Complete] || [Complete] |
| 4 _ 8 || [Complete] || [Complete] |
| 7 6 5 || [Complete] || [Complete] |
|---|---|---|
| Move: Up || Complete || Complete |

### Step 5

|---|---|---|
| 1 2 3 || [Complete] || [Complete] |
| 4 8 _ || [Complete] || [Complete] |
| 7 6 5 || [Complete] || [Complete] |
|---|---|---|
| Move: Right || Complete || Complete |

### Step 6

|---|---|---|
| 1 2 3 || [Complete] || [Complete] |
| 4 8 5 || [Complete] || [Complete] |
| 7 6 _ || [Complete] || [Complete] |
|---|---|---|
| Move: Down || Complete || Complete |

### Step 7

|---|---|---|
| 1 2 3 || [Complete] || [Complete] |
| 4 8 5 || [Complete] || [Complete] |
| 7 _ 6 || [Complete] || [Complete] |
|---|---|---|
| Move: Left || Complete || Complete |

### Step 8

|---|---|---|
| 1 2 3 || [Complete] || [Complete] |
| 4 _ 5 || [Complete] || [Complete] |
| 7 8 6 || [Complete] || [Complete] |
|---|---|---|
| Move: Up || Complete || Complete |

### Step 9

|---|---|---|
| 1 2 3 || [Complete] || [Complete] |
| 4 5 _ || [Complete] || [Complete] |
| 7 8 6 || [Complete] || [Complete] |
|---|---|---|
| Move: Right || Complete || Complete |

### Step 10

|---|---|---|
| 1 2 3 || [Complete] || [Complete] |
| 4 5 6 || [Complete] || [Complete] |
| 7 8 _ || [Complete] || [Complete] |
|---|---|---|
| Move: Down || Complete || Complete |

### Final Stats

| Metric | DFS | BFS | A* |
|:--|:--|:--|:--|
| Steps | 10 | 2 | 2 |
| Time | 21.2964ms | 3.8353301s | 304.7µs |
