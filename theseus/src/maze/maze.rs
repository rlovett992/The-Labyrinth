use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Maze {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<Cell>>
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Cell {
    pub north: bool,
    pub east: bool,
    pub south: bool,
    pub west: bool,
    pub visited: bool
}
impl Maze {
    pub fn neighbors(&self, position: (usize, usize)) -> Vec<(usize, usize)> {
        let (x, y) = position;
        let cell = &self.cells[y][x];

        let mut neighbors = Vec::new();

        if !cell.north && y > 0 {
            neighbors.push((x, y - 1));
        }

        if !cell.east && x + 1 < self.width {
            neighbors.push((x + 1, y));
        }

        if !cell.south && y + 1 < self.height {
            neighbors.push((x, y + 1));
        }

        if !cell.west && x > 0 {
            neighbors.push((x - 1, y));
        }

        neighbors
    }
}