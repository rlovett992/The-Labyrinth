mod maze;
mod solution;
mod solver;

use maze::loader::load;
use solution::{export_solution_svg_astar, export_solution_svg_bfs, export_solution_svg_dfs, export_solution_svg_random};
use solver::{astar, bfs, dfs, random};

fn main() {
    let maze = load("output/maze.json")
        .expect("Failed to load maze");

    println!("Loaded maze: {}x{}", maze.width, maze.height);

    if let Some(path) = bfs::solve(&maze) {
        println!("BFS solved maze.");
        println!("BFS path length: {} cells", path.len());

        export_solution_svg_bfs(&maze, &path).expect("Failed to export BFS solution");
    }

    if let Some(path) = dfs::solve(&maze) {
        println!("DFS solved maze.");
        println!("DFS path length: {} cells", path.len());

        export_solution_svg_dfs(&maze, &path).expect("Failed to export DFS solution");
    }

    if let Some(path) = random::solve(&maze) {
    println!("Random solved maze.");
    println!("Random path length: {} cells", path.len());

    export_solution_svg_random(&maze, &path).expect("Failed to export Random solution");
}

    if let Some(path) = astar::solve(&maze) {
        println!("A* solved maze.");
        println!("A* path length: {} cells", path.len());

        export_solution_svg_astar(&maze, &path).expect("Failed to export A* solution");
    }
}