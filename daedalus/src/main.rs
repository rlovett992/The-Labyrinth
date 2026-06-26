mod exporter;
mod generator;
mod maze;
mod config;

use config::Difficulty;
use exporter::{export_json, export_svg};
use generator::generate_maze;
use std::fs;
use std::io::{self, Write};


fn main() -> std::io::Result<()> {
    let difficulty = choose_difficulty()?;
    let size = difficulty.random_size();
    let maze = generate_maze(size, size);

    fs::create_dir_all("output")?;

    export_json(&maze, "output/maze.json")?;
    export_svg(&maze, "output/maze.svg")?;

    println!("Generated {} maze: {}x{}", difficulty.label(), maze.width, maze.height);

    println!("Exported JSON to output/maze.json");
    println!("Exported SVG to output/maze.svg");
    
    Ok(())
}

fn choose_difficulty() -> std::io::Result<Difficulty> {
    loop {
        println!("Choose a difficulty:");
        println!("1. Easy");
        println!("2. Medium");
        println!("3. Hard");
        println!("4. Labyrinthian");
        print!("Enter choice: ");

        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim() {
            "1" => return Ok(Difficulty::Easy),
            "2" => return Ok(Difficulty::Medium),
            "3" => return Ok(Difficulty::Hard),
            "4" => return Ok(Difficulty::Labyrinthian),
            _ => println!("Invalid choice. Please enter 1, 2, 3, or 4.\n")
        }
    }
}