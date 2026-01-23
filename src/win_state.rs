use piston_window::{image, Context, Flip, G2d, G2dTexture, Glyphs, PistonWindow, Texture, TextureSettings, Transformed, Window};
use crate::app_state::AppState;
use crate::button::ButtonRect;
use crate::display::{State, BTN_HOVER, WINDOW_H, WINDOW_W};

pub struct Win {
    retry: ButtonRect,
    menu: ButtonRect,
    quit: ButtonRect,
    texture_win: G2dTexture,
}

impl Win {
    pub fn new(window: &mut PistonWindow)-> Self {
        Self {
            retry : ButtonRect::flat((WINDOW_W / 2.0) + 80.0, (WINDOW_H / 2.0) + 10.0, 70.0, 38.0, "Retry", BTN_HOVER, [0.87, 0.66, 1.0, 1.0]),
            menu: ButtonRect::flat(WINDOW_W / 3.2, (WINDOW_H / 2.0) + 10.0, 70.0, 38.0, "Menu", BTN_HOVER, [0.87, 0.66, 1.0, 1.0]),
            quit: ButtonRect::flat((WINDOW_W / 2.0) - 30.0, WINDOW_H / 1.47, 70.0, 38.0, "Quit", BTN_HOVER, [0.87, 0.66, 1.0, 1.0]),
            texture_win: Texture::from_path(&mut window.create_texture_context(), "assets/images/win_state.png",  Flip::None, &TextureSettings::new(),).expect(" Download failed : game-over."),
        }
    }

    pub fn press_button_win (&self,
                              mouse: [f64; 2],
                              state: &mut State,
                              window: &mut PistonWindow,
                             life: &mut u32,
    ) {
        if self.retry.is_hovered(mouse) {
            *state = State::Play;
            *life = 3;
        }
        if self.menu.is_hovered(mouse) {
            *state = State::Menu
        }
        if self.quit.is_hovered(mouse) {
            window.set_should_close(true)
        }
    }

    pub fn display_win_state(&self,
                              app_state: &mut AppState,
                              c: &Context,
                              g: &mut G2d,
                              glyphs: &mut Glyphs,
    ) {
        let transform = c.transform
            .trans((WINDOW_W / 2.0) - 320.0, WINDOW_H / 20.0);
        image(&self.texture_win, transform, g);
        self.retry.draw(c, g, glyphs, self.retry.is_hovered(app_state.get_mousse_pos()), 18);
        self.menu.draw(c, g, glyphs, self.menu.is_hovered(app_state.get_mousse_pos()), 18);
        self.quit.draw(c, g, glyphs, self.quit.is_hovered(app_state.get_mousse_pos()), 18);
    }
}