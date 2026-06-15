# The Labyrinth

A Rust project consisting of two independent but related applications:

## Daedalus

A maze generator.

---

### Current Status

Daedalus can generate and export mazes.

### Next Feature(s) to be Added

- The difficulty selector for the mazes
- Variable height/width
- Square only mode

---

## Theseus

An AI-assisted maze solver.

---

### Current Status

Not yet started

### Next Feature(s) to be Added


---

# Project Goals

Create a system where:

1. Daedalus generates solvable mazes.
2. Daedalus exports those mazes in both human-readable and machine-readable formats.
3. Theseus receives only the exported maze.
4. Theseus solves the maze without knowledge of how it was generated.
5. Theseus eventually learns from large numbers of generated mazes and becomes faster at solving new mazes.

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