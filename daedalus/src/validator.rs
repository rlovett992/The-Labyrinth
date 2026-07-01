use std::collections::VecDeque;

use crate::maze::Maze;

pub fn validate_maze(maze: &Maze) -> Result<(), String> {
    validate_dimensions(maze)?;
    validate_wall_consistency(maze)?;
    validate_reachability(maze)?;
    validate_perfect_maze(maze)?;

    Ok(())
}

fn validate_dimensions(maze: &Maze) -> Result<(), String> {
    if maze.width == 0 || maze.height == 0 {
        return Err("Maze width and height must be greater than 0.".to_string());
    }

    if maze.cells.len() != maze.height {
        return Err(format!(
            "Maze height mismatch: expected {}, found {} rows.",
            maze.height,
            maze.cells.len()
        ));
    }

    for (row_index, row) in maze.cells.iter().enumerate() {
        if row.len() != maze.width {
            return Err(format!(
                "Maze width mismatch on row {}: expected {}. found {} cells.",
                row_index,
                maze.width,
                row.len()
            ));
        }
    }

    Ok(())
}

fn validate_wall_consistency(maze: &Maze) -> Result<(), String> {
    
}