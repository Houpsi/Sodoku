use piston_window::{rectangle, text, Context, G2d, Glyphs, MouseButton, Transformed};
use crate::app_state::AppState;
use crate::button::ButtonRect;
use crate::display::read_file_play;

const MAX_SUDOKU: usize = 5;

pub struct Number {
   vector: Vec<ButtonRect>
}

impl Number {
    pub fn new() -> Self {
        Self {
            vector: vec![]
        }
    }

    pub fn fill_vector(&mut self) {
        let mut start_x = 460.0 + 50.0;
        let mut start_y = 115.0;
        for i in 0..9 {
            self.vector.push( ButtonRect::flat(start_x, start_y, 45.0, 38.0, &*(i+1).to_string(), [0.61, 0.30, 0.8, 1.0], [0.87, 0.66, 1.0, 1.0]));
            start_x += 50.0;
            if i == 2 || i == 5 {
                start_y += 50.0;
                start_x = 460.0 + 50.0;
            }
        }
    }
}

// pub fn init_play_state() {
//
// }

pub fn press_number_button(
    numbers: &Number,
    mouse: [f64; 2],
    app_state: &mut AppState,
) {
    let x = app_state.selected_cell().unwrap().0;
    let y = app_state.selected_cell().unwrap().1;

    if app_state.get_grid().original[y][x] {
        return;
    }
    print!("{} - {} is hovered\n", x, y);
    for (i, button) in numbers.vector.iter().enumerate() {
        if button.is_hovered(mouse) {
            // check if it s not alredy at true
            let value = (i + 1) as u8;
            app_state
                .grid_mut()
                .add_to_grid(y, x, value as u32);
        }
    }
}

pub fn press_button_play (
    mouse: [f64; 2],
    new_sudoku: &ButtonRect,
    app_state: &mut AppState
) {
    if new_sudoku.is_hovered(mouse) {
        let file_name = format!("sudoku_not_resolved/sudoku_{}.txt", app_state.sudoku_counter());
        if let Ok(grid) = read_file_play(file_name) {
            app_state.set_grid(grid);
        }
        if app_state.sudoku_counter() >= MAX_SUDOKU {
            app_state.set_sudoku_counter(0);
        }
        app_state.set_sudoku_counter(app_state.sudoku_counter() + 1);
    }
}

pub fn display_play(vector: &Number,
                    app_state: &AppState,
                    c: &Context,
                    g: &mut G2d,
                    glyphs: &mut Glyphs,
                    new_sudoku: &ButtonRect
) {
    for i in 0..9 {
        vector.vector[i].draw(c, g, glyphs, vector.vector[i].is_hovered(app_state.get_mousse_pos()));
    }
    new_sudoku.draw(c, g, glyphs, new_sudoku.is_hovered(app_state.get_mousse_pos()));
}
