use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

const CHECKPOINT_DIRECTORY: &str = "output/theseus/checkpoints";
const CHECKPOINT_SLOTS: usize = 5;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingCheckpoint {
    pub mazes_completed: u128,
    pub latest_teacher: String,
    pub teacher_nodes_explored: usize,
    pub teacher_duration_nanos: u128,
    pub teacher_path_length: usize,
    pub maze_width: usize,
    pub maze_height: usize,
    pub saved_at_unix_seconds: u64,
}

pub fn save_checkpoint(
    checkpoint: &TrainingCheckpoint,
) -> io::Result<PathBuf> {
    fs::create_dir_all(CHECKPOINT_DIRECTORY)?;

    let destination = choose_checkpoint_slot()?;
    let temporary = destination.with_extension("tmp");

    let json = serde_json::to_string_pretty(checkpoint)
        .map_err(io::Error::other)?;

    fs::write(&temporary, json)?;

    if destination.exists() {
        fs::remove_file(&destination)?;
    }

    fs::rename(&temporary, &destination)?;

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
            Ok(checkpoint) => checkpoints.push(checkpoint),
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
        let path = checkpoint_path(slot);

        if path.exists() {
            fs::remove_file(path)?;
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
    Path::new(CHECKPOINT_DIRECTORY)
        .join(format!("checkpoint_{slot}.json"))
}