mod benchmark;
mod maze;
mod solution;
mod solver;
mod training;

use std::env;
use std::io::{self, Write};
use std::process::ExitCode;

use benchmark::benchmark;
use maze::loader::load;
use training::trainer::{
    resume_training, resume_training_for_hours, resume_training_for_mazes, start_new_training,
    start_new_training_for_hours, start_new_training_for_mazes, view_training_statistics,
};

const MAZE_PATH: &str = "output/daedalus_maze.json";

fn main() -> ExitCode {
    let arguments: Vec<String> = env::args().skip(1).collect();

    if arguments.is_empty() {
        run_interactive_menu();
        return ExitCode::SUCCESS;
    }

    match run_command(&arguments) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("Theseus command failed: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run_command(arguments: &[String]) -> Result<(), String> {
    match arguments[0].as_str() {
        "train-new" => {
            let limit = parse_training_limit(&arguments[1..])?;

            match limit {
                CommandTrainingLimit::Mazes(mazes) => start_new_training_for_mazes(mazes),
                CommandTrainingLimit::Hours(hours) => start_new_training_for_hours(hours),
            }
        }

        "train-resume" => {
            let limit = parse_training_limit(&arguments[1..])?;

            match limit {
                CommandTrainingLimit::Mazes(mazes) => resume_training_for_mazes(mazes),
                CommandTrainingLimit::Hours(hours) => resume_training_for_hours(hours),
            }
        }

        "training-stats" => {
            view_training_statistics();
            Ok(())
        }

        "solve" => {
            solve_maze();
            Ok(())
        }

        "help" | "--help" | "-h" => {
            print_command_help();
            Ok(())
        }

        command => Err(format!(
            "Unknown command \"{command}\".\n\
             Run `cargo run -p theseus -- help`."
        )),
    }
}

enum CommandTrainingLimit {
    Mazes(u64),
    Hours(f64),
}

fn parse_training_limit(arguments: &[String]) -> Result<CommandTrainingLimit, String> {
    if arguments.len() != 2 {
        return Err("Supply exactly one training limit:\n\
             --mazes <count>\n\
             --hours <hours>"
            .to_string());
    }

    match arguments[0].as_str() {
        "--mazes" => {
            let mazes = arguments[1]
                .parse::<u64>()
                .map_err(|_| "Maze count must be a whole number.".to_string())?;

            if mazes == 0 {
                return Err("Maze count must be greater than zero.".to_string());
            }

            Ok(CommandTrainingLimit::Mazes(mazes))
        }

        "--hours" => {
            let hours = arguments[1]
                .parse::<f64>()
                .map_err(|_| "Hours must be a number.".to_string())?;

            if !hours.is_finite() || hours <= 0.0 {
                return Err("Hours must be a finite number greater than zero.".to_string());
            }

            Ok(CommandTrainingLimit::Hours(hours))
        }

        option => Err(format!(
            "Unknown training limit \"{option}\".\n\
             Use --mazes <count> or --hours <hours>."
        )),
    }
}

fn print_command_help() {
    println!("Theseus commands:");
    println!();
    println!(
        "  cargo run -p theseus -- train-new \
         --mazes <count>"
    );
    println!(
        "  cargo run -p theseus -- train-new \
         --hours <hours>"
    );
    println!(
        "  cargo run -p theseus -- train-resume \
         --mazes <count>"
    );
    println!(
        "  cargo run -p theseus -- train-resume \
         --hours <hours>"
    );
    println!("  cargo run -p theseus -- training-stats");
    println!("  cargo run -p theseus -- solve");
}

fn run_interactive_menu() {
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
                println!(
                    "Invalid selection. \
                     Enter a number from 0 through 4."
                );
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
            eprintln!(
                "Failed to load maze from \
                 {MAZE_PATH}: {error}"
            );
            eprintln!(
                "Generate a maze with Daedalus \
                 before running this option."
            );
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
