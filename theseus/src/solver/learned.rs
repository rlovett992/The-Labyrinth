use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

use crate::maze::maze::Maze;
use crate::solver::solver::{
    Direction, Position, SearchStep, SolutionStats, SolverOutput, direction_between,
    mark_solution_path,
};
use crate::training::example::encode_maze_state;
use crate::training::model::TheseusModel;

pub fn solve(maze: &Maze, model: &TheseusModel) -> SolverOutput {
    let start_time = Instant::now();

    if maze.width == 0 || maze.height == 0 {
        return failed_output(start_time, 0, Vec::new());
    }

    let start = (0, 0);
    let goal = (maze.width - 1, maze.height - 1);

    let mut nodes_explored = 0;
    let mut trace = Vec::new();

    let mut stack = Vec::new();
    let mut visited = HashSet::new();
    let mut came_from: HashMap<Position, Position> = HashMap::new();

    stack.push(start);
    visited.insert(start);
    came_from.insert(start, start);

    while let Some(current) = stack.pop() {
        nodes_explored += 1;

        record_search_step(current, start, &came_from, &mut trace);

        if current == goal {
            let path = reconstruct_path(&came_from, start, goal);

            mark_solution_path(&mut trace, &path);

            return SolverOutput {
                path: Some(path.clone()),
                trace,
                stats: SolutionStats {
                    algorithm: "Theseus",
                    solved: true,
                    path_length: path.len(),
                    nodes_explored,
                    duration: start_time.elapsed(),
                },
            };
        }

        let input = encode_maze_state(maze, current, &visited);

        let probabilities = model.predict_probabilities(&input);

        let mut ranked_neighbors: Vec<RankedNeighbor> = maze
            .neighbors(current)
            .into_iter()
            .filter(|neighbor| !visited.contains(neighbor))
            .filter_map(|neighbor| {
                let direction = direction_between(current, neighbor)?;

                let probability = probabilities[direction_index(direction)];

                Some(RankedNeighbor {
                    position: neighbor,
                    probability,
                })
            })
            .collect();

        /*
            The stack is last-in, first-out.

            Sorting from lowest to highest means the highest-scoring
            neighbor is pushed last and therefore explored first.
        */
        ranked_neighbors.sort_by(|left, right| {
            left.probability
                .partial_cmp(&right.probability)
                .unwrap_or(Ordering::Equal)
        });

        for ranked_neighbor in ranked_neighbors {
            let neighbor = ranked_neighbor.position;

            if visited.insert(neighbor) {
                came_from.insert(neighbor, current);
                stack.push(neighbor);
            }
        }
    }

    failed_output(start_time, nodes_explored, trace)
}

struct RankedNeighbor {
    position: Position,
    probability: f32,
}

fn direction_index(direction: Direction) -> usize {
    match direction {
        Direction::North => 0,
        Direction::East => 1,
        Direction::South => 2,
        Direction::West => 3,
    }
}

fn record_search_step(
    current: Position,
    start: Position,
    came_from: &HashMap<Position, Position>,
    trace: &mut Vec<SearchStep>,
) {
    if current == start {
        return;
    }

    let Some(&parent) = came_from.get(&current) else {
        return;
    };

    trace.push(SearchStep {
        from: parent,
        to: current,
        on_solution_path: false,
    });
}

fn reconstruct_path(
    came_from: &HashMap<Position, Position>,
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

fn failed_output(
    start_time: Instant,
    nodes_explored: usize,
    trace: Vec<SearchStep>,
) -> SolverOutput {
    SolverOutput {
        path: None,
        trace,
        stats: SolutionStats {
            algorithm: "Theseus",
            solved: false,
            path_length: 0,
            nodes_explored,
            duration: start_time.elapsed(),
        },
    }
}
