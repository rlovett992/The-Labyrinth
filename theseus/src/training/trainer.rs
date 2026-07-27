use std::fs;
use std::io::{self, Write};
use std::time::{Duration, Instant};

use daedalus::config::Difficulty;
use daedalus::generator::generate_maze;
use rand::Rng;

use crate::maze::loader::load;
use crate::maze::maze::Maze;

use super::checkpoint::{
    TrainingCheckpoint,
    clear_checkpoints,
    current_unix_seconds,
    load_all_checkpoints,
    load_newest_checkpoint,
    save_checkpoint,
};
use super::example::create_training_examples;
use super::model::{
    TheseusModel,
    TrainingResult,
};
use super::teacher::select_teacher;

const MAZE_PATH: &str = "output/maze.json";

const MAX_CONSECUTIVE_FAILURES: usize = 10;

#[derive(Debug, Clone, Copy)]
enum TrainingLimit {
    Mazes(u64),
    Duration(Duration),
}

struct ProcessedMaze {
    checkpoint: TrainingCheckpoint,
    training_result: TrainingResult,
    difficulty: Difficulty,
}

pub fn start_new_training() {
    println!();
    println!("=== Start New Training ===");

    let existing_checkpoints = checkpoints_exist();

    if existing_checkpoints {
        println!(
            "Starting new training will remove the existing checkpoints."
        );

        if !confirm("Continue? (y/n): ") {
            println!("New training cancelled.");
            return;
        }
    }

    let Some(limit) = prompt_for_training_limit() else {
        println!("New training cancelled.");
        return;
    };

    if existing_checkpoints {
        if let Err(error) = clear_checkpoints() {
            eprintln!(
                "Failed to clear existing checkpoints: {error}"
            );
            return;
        }
    }

    let model = TheseusModel::new();

    println!();
    println!("Created a new Theseus model.");
    println!(
        "Learning rate: {}",
        model.learning_rate()
    );

    run_training_session(0, limit, model);
}

pub fn resume_training() {
    println!();
    println!("=== Resume Training ===");

    let checkpoint = match load_newest_checkpoint() {
        Ok(Some(checkpoint)) => checkpoint,
        Ok(None) => {
            println!("No compatible checkpoint was found.");
            println!("Use Start New Training first.");
            return;
        }
        Err(error) => {
            eprintln!(
                "Failed to load checkpoints: {error}"
            );
            return;
        }
    };

    println!(
        "Loaded checkpoint after {} completed maze(s).",
        checkpoint.mazes_completed
    );
    println!(
        "Examples already trained: {}",
        checkpoint.total_examples_trained
    );
    println!(
        "Latest training loss:     {:.6}",
        checkpoint.latest_training_loss
    );
    println!(
        "Latest accuracy:          {:.2}%",
        checkpoint.latest_training_accuracy * 100.0
    );

    let Some(limit) = prompt_for_training_limit() else {
        println!("Resume training cancelled.");
        return;
    };

    let previously_completed =
        checkpoint.mazes_completed;

    let model = checkpoint.model;

    run_training_session(
        previously_completed,
        limit,
        model,
    );
}

