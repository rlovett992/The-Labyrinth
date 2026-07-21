use std::collections::HashSet;
use std::time::Duration;

pub type Position = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone)]
pub struct SearchStep {
    pub from: Position,
    pub to: Position,
    pub direction: Direction,
    pub visit_order: usize,
    pub on_solution_path: bool,
}

#[derive(Debug, Clone)]
pub struct SolutionStats {
    pub algorithm: &'static str,
    pub solved: bool,
    pub path_length: usize,
    pub nodes_explored: usize,
    pub duration: Duration,
}

#[derive(Debug, Clone)]
pub struct SolverOutput {
    pub path: Option<Vec<Position>>,
    pub trace: Vec<SearchStep>,
    pub stats: SolutionStats,
}

pub fn direction_between(from: Position, to: Position) -> Option<Direction> {
    match (
        to.0 as isize - from.0 as isize,
        to.1 as isize - from.1 as isize,
    ) {
        (0, -1) => Some(Direction::North),
        (1, 0) => Some(Direction::East),
        (0, 1) => Some(Direction::South),
        (-1, 0) => Some(Direction::West),
        _ => None,
    }
}

pub fn mark_solution_path(trace: &mut [SearchStep], path: &[Position]) {
    let solution_edges: HashSet<(Position, Position)> = path
        .windows(2)
        .map(|positions| (positions[0], positions[1]))
        .collect();

    for step in trace {
        step.on_solution_path =
            solution_edges.contains(&(step.from, step.to));
    }
}