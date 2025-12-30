// Grid of the Sodoku

pub struct Grid {
    pub grid: [[u32; 9]; 9],
    pub original: [[bool; 9]; 9],
}

impl Grid {
    pub fn set_grid(&mut self, grid: [[u32 ; 9]; 9]) {
        self.grid = grid;
    }
    pub fn set_grid_ori(&mut self, ori: [[bool ; 9]; 9]) {
        self.original = ori;
    }

    pub fn get_grid(&self) -> [[u32 ; 9]; 9] {
        self.grid
    }

    pub fn add_to_grid(&mut self, x: usize, y: usize, value: u32) {
        // TO-DO check if it in the size of the grid
        self.grid[x][y] = value;
    }
}