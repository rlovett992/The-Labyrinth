use std::fs;
use std::io;

use crate::maze::maze::Maze;
use crate::solver::bfs::Position;

/// Shared renderer used by all algorithms.
fn export_solution_svg(maze: &Maze, path: &[Position], output_path: &str) -> io::Result<()> {
    let cell_size = 20;
    let stroke_width = 2;

    let svg_width = maze.width * cell_size;
    let svg_height = maze.height * cell_size;

    let mut svg = String::new();

    svg.push_str(&format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{svg_width}" height="{svg_height}" viewBox="0 0 {svg_width} {svg_height}">"#
    ));

    svg.push_str(r#"<rect width="100%" height="100%" fill="white"/>"#);

    // ---------- Solution Path ----------
    svg.push_str(r#"<polyline points=""#);

    for (x, y) in path {
        let cx = x * cell_size + cell_size / 2;
        let cy = y * cell_size + cell_size / 2;
        svg.push_str(&format!("{cx},{cy} "));
    }

    svg.push_str(
        r#"" fill="none"
        stroke="red"
        stroke-width="4"
        stroke-linecap="round"
        stroke-linejoin="round"/>"#,
    );

    // ---------- Maze Walls ----------
    svg.push_str(&format!(
        r#"<g stroke="black" stroke-width="{stroke_width}" fill="none" stroke-linecap="square">"#
    ));

    for y in 0..maze.height {
        for x in 0..maze.width {
            let cell = &maze.cells[y][x];

            let x1 = x * cell_size;
            let y1 = y * cell_size;
            let x2 = x1 + cell_size;
            let y2 = y1 + cell_size;

            if cell.north {
                svg.push_str(&format!(
                    r#"<line x1="{x1}" y1="{y1}" x2="{x2}" y2="{y1}"/>"#
                ));
            }

            if cell.east {
                svg.push_str(&format!(
                    r#"<line x1="{x2}" y1="{y1}" x2="{x2}" y2="{y2}"/>"#
                ));
            }

            if cell.south {
                svg.push_str(&format!(
                    r#"<line x1="{x1}" y1="{y2}" x2="{x2}" y2="{y2}"/>"#
                ));
            }

            if cell.west {
                svg.push_str(&format!(
                    r#"<line x1="{x1}" y1="{y1}" x2="{x1}" y2="{y2}"/>"#
                ));
            }
        }
    }

    svg.push_str("</g>");
    svg.push_str("</svg>");

    fs::write(output_path, svg)
}

/// Export a BFS solution.
pub fn export_solution_svg_bfs(maze: &Maze, path: &[Position]) -> io::Result<()> {
    export_solution_svg(maze, path, "output/solved_maze_bfs.svg")
}

/// Export a DFS solution.
pub fn export_solution_svg_dfs(maze: &Maze, path: &[Position]) -> io::Result<()> {
    export_solution_svg(maze, path, "output/solved_maze_dfs.svg")
}

pub fn export_solution_svg_astar(maze: &Maze, path: &[Position]) -> io::Result<()> {
    export_solution_svg(maze, path, "output/solved_maze_astar.svg")
}

pub fn export_solution_svg_random(maze: &Maze, path: &[Position]) -> io::Result<()> {
    export_solution_svg(maze, path, "output/solved_maze_random.svg")
}