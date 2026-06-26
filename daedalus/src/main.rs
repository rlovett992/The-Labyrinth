mod exporter;
mod generator;
mod maze;
mod config;

use exporter::{export_json, export_svg};
use generator::generate_maze;
//use rand::Rng;
use std::fs;
use config::Difficulty;

fn main() -> std::io::Result<()> {
    let size = Difficulty::Easy.random_size();
    let maze = generate_maze(size, size);

    fs::create_dir_all("output")?;

    export_json(&maze, "output/maze.json")?;
    export_svg(&maze, "output/maze.svg")?;

    println!("Generated maze: {}x{}", maze.width, maze.height);
    println!("Exported JSON to output/maze.json");
    println!("Exported SVG to output/maze.svg");
    
    Ok(())
}
