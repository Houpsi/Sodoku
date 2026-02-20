// Grid of the Sodoku

#[derive(Clone)]
pub struct Grid {
    pub grid: [[u32; 9]; 9],
    pub original: [[bool; 9]; 9],
}

impl Grid {
    pub fn get_grid(&self) -> [[u32 ; 9]; 9] {
        self.grid
    }
    pub fn get_grid_ori(&self) -> [[bool ; 9]; 9] {
        self.original
    }

    pub fn add_to_grid(&mut self, x: usize, y: usize, value: u32) {
        // TODO check if it in the size of the grid
        self.grid[x][y] = value;
    }
}