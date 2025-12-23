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
    display::init_window()

}
