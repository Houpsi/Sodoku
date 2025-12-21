use crate::grid::Grid;

mod solver;
mod grid;
mod parser;
mod error;
mod display;

fn main() {
    let mut my_grid = Grid {
        grid: [[0; 9]; 9],
    };

    my_grid.add_to_grid(0,0, 5);

    display::display_grid(my_grid.get_grid());
}
