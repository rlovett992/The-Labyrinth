use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use super::model::TheseusModel;

const CHECKPOINT_DIRECTORY: &str = "output/theseus/checkpoints";
const CURRENT_MAZE_PATH: &str = "output/theseus/checkpoints/maze.json";
const CHECKPOINT_SLOTS: usize = 5;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingCheckpoint {
    pub mazes_completed: u128,

    pub model: TheseusModel,
    pub total_examples_trained: u128,
    pub latest_training_loss: f32,
    pub latest_training_accuracy: f32,

    pub theseus_solved: bool,
    pub theseus_nodes_explored: usize,
    pub theseus_duration_nanos: u128,
    pub theseus_path_length: usize,

    pub latest_teacher: String,
    pub teacher_nodes_explored: usize,
    pub teacher_duration_nanos: u128,
    pub teacher_path_length: usize,

    pub maze_width: usize,
    pub maze_height: usize,

    pub saved_at_unix_seconds: u64,
}

pub fn save_checkpoint(checkpoint: &TrainingCheckpoint) -> io::Result<PathBuf> {
    fs::create_dir_all(CHECKPOINT_DIRECTORY)?;

    let destination = choose_checkpoint_slot()?;
    let temporary = destination.with_extension("tmp");

    let json = serde_json::to_string_pretty(checkpoint).map_err(io::Error::other)?;

    fs::write(&temporary, json)?;

    if destination.exists() {
        fs::remove_file(&destination)?;
    }

    fs::rename(&temporary, &destination)?;

    let slot = destination
        .file_stem()
        .and_then(|name| name.to_str())
        .and_then(|name| name.strip_prefix("checkpoint_"))
        .and_then(|number| number.parse::<usize>().ok())
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "Could not determine checkpoint slot",
            )
        })?;

    let maze_destination = checkpoint_maze_path(slot);

    if maze_destination.exists() {
        fs::remove_file(&maze_destination)?;
    }

    fs::copy(CURRENT_MAZE_PATH, &maze_destination)?;

    Ok(destination)
}

pub fn load_newest_checkpoint() -> io::Result<Option<TrainingCheckpoint>> {
    let checkpoints = load_all_checkpoints()?;

    Ok(checkpoints
        .into_iter()
        .max_by_key(|checkpoint| checkpoint.mazes_completed))
}

pub fn load_all_checkpoints() -> io::Result<Vec<TrainingCheckpoint>> {
    let mut checkpoints = Vec::new();

    for slot in 1..=CHECKPOINT_SLOTS {
        let path = checkpoint_path(slot);

        if !path.exists() {
            continue;
        }

        match read_checkpoint(&path) {
            Ok(checkpoint) => {
                checkpoints.push(checkpoint);
            }
            Err(error) => {
                eprintln!(
                    "Warning: could not read checkpoint {}: {error}",
                    path.display()
                );
            }
        }
    }

    Ok(checkpoints)
}

pub fn clear_checkpoints() -> io::Result<()> {
    if !Path::new(CHECKPOINT_DIRECTORY).exists() {
        return Ok(());
    }

    for slot in 1..=CHECKPOINT_SLOTS {
        let checkpoint = checkpoint_path(slot);
        let maze = checkpoint_maze_path(slot);

        if checkpoint.exists() {
            fs::remove_file(checkpoint)?;
        }

        if maze.exists() {
            fs::remove_file(maze)?;
        }
    }

    Ok(())
}
pub fn current_unix_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn choose_checkpoint_slot() -> io::Result<PathBuf> {
    for slot in 1..=CHECKPOINT_SLOTS {
        let path = checkpoint_path(slot);

        if !path.exists() {
            return Ok(path);
        }
    }

    let mut oldest_slot = 1;
    let mut oldest_completed = u128::MAX;

    for slot in 1..=CHECKPOINT_SLOTS {
        let path = checkpoint_path(slot);

        match read_checkpoint(&path) {
            Ok(checkpoint) => {
                if checkpoint.mazes_completed < oldest_completed {
                    oldest_completed = checkpoint.mazes_completed;

                    oldest_slot = slot;
                }
            }
            Err(_) => {
                return Ok(path);
            }
        }
    }

    Ok(checkpoint_path(oldest_slot))
}

fn read_checkpoint(path: &Path) -> io::Result<TrainingCheckpoint> {
    let json = fs::read_to_string(path)?;

    serde_json::from_str(&json).map_err(io::Error::other)
}

fn checkpoint_path(slot: usize) -> PathBuf {
    Path::new(CHECKPOINT_DIRECTORY).join(format!("checkpoint_{slot}.json"))
}

fn checkpoint_maze_path(slot: usize) -> PathBuf {
    Path::new(CHECKPOINT_DIRECTORY).join(format!("checkpoint_{slot}_maze.json"))
}