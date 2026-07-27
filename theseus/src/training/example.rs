use std::collections::HashSet;
use std::error::Error;
use std::fmt;

use crate::maze::maze::Maze;
use crate::solver::solver::{Direction, Position, direction_between};

/// One supervised-learning example created from a teacher solution path.
///
/// The model will eventually receive the maze state described by this
/// structure and learn to predict `target_direction`.
#[derive(Debug, Clone)]
pub struct TrainingExample {
    /// Position where the decision is being made.
    pub position: Position,

    /// Whether each side of the current cell contains a wall.
    ///
    /// Order:
    /// [North, East, South, West]
    pub walls: [bool; 4],

    /// Current position normalized to the range 0.0 through 1.0.
    pub current_x_ratio: f32,
    pub current_y_ratio: f32,

    /// Goal position normalized to the range 0.0 through 1.0.
    pub goal_x_ratio: f32,
    pub goal_y_ratio: f32,

    /// Whether neighboring cells have already appeared earlier in the
    /// teacher's solution path.
    ///
    /// Order:
    /// [North, East, South, West]
    pub visited_neighbors: [bool; 4],

    /// Direction selected by the teacher.
    pub target_direction: Direction,
}

impl TrainingExample {
    /// Converts the example into the numeric input that will eventually
    /// be passed to the neural network.
    ///
    /// Input order:
    /// - Four wall values
    /// - Current X and Y ratios
    /// - Goal X and Y ratios
    /// - Four visited-neighbor values
    pub fn input_values(&self) -> [f32; 12] {
        [
            bool_to_f32(self.walls[0]),
            bool_to_f32(self.walls[1]),
            bool_to_f32(self.walls[2]),
            bool_to_f32(self.walls[3]),
            self.current_x_ratio,
            self.current_y_ratio,
            self.goal_x_ratio,
            self.goal_y_ratio,
            bool_to_f32(self.visited_neighbors[0]),
            bool_to_f32(self.visited_neighbors[1]),
            bool_to_f32(self.visited_neighbors[2]),
            bool_to_f32(self.visited_neighbors[3]),
        ]
    }

    /// Converts the target direction into a class index.
    ///
    /// 0 = North
    /// 1 = East
    /// 2 = South
    /// 3 = West
    pub fn target_index(&self) -> usize {
        match self.target_direction {
            Direction::North => 0,
            Direction::East => 1,
            Direction::South => 2,
            Direction::West => 3,
        }
    }
}

#[derive(Debug)]
pub enum TrainingExampleError {
    PathTooShort,
    PositionOutsideMaze {
        position: Position,
    },
    NonAdjacentPositions {
        from: Position,
        to: Position,
    },
    TeacherMovedThroughWall {
        from: Position,
        to: Position,
        direction: Direction,
    },
}

impl fmt::Display for TrainingExampleError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TrainingExampleError::PathTooShort => {
                write!(
                    formatter,
                    "teacher path must contain at least two positions"
                )
            }
            TrainingExampleError::PositionOutsideMaze { position } => {
                write!(
                    formatter,
                    "teacher path position ({}, {}) is outside the maze",
                    position.0, position.1
                )
            }
            TrainingExampleError::NonAdjacentPositions { from, to } => {
                write!(
                    formatter,
                    "teacher path contains non-adjacent positions: \
                     ({}, {}) to ({}, {})",
                    from.0, from.1, to.0, to.1
                )
            }
            TrainingExampleError::TeacherMovedThroughWall {
                from,
                to,
                direction,
            } => {
                write!(
                    formatter,
                    "teacher attempted to move through a wall from \
                     ({}, {}) to ({}, {}) heading {:?}",
                    from.0, from.1, to.0, to.1, direction
                )
            }
        }
    }
}

impl Error for TrainingExampleError {}

/// Converts a teacher solution path into supervised training examples.
///
/// Every pair of consecutive path positions becomes one example. The
/// current position is the model input state and the movement to the next
/// position is the expected output.
pub fn create_training_examples(
    maze: &Maze,
    teacher_path: &[Position],
) -> Result<Vec<TrainingExample>, TrainingExampleError> {
    if teacher_path.len() < 2 {
        return Err(TrainingExampleError::PathTooShort);
    }

    validate_position(maze, teacher_path[0])?;

    let goal = (maze.width.saturating_sub(1), maze.height.saturating_sub(1));

    let mut visited_positions = HashSet::new();
    let mut examples = Vec::with_capacity(teacher_path.len() - 1);

    for positions in teacher_path.windows(2) {
        let current = positions[0];
        let next = positions[1];

        validate_position(maze, current)?;
        validate_position(maze, next)?;

        let target_direction =
            direction_between(current, next).ok_or(TrainingExampleError::NonAdjacentPositions {
                from: current,
                to: next,
            })?;

        if !direction_is_open(maze, current, target_direction) {
            return Err(TrainingExampleError::TeacherMovedThroughWall {
                from: current,
                to: next,
                direction: target_direction,
            });
        }

        let cell = &maze.cells[current.1][current.0];

        let example = TrainingExample {
            position: current,
            walls: [cell.north, cell.east, cell.south, cell.west],
            current_x_ratio: normalize_coordinate(current.0, maze.width),
            current_y_ratio: normalize_coordinate(current.1, maze.height),
            goal_x_ratio: normalize_coordinate(goal.0, maze.width),
            goal_y_ratio: normalize_coordinate(goal.1, maze.height),
            visited_neighbors: visited_neighbor_states(maze, current, &visited_positions),
            target_direction,
        };

        // Call these now so the conversion methods are exercised while
        // the model is still being added.
        let _input = example.input_values();
        let _target = example.target_index();

        examples.push(example);
        visited_positions.insert(current);
    }

    Ok(examples)
}

fn validate_position(maze: &Maze, position: Position) -> Result<(), TrainingExampleError> {
    if position.0 >= maze.width || position.1 >= maze.height {
        return Err(TrainingExampleError::PositionOutsideMaze { position });
    }

    Ok(())
}

fn normalize_coordinate(coordinate: usize, dimension: usize) -> f32 {
    if dimension <= 1 {
        return 0.0;
    }

    coordinate as f32 / (dimension - 1) as f32
}

fn visited_neighbor_states(
    maze: &Maze,
    position: Position,
    visited_positions: &HashSet<Position>,
) -> [bool; 4] {
    let (x, y) = position;

    let north = y > 0 && visited_positions.contains(&(x, y - 1));

    let east = x + 1 < maze.width && visited_positions.contains(&(x + 1, y));

    let south = y + 1 < maze.height && visited_positions.contains(&(x, y + 1));

    let west = x > 0 && visited_positions.contains(&(x - 1, y));

    [north, east, south, west]
}

fn direction_is_open(maze: &Maze, position: Position, direction: Direction) -> bool {
    let (x, y) = position;
    let cell = &maze.cells[y][x];

    match direction {
        Direction::North => !cell.north && y > 0,
        Direction::East => !cell.east && x + 1 < maze.width,
        Direction::South => !cell.south && y + 1 < maze.height,
        Direction::West => !cell.west && x > 0,
    }
}

pub fn encode_maze_state(
    maze: &Maze,
    position: Position,
    visited_positions: &HashSet<Position>,
) -> [f32; 12] {
    let cell = &maze.cells[position.1][position.0];

    let goal = (maze.width.saturating_sub(1), maze.height.saturating_sub(1));

    let visited_neighbors = visited_neighbor_states(maze, position, visited_positions);

    [
        bool_to_f32(cell.north),
        bool_to_f32(cell.east),
        bool_to_f32(cell.south),
        bool_to_f32(cell.west),
        normalize_coordinate(position.0, maze.width),
        normalize_coordinate(position.1, maze.height),
        normalize_coordinate(goal.0, maze.width),
        normalize_coordinate(goal.1, maze.height),
        bool_to_f32(visited_neighbors[0]),
        bool_to_f32(visited_neighbors[1]),
        bool_to_f32(visited_neighbors[2]),
        bool_to_f32(visited_neighbors[3]),
    ]
}

fn bool_to_f32(value: bool) -> f32 {
    if value { 1.0 } else { 0.0 }
}
