use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

use crate::maze::maze::Maze;

pub type Position = (usize, usize);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Node {
    position: Position,
    g_score: usize,
    f_score: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .f_score
            .cmp(&self.f_score)
            .then_with(|| other.g_score.cmp(&self.g_score))
            .then_with(|| other.position.cmp(&self.position))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn solve(maze: &Maze) -> Option<Vec<Position>> {
    let start = (0, 0);
    let goal = (maze.width - 1, maze.height - 1);

    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<Position, Position> = HashMap::new();
    let mut g_scores: HashMap<Position, usize> = HashMap::new();

    open_set.push(Node {
        position: start,
        g_score: 0,
        f_score: heuristic(start, goal),
    });

    came_from.insert(start, start);
    g_scores.insert(start, 0);

    while let Some(current_node) = open_set.pop() {
        let current = current_node.position;

        if current == goal {
            return Some(reconstruct_path(came_from, start, goal));
        }

        let current_best_g = g_scores[&current];

        if current_node.g_score > current_best_g {
            continue;
        }

        for neighbor in maze.neighbors(current) {
            let tentative_g_score = current_best_g + 1;

            if tentative_g_score < *g_scores.get(&neighbor).unwrap_or(&usize::MAX) {
                came_from.insert(neighbor, current);
                g_scores.insert(neighbor, tentative_g_score);

                open_set.push(Node {
                    position: neighbor,
                    g_score: tentative_g_score,
                    f_score: tentative_g_score + heuristic(neighbor, goal),
                });
            }
        }
    }

    None
}

fn heuristic(position: Position, goal: Position) -> usize {
    position.0.abs_diff(goal.0) + position.1.abs_diff(goal.1)
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