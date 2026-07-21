use std::fs;
use std::io;

use crate::maze::maze::Maze;
use crate::solver::solver::{Position, SearchStep};

/// Shared renderer used by all algorithms.
fn export_solution_svg(
    maze: &Maze,
    trace: &[SearchStep],
    path: &[Position],
    output_path: &str,
) -> io::Result<()> {
    let cell_size = 20;
    let stroke_width = 2;

    let svg_width = maze.width * cell_size;
    let svg_height = maze.height * cell_size;

    let mut svg = String::new();

    svg.push_str(&format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{svg_width}" height="{svg_height}" viewBox="0 0 {svg_width} {svg_height}">"#
    ));

    svg.push_str(r#"<rect width="100%" height="100%" fill="white"/>"#);

    // ---------- Explored False Paths ----------
    //
    // Draw these first so the red solution path remains visible on top.
    svg.push_str(
        r#"<g stroke="royalblue"
        stroke-width="4"
        fill="none"
        stroke-linecap="round"
        stroke-linejoin="round">"#,
    );

    for step in trace {
        if step.on_solution_path {
            continue;
        }

        let from_x = step.from.0 * cell_size + cell_size / 2;
        let from_y = step.from.1 * cell_size + cell_size / 2;
        let to_x = step.to.0 * cell_size + cell_size / 2;
        let to_y = step.to.1 * cell_size + cell_size / 2;

        svg.push_str(&format!(
            r#"<line x1="{from_x}" y1="{from_y}" x2="{to_x}" y2="{to_y}"/>"#
        ));
    }

    svg.push_str("</g>");

    // ---------- Final Solution Path ----------
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
pub fn export_solution_svg_bfs(
    maze: &Maze,
    trace: &[SearchStep],
    path: &[Position],
) -> io::Result<()> {
    export_solution_svg(
        maze,
        trace,
        path,
        "output/solved_maze_bfs.svg",
    )
}

/// Export a DFS solution.
pub fn export_solution_svg_dfs(
    maze: &Maze,
    trace: &[SearchStep],
    path: &[Position],
) -> io::Result<()> {
    export_solution_svg(
        maze,
        trace,
        path,
        "output/solved_maze_dfs.svg",
    )
}

/// Export an A* solution.
pub fn export_solution_svg_astar(
    maze: &Maze,
    trace: &[SearchStep],
    path: &[Position],
) -> io::Result<()> {
    export_solution_svg(
        maze,
        trace,
        path,
        "output/solved_maze_astar.svg",
    )
}

/// Export a Random solution.
pub fn export_solution_svg_random(
    maze: &Maze,
    trace: &[SearchStep],
    path: &[Position],
) -> io::Result<()> {
    export_solution_svg(
        maze,
        trace,
        path,
        "output/solved_maze_random.svg",
    )
}