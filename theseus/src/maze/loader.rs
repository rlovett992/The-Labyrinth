use std::fs;
use std::io;

use super::maze::Maze;

pub fn load(path: &str) -> io::Result<Maze> {
    let json = fs::read_to_string(path)?;

    let maze: Maze = serde_json::from_str(&json).expect("Failed to parse maze JSON");

    Ok(maze)
}