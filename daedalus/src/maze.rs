#[derive(Clone)]
pub struct Cell {
    pub visited: bool,
    pub north: bool,
    pub east: bool,
    pub south: bool, 
    pub west: bool,
}

impl Cell {
    pub fn new() -> Self {
        Self {
            visited: false,
            north: true,
            east: true,
            south: true,
            west: true,
        }
    }
}

pub struct Maze {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<Cell>>,
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Self {
        let mut cells = Vec::new();

        for _ in 0..height {
            let mut row = Vec::new();

            for _ in 0..width {
                row.push(Cell::new());
            }

            cells.push(row);
        }

        Self {
            width,
            height,
            cells,
        }
    }
}