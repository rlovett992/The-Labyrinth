mod benchmark;
mod maze;
mod solution;
mod solver;

use benchmark::benchmark;
use maze::loader::load;

fn main() {
    let maze = load("output/maze.json")
        .expect("Failed to load maze");

    benchmark(&maze);
}