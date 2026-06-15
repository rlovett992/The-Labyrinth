mod exporter;
mod generator;
mod maze;
mod renderer;

use exporter::{export_json, export_svg};
use generator::generate_maze;
use renderer::render_ascii;
use std::fs;

fn main() -> std::io::Result<()> {
    let maze = generate_maze(10, 10);

    fs::create_dir_all("output")?;

    export_json(&maze, "output/maze.json")?;
    export_svg(&maze, "output/maze.svg")?;

    println!("Generated maze: {}x{}", maze.width, maze.height);
    println!("Exported JSON to output/maze.json");
    println!("Exported SVG to output/maze.svg");

    render_ascii(&maze);
    
    Ok(())
}
