use crate::grid::Grid;

mod solver;
mod grid;
mod parser;
mod error;
mod display;
mod button;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("filepath")
    }
    let contents = fs::read_to_string(args[1].clone())
        .expect("Should have been able to read the file");
    let mut my_grid = Grid {
        grid: parser::parser_file(&contents, Some('.')),
    };
    solver::is_valid(&mut my_grid, 0);
    display::display_grid(my_grid.get_grid());

    display::init_window(&my_grid)

}
