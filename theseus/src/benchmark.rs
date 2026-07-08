use crate::maze::maze::Maze;
use crate::solution::{
    export_solution_svg_astar,
    export_solution_svg_bfs,
    export_solution_svg_dfs,
    export_solution_svg_random,
};
use crate::solver::solver::SolverOutput;
use crate::solver::{astar, bfs, dfs, random};

fn print_results(output: &SolverOutput) {
    println!("{}", output.stats.algorithm);
    println!("  Path length:    {}", output.stats.path_length);
    println!("  Nodes explored: {}", output.stats.nodes_explored);
    println!("  Time:           {:.2?}", output.stats.duration);
}

pub fn benchmark(maze: &Maze) {
    println!();
    println!("========== THESEUS BENCHMARK ==========");
    println!("Maze: {}x{}", maze.width, maze.height);
    println!();

    // ---------------- BFS ----------------

    let bfs_output = bfs::solve(maze);

    if bfs_output.stats.solved {
        print_results(&bfs_output);

        if let Some(path) = bfs_output.path {
            export_solution_svg_bfs(maze, &path)
                .expect("Failed to export BFS solution");
        }
    }

    println!();

    // ---------------- DFS ----------------

    let dfs_output = dfs::solve(maze);

    if dfs_output.stats.solved {
        print_results(&dfs_output);

        if let Some(path) = dfs_output.path {
            export_solution_svg_dfs(maze, &path)
                .expect("Failed to export DFS solution");
        }
    }

    println!();

    // ---------------- RANDOM ----------------

    let random_output = random::solve(maze);

    if random_output.stats.solved {
        print_results(&random_output);

        if let Some(path) = random_output.path {
            export_solution_svg_random(maze, &path)
                .expect("Failed to export Random solution");
        }
    }

    println!();

    // ---------------- A* ----------------

    let astar_output = astar::solve(maze);

    if astar_output.stats.solved {
        print_results(&astar_output);

        if let Some(path) = astar_output.path {
            export_solution_svg_astar(maze, &path)
                .expect("Failed to export A* solution");
        }
    }

    println!();
    println!("=======================================");
}