use piston_window::{image, rectangle, Context, G2d, G2dTexture, Glyphs, Transformed};
use crate::app_state::AppState;
use crate::button::ButtonRect;
use crate::display::{read_file_play, BTN_BG, BTN_HOVER};
use crate::solver;

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
        let mut start_x = 500.0;
        let mut start_y = 160.0;
        for i in 0..9 {
            self.vector.push( ButtonRect::flat(start_x, start_y, 60.0, 60.0, &*(i+1).to_string(), BTN_BG, BTN_HOVER));
            start_x += 70.0;
            if i == 2 || i == 5 {
                start_y += 70.0;
                start_x = 500.0;
            }
        }
    }
}

pub fn press_number_button(
    numbers: &Number,
    mouse: [f64; 2],
    app_state: &mut AppState,
    life: &mut u32
) {
    let x = app_state.selected_cell().unwrap().0;
    let y = app_state.selected_cell().unwrap().1;

    if app_state.get_grid().original[y][x] {
        return;
    }

    let mut grid = app_state.grid_mut().clone();

    solver::is_valid(&mut grid, 0);

    for (i, button) in numbers.vector.iter().enumerate() {
        if button.is_hovered(mouse) {
            let value = (i + 1) as u8;

            if value as u32 != grid.grid[y][x] {
                *life -= 1;
                continue;
            }
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
        parse_file(new_sudoku, app_state);
    }
}

pub fn parse_file(new_sudoku: &ButtonRect,
              app_state: &mut AppState) {
    let file_name = format!("sudoku_not_resolved/sudoku_{}.txt", app_state.sudoku_counter());
    if let Ok(grid) = read_file_play(file_name) {
        app_state.set_grid(grid);
    }
    if app_state.sudoku_counter() >= MAX_SUDOKU {
        app_state.set_sudoku_counter(0);
    }
    app_state.set_sudoku_counter(app_state.sudoku_counter() + 1);
}

pub fn display_play(vector: &Number,
                    app_state: &mut AppState,
                    c: &Context,
                    g: &mut G2d,
                    glyphs: &mut Glyphs,
                    new_sudoku: &ButtonRect,
                    texture_life: &G2dTexture,
                    life: u32
) {
    if !app_state.grid_mut().original[0].contains(&true) {
        parse_file(&new_sudoku, app_state);
    }
    rectangle(
        [0.90, 0.92, 0.97, 1.0],
        [480.0, 140.0, 250.0, 240.0],
        c.transform,
        g,
    );
    for i in 0..9 {
        vector.vector[i].draw(c, g, glyphs, vector.vector[i].is_hovered(app_state.get_mousse_pos()), 20);
    }
    new_sudoku.draw(c, g, glyphs, new_sudoku.is_hovered(app_state.get_mousse_pos()), 18);

    let mut x = 50.0;
    for i in 0..life {
        let transform = c.transform
            .trans(x, 85.0)
            .scale(0.05, 0.05);
        image(texture_life, transform, g);
        x += 20.0;
    }
}
