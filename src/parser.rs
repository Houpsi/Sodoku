// parse file
// to fill the grid

use crate::error::check_number;

pub fn parser_file(content: &str, character: Option<char> ) -> [[u32; 9]; 9] {
    let mut grid: [[u32; 9]; 9] = [[0; 9]; 9];
    let empty = character.unwrap_or('.');
    let mut x:usize = 0;
    let mut y: usize = 0;
    for c in content.chars() {
        if c == '\n' {
            x += 1;
            y = 0;
            continue
        }
        if !check_number(c as u32) {
            panic!("Invalid number: {}", c)
        }
        if c == ' ' {
            continue
        }
        if c == empty {
            grid[x][y] = 0;
            y += 1;
            continue
        }
        if !c.is_ascii_digit() {
            panic!("{} is not a number", c)
        }
        grid[x][y] = c.to_digit(10).expect("Invalid digit") as u32;
        y += 1;
    }
    return grid;
}