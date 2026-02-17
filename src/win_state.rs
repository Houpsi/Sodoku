use std::{fs, io};
use std::fs::File;
use std::io::{BufRead, BufReader};
use piston_window::{image, text, Context, Flip, G2d, G2dTexture, Glyphs, PistonWindow, Texture, TextureSettings, Transformed, Window};
use crate::app_state::AppState;
use crate::button::ButtonRect;
use crate::display::{State, BTN_HOVER, WINDOW_H, WINDOW_W};
use crate::play_state::Play;

pub struct Win {
    retry: ButtonRect,
    menu: ButtonRect,
    quit: ButtonRect,
    texture_win: G2dTexture,
    leader_board: Vec<(String, u32)>
}

impl Win {
    pub fn new(window: &mut PistonWindow)-> Self {
        Self {
            retry : ButtonRect::flat((WINDOW_W / 2.0) + 80.0, (WINDOW_H / 2.0) + 10.0, 70.0, 38.0, "Retry", BTN_HOVER, [0.87, 0.66, 1.0, 1.0]),
            menu: ButtonRect::flat(WINDOW_W / 3.2, (WINDOW_H / 2.0) + 10.0, 70.0, 38.0, "Menu", BTN_HOVER, [0.87, 0.66, 1.0, 1.0]),
            quit: ButtonRect::flat((WINDOW_W / 2.0) - 30.0, WINDOW_H / 1.47, 70.0, 38.0, "Quit", BTN_HOVER, [0.87, 0.66, 1.0, 1.0]),
            texture_win: Texture::from_path(&mut window.create_texture_context(), "assets/images/you_win.png",  Flip::None, &TextureSettings::new(),).expect(" Download failed : you-win."),
            leader_board: Self::parse_leader_board().unwrap_or_else(|_| Vec::new())
        }
    }

    pub fn press_button_win (&self,
                              mouse: [f64; 2],
                              state: &mut State,
                              window: &mut PistonWindow,
                              play: &mut Play,
    ) {
        if self.retry.is_hovered(mouse) {
            *state = State::Play;
            play.set_life(3);
            play.set_score(3);
        }
        if self.menu.is_hovered(mouse) {
            *state = State::Menu
        }
        if self.quit.is_hovered(mouse) {
            window.set_should_close(true)
        }
    }

    pub fn parse_leader_board() -> io::Result<Vec<(String, u32)>> {
        let f = File::open("leaderboard/leaderBoard.txt")?;
        let f = BufReader::new(f);
        let mut leader_board = Vec::new();

        for line in f.lines() {
            let line = line?;
            let mut parts = line.split_whitespace();
            let name = match parts.next() {
                Some(n) => n.to_string(),
                None => continue,
            };
            let score = match parts.next().and_then(|s| s.parse::<u32>().ok()) {
                Some(s) => s,
                None => continue,
            };
            leader_board.push((name, score));
        }
        Ok(leader_board)
    }

    pub fn display_win_state(&self,
                              app_state: &mut AppState,
                              c: &Context,
                              g: &mut G2d,
                              glyphs: &mut Glyphs,
    ) {
        let transform = c.transform
            .trans((WINDOW_W / 2.0) - 226.5, WINDOW_H / 5.0);
        image(&self.texture_win, transform, g);
        self.retry.draw(c, g, glyphs, self.retry.is_hovered(app_state.get_mousse_pos()), 18);
        self.menu.draw(c, g, glyphs, self.menu.is_hovered(app_state.get_mousse_pos()), 18);
        self.quit.draw(c, g, glyphs, self.quit.is_hovered(app_state.get_mousse_pos()), 18);
        self.display_leader_board(c, g, glyphs);
    }

    fn display_leader_board(&self,
                            c: &Context,
                            g: &mut G2d,
                            glyphs: &mut Glyphs
    ) {
        let start_y = WINDOW_H / 2.5;

        for (i, (name, score)) in self.leader_board.iter().take(10).enumerate() {
            let color = if i == 0 {
                [1.0, 0.8, 0.2, 1.0]
            } else {
                [0.15, 0.15, 0.2, 1.0]
            };
            let line = format!("{}. {} - {}", i + 1, name, score);

            text::Text::new_color(color, 15)
                .draw(
                    &line,
                    glyphs,
                    &c.draw_state,
                    c.transform.trans((WINDOW_W / 1.35), start_y + i as f64 * 20.0),
                    g,
                )
                .unwrap();
        }
    }
}