pub fn view_training_statistics() {
    println!();
    println!("=== Training Statistics ===");

    let mut checkpoints =
        match load_all_checkpoints() {
            Ok(checkpoints) => checkpoints,
            Err(error) => {
                eprintln!(
                    "Failed to read training checkpoints: \
                     {error}"
                );
                return;
            }
        };

    if checkpoints.is_empty() {
        println!(
            "No compatible training checkpoints are available."
        );
        return;
    }

    checkpoints.sort_by_key(
        |checkpoint| checkpoint.mazes_completed,
    );

    let newest = checkpoints
        .last()
        .expect("Checkpoint list should not be empty");

    println!(
        "Stored checkpoints:     {}",
        checkpoints.len()
    );
    println!(
        "Mazes completed:        {}",
        newest.mazes_completed
    );
    println!(
        "Examples trained:       {}",
        newest.total_examples_trained
    );
    println!(
        "Latest training loss:   {:.6}",
        newest.latest_training_loss
    );
    println!(
        "Latest accuracy:        {:.2}%",
        newest.latest_training_accuracy * 100.0
    );
    println!(
        "Model learning rate:    {}",
        newest.model.learning_rate()
    );
    println!(
        "Latest teacher:         {}",
        newest.latest_teacher
    );
    println!(
        "Teacher explored:       {} nodes",
        newest.teacher_nodes_explored
    );
    println!(
        "Teacher time:           {} ns",
        newest.teacher_duration_nanos
    );
    println!(
        "Latest maze:            {}x{}",
        newest.maze_width,
        newest.maze_height
    );

    println!();
    println!("Checkpoint history:");

    for checkpoint in checkpoints {
        println!(
            "  Maze {:>5}: {:<8} | explored {:>8} | \
             loss {:>9.6} | accuracy {:>6.2}% | {}x{}",
            checkpoint.mazes_completed,
            checkpoint.latest_teacher,
            checkpoint.teacher_nodes_explored,
            checkpoint.latest_training_loss,
            checkpoint.latest_training_accuracy * 100.0,
            checkpoint.maze_width,
            checkpoint.maze_height
        );
    }
}

fn prompt_for_training_limit() -> Option<TrainingLimit> {
    loop {
        println!();
        println!("Choose a training limit:");
        println!("1. Number of mazes");
        println!("2. Length of time in hours");
        println!("0. Cancel");
        println!();

        let selection =
            read_input("Selection: ")?;

        match selection.trim() {
            "1" => {
                return prompt_for_maze_count();
            }
            "2" => {
                return prompt_for_training_hours();
            }
            "0" => {
                return None;
            }
            _ => {
                println!("Invalid selection.");
                println!("Enter 1, 2, or 0.");
            }
        }
    }
}

fn prompt_for_maze_count() -> Option<TrainingLimit> {
    loop {
        println!();

        let input = read_input(
            "How many mazes should Theseus train on? ",
        )?;

        let trimmed = input.trim();

        if trimmed.eq_ignore_ascii_case("cancel")
            || trimmed == "0"
        {
            return None;
        }

        match trimmed.parse::<u64>() {
            Ok(count) if count > 0 => {
                return Some(
                    TrainingLimit::Mazes(count),
                );
            }
            _ => {
                println!(
                    "Enter a whole number greater than zero."
                );
                println!(
                    "Enter 0 or \"cancel\" to cancel."
                );
            }
        }
    }
}

fn prompt_for_training_hours() -> Option<TrainingLimit> {
    loop {
        println!();

        let input = read_input(
            "How many hours should Theseus train? ",
        )?;

        let trimmed = input.trim();

        if trimmed.eq_ignore_ascii_case("cancel")
            || trimmed == "0"
        {
            return None;
        }

        let hours = match trimmed.parse::<f64>() {
            Ok(hours) => hours,
            Err(_) => {
                println!(
                    "Enter a number greater than zero."
                );
                println!(
                    "Decimals such as 0.5 are allowed."
                );
                continue;
            }
        };

        if !hours.is_finite() || hours <= 0.0 {
            println!(
                "Enter a finite number greater than zero."
            );
            continue;
        }

        let seconds = hours * 60.0 * 60.0;

        if !seconds.is_finite()
            || seconds > u64::MAX as f64
        {
            println!(
                "The requested duration is too large."
            );
            continue;
        }

        let duration =
            Duration::from_secs_f64(seconds);

        if duration.is_zero() {
            println!(
                "The requested duration is too short."
            );
            continue;
        }

        return Some(
            TrainingLimit::Duration(duration),
        );
    }
}

fn run_training_session(
    previously_completed: u128,
    limit: TrainingLimit,
    mut model: TheseusModel,
) {
    println!();
    println!("=== Training Session Started ===");

    match limit {
        TrainingLimit::Mazes(target) => {
            println!(
                "Session target: {target} maze(s)"
            );
        }
        TrainingLimit::Duration(duration) => {
            println!(
                "Session duration: {}",
                format_duration(duration)
            );
        }
    }

    println!(
        "Previous mazes completed: {previously_completed}"
    );
    println!(
        "Previous examples trained: {}",
        model.examples_trained()
    );
    println!();

    let session_started = Instant::now();

    let mut session_mazes_completed = 0_u64;
    let mut total_mazes_completed =
        previously_completed;
    let mut consecutive_failures = 0_usize;

    while !training_limit_reached(
        limit,
        session_mazes_completed,
        session_started,
    ) {
        let next_maze_number =
            total_mazes_completed + 1;

        match process_one_maze(
            next_maze_number,
            &mut model,
        ) {
            Ok(processed_maze) => {
                match save_checkpoint(
                    &processed_maze.checkpoint,
                ) {
                    Ok(_) => {
                        session_mazes_completed += 1;

                        total_mazes_completed =
                            processed_maze
                                .checkpoint
                                .mazes_completed;

                        consecutive_failures = 0;

                        print_training_progress(
                            limit,
                            session_mazes_completed,
                            session_started,
                            &processed_maze,
                        );
                    }
                    Err(error) => {
                        consecutive_failures += 1;

                        eprintln!(
                            "Failed to save checkpoint for maze \
                             {next_maze_number}: {error}"
                        );
                    }
                }
            }
            Err(error) => {
                consecutive_failures += 1;

                eprintln!(
                    "Training maze failed: {error}"
                );
            }
        }

        if consecutive_failures
            >= MAX_CONSECUTIVE_FAILURES
        {
            eprintln!();
            eprintln!(
                "Training stopped after \
                 {MAX_CONSECUTIVE_FAILURES} \
                 consecutive failures."
            );

            break;
        }
    }

    let elapsed = session_started.elapsed();

    println!();
    println!("=== Training Session Finished ===");
    println!(
        "Session mazes completed: {}",
        session_mazes_completed
    );
    println!(
        "Total mazes completed:   {}",
        total_mazes_completed
    );
    println!(
        "Total examples trained:  {}",
        model.examples_trained()
    );
    println!(
        "Session elapsed time:    {}",
        format_duration(elapsed)
    );

    match limit {
        TrainingLimit::Mazes(target)
            if session_mazes_completed >= target =>
        {
            println!(
                "Stop reason:            Maze limit reached"
            );
        }
        TrainingLimit::Duration(duration)
            if elapsed >= duration =>
        {
            println!(
                "Stop reason:            Time limit reached"
            );
        }
        _ => {
            println!(
                "Stop reason:            Training failure"
            );
        }
    }
}

fn process_one_maze(
    maze_number: u128,
    model: &mut TheseusModel,
) -> Result<ProcessedMaze, String> {
    let (maze, difficulty) =
        generate_training_maze()
            .map_err(|error| {
                format!(
                    "failed to generate maze \
                     {maze_number}: {error}"
                )
            })?;

    let teacher =
        select_teacher(&maze).ok_or_else(|| {
            format!(
                "no solver successfully solved maze \
                 {maze_number}"
            )
        })?;

    let training_examples =
        create_training_examples(
            &maze,
            &teacher.path,
        )
        .map_err(|error| {
            format!(
                "failed to create training examples for \
                 maze {maze_number}: {error}"
            )
        })?;

    let training_result =
        model.train(&training_examples);

    if !training_result.average_loss.is_finite() {
        return Err(format!(
            "model produced a non-finite loss on maze \
             {maze_number}"
        ));
    }

    let checkpoint = TrainingCheckpoint {
        mazes_completed: maze_number,

        model: model.clone(),
        total_examples_trained:
            model.examples_trained(),
        latest_training_loss:
            training_result.average_loss,
        latest_training_accuracy:
            training_result.accuracy,

        latest_teacher: teacher.algorithm,
        teacher_nodes_explored:
            teacher.nodes_explored,
        teacher_duration_nanos:
            teacher.duration_nanos,
        teacher_path_length:
            teacher.path.len(),

        maze_width: maze.width,
        maze_height: maze.height,

        saved_at_unix_seconds:
            current_unix_seconds(),
    };

    Ok(ProcessedMaze {
        checkpoint,
        training_result,
        difficulty,
    })
}

fn training_limit_reached(
    limit: TrainingLimit,
    session_mazes_completed: u64,
    session_started: Instant,
) -> bool {
    match limit {
        TrainingLimit::Mazes(target) => {
            session_mazes_completed >= target
        }
        TrainingLimit::Duration(duration) => {
            session_started.elapsed() >= duration
        }
    }
}

fn print_training_progress(
    limit: TrainingLimit,
    session_mazes_completed: u64,
    session_started: Instant,
    processed_maze: &ProcessedMaze,
) {
    let checkpoint =
        &processed_maze.checkpoint;

    let result =
        processed_maze.training_result;

    let difficulty =
        processed_maze.difficulty.label();

    let elapsed =
        session_started.elapsed();

    match limit {
        TrainingLimit::Mazes(target) => {
            println!(
                "Maze {}/{} | Total {} | {:<13} | \
                 Teacher: {} | Nodes: {} | \
                 Examples: {} | Correct: {}/{} | \
                 Loss: {:.6} | Accuracy: {:.2}% | \
                 Elapsed: {}",
                session_mazes_completed,
                target,
                checkpoint.mazes_completed,
                difficulty,
                checkpoint.latest_teacher,
                checkpoint.teacher_nodes_explored,
                result.example_count,
                result.correct_predictions,
                result.example_count,
                result.average_loss,
                result.accuracy * 100.0,
                format_duration(elapsed)
            );
        }

        TrainingLimit::Duration(duration) => {
            let remaining =
                duration.saturating_sub(elapsed);

            println!(
                "Maze {} | Total {} | {:<13} | \
                 Teacher: {} | Nodes: {} | \
                 Examples: {} | Correct: {}/{} | \
                 Loss: {:.6} | Accuracy: {:.2}% | \
                 Elapsed: {} | Remaining: {}",
                session_mazes_completed,
                checkpoint.mazes_completed,
                difficulty,
                checkpoint.latest_teacher,
                checkpoint.teacher_nodes_explored,
                result.example_count,
                result.correct_predictions,
                result.example_count,
                result.average_loss,
                result.accuracy * 100.0,
                format_duration(elapsed),
                format_duration(remaining)
            );
        }
    }
}

fn generate_training_maze(
) -> io::Result<(Maze, Difficulty)> {
    let difficulty =
        random_training_difficulty();

    let width =
        difficulty.random_size();

    let height =
        difficulty.random_size();

    let generated_maze =
        generate_maze(width, height);

    let json =
        serde_json::to_string_pretty(
            &generated_maze,
        )
        .map_err(io::Error::other)?;

    fs::create_dir_all("output")?;
    fs::write(MAZE_PATH, json)?;

    let maze = load(MAZE_PATH)?;

    Ok((maze, difficulty))
}

fn random_training_difficulty() -> Difficulty {
    let mut rng = rand::rng();

    let roll =
        rng.random_range(0..100);

    match roll {
        0..10 => Difficulty::Easy,
        10..30 => Difficulty::Medium,
        30..60 => Difficulty::Hard,
        _ => Difficulty::Labyrinthian,
    }
}

fn checkpoints_exist() -> bool {
    load_all_checkpoints()
        .map(|checkpoints| {
            !checkpoints.is_empty()
        })
        .unwrap_or(false)
}

fn confirm(prompt: &str) -> bool {
    let Some(input) = read_input(prompt) else {
        return false;
    };

    matches!(
        input.trim().to_lowercase().as_str(),
        "y" | "yes"
    )
}

fn read_input(
    prompt: &str,
) -> Option<String> {
    print!("{prompt}");

    if let Err(error) =
        io::stdout().flush()
    {
        eprintln!(
            "Failed to display prompt: {error}"
        );
        return None;
    }

    let mut input = String::new();

    if let Err(error) =
        io::stdin().read_line(&mut input)
    {
        eprintln!(
            "Failed to read input: {error}"
        );
        return None;
    }

    Some(input)
}

fn format_duration(
    duration: Duration,
) -> String {
    let total_seconds =
        duration.as_secs();

    let hours =
        total_seconds / 3_600;

    let minutes =
        (total_seconds % 3_600) / 60;

    let seconds =
        total_seconds % 60;

    format!(
        "{hours:02}:{minutes:02}:{seconds:02}"
    )
}