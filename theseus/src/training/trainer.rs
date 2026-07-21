use std::io::{self, Write};

use crate::maze::loader::load;

use super::checkpoint::{
    TrainingCheckpoint,
    clear_checkpoints,
    current_unix_seconds,
    load_all_checkpoints,
    load_newest_checkpoint,
    save_checkpoint,
};
use super::teacher::select_teacher;

const MAZE_PATH: &str = "output/maze.json";

pub fn start_new_training() {
    println!();
    println!("=== Start New Training ===");

    if checkpoints_exist() {
        println!("Starting new training will remove the existing checkpoints.");

        if !confirm("Continue? (y/n): ") {
            println!("New training cancelled.");
            return;
        }

        if let Err(error) = clear_checkpoints() {
            eprintln!("Failed to clear existing checkpoints: {error}");
            return;
        }
    }

    train_one_maze(0);
}

pub fn resume_training() {
    println!();
    println!("=== Resume Training ===");

    let checkpoint = match load_newest_checkpoint() {
        Ok(Some(checkpoint)) => checkpoint,
        Ok(None) => {
            println!("No checkpoint was found.");
            println!("Use Start New Training first.");
            return;
        }
        Err(error) => {
            eprintln!("Failed to load checkpoints: {error}");
            return;
        }
    };

    println!(
        "Loaded checkpoint after {} completed maze(s).",
        checkpoint.mazes_completed
    );

    train_one_maze(checkpoint.mazes_completed);
}

pub fn view_training_statistics() {
    println!();
    println!("=== Training Statistics ===");

    let mut checkpoints = match load_all_checkpoints() {
        Ok(checkpoints) => checkpoints,
        Err(error) => {
            eprintln!("Failed to read training checkpoints: {error}");
            return;
        }
    };

    if checkpoints.is_empty() {
        println!("No training checkpoints are available.");
        return;
    }

    checkpoints.sort_by_key(|checkpoint| checkpoint.mazes_completed);

    let newest = checkpoints
        .last()
        .expect("Checkpoint list should not be empty");

    println!("Stored checkpoints: {}", checkpoints.len());
    println!("Mazes completed:    {}", newest.mazes_completed);
    println!("Latest teacher:     {}", newest.latest_teacher);
    println!(
        "Teacher explored:   {} nodes",
        newest.teacher_nodes_explored
    );
    println!(
        "Teacher time:       {} ns",
        newest.teacher_duration_nanos
    );
    println!(
        "Latest maze:        {}x{}",
        newest.maze_width,
        newest.maze_height
    );

    println!();
    println!("Checkpoint history:");

    for checkpoint in checkpoints {
        println!(
            "  Maze {:>5}: {:<8} | explored {:>8} | {}x{}",
            checkpoint.mazes_completed,
            checkpoint.latest_teacher,
            checkpoint.teacher_nodes_explored,
            checkpoint.maze_width,
            checkpoint.maze_height
        );
    }
}

fn train_one_maze(previously_completed: u128) {
    println!();
    println!("Loading training maze from {MAZE_PATH}...");

    let maze = match load(MAZE_PATH) {
        Ok(maze) => maze,
        Err(error) => {
            eprintln!("Failed to load training maze: {error}");
            eprintln!("Generate a maze with Daedalus first.");
            return;
        }
    };

    println!("Training maze: {}x{}", maze.width, maze.height);
    println!("Running teacher solvers...");

    let teacher = match select_teacher(&maze) {
        Some(teacher) => teacher,
        None => {
            eprintln!("No solver successfully solved the training maze.");
            return;
        }
    };

    println!();
    println!("Selected teacher:  {}", teacher.algorithm);
    println!("Nodes explored:    {}", teacher.nodes_explored);
    println!("Runtime:           {} ns", teacher.duration_nanos);
    println!("Path cells:        {}", teacher.path.len());

    /*
        The actual learning model will be updated here.

        The teacher path is now available as:

            teacher.path

        Each consecutive pair of positions can eventually become a
        training example showing Theseus which direction to take.
    */

    let checkpoint = TrainingCheckpoint {
        mazes_completed: previously_completed + 1,
        latest_teacher: teacher.algorithm,
        teacher_nodes_explored: teacher.nodes_explored,
        teacher_duration_nanos: teacher.duration_nanos,
        teacher_path_length: teacher.path.len(),
        maze_width: maze.width,
        maze_height: maze.height,
        saved_at_unix_seconds: current_unix_seconds(),
    };

    match save_checkpoint(&checkpoint) {
        Ok(path) => {
            println!();
            println!("Checkpoint saved: {}", path.display());
        }
        Err(error) => {
            eprintln!("Failed to save checkpoint: {error}");
        }
    }
}

fn checkpoints_exist() -> bool {
    load_all_checkpoints()
        .map(|checkpoints| !checkpoints.is_empty())
        .unwrap_or(false)
}

fn confirm(prompt: &str) -> bool {
    print!("{prompt}");

    if io::stdout().flush().is_err() {
        return false;
    }

    let mut input = String::new();

    if io::stdin().read_line(&mut input).is_err() {
        return false;
    }

    matches!(input.trim().to_lowercase().as_str(), "y" | "yes")
}