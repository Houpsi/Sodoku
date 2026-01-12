use piston_window::{Context, G2d, Glyphs};
use crate::app_state::AppState;
use crate::button::ButtonRect;
use crate::display::{State, WINDOW_H, WINDOW_W};

pub struct Lost {
    retry: ButtonRect,
    menu: ButtonRect,
    quit: ButtonRect,
    // background
    // ecrit game over en grand
}

impl Lost {
    pub fn new()-> Self {
        Self {
            retry :  ButtonRect::flat((WINDOW_W / 2.0) + 30.0, (WINDOW_H / 2.0) - 20.0, 70.0, 38.0, "Retry", [0.61, 0.30, 0.8, 1.0], [0.87, 0.66, 1.0, 1.0]),
            menu:  ButtonRect::flat((WINDOW_W / 2.0) - 70.0, (WINDOW_H / 2.0) - 20.0, 70.0, 38.0, "Menu", [0.61, 0.30, 0.8, 1.0], [0.87, 0.66, 1.0, 1.0]),
            quit:  ButtonRect::flat((WINDOW_W / 2.0) - 30.0, (WINDOW_H / 2.0) + 40.0, 70.0, 38.0, "Quit", [0.61, 0.30, 0.8, 1.0], [0.87, 0.66, 1.0, 1.0]),
        }
    }

    pub fn press_button_lost (&self,
                              mouse: [f64; 2],
                              app_state: &mut AppState,
                              state: &mut State,
    ) {
        if self.retry.is_hovered(mouse) {
        }
        if self.menu.is_hovered(mouse) {
            *state = State::Menu
        }
        if self.quit.is_hovered(mouse) {
        }
    }

    pub fn display_lost_state(&self,
                              app_state: &mut AppState,
                              c: &Context,
                              g: &mut G2d,
                              glyphs: &mut Glyphs,
    ) {
        self.retry.draw(c, g, glyphs, self.retry.is_hovered(app_state.get_mousse_pos()), 18);
        self.menu.draw(c, g, glyphs, self.menu.is_hovered(app_state.get_mousse_pos()), 18);
        self.quit.draw(c, g, glyphs, self.quit.is_hovered(app_state.get_mousse_pos()), 18);
    }
}