use std::path::PathBuf;
use crate::grid::Grid;

pub struct AppState {
    grid: Grid,
    file_chosen: Option<PathBuf>,
    mouse_pos: [f64; 2],
    click_on_file: bool,
    selected_cell: Option<(usize, usize)>,
    sudoku_counter: usize,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            grid: Grid { grid: [[0; 9]; 9], original: [[false; 9]; 9] },
            file_chosen: None,
            mouse_pos: [0.0, 0.0],
            click_on_file: false,
            selected_cell: None,
            sudoku_counter: 0,
        }
    }

    pub fn get_mouse_pos(&self) -> [f64; 2] {
        self.mouse_pos
    }
    pub fn get_grid(&self)-> &Grid {
        &self.grid
    }
    pub fn grid_mut(&mut self) -> &mut Grid {
        &mut self.grid
    }
    pub fn get_file_chosen(&self)-> &Option<PathBuf> {
        &self.file_chosen
    }

    pub fn set_mousse_pos(&mut self, new_pos: [f64; 2]) {
        self.mouse_pos = new_pos
    }
    pub fn set_grid(&mut self, new_grid: Grid) {
        self.grid = new_grid
    }
    pub fn set_file_chosen(&mut self, new_file: Option<PathBuf>) {
        self.file_chosen = new_file
    }
    pub fn set_click_on_file(&mut self, new_state: bool) {
        self.click_on_file = new_state
    }

    pub fn set_selected_cell(&mut self, x: usize, y: usize) {
        self.selected_cell = Some((x, y));
    }
    pub fn clear_selected_cell(&mut self) {
        self.selected_cell = None;
    }
    
    pub fn clear_grid(&mut self) {
        self.grid.grid = [[0; 9]; 9];
    }

    pub fn clear_grid_ori(&mut self) {
        self.grid.original = [[false; 9]; 9];
    }
    pub fn selected_cell(&self) -> Option<(usize, usize)> {
        self.selected_cell
    }
    
    pub fn sudoku_counter(&self) -> usize {
        self.sudoku_counter
    }
    pub fn set_sudoku_counter(&mut self, new_counter: usize) {
        self.sudoku_counter = new_counter;
    }
}
