# The Labyrinth

A Rust project consisting of two independent but related applications:

## Daedalus

A maze generator.

---

### Current Status

Daedalus can generate and export mazes with a random size based on the selected diffuculty. Also allows the user to pick if the maze is a square or not and validates the maze to make sure it is solvable

### Next Feature(s) to be Added

- None at the moment

---

## Theseus

An AI-assisted maze solver.

---

### Current Status

 - Can solve mazes with A*, BFS, DFS, and random stepping

### Next Feature(s) to be Added

 - The neural network portion so it can start training

---

# Project Goals

Create a system where:

1. Daedalus generates solvable mazes.
2. Daedalus exports those mazes in both human-readable and machine-readable formats.
3. Theseus receives only the exported maze.
4. Theseus solves the maze without knowledge of how it was generated.
5. Theseus eventually learns from large numbers of generated mazes and becomes faster at solving new mazes.

---

## How to Run

The Labyrinth workspace currently contains three executables:

- **Daedalus** – Maze generator
- **Theseus** – Maze solver
- **Hermes** – Discord interface

Each can be run independently from the workspace root.

---

### Running Daedalus

Daedalus generates a new maze and exports it to the `output` directory.

```bash
cargo run -p daedalus
```

Output:

- `maze.svg`
- `maze.json`

---

### Running Theseus

Theseus loads the most recently generated maze, solves it using every implemented algorithm, and exports a solution SVG for each solver.

```bash
cargo run -p theseus
```

Current solvers:

- Breadth-First Search (BFS)
- Depth-First Search (DFS)
- A*
- Random

Example output:

- `solved_maze_bfs.svg`
- `solved_maze_dfs.svg`
- `solved_maze_astar.svg`
- `solved_maze_random.svg`

---

### Running Hermes

Hermes is the Discord bot for The Labyrinth.

Before starting Hermes, ensure a valid Discord bot token is present in your `.env` file.

Start Hermes with:

```bash
cargo run -p hermes
```

Once running, Hermes registers its slash commands and waits for requests from Discord.

Current commands:

- `/generate` – Generate a maze.
- `/theseus` – Execute the current Theseus solver and return the generated solution files.
- See the Hermes section below for more details

---

### Building the Entire Workspace

To compile every crate without running them:

```bash
cargo build
```

---

### Running Tests

Execute all tests across the workspace:

```bash
cargo test
```

---

# Daedalus

## Purpose

Generate high-quality, solvable mazes for users and for Theseus training.

## Maze Rules

All generated mazes:

- Start position: Top-left corner
- Exit position: Bottom-right corner

### Allowed Movement

- Up
- Down
- Left
- Right

### Forbidden Movement

- Diagonal movement

### Requirements

- Must be solvable

---

## Difficulty Levels

### Easy

- Width range: 20–49
- Height range: 20–49

### Medium

- Width range: 50–99
- Height range: 50–99

### Hard

- Width range: 100–249
- Height range: 100–249

### Labyrinthian

- Width range: 250–1000
- Height range: 250–1000

---

## Size Generation

### Default Mode

Width and height are randomized independently.

Examples:

- 32 × 47
- 91 × 53
- 183 × 241
- 407 × 982

### Square Mode

Optional toggle.

When enabled:

```text
Width = Height
```

Examples:

- 35 × 35
- 72 × 72
- 201 × 201
- 814 × 814

---

## Maximum Maze Size

```text
1000 × 1000 cells
```

Maximum cell count:

```text
1,000,000 cells
```

---

## Random Seed Support

Optional seed value.

Example:

```text
Seed: 123456789
```

Allows exact maze reproduction.

---

## Exports

### SVG

Purpose:

- Human readable
- Infinitely scalable
- Printable
- Viewable in browsers

### JSON

Purpose:

- Machine readable
- Used by Theseus
- Official maze specification

Example metadata:

```json
{
  "difficulty": "Hard",
  "width": 183,
  "height": 241,
  "start": [0, 0],
  "exit": [182, 240],
  "seed": 123456789
}
```

---

## Future Dataset Mode

Generate large numbers of mazes automatically.

Example:

```text
Generate 10,000 mazes
Difficulty: Random
Output: Dataset folder
```

Used for AI training.

---

# Theseus

## Purpose

Solve maze files produced by Daedalus.

Eventually use machine learning to improve solving speed.

---

## Input

Theseus receives:

- Maze JSON

Optionally:

- SVG visualization

Theseus never receives:

- Generator algorithm
- Generation seed
- Hidden solution path
- Internal Daedalus state

---

## Initial Solver Phase

Before any AI, implement traditional algorithms.

### Breadth-First Search (BFS)

**Pros**

- Guaranteed shortest path
- Reliable

### Depth-First Search (DFS)

**Pros**

- Simple
- Fast

### A*

**Pros**

- Very efficient
- Excellent baseline

 --- 

## Hermes

Hermes is the Discord interface for **The Labyrinth**. It allows maze generation and solver execution directly from a Discord server using slash commands.

### Current Features

- Generate mazes directly from Discord.
- Supports all maze difficulties:
  - Easy
  - Medium
  - Hard
  - Labyrinthian
- Optional square maze generation.
- Automatically exports generated mazes as:
  - SVG
  - JSON
- Uploads generated files directly back to Discord.
- Execute the current Theseus solver directly from Discord.
- Returns generated solution SVGs after the solver completes.
- Global slash command registration (no per-server configuration required).

Hermes acts as the control center for The Labyrinth project, providing a simple Discord interface instead of requiring terminal commands.

---

## Slash Commands

### `/generate`

Generate a new maze.

**Parameters**

| Parameter | Description | Required |
|-----------|-------------|----------|
| difficulty | easy, medium, hard, labyrinthian | Yes |
| square | Generate a square maze | No |

Example:

```
/generate difficulty:hard square:true
```

Outputs:

- maze.svg
- maze.json

---

### `/theseus`

Runs the current Theseus solver against the latest maze.

Outputs:

- solved_maze_bfs.svg
- solved_maze_dfs.svg

The command also displays the solver output directly in Discord.

---

## How to Use

### 1. Start Hermes

From the project workspace:

```bash
cargo run -p hermes
```

When Hermes starts successfully you should see:

```
Hermes is online.
```

---

### 2. Invite Hermes to Your Discord Server

Ensure the bot has permission to:

- Use Slash Commands
- Send Messages
- Attach Files
- Read Message History

---

### 3. Generate a Maze

Use:

```
/generate
```

Choose:

- difficulty
- optional square mode

Hermes will generate a maze and upload both the SVG and JSON files.

---

### 4. Run Theseus

Use:

```
/theseus
```

Hermes will:

1. Launch the Theseus crate.
2. Execute all currently implemented solvers.
3. Capture console output.
4. Upload generated solution SVGs.
5. Report whether execution succeeded or failed.

---

### 5. Repeat

Generate another maze or rerun the solver at any time using the slash commands.