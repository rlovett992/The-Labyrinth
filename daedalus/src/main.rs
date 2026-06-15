mod generator;
mod maze;
mod renderer;

use generator::generate_maze;
use renderer::render_ascii;

fn main() {
    let maze = generate_maze(10, 10);

    println!("Generated maze: {}x{}", maze.width, maze.height);
    render_ascii(&maze);
}
