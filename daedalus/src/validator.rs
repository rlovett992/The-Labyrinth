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
                "Maze width mismatch on row {}: expected {}, found {} cells.",
                row_index,
                maze.width,
                row.len()
            ));
        }
    }

    Ok(())
}

fn validate_wall_consistency(maze: &Maze) -> Result<(), String> {
    for y in 0..maze.height {
        for x in 0..maze.width {
            let cell = &maze.cells[y][x];

            if x + 1 < maze.width {
                let east_neighbor = &maze.cells[y][x + 1];

                if cell.east != east_neighbor.west {
                    return Err(format!(
                        "Wall mismatch between ({}, {}) east and ({}, {}) west.",
                        x,
                        y,
                        x + 1,
                        y
                    ));
                }
            }

            if y + 1 < maze.height {
                let south_neighbor = &maze.cells[y + 1][x];

                if cell.south != south_neighbor.north {
                    return Err(format!(
                        "Wall mismatch between ({}, {}) south and ({}, {}) north.",
                        x,
                        y,
                        x,
                        y + 1
                    ));
                }
            }
        }
    }

    Ok(())
}

fn validate_reachability(maze: &Maze) -> Result<(), String> {
    let mut visited = vec![vec![false; maze.width]; maze.height];
    let mut queue = VecDeque::new();

    visited[0][0] = true;
    queue.push_back((0usize, 0usize));

    let mut visited_count = 0;

    while let Some((x, y)) = queue.pop_front() {
        visited_count += 1;

        let cell = &maze.cells[y][x];

        if !cell.north && y > 0 && !visited[y - 1][x] {
            visited[y - 1][x] = true;
            queue.push_back((x, y - 1));
        }

        if !cell.east && x + 1 < maze.width && !visited[y][x + 1] {
            visited[y][x + 1] = true;
            queue.push_back((x + 1, y));
        }

        if !cell.south && y + 1 < maze.height && !visited[y + 1][x] {
            visited[y + 1][x] = true;
            queue.push_back((x, y + 1));
        }

        if !cell.west && x > 0 && !visited[y][x - 1] {
            visited[y][x - 1] = true;
            queue.push_back((x - 1, y));
        }
    }

    let expected_count = maze.width * maze.height;

    if visited_count != expected_count {
        return Err(format!(
            "Maze is not fully connected: visited {} of {} cells.",
            visited_count, expected_count
        ));
    }

    if !visited[maze.height - 1][maze.width - 1] {
        return Err("Exit is not reachable from the start.".to_string());
    }

    Ok(())
}

fn validate_perfect_maze(maze: &Maze) -> Result<(), String> {
    let mut passages = 0;

    for y in 0..maze.height {
        for x in 0..maze.width {
            let cell = &maze.cells[y][x];

            if x + 1 < maze.width && !cell.east {
                passages += 1;
            }

            if y + 1 < maze.height && !cell.south {
                passages += 1;
            }
        }
    }

    let expected_passages = maze.width * maze.height - 1;

    if passages != expected_passages {
        return Err(format!(
            "Maze is not perfect: expected {} passages, found {}.",
            expected_passages, passages
        ));
    }

    Ok(())
}