use piston_window::{image, Context, Flip, G2d, G2dTexture, Glyphs, PistonWindow, Texture, TextureSettings, Transformed, Window};
use crate::app_state::AppState;
use crate::button::ButtonRect;
use crate::display::{State, BTN_HOVER, WINDOW_H, WINDOW_W};
use crate::play_state::Play;

pub struct Lost {
    retry: ButtonRect,
    menu: ButtonRect,
    quit: ButtonRect,
    texture_game_over: G2dTexture,
    // ecrit game over en grand
}

impl Lost {
    pub fn new(window: &mut PistonWindow)-> Self {
        Self {
            retry : ButtonRect::flat((WINDOW_W / 2.0) + 80.0, (WINDOW_H / 2.0) + 10.0, 70.0, 38.0, "Retry", BTN_HOVER, [0.87, 0.66, 1.0, 1.0]),
            menu: ButtonRect::flat(WINDOW_W / 3.2, (WINDOW_H / 2.0) + 10.0, 70.0, 38.0, "Menu", BTN_HOVER, [0.87, 0.66, 1.0, 1.0]),
            quit: ButtonRect::flat((WINDOW_W / 2.0) - 30.0, WINDOW_H / 1.47, 70.0, 38.0, "Quit", BTN_HOVER, [0.87, 0.66, 1.0, 1.0]),
            texture_game_over: Texture::from_path(&mut window.create_texture_context(), "assets/images/you_lost.png", Flip::None, &TextureSettings::new(),).expect(" Download failed : game-over."),
        }
    }

    pub fn press_button_lost (&self,
                              mouse: [f64; 2],
                              state: &mut State,
                              window: &mut PistonWindow,
                              play: &mut Play,
    ) {
        if self.retry.is_hovered(mouse) {
            *state = State::Play;
            play.set_life(3);
            play.set_score(0);
        }
        if self.menu.is_hovered(mouse) {
            *state = State::Menu
        }
        if self.quit.is_hovered(mouse) {
            window.set_should_close(true)
        }
    }

    pub fn display_lost_state(&self,
                              app_state: &mut AppState,
                              c: &Context,
                              g: &mut G2d,
                              glyphs: &mut Glyphs,
    ) {
        let transform = c.transform
            .trans((WINDOW_W / 2.0) - 246.0, WINDOW_H / 5.0);
        image(&self.texture_game_over, transform, g);
        self.retry.draw(c, g, glyphs, self.retry.is_hovered(app_state.get_mousse_pos()), 18);
        self.menu.draw(c, g, glyphs, self.menu.is_hovered(app_state.get_mousse_pos()), 18);
        self.quit.draw(c, g, glyphs, self.quit.is_hovered(app_state.get_mousse_pos()), 18);
    }
}
