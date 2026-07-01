use daedalus::config::Difficulty;
use daedalus::exporter::{export_json, export_svg};
use daedalus::generator::generate_maze;
use daedalus::validator;
use std::fs;
use std::io::{self, Write};


fn main() -> std::io::Result<()> {
    let difficulty = choose_difficulty()?;
    let square_mode = square_mode()?;

    let width = difficulty.random_size();

    let height = if square_mode {
        width
    } else {
        difficulty.random_size()
    };

    let maze = generate_maze(width, height);

    validator::validate_maze(&maze).expect("Generated maze failed to valiadte");

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

fn square_mode() -> std::io::Result<bool> {
    loop {
        print!("Generate a square maze? (y/n): ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => return Ok(true),
            "n" | "no" => return Ok(false),
            _ => println!("Invalid choice. Please enter y of n.\n")
        }
    }
}