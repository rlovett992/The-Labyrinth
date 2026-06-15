use crate::maze::Maze;

pub fn render_ascii(maze: &Maze) {
    for x in 0..maze.width {
        print!("+");
        if maze.cells[0][x].north {
            print!("---");
        } else {
            print!("   ");
        }
    } 
    println!("+");

    for y in 0..maze.height {
        for x in 0..maze.width {
            if maze.cells[y][x].west {
                print!("|");
            } else {
                print!(" ");
            }

            print!("   ");
        }

        if maze.cells[y][maze.width - 1].east {
            println!("|");
        } else {
            println!(" ");
        }

        for x in 0..maze.width {
            print!("+");

            if maze.cells[y][x].south {
                print!("---");
            } else {
                print!("   ");
            }
        }

        println!("+");
    }
}