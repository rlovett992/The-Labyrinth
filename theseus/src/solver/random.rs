use std::collections::HashMap;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use crate::maze::maze::Maze;
use crate::solver::solver::{Position, SolverOutput, SolutionStats};


pub fn solve(maze: &Maze) -> SolverOutput {
    let start_time = Instant::now();

    let start = (0, 0);
    let goal = (maze.width - 1, maze.height - 1);

    let mut nodes_explored = 0;

    let mut rng = SimpleRng::new();
    let mut stack = Vec::new();
    let mut came_from: HashMap<Position, Position> = HashMap::new();

    stack.push(start);
    came_from.insert(start, start);

    while let Some(current) = stack.pop() {
        nodes_explored += 1;

        if current == goal {
            let path = reconstruct_path(came_from, start, goal);

            return SolverOutput {
                path: Some(path.clone()),
                stats: SolutionStats {
                    algorithm: "Random",
                    solved: true,
                    path_length: path.len(),
                    nodes_explored,
                    duration: start_time.elapsed(),
                },
            };
        }

        let mut neighbors = maze.neighbors(current);
        shuffle(&mut neighbors, &mut rng);

        for neighbor in neighbors {
            if !came_from.contains_key(&neighbor) {
                stack.push(neighbor);
                came_from.insert(neighbor, current);
            }
        }
    }

    SolverOutput {
        path: None,
        stats: SolutionStats {
            algorithm: "Random",
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

fn shuffle<T>(items: &mut [T], rng: &mut SimpleRng) {
    for i in (1..items.len()).rev() {
        let j = rng.next_usize(i + 1);
        items.swap(i, j);
    }
}

struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    fn new() -> Self {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;

        Self { state: seed }
    }

    fn next_usize(&mut self, max: usize) -> usize {
        self.state = self
            .state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1);

        (self.state as usize) % max
    }
}