# The Labyrinth

The Labyrinth is a Rust workspace consisting of three independent applications that together create a complete maze generation, solving, and machine learning framework.

- **Daedalus** generates perfect mazes.
- **Theseus** solves mazes using both classical search algorithms and a trainable neural network.
- **Hermes_01** provides Discord integration for remote maze generation, training control, monitoring, and progress reporting.
The Labyrinth is a Rust workspace consisting of three independent applications that together create a complete maze generation, solving, and machine learning framework.

- **Daedalus** generates perfect mazes.
- **Theseus** solves mazes using both classical search algorithms and a trainable neural network.
- **Hermes_01** provides Discord integration for remote maze generation, training control, monitoring, and progress reporting.

---

# Project Goals

The long-term objective of The Labyrinth is to create a machine learning solver capable of solving perfect mazes as efficiently as, or better than, traditional uninformed search algorithms.

Rather than hard-coding maze-solving behavior, Theseus learns by observing the best-performing classical algorithm for each generated maze. Over time the neural network is expected to require fewer explored cells while maintaining a 100% solve rate.
The long-term objective of The Labyrinth is to create a machine learning solver capable of solving perfect mazes as efficiently as, or better than, traditional uninformed search algorithms.

Rather than hard-coding maze-solving behavior, Theseus learns by observing the best-performing classical algorithm for each generated maze. Over time the neural network is expected to require fewer explored cells while maintaining a 100% solve rate.

---

# Workspace Structure

```
The-Labyrinth/
│
├── daedalus/
│   Maze generation
│
├── theseus/
│   Maze solving
│   Neural network training
│
├── hermes/
│   Discord bot
│
└── output/
    Generated mazes
    Solver output
    Training checkpoints
```
# Workspace Structure

```
The-Labyrinth/
│
├── daedalus/
│   Maze generation
│
├── theseus/
│   Maze solving
│   Neural network training
│
├── hermes/
│   Discord bot
│
└── output/
    Generated mazes
    Solver output
    Training checkpoints
```

---

# Daedalus
# Daedalus

Daedalus is responsible for generating perfect mazes.

## Features

- Recursive backtracking maze generation
- Perfect maze validation
- Random maze dimensions
- Four difficulty levels
- Optional square generation
- SVG export
- JSON export

## Difficulty Levels

| Difficulty | Size |
|------------|------------|
| Easy | 20-49 |
| Medium | 50-99 |
| Hard | 100-249 |
| Labyrinthian | 250-1000 |

Every generated maze contains:

- One unique solution
- Connected graph
- No isolated cells
- Entrance at the upper-left
- Exit at the lower-right

---

# Theseus

Theseus is both a traditional maze solver and a machine learning system.

## Classical Solvers
Daedalus is responsible for generating perfect mazes.

## Features

- Recursive backtracking maze generation
- Perfect maze validation
- Random maze dimensions
- Four difficulty levels
- Optional square generation
- SVG export
- JSON export

## Difficulty Levels

| Difficulty | Size |
|------------|------------|
| Easy | 20-49 |
| Medium | 50-99 |
| Hard | 100-249 |
| Labyrinthian | 250-1000 |

Every generated maze contains:

- One unique solution
- Connected graph
- No isolated cells
- Entrance at the upper-left
- Exit at the lower-right

---

# Theseus

Theseus is both a traditional maze solver and a machine learning system.

## Classical Solvers

- Breadth-First Search (BFS)
- Depth-First Search (DFS)
- Random Search
- A* Search

Each solver records:

- Nodes explored
- Runtime
- Path length
- Solution path

---

# Machine Learning

Theseus trains a neural network to imitate the best-performing classical solver.

For every generated maze:

```
Generate unseen maze
        ↓
Evaluate current model
        ↓
Run all classical solvers
        ↓
Select best teacher
        ↓
Generate training examples
        ↓
Train neural network
        ↓
Save checkpoint
        ↓
Repeat
```

The neural network learns entirely from generated mazes and does not rely on pre-built datasets.
- Random Search
- A* Search

Each solver records:

- Nodes explored
- Runtime
- Path length
- Solution path

---

# Machine Learning

Theseus trains a neural network to imitate the best-performing classical solver.

For every generated maze:

```
Generate unseen maze
        ↓
Evaluate current model
        ↓
Run all classical solvers
        ↓
Select best teacher
        ↓
Generate training examples
        ↓
Train neural network
        ↓
Save checkpoint
        ↓
Repeat
```

The neural network learns entirely from generated mazes and does not rely on pre-built datasets.

---

# Training

Training may be started either interactively or through Hermes.

Training supports:

- Maze-count limited sessions
- Time-limited sessions
- Resume from checkpoint
- Automatic checkpoint rotation
- Automatic model persistence
- Unseen-maze evaluation before learning

---

# Training Checkpoints

After every completed maze Theseus creates a checkpoint.

Each checkpoint stores:

- Neural network model
- Total mazes completed
- Total examples trained
- Training loss
- Training accuracy
- Teacher algorithm
- Teacher performance
- Learned solver performance
- Maze dimensions
- Timestamp

Only the five newest checkpoints are retained.

---

# Hermes_01

Hermes_01 provides remote control through Discord.

## Commands

### Maze Generation

```
/generate
```

Generates a maze using Daedalus.

Options:

- Difficulty
- Square mode

---

### Training

```
/training_start
```

Starts a brand-new training session.

Supports:
# Training

Training may be started either interactively or through Hermes.

Training supports:

- Maze-count limited sessions
- Time-limited sessions
- Resume from checkpoint
- Automatic checkpoint rotation
- Automatic model persistence
- Unseen-maze evaluation before learning

---

# Training Checkpoints

After every completed maze Theseus creates a checkpoint.

Each checkpoint stores:

- Neural network model
- Total mazes completed
- Total examples trained
- Training loss
- Training accuracy
- Teacher algorithm
- Teacher performance
- Learned solver performance
- Maze dimensions
- Timestamp

Only the five newest checkpoints are retained.

---

# Hermes_01

Hermes_01 provides remote control through Discord.

## Commands

### Maze Generation

```
/generate
```

Generates a maze using Daedalus.

Options:

- Difficulty
- Square mode

---

### Training

```
/training_start
```

Starts a brand-new training session.

Supports:

- Number of mazes
- Number of hours

---

```
/training_resume
```

Continues training from the newest checkpoint.

Supports:

- Number of mazes
- Number of hours

---

```
/training_status
```

Displays the current training session.

Includes:

- Current maze count
- Total examples
- Current teacher
- Latest performance
- Training progress

---

```
/training_data
```

Displays information from the newest checkpoint.
- Number of mazes
- Number of hours

---

```
/training_resume
```

Continues training from the newest checkpoint.

Supports:

- Number of mazes
- Number of hours

---

```
/training_status
```

Displays the current training session.

Includes:

- Current maze count
- Total examples
- Current teacher
- Latest performance
- Training progress

---

```
/training_data
```

Displays information from the newest checkpoint.

---

# Automatic Progress Updates

While training is active Hermes:

- Launches Theseus in the background
- Monitors checkpoint files
- Posts Discord updates every 10 minutes
- Reports training completion
- Reports unexpected failures

---

# Running the Applications

## Daedalus

```
cargo run -p daedalus
```
# Automatic Progress Updates

While training is active Hermes:

- Launches Theseus in the background
- Monitors checkpoint files
- Posts Discord updates every 10 minutes
- Reports training completion
- Reports unexpected failures

---

# Running the Applications

## Daedalus

```
cargo run -p daedalus
```

---

## Theseus

Interactive mode:

```
cargo run -p theseus
```

Command line mode:

```
cargo run -p theseus -- train-new --mazes 1000
```

```
cargo run -p theseus -- train-new --hours 5
```

```
cargo run -p theseus -- train-resume --mazes 1000
```

```
cargo run -p theseus -- train-resume --hours 5
```

```
cargo run -p theseus -- training-stats
```
## Theseus

Interactive mode:

```
cargo run -p theseus
```

Command line mode:

```
cargo run -p theseus -- train-new --mazes 1000
```

```
cargo run -p theseus -- train-new --hours 5
```

```
cargo run -p theseus -- train-resume --mazes 1000
```

```
cargo run -p theseus -- train-resume --hours 5
```

```
cargo run -p theseus -- training-stats
```

---
---

## Hermes

```
```
cargo run -p hermes
```

---

# Output

Generated files are written to:
---

# Output

Generated files are written to:

```
output/
```

Including:

```
maze.json
maze.svg

solved_maze_bfs.svg
solved_maze_dfs.svg
solved_maze_random.svg
solved_maze_astar.svg

theseus/
    checkpoints/
```

---

# Current Status

Current implementation includes:

- ✅ Perfect maze generation
- ✅ Maze validation
- ✅ Four classical solvers
- ✅ Automatic benchmarking
- ✅ Teacher selection
- ✅ Neural network implementation
- ✅ Machine learning pipeline
- ✅ Checkpoint persistence
- ✅ Resume training
- ✅ Discord integration
- ✅ Remote training control
- ✅ Automatic training progress updates
- ✅ Zero compiler warnings

---

# Future Goals

- Improve learned solver performance
- Benchmark learned solver against classical algorithms
- Training history visualization
- Additional neural network experimentation
- Distributed training support
output/
```

Including:

```
maze.json
maze.svg

solved_maze_bfs.svg
solved_maze_dfs.svg
solved_maze_random.svg
solved_maze_astar.svg

theseus/
    checkpoints/
```

---

# Current Status

Current implementation includes:

- ✅ Perfect maze generation
- ✅ Maze validation
- ✅ Four classical solvers
- ✅ Automatic benchmarking
- ✅ Teacher selection
- ✅ Neural network implementation
- ✅ Machine learning pipeline 
- ✅ Checkpoint persistence
- ✅ Resume training
- ✅ Discord integration
- ✅ Remote training control
- ✅ Automatic training progress updates
- ✅ Zero compiler warnings

---

# Future Goals

- Improve learned solver performance
- Benchmark learned solver against classical algorithms
- Training history visualization
- Additional neural network experimentation
- Distributed training support