use std::fs;
use std::io;

use crate::maze::Maze;

pub fn export_json(maze:&Maze, path: &str) -> io::Result<()> {
    let json = serde_json::to_string_pretty(maze).expect("Failed to serialize maze to JSON");

    fs::write(path, json)
}

pub fn export_svg(maze: &Maze, path: &str) -> io::Result<()> {
    let cell_size = 20;
    let stroke_width = 2;

    let svg_width = maze.width * cell_size;
    let svg_height = maze.height * cell_size;

    let mut svg = String::new();

    svg.push_str(&format!(r#"<svg xmlns="http://www.w3.org/2000/svg" width="{svg_width}" height="{svg_height}" viewBox="0 0 {svg_width} {svg_height}">"#));

    svg.push_str(&format!(r#"<rect width="100%" height="100%" fill="white"/>"#));

    svg.push_str(&format!(r#"<g stroke="black" stroke-width="{stroke_width}" fill="none" stroke-linecap="square">"#));

    for y in 0..maze.height {
        for x in 0..maze.width {
            let cell = &maze.cells[y][x];

            let x1 = x * cell_size;
            let y1 = y * cell_size;
            let x2 = x1 + cell_size;
            let y2 = y1 + cell_size;

            if cell.north{
                svg.push_str(&format!(r#"<line x1="{x1}" y1="{y1}" x2="{x2}" y2="{y1}"/>"#));
            }

            if cell.east{
                svg.push_str(&format!(r#"<line x1="{x2}" y1="{y1}" x2="{x2}" y2="{y2}"/>"#));
            }

            if cell.south{
                svg.push_str(&format!(r#"<line x1="{x1}" y1="{y2}" x2="{x2}" y2="{y2}"/>"#));
            }

            if cell.west{
                svg.push_str(&format!(r#"<line x1="{x1}" y1="{y1}" x2="{x1}" y2="{y2}"/>"#));
            }
        }
    }

    svg.push_str("</g>");
    svg.push_str("</svg>");

    fs::write(path, svg)
}