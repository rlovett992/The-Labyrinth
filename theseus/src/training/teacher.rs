use crate::maze::maze::Maze;
use crate::solver::solver::{
    Position,
    SearchStep,
    SolverOutput,
};
use crate::solver::{astar, bfs, dfs, random};

#[derive(Debug, Clone)]
pub struct TeacherResult {
    pub algorithm: String,
    pub path: Vec<Position>,
    pub trace: Vec<SearchStep>,
    pub nodes_explored: usize,
    pub duration_nanos: u128,
}

pub fn select_teacher(maze: &Maze) -> Option<TeacherResult> {
    let outputs = [
        bfs::solve(maze),
        dfs::solve(maze),
        random::solve(maze),
        astar::solve(maze),
    ];

    outputs
        .into_iter()
        .filter_map(valid_teacher_result)
        .min_by(|left, right| {
            left.nodes_explored
                .cmp(&right.nodes_explored)
                .then_with(|| {
                    left.duration_nanos.cmp(&right.duration_nanos)
                })
        })
}

fn valid_teacher_result(output: SolverOutput) -> Option<TeacherResult> {
    if !output.stats.solved {
        return None;
    }

    let path = output.path?;

    Some(TeacherResult {
        algorithm: output.stats.algorithm.to_string(),
        path,
        trace: output.trace,
        nodes_explored: output.stats.nodes_explored,
        duration_nanos: output.stats.duration.as_nanos(),
    })
}