use std::collections::HashMap;

use crate::maze::maze::Maze;
use crate::solver::bfs::Position;

pub fn solve(maze: &Maze) -> Option<Vec<Position>> {
    let start = (0, 0);
    let goal = (maze.width - 1, maze.height - 1);

    let mut stack = Vec::new();
    let mut came_from: HashMap<Position, Position> = HashMap::new();

    stack.push(start);
    came_from.insert(start, start);

    while let Some(current) = stack.pop() {
        if current == goal {
            return Some(reconstruct_path(came_from, start, goal));
        }

        for neighbor in maze.neighbors(current) {
            if !came_from.contains_key(&neighbor) {
                stack.push(neighbor);
                came_from.insert(neighbor, current);
            }
        }
    }

    None
}

fn reconstruct_path(came_from: HashMap<Position, Position>, start: Position, goal: Position) -> Vec<Position> {
    let mut path = vec![goal];
    let mut current = goal;

    while current != start {
        current = came_from[&current];
        path.push(current);
    }

    path.reverse();
    path
}