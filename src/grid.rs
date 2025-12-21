// Grid of the Sodoku

pub struct Grid {
    pub grid: [[i32; 9]; 9],
}

impl Grid {
    pub fn set_grid(&mut self, grid: [[i32 ; 9]; 9]) {
        self.grid = grid;
    }

    pub fn get_grid(&self) -> [[i32 ; 9]; 9] {
        self.grid
    }

    pub fn add_to_grid(&mut self, x: usize, y: usize, value: i32) {
        // TO-DO check if it in the size of the grid
        self.grid[x][y] = value;
    }
}