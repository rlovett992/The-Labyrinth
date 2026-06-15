use rand::seq::SliceRandom;

use crate::maze::Maze;

#[derive(Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

pub fn generate_maze(width: usize, height: usize) -> Maze {
    let mut maze = Maze::new(width, height);

    carve_from(0, 0, &mut maze);

    maze.cells[0][0].west = false;
    maze.cells[height - 1][width - 1].east = false;

    maze
}

fn carve_from(x: usize, y: usize, maze: &mut Maze) {
    maze.cells[y][x].visited = true;

    let mut directions = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];

    let mut rng = rand::rng();
    directions.shuffle(&mut rng);

    for direction in directions {
        let Some((next_x, next_y)) = get_neighbor(x, y, direction, maze.width, maze.height) else {
            continue;
        };

        if !maze.cells[next_y][next_x].visited {
            remove_wall(x, y, next_x, next_y, direction, maze);
            carve_from(next_x, next_y, maze);
        }
    }
}

fn get_neighbor(x: usize, y: usize, direction: Direction, width: usize, height: usize,) -> Option<(usize, usize)> {
    match direction {
        Direction::North => {
            if y > 0 {
                Some((x, y - 1))
            }
            else {
                None
            }
        }
        Direction::East => {
            if x + 1 < width {
                Some((x + 1, y))
            }
            else {
                None
            }
        }
        Direction::South => {
            if y + 1 < height {
                Some((x, y + 1))
            }
            else {
                None
            }
        }
        Direction::West => {
            if x > 0 {
                Some((x - 1, y))
            }
            else {
                None
            }
        }
    }
}

fn remove_wall(x: usize, y: usize, next_x: usize, next_y: usize, direction: Direction, maze: &mut Maze,) {
    match direction {
        Direction::North => {
            maze.cells[y][x].north = false;
            maze.cells[next_y][next_x].south = false;
        }
        Direction::East => {
            maze.cells[y][x].east = false;
            maze.cells[next_y][next_x].west = false;
        }
        Direction::South => {
            maze.cells[y][x].south = false;
            maze.cells[next_y][next_x].north = false;
        }
        Direction::West => {
            maze.cells[y][x].west = false;
            maze.cells[next_y][next_x].east = false;
        }
    }
}