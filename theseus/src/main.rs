mod maze;
mod solution;
mod solver;

use maze::loader::load;
use solution::export_solution_svg_bfs;
use solver::bfs;

fn main() {
    let maze = load("output/maze.json").expect("Failed to load maze");

    println!("Loaded {}x{} maze", maze.width, maze.height);

    match bfs::solve(&maze) {
    Some(path) => {
        println!("Maze solved successfully.");
        println!("Start: {:?}", path.first().unwrap());
        println!("Goal : {:?}", path.last().unwrap());
        println!("Path length: {} cells", path.len());

        export_solution_svg_bfs(&maze, &path, "output/solved_maze_bfs.svg").expect("Failed to export solved maze SVG");
        println!("Solved maze exported to output/solved_maze.svg");
    }
    None => {
        println!("No solution found.");
    }

    }
}