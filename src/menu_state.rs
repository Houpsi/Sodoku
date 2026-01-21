use piston_window::{Context, G2d, Glyphs};
use crate::app_state::AppState;
use crate::button::ButtonRect;
use crate::display::{State, WINDOW_H, WINDOW_W};

pub struct Menu {
    chose_solver: ButtonRect,
    chose_play: ButtonRect,
}

impl Menu {
    pub fn new() -> Self {
        Self{
            chose_solver: ButtonRect::flat((WINDOW_W / 2.0) -75.0, (WINDOW_H / 2.0) - 50.0, 150.0, 38.0, "Solve Sudoku", [0.61, 0.30, 0.8, 1.0], [0.87, 0.66, 1.0, 1.0]),
            chose_play: ButtonRect::flat((WINDOW_W / 2.0) -55.0, (WINDOW_H / 2.0) + 10.0, 110.0, 38.0, "Play", [0.61, 0.30, 0.8, 1.0], [0.87, 0.66, 1.0, 1.0]),
        }
    }

    pub fn press_button_menu (&self,
                              mouse: [f64; 2],
                              state: &mut State
    ) {
        if self.chose_play.is_hovered(mouse) {
            *state = State::Play
        }
        if self.chose_solver.is_hovered(mouse) {
            *state = State::Solver
        }
    }

    pub fn display_menu_state(&self,
                              app_state: &mut AppState,
                              c: &Context,
                              g: &mut G2d,
                              glyphs: &mut Glyphs,
    ) {
        crate::display::draw_title(&c, g, glyphs);
        self.chose_play.draw(&c, g, glyphs, self.chose_play.is_hovered(app_state.get_mousse_pos()), 18);
        self.chose_solver.draw(&c, g, glyphs, self.chose_solver.is_hovered(app_state.get_mousse_pos()), 18);
    }
}

