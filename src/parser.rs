// parse file
// to fill the grid

use crate::error::check_number;

pub fn parser_file(content: &str, character: Option<char> ) -> Result<[[u32; 9]; 9], String> {
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
            return Err("Grid is too big (more than 9 lines or columns)".to_string());
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
            return Err("There is a character who's not a number ".to_string());
        }
        let value = c
            .to_digit(10)
            .ok_or(format!("Impossible conversion '{}' in number", c))?;
        grid[x][y] = value;
        y += 1;
    }
    Ok(grid)
}

pub fn parser_ori(content: &str, character: Option<char> ) -> Result<[[bool; 9]; 9], String> {
    let mut original: [[bool; 9]; 9] = [[false; 9]; 9];
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
            return Err("Grid is too big (more than 9 lines or columns)".to_string());
        }
        if c == ' ' {
            continue
        }
        if c == empty {
            y += 1;
            continue
        }
        if !c.is_ascii_digit() {
            return Err("There is a character who's not a number ".to_string());
        }
        c.to_digit(10)
            .ok_or(format!("Impossible conversion '{}' in number", c))?;
        original[x][y] = true;
        y += 1;
    }
    Ok(original)
}