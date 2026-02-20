use piston_window::{image, rectangle, text, Context, Flip, G2d, G2dTexture, Glyphs, PistonWindow, Texture, TextureSettings, Transformed};
use crate::app_state::AppState;
use crate::button::ButtonRect;
use crate::display::{read_file_play, State, BTN_BG, BTN_HOVER, WINDOW_H, WINDOW_W};
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


pub struct Play {
    new_sudoku: ButtonRect,
    back: ButtonRect,
    life: u32,
    numbers: Number,
    texture_lives: G2dTexture,
    score: u32,
}

impl Play {
    pub fn new(window: &mut PistonWindow) -> Self {
        Self {
            new_sudoku: ButtonRect::flat(WINDOW_W / 1.3, WINDOW_H / 15.0, 130.0, 38.0, "New sudoku", [0.61, 0.30, 0.8, 1.0], [0.87, 0.66, 1.0, 1.0]),
            back: ButtonRect::flat(10.0, 10.0, 40.0, 38.0, " < ", BTN_BG, BTN_HOVER),
            life: 3,
            numbers: Number::new(),
            texture_lives: Texture::from_path(&mut window.create_texture_context(), "assets/images/life.png", Flip::None, &TextureSettings::new(), ).expect("Download failed : life"),
            score: 0,
        }
    }

    pub fn set_life(&mut self, new_life: u32) {
        self.life = new_life
    }
    pub fn set_score(&mut self, new_score: u32) {
        self.score = new_score
    }

    pub fn get_score(&self) -> u32 {
        self.score
    }

    pub fn init_number(&mut self) {
        self.numbers.fill_vector();
    }

    pub fn parse_file(&mut self,
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

    pub fn press_button_play(&mut self,
                              mouse: [f64; 2],
                              app_state: &mut AppState,
                             state: &mut State,
    ) {
        if let Some((x, y)) = crate::display::get_cell_from_mouse(mouse) {
            if !app_state.get_grid().get_grid_ori()[y][x] {
                app_state.set_selected_cell(x, y);
            }
        }
        if app_state.selected_cell().is_some() {
            self.press_number_button(mouse, app_state);
        }
        if self.check_win(app_state) {
            *state = State::Win;
            self.set_life(3);
            self.set_score(0);
            app_state.clear_selected_cell();
            app_state.clear_grid();
        }
        if self.new_sudoku.is_hovered(mouse) {
            self.parse_file(app_state);
            self.set_life(3);
            self.set_score(0);
        }
        if self.back.is_hovered(mouse) {
            *state = State::Menu;
            self.set_life(3);
            self.set_score(0);
            app_state.clear_selected_cell();
            app_state.clear_grid();
            app_state.clear_grid_ori();
            app_state.set_sudoku_counter(app_state.sudoku_counter() - 1)
        }
    }

    pub fn check_remain_life(&self, state: &mut State) {
        if self.life < 1 {
            *state = State::Lost;
        }
    }

    pub fn display_play(&mut self,
                        app_state: &mut AppState,
                        c: &Context,
                        g: &mut G2d,
                        glyphs: &mut Glyphs,
    ) {
        if !app_state.grid_mut().original[0].contains(&true) {
            self.parse_file(app_state);
        }
        rectangle(
            [0.90, 0.92, 0.97, 1.0],
            [480.0, 140.0, 250.0, 240.0],
            c.transform,
            g,
        );
        for i in 0..9 {
            self.numbers.vector[i].draw(c, g, glyphs, self.numbers.vector[i].is_hovered(app_state.get_mouse_pos()), 20);
        }
        self.new_sudoku.draw(c, g, glyphs, self.new_sudoku.is_hovered(app_state.get_mouse_pos()), 18);
        self.back.draw(c, g, glyphs, self.back.is_hovered(app_state.get_mouse_pos()), 18);
        let score_str = self.score.to_string();
        text::Text::new_color([0.15, 0.15, 0.2, 1.0], 24)
            .draw(
                &score_str,
                glyphs,
                &c.draw_state,
                c.transform.trans(370.0, 110.0),
                g,
            )
            .unwrap();

        let mut x = 50.0;
        for _i in 0..self.life {
            let transform = c.transform
                .trans(x, 85.0)
                .scale(0.05, 0.05);
            image(&self.texture_lives, transform, g);
            x += 20.0;
        }
    }

    pub fn press_number_button(&mut self,
        mouse: [f64; 2],
        app_state: &mut AppState,
    ) {
        let x = app_state.selected_cell().unwrap().0;
        let y = app_state.selected_cell().unwrap().1;

        if app_state.get_grid().original[y][x] {
            return;
        }

        let mut grid = app_state.grid_mut().clone();

        solver::is_valid(&mut grid, 0);

        for (i, button) in self.numbers.vector.iter().enumerate() {
            if button.is_hovered(mouse) {
                let value = (i + 1) as u8;

                if value as u32 != grid.grid[y][x] {
                    self.life -= 1;
                    continue;
                }
                if app_state.get_grid().grid[y][x] == 0 {
                    self.score += 100;
                    app_state
                        .grid_mut()
                        .add_to_grid(y, x, value as u32);
                }
            }
        }
    }

    fn check_win(&mut self, app_state: &mut AppState) -> bool {
        let mut grid = app_state.grid_mut().clone();

        solver::is_valid(&mut grid, 0);
       for i in 0..8 {
           for y in 0..8 {
               if app_state.grid_mut().grid[i][y] != grid.grid[i][y] {
                   return false
               }
           }
       }
        true
    }
}
