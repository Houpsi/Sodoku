use std::{fs, io};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::os::linux::raw::blkcnt_t;
use piston_window::{image, line, rectangle, text, Context, Flip, G2d, G2dTexture, Glyphs, Key, PistonWindow, Texture, TextureSettings, Transformed, Window};
use crate::app_state::AppState;
use crate::button::ButtonRect;
use crate::display::{State, BTN_HOVER, WINDOW_H, WINDOW_W};
use crate::play_state::Play;

const INPUT_BG:          [f32; 4] = [0.97, 0.97, 0.99, 1.0];
const INPUT_BORDER:      [f32; 4] = [0.36, 0.33, 0.85, 1.0];
const INPUT_BORDER_DIM:  [f32; 4] = [0.82, 0.82, 0.88, 1.0];
const INPUT_TEXT:        [f32; 4] = [0.12, 0.12, 0.18, 1.0];
const INPUT_PLACEHOLDER: [f32; 4] = [0.65, 0.65, 0.72, 1.0];
const INPUT_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

const LB_BG:      [f32; 4] = [0.98, 0.98, 1.00, 1.0];
const LB_HEADER:  [f32; 4] = [0.36, 0.33, 0.85, 1.0];
const LB_GOLD:    [f32; 4] = [0.95, 0.70, 0.10, 1.0];
const LB_SILVER:  [f32; 4] = [0.55, 0.55, 0.60, 1.0];
const LB_BRONZE:  [f32; 4] = [0.72, 0.45, 0.20, 1.0];
const LB_TEXT:    [f32; 4] = [0.20, 0.20, 0.28, 1.0];
const LB_DIVIDER: [f32; 4] = [0.88, 0.88, 0.93, 1.0];

const SAVE_BTN_BG:    [f32; 4] = [0.20, 0.78, 0.56, 1.0]; // vert mint
const SAVE_BTN_HOVER: [f32; 4] = [0.15, 0.66, 0.46, 1.0];
const SAVE_BTN_DONE:  [f32; 4] = [0.75, 0.75, 0.80, 1.0]; // gris = déjà sauvé

const LB_X: f64 = 560.0;  const LB_Y: f64 = 200.0;
const LB_W: f64 = 210.0;  const LB_ROW_H: f64 = 22.0;
const INPUT_X: f64 = 560.0; const INPUT_Y: f64 = 370.0;
const INPUT_W: f64 = 230.0; const INPUT_H: f64 = 36.0;

pub struct Win {
    retry: ButtonRect,
    menu: ButtonRect,
    quit: ButtonRect,
    texture_win: G2dTexture,
    leader_board: Vec<(String, u32)>,
    username: String,
    score_saved:  bool,
    input_active: bool,
}

impl Win {
    pub fn new(window: &mut PistonWindow)-> Self {
        let btn_y  = (WINDOW_H / 2.0);
        let btn_w  = 80.0;
        let btn_h  = 36.0;
        let center = WINDOW_W / 2.0 - 160.0;
        Self {
            retry : ButtonRect::flat(center, btn_y, btn_w, btn_h, "Retry", BTN_HOVER, INPUT_BORDER),
            menu: ButtonRect::flat(center + 95.0, btn_y, btn_w, btn_h, "Menu", BTN_HOVER, INPUT_BORDER),
            quit: ButtonRect::flat(center + 190.0, btn_y, btn_w, btn_h, "Quit", BTN_HOVER, INPUT_BORDER),
            texture_win: Texture::from_path(&mut window.create_texture_context(), "assets/images/you_win.png",  Flip::None, &TextureSettings::new(),).expect(" Download failed : you-win."),
            leader_board: Self::parse_leader_board().unwrap_or_else(|_| Vec::new()),
            username: String::new(),
            score_saved: false,
            input_active: true,
        }
    }

    pub fn press_button_win(&mut self,
                            mouse: [f64; 2],
                            state: &mut State,
                            window: &mut PistonWindow,
                            play: &mut Play,
                            app_state: &mut AppState
    ) {
        if Self::point_in_rect(mouse, INPUT_X, INPUT_Y, INPUT_W, INPUT_H) {
            self.input_active = true;
            return;
        }
        let save_y = INPUT_Y + INPUT_H + 10.0;
        if Self::point_in_rect(mouse, INPUT_X, save_y, INPUT_W, 36.0)
            && !self.score_saved && !self.username.is_empty()
        {
            self.save_score_with_username(play.get_score());
            self.leader_board = Self::parse_leader_board().unwrap_or_default();
            self.score_saved  = true;
            self.username = String::new();
            return;
        }
        self.input_active = false;

        if self.retry.is_hovered(mouse) {
            *state = State::Play;
            play.set_life(3);
            play.set_score(3);
            app_state.set_sudoku_counter(app_state.sudoku_counter() - 1);
            play.parse_file(app_state)
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
                              glyphs: &mut Glyphs
    ) {
        let transform = c.transform
            .trans((WINDOW_W / 2.0) - 226.5, WINDOW_H / 5.0);
        image(&self.texture_win, transform, g);
        self.retry.draw(c, g, glyphs, self.retry.is_hovered(app_state.get_mousse_pos()), 18);
        self.menu.draw(c, g, glyphs, self.menu.is_hovered(app_state.get_mousse_pos()), 18);
        self.quit.draw(c, g, glyphs, self.quit.is_hovered(app_state.get_mousse_pos()), 18);

        self.draw_username_input(c, g, glyphs, app_state.get_mousse_pos());
        self.display_leader_board(c, g, glyphs);
    }

    fn display_leader_board(&self, c: &Context, g: &mut G2d, glyphs: &mut Glyphs) {
        let rows    = self.leader_board.len().min(10);
        let panel_h = 30.0 + rows as f64 * LB_ROW_H + 10.0;

        rectangle(LB_BG,    [LB_X - 10.0, LB_Y - 10.0, LB_W + 20.0, panel_h], c.transform, g);
        rectangle(LB_HEADER,[LB_X - 10.0, LB_Y - 10.0, 4.0, panel_h], c.transform, g);

        text::Text::new_color(LB_HEADER, 14)
            .draw("LEADERBOARD", glyphs, &c.draw_state,
                  c.transform.trans(LB_X + 4.0, LB_Y + 8.0), g).unwrap();

        line(LB_DIVIDER, 1.0, [LB_X - 10.0, LB_Y + 14.0, LB_X + LB_W + 10.0, LB_Y + 14.0], c.transform, g);

        for (i, (name, score)) in self.leader_board.iter().take(10).enumerate() {
            let row_y = LB_Y + 28.0 + i as f64 * LB_ROW_H;

            if i % 2 == 0 {
                rectangle([0.94, 0.94, 0.98, 1.0],
                          [LB_X - 10.0, row_y - 14.0, LB_W + 20.0, LB_ROW_H], c.transform, g);
            }

            let rank_color = match i { 0 => LB_GOLD, 1 => LB_SILVER, 2 => LB_BRONZE, _ => LB_TEXT };
            let rank_str   = format!("{}.", i + 1);

            text::Text::new_color(rank_color, 13)
                .draw(&rank_str, glyphs, &c.draw_state, c.transform.trans(LB_X, row_y), g).unwrap();

            let name_display = if name.len() > 12 { format!("{}...", &name[..11]) } else { name.clone() };
            text::Text::new_color(LB_TEXT, 13)
                .draw(&name_display, glyphs, &c.draw_state, c.transform.trans(LB_X + 28.0, row_y), g).unwrap();

            let score_str = score.to_string();
            let score_x   = LB_X + LB_W - score_str.len() as f64 * 8.0;
            text::Text::new_color(rank_color, 13)
                .draw(&score_str, glyphs, &c.draw_state, c.transform.trans(score_x, row_y), g).unwrap();
        }
    }

    pub fn save_score_with_username(&self, score: u32) {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("leaderboard/leaderBoard.txt").expect("Unable to open file");

        let final_info = format!("{} {}\n", self.username, score);
        file.write_all(final_info.as_ref()).expect("Unable to write to file");
    }

    pub fn get_input_key(&mut self, key: Key) {
        match key {
            Key::Backspace => { self.username.pop(); }
            _ => {}
        }
    }

    pub fn get_input_char(&mut self, text: String) {
        if !self.input_active || self.username.len() >= 20 { return; }
        self.username.push_str(&text);
    }

    fn draw_username_input(&self, c: &Context, g: &mut G2d, glyphs: &mut Glyphs, mouse: [f64; 2]) {
        text::Text::new_color(INPUT_TEXT, 13)
            .draw("Enter your name", glyphs, &c.draw_state,
                  c.transform.trans(INPUT_X, INPUT_Y - 8.0), g).unwrap();

        rectangle(INPUT_BG, [INPUT_X, INPUT_Y, INPUT_W, INPUT_H], c.transform, g);

        let (color, thickness) = if self.input_active { (INPUT_BORDER, 2.0) } else { (INPUT_BORDER_DIM, 1.0) };
        Self::draw_rect_border(c, g, INPUT_X, INPUT_Y, INPUT_W, INPUT_H, color, thickness);

        // curseur "|"
        if self.username.is_empty() {
            text::Text::new_color(INPUT_PLACEHOLDER, 16)
                .draw("your username...", glyphs, &c.draw_state,
                      c.transform.trans(INPUT_X + 10.0, INPUT_Y + INPUT_H / 2.0 + 6.0), g).unwrap();
        } else {
            let display = if self.input_active { format!("{}|", self.username) } else { self.username.clone() };
            text::Text::new_color(INPUT_TEXT, 16)
                .draw(&display, glyphs, &c.draw_state,
                      c.transform.trans(INPUT_X + 10.0, INPUT_Y + INPUT_H / 2.0 + 6.0), g).unwrap();
        }

        let btn_y = INPUT_Y + INPUT_H + 10.0;
        if !self.username.is_empty() && !self.score_saved {
            let color = if Self::point_in_rect(mouse, INPUT_X, btn_y, INPUT_W, 36.0) { SAVE_BTN_HOVER } else { SAVE_BTN_BG };
            rectangle(color, [INPUT_X, btn_y, INPUT_W, 36.0], c.transform, g);
            text::Text::new_color(INPUT_COLOR, 15)
                .draw("Save my score", glyphs, &c.draw_state,
                      c.transform.trans(INPUT_X + INPUT_W / 2.0 - 48.0, btn_y + 23.0), g).unwrap();
        } else if self.score_saved {
            rectangle(SAVE_BTN_DONE, [INPUT_X, btn_y, INPUT_W, 36.0], c.transform, g);
            text::Text::new_color(INPUT_COLOR, 15)
                .draw("Score saved!", glyphs, &c.draw_state,
                      c.transform.trans(INPUT_X + INPUT_W / 2.0 - 42.0, btn_y + 23.0), g).unwrap();
        }
    }

    fn point_in_rect(p: [f64; 2], x: f64, y: f64, w: f64, h: f64) -> bool {
        p[0] >= x && p[0] <= x + w && p[1] >= y && p[1] <= y + h
    }

    fn draw_rect_border(c: &Context, g: &mut G2d, x: f64, y: f64, w: f64, h: f64, color: [f32;4], t: f64) {
        line(color, t, [x,     y,     x + w, y    ], c.transform, g);
        line(color, t, [x,     y + h, x + w, y + h], c.transform, g);
        line(color, t, [x,     y,     x,     y + h], c.transform, g);
        line(color, t, [x + w, y,     x + w, y + h], c.transform, g);
    }
}
