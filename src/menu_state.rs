use piston_window::{Context, G2d, Glyphs};
use crate::app_state::AppState;
use crate::button::ButtonRect;
use crate::display::{State, BTN_BG, BTN_HOVER, WINDOW_H, WINDOW_W};

pub struct Menu {
    chose_solver: ButtonRect,
    chose_play: ButtonRect,
}

impl Menu {
    pub fn new() -> Self {
        let btn_w = 180.0;
        let btn_h = 46.0;
        let cx = (WINDOW_W - btn_w) / 2.0;
        let cy = WINDOW_H / 2.0;
        Self {
            chose_solver: ButtonRect::flat(cx, cy - 60.0, btn_w, btn_h, "Solve Sudoku", BTN_BG, BTN_HOVER),
            chose_play:   ButtonRect::flat(cx, cy + 10.0, btn_w, btn_h, "Play",         BTN_BG, BTN_HOVER),
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
        self.chose_play.draw(&c, g, glyphs, self.chose_play.is_hovered(app_state.get_mouse_pos()), 18);
        self.chose_solver.draw(&c, g, glyphs, self.chose_solver.is_hovered(app_state.get_mouse_pos()), 18);
    }
}

