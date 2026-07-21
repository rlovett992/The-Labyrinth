mod benchmark;
mod maze;
mod solution;
mod solver;
mod training;

use std::io::{self, Write};

use benchmark::benchmark;
use maze::loader::load;
use training::trainer::{
    resume_training,
    start_new_training,
    view_training_statistics,
};

const MAZE_PATH: &str = "output/maze.json";

fn main() {
    loop {
        print_main_menu();

        match read_menu_choice() {
            Ok(0) => {
                println!("Exiting Theseus.");
                break;
            }
            Ok(1) => solve_maze(),
            Ok(2) => start_new_training(),
            Ok(3) => resume_training(),
            Ok(4) => view_training_statistics(),
            Ok(_) => {
                println!();
                println!("Invalid selection. Enter a number from 0 through 4.");
            }
            Err(error) => {
                eprintln!();
                eprintln!("Failed to read menu selection: {error}");
            }
        }

        pause();
    }
}

fn print_main_menu() {
    println!();
    println!("==========================");
    println!("         THESEUS");
    println!("==========================");
    println!("1. Solve Maze");
    println!("2. Start New Training");
    println!("3. Resume Training");
    println!("4. View Training Statistics");
    println!("0. Exit");
    println!();
}

fn read_menu_choice() -> io::Result<u32> {
    print!("Select an option: ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().parse::<u32>().unwrap_or(u32::MAX))
}

fn solve_maze() {
    println!();
    println!("=== Solve Maze ===");

    let maze = match load(MAZE_PATH) {
        Ok(maze) => maze,
        Err(error) => {
            eprintln!("Failed to load maze from {MAZE_PATH}: {error}");
            eprintln!("Generate a maze with Daedalus before running this option.");
            return;
        }
    };

    println!("Loaded maze: {}x{}", maze.width, maze.height);
    println!();

    benchmark(&maze);
}

fn pause() {
    println!();
    print!("Press Enter to return to the main menu...");

    if let Err(error) = io::stdout().flush() {
        eprintln!("Failed to display pause prompt: {error}");
        return;
    }

    let mut input = String::new();

    if let Err(error) = io::stdin().read_line(&mut input) {
        eprintln!("Failed to read input: {error}");
    }
}