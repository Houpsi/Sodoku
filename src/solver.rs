// solver sudoku
// used in line, column, cell
// backtracking

use crate::grid::Grid;

pub fn is_valid(grid: &mut Grid, pos: usize) -> bool  {
    let grid_ = grid.get_grid();
    if pos == 9 * 9 {
        return true;
    }
    let x = pos / 9;
    let y = pos % 9;
    if grid_[x][y] != 0 {
        return is_valid(grid, pos + 1)
    }
    for k in 1..=9 {
        if (!exist_line(&grid_, x, k) && !exist_column(&grid_, y, k) && !exist_cell(&grid_, x, y, k)) {
            grid.add_to_grid(x, y, k);
            if is_valid(grid, pos + 1) {
                return true;
            }
            grid.add_to_grid(x, y, 0);
        }
    }
    // grid[x][y] = 0;
    false
}

pub fn exist_line(grid: &[[u32; 9]; 9], line: usize, num: u32) -> bool {
    for x in 0..9 {
        if grid[line][x] == num {
            return true;
        }
    }
    false
}

pub fn exist_column(grid: &[[u32; 9]; 9], column: usize, num: u32) -> bool {
    for x in 0..9 {
        if grid[x][column] == num {
            return true;
        }
    }
    false
}

pub fn exist_cell(grid: &[[u32; 9]; 9], x: usize, y : usize, num: u32) -> bool {
    let i = x - (x % 3);
    let j = y - (y % 3);
    for x in i..(i + 3) {
       for y in j..(j + 3) {
           if grid[x][y] == num {
               return true;
           }
       }
    }
    false
}
