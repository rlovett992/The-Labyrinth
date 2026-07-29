use std::fs;
use std::io;
use std::path::Path;

use crate::maze::maze::Maze;
use crate::solver::solver::{Position, SearchStep};

const CELL_SIZE: usize = 20;
const WALL_STROKE_WIDTH: usize = 2;
const TRACE_STROKE_WIDTH: usize = 3;
const SOLUTION_STROKE_WIDTH: usize = 4;

const THESEUS_COLOR: &str = "royalblue";
const BFS_COLOR: &str = "orange";
const DFS_COLOR: &str = "purple";
const RANDOM_COLOR: &str = "green";
const ASTAR_COLOR: &str = "darkcyan";
const SOLUTION_COLOR: &str = "red";

/// Draws the non-solution portions of one solver's search trace.
fn draw_false_paths(svg: &mut String, trace: &[SearchStep], color: &str) {
    svg.push_str(&format!(
        r#"<g stroke="{color}"
        stroke-width="{TRACE_STROKE_WIDTH}"
        fill="none"
        stroke-linecap="round"
        stroke-linejoin="round"
        opacity="0.75">"#
    ));

    for step in trace {
        if step.on_solution_path {
            continue;
        }

        let from_x = step.from.0 * CELL_SIZE + CELL_SIZE / 2;
        let from_y = step.from.1 * CELL_SIZE + CELL_SIZE / 2;
        let to_x = step.to.0 * CELL_SIZE + CELL_SIZE / 2;
        let to_y = step.to.1 * CELL_SIZE + CELL_SIZE / 2;

        svg.push_str(&format!(
            r#"<line x1="{from_x}" y1="{from_y}" x2="{to_x}" y2="{to_y}"/>"#
        ));
    }

    svg.push_str("</g>");
}

/// Draws the final solution path.
fn draw_solution_path(svg: &mut String, path: &[Position]) {
    if path.is_empty() {
        return;
    }

    svg.push_str(r#"<polyline points=""#);

    for &(x, y) in path {
        let center_x = x * CELL_SIZE + CELL_SIZE / 2;
        let center_y = y * CELL_SIZE + CELL_SIZE / 2;

        svg.push_str(&format!("{center_x},{center_y} "));
    }

    svg.push_str(&format!(
        r#"" fill="none"
        stroke="{SOLUTION_COLOR}"
        stroke-width="{SOLUTION_STROKE_WIDTH}"
        stroke-linecap="round"
        stroke-linejoin="round"/>"#
    ));
}

/// Draws all maze walls.
fn draw_maze_walls(svg: &mut String, maze: &Maze) {
    svg.push_str(&format!(
        r#"<g stroke="black"
        stroke-width="{WALL_STROKE_WIDTH}"
        fill="none"
        stroke-linecap="square">"#
    ));

    for y in 0..maze.height {
        for x in 0..maze.width {
            let cell = &maze.cells[y][x];

            let x1 = x * CELL_SIZE;
            let y1 = y * CELL_SIZE;
            let x2 = x1 + CELL_SIZE;
            let y2 = y1 + CELL_SIZE;

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
}

/// Adds a color legend whose size scales with the SVG dimensions.
///
/// The legend uses percentages of the rendered maze size so it remains
/// visible when large maze SVGs are scaled down for viewing.
fn draw_legend(svg: &mut String, svg_width: usize, svg_height: usize) {
    let entries = [
        ("Solution", SOLUTION_COLOR),
        ("Theseus", THESEUS_COLOR),
        ("BFS", BFS_COLOR),
        ("DFS", DFS_COLOR),
        ("Random", RANDOM_COLOR),
    ];

    let width = svg_width as f64;
    let height = svg_height as f64;
    let base = width.min(height);

    // Percentage-based sizing with minimums for smaller mazes.
    let margin = (base * 0.0125).max(8.0);
    let legend_width = (width * 0.18).max(150.0);
    let padding = (base * 0.0125).max(10.0);
    let row_height = (base * 0.035).max(22.0);
    let font_size = (base * 0.0225).max(14.0);
    let sample_length = (legend_width * 0.20).max(30.0);
    let sample_stroke_width = (base * 0.006).max(4.0);
    let border_stroke_width = (base * 0.0015).max(1.0);

    let legend_height = padding * 2.0 + row_height * entries.len() as f64;

    svg.push_str(&format!(
        r#"<g>
        <rect
            x="{margin:.2}"
            y="{margin:.2}"
            width="{legend_width:.2}"
            height="{legend_height:.2}"
            rx="{:.2}"
            ry="{:.2}"
            fill="white"
            stroke="black"
            stroke-width="{border_stroke_width:.2}"
            opacity="0.9"
        />"#,
        padding * 0.4,
        padding * 0.4
    ));

    let line_x1 = margin + padding;
    let line_x2 = line_x1 + sample_length;
    let text_x = line_x2 + padding;
    let first_row_center = margin + padding + row_height / 2.0;

    for (index, (label, color)) in entries.iter().enumerate() {
        let y = first_row_center + index as f64 * row_height;
        let text_y = y + font_size * 0.35;

        svg.push_str(&format!(
            r#"<line
                x1="{line_x1:.2}"
                y1="{y:.2}"
                x2="{line_x2:.2}"
                y2="{y:.2}"
                stroke="{color}"
                stroke-width="{sample_stroke_width:.2}"
                stroke-linecap="round"
            />
            <text
                x="{text_x:.2}"
                y="{text_y:.2}"
                font-family="Arial, sans-serif"
                font-size="{font_size:.2}"
                fill="black"
            >{label}</text>"#
        ));
    }

    svg.push_str("</g>");
}

/// Creates the parent directory for an SVG output path when needed.
fn prepare_output_directory(output_path: &str) -> io::Result<()> {
    if let Some(parent) = Path::new(output_path).parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }

    Ok(())
}

/// Shared renderer for a single solver.
fn export_solution_svg(
    maze: &Maze,
    trace: &[SearchStep],
    path: &[Position],
    false_path_color: &str,
    output_path: &str,
) -> io::Result<()> {
    prepare_output_directory(output_path)?;

    let svg_width = maze.width * CELL_SIZE;
    let svg_height = maze.height * CELL_SIZE;

    let mut svg = String::new();

    svg.push_str(&format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg"
        width="{svg_width}"
        height="{svg_height}"
        viewBox="0 0 {svg_width} {svg_height}">"#
    ));

    svg.push_str(r#"<rect width="100%" height="100%" fill="white"/>"#);

    // Draw false paths first so the solution remains visible above them.
    draw_false_paths(&mut svg, trace, false_path_color);

    // Draw the final solution above the false paths.
    draw_solution_path(&mut svg, path);

    // Draw walls above all route lines.
    draw_maze_walls(&mut svg, maze);

    svg.push_str("</svg>");

    fs::write(output_path, svg)
}

/// Exports one comparison SVG containing Theseus, BFS, DFS, and Random traces.
///
/// This is intended for the five rotating training checkpoints.
pub fn export_checkpoint_comparison_svg(
    maze: &Maze,
    theseus_trace: &[SearchStep],
    bfs_trace: &[SearchStep],
    dfs_trace: &[SearchStep],
    random_trace: &[SearchStep],
    solution_path: &[Position],
    output_path: &str,
) -> io::Result<()> {
    prepare_output_directory(output_path)?;

    let svg_width = maze.width * CELL_SIZE;
    let svg_height = maze.height * CELL_SIZE;

    let mut svg = String::new();

    svg.push_str(&format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg"
        width="{svg_width}"
        height="{svg_height}"
        viewBox="0 0 {svg_width} {svg_height}">"#
    ));

    svg.push_str(r#"<rect width="100%" height="100%" fill="white"/>"#);

    // Comparison traces are drawn before the final solution.
    // The order keeps Theseus more visible than the comparison solvers.
    draw_false_paths(&mut svg, random_trace, RANDOM_COLOR);
    draw_false_paths(&mut svg, dfs_trace, DFS_COLOR);
    draw_false_paths(&mut svg, bfs_trace, BFS_COLOR);
    draw_false_paths(&mut svg, theseus_trace, THESEUS_COLOR);

    // Every solver reaches the same unique maze solution.
    draw_solution_path(&mut svg, solution_path);

    // Keep maze walls clearly visible.
    draw_maze_walls(&mut svg, maze);

    // Explain the trace colors.
    draw_legend(&mut svg, svg_width, svg_height);

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
        BFS_COLOR,
        "output/theseus/solved_maze_bfs.svg",
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
        DFS_COLOR,
        "output/theseus/solved_maze_dfs.svg",
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
        ASTAR_COLOR,
        "output/theseus/solved_maze_astar.svg",
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
        RANDOM_COLOR,
        "output/theseus/solved_maze_random.svg",
    )
}