use std::collections::{HashMap, VecDeque};
use std::time::Instant;

use crate::maze::maze::Maze;
use crate::solver::solver::{
    Position,
    SearchStep,
    SolutionStats,
    SolverOutput,
    direction_between,
    mark_solution_path,
};

pub fn solve(maze: &Maze) -> SolverOutput {
    let start_time = Instant::now();

    let start = (0, 0);
    let goal = (maze.width - 1, maze.height - 1);

    let mut nodes_explored = 0;
    let mut trace = Vec::new();

    let mut queue = VecDeque::new();
    let mut came_from: HashMap<Position, Position> = HashMap::new();

    queue.push_back(start);
    came_from.insert(start, start);

    while let Some(current) = queue.pop_front() {
        nodes_explored += 1;

        record_search_step(
            current,
            start,
            nodes_explored,
            &came_from,
            &mut trace,
        );

        if current == goal {
            let path = reconstruct_path(&came_from, start, goal);

            mark_solution_path(&mut trace, &path);

            return SolverOutput {
                path: Some(path.clone()),
                trace,
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
        trace,
        stats: SolutionStats {
            algorithm: "BFS",
            solved: false,
            path_length: 0,
            nodes_explored,
            duration: start_time.elapsed(),
        },
    }
}

fn record_search_step(
    current: Position,
    start: Position,
    visit_order: usize,
    came_from: &HashMap<Position, Position>,
    trace: &mut Vec<SearchStep>,
) {
    if current == start {
        return;
    }

    let Some(&parent) = came_from.get(&current) else {
        return;
    };

    let Some(direction) = direction_between(parent, current) else {
        return;
    };

    trace.push(SearchStep {
        from: parent,
        to: current,
        direction,
        visit_order,
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