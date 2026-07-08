use std::time::Duration;

pub type Position = (usize, usize);

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
    pub stats: SolutionStats,
}