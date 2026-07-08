mod maze;
mod solution;
mod solver;

use maze::loader::load;
use solution::{
    export_solution_svg_astar,
    export_solution_svg_bfs,
    export_solution_svg_dfs,
    export_solution_svg_random,
};
use solver::{astar, bfs, dfs, random};

fn main() {
    let maze = load("output/maze.json")
        .expect("Failed to load maze");

    println!("Loaded maze: {}x{}\n", maze.width, maze.height);

    let bfs_output = bfs::solve(&maze);
    if bfs_output.stats.solved {
        println!("{} solved maze.", bfs_output.stats.algorithm);
        println!("Path length: {}", bfs_output.stats.path_length);
        println!("Nodes explored: {}", bfs_output.stats.nodes_explored);
        println!("Time: {:.2?}\n", bfs_output.stats.duration);

        if let Some(path) = bfs_output.path {
            export_solution_svg_bfs(&maze, &path)
                .expect("Failed to export BFS solution");
        }
    } else {
        println!("{} failed to solve the maze.", bfs_output.stats.algorithm);
    }

    let dfs_output = dfs::solve(&maze);
    if dfs_output.stats.solved {
        println!("{} solved maze.", dfs_output.stats.algorithm);
        println!("Path length: {}", dfs_output.stats.path_length);
        println!("Nodes explored: {}", dfs_output.stats.nodes_explored);
        println!("Time: {:.2?}\n", dfs_output.stats.duration);

        if let Some(path) = dfs_output.path {
            export_solution_svg_dfs(&maze, &path)
                .expect("Failed to export DFS solution");
        }
    } else {
        println!("{} failed to solve the maze.", dfs_output.stats.algorithm);
    }

    let random_output = random::solve(&maze);
    if random_output.stats.solved {
        println!("{} solved maze.", random_output.stats.algorithm);
        println!("Path length: {}", random_output.stats.path_length);
        println!("Nodes explored: {}", random_output.stats.nodes_explored);
        println!("Time: {:.2?}\n", random_output.stats.duration);

        if let Some(path) = random_output.path {
            export_solution_svg_random(&maze, &path)
                .expect("Failed to export Random solution");
        }
    } else {
        println!("{} failed to solve the maze.", random_output.stats.algorithm);
    }

    let astar_output = astar::solve(&maze);
    if astar_output.stats.solved {
        println!("{} solved maze.", astar_output.stats.algorithm);
        println!("Path length: {}", astar_output.stats.path_length);
        println!("Nodes explored: {}", astar_output.stats.nodes_explored);
        println!("Time: {:.2?}", astar_output.stats.duration);

        if let Some(path) = astar_output.path {
            export_solution_svg_astar(&maze, &path)
                .expect("Failed to export A* solution");
        }
    } else {
        println!("{} failed to solve the maze.", astar_output.stats.algorithm);
    }
}