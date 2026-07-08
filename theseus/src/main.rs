mod maze;
mod solver;

use maze::loader::load;
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
    }
    None => {
        println!("No solution found.");
    }

    }
}