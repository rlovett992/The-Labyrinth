use std::collections::{HashMap, VecDeque};
use std::time::Instant;

use crate::maze::maze::Maze;
use crate::solver::solver::{Position, SolverOutput, SolutionStats};


pub fn solve(maze: &Maze) -> SolverOutput {
    let start_time = Instant::now();

    let start = (0, 0);
    let goal = (maze.width - 1, maze.height - 1);

    let mut nodes_explored = 0;

    let mut queue = VecDeque::new();
    let mut came_from: HashMap<Position, Position> = HashMap::new();

    queue.push_back(start);
    came_from.insert(start, start);

    while let Some(current) = queue.pop_front() {
        nodes_explored += 1;

        if current == goal {
            let path = reconstruct_path(came_from, start, goal);

            return SolverOutput {
                path: Some(path.clone()),
                stats: SolutionStats {
                    algorithm: "BFS",
                    solved: true,
                    path_length: path.len(),
                    nodes_explored,
                    duration: start_time.elapsed(),
                },
            };
        }

        for neighbor in maze.neighbors(current) {
            if !came_from.contains_key(&neighbor) {
                queue.push_back(neighbor);
                came_from.insert(neighbor, current);
            }
        }
    }

    SolverOutput {
        path: None,
        stats: SolutionStats {
            algorithm: "BFS",
            solved: false,
            path_length: 0,
            nodes_explored,
            duration: start_time.elapsed(),
        },
    }
}

fn reconstruct_path(
    came_from: HashMap<Position, Position>,
    start: Position,
    goal: Position,
) -> Vec<Position> {
    let mut path = vec![goal];
    let mut current = goal;

    while current != start {
        current = came_from[&current];
        path.push(current);
    }

    path.reverse();
    path
}