use std::fs;
use std::path::{PathBuf};
use piston_window::{clear, line, rectangle, text, Button, Context, G2d, Glyphs, MouseCursorEvent, PistonWindow, PressEvent, TextEvent, Transformed, WindowSettings};
use piston_window::types::Color;
use crate::grid::Grid;
use crate::{parser, solver};
use crate::app_state::AppState;
use crate::lost_state::Lost;
use crate::menu_state::Menu;
use crate::play_state::{Play};
use crate::solver_state::{Solver};
use crate::win_state::Win;

pub(crate) const WINDOW_W: f64 = 800.0;
pub(crate) const WINDOW_H: f64 = 500.0;

const GRID_SIZE: usize = 9;
pub(crate) const CELL_SIZE: f64 = 40.0;

// Colors
const BG_COLOR: Color = [0.96, 0.97, 0.98, 1.0];
const GRID_LINE: Color = [0.7, 0.7, 0.7, 1.0];
const GRID_LINE_BOLD: Color = [0.4, 0.4, 0.4, 1.0];
pub const BTN_BG: Color = [0.36, 0.33, 0.85, 1.0];
pub const BTN_HOVER: Color = [0.30, 0.28, 0.78, 1.0];

const CELL_BG: Color = [1.0, 1.0, 1.0, 1.0];
const CELL_ORIGINAL: Color = [0.92, 0.92, 0.94, 1.0];

const TEXT_ORIGINAL: Color = [0.1, 0.1, 0.1, 1.0];
const TEXT_SOLVED: Color = [0.25, 0.45, 0.85, 1.0];

#[derive(PartialEq)]
pub enum State {
    Menu,
    Solver,
    Play,
    Win,
    Lost,
}

pub fn init_window() {
    let mut state = State::Menu;
    let mut window: PistonWindow =
        WindowSettings::new("Sudoku Solver", [WINDOW_W as u32, WINDOW_H as u32])
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut glyphs = window.load_font("assets/fonts/font.ttf").unwrap();
    let mut app_state = AppState::new();

    let menu = Menu::new();
    let solver = Solver::new();
    let mut play = Play::new(&mut window);
    let lost = Lost::new(&mut window);
    let mut win = Win::new(&mut window);

    play.init_number();

    while let Some(e) = window.next() {
        e.mouse_cursor(|pos| app_state.set_mousse_pos(pos));

        if let Some(Button::Mouse(_)) = e.press_args() {
            let mouse = app_state.get_mouse_pos();

            match state {
                State::Menu => {
                    menu.press_button_menu(mouse, &mut state)
                }
                State::Solver => {
                    solver.press_button_solver(mouse, &mut app_state, &mut state);
                }
                State::Play => {
                    play.press_button_play(mouse, &mut app_state, &mut state);
                }
                State::Lost => {
                    lost.press_button_lost(mouse, &mut state, &mut window, &mut play);
                }
                State::Win => {
                    win.press_button_win(mouse, &mut state, &mut window, &mut play, &mut app_state);
                }
            }
        }
        if let Some(Button::Keyboard(key)) = e.press_args() {
           win.get_input_key(key);
        }
        if let Some(text) = e.text_args() {
            win.get_input_char(text)
        }

        if state == State::Play {
            play.check_remain_life(&mut state);
        }

        window.draw_2d(&e, |c, g, device| {
            clear(BG_COLOR, g);

            if state == State::Menu {
               menu.display_menu_state(&mut app_state, &c, g, &mut glyphs);
            }

            if state == State::Solver {
                solver.display_solver(&mut app_state, &c, g, &mut glyphs);
            }
            if state == State::Play {
                play.display_play(&mut app_state, &c, g, &mut glyphs);
            }
            if state == State::Lost {
                lost.display_lost_state(&mut app_state, &c, g, &mut glyphs);
            }
            if state == State::Win {
                win.display_win_state(&mut app_state, &c, g, &mut glyphs);
            }
            if state == State::Play || state == State::Solver {
                draw_grid(&app_state.get_grid(), &c, g, &mut glyphs, &app_state);
            }
            glyphs.factory.encoder.flush(device);
        });
    }
}

pub(crate) fn draw_title(c: &Context, g: &mut G2d, glyphs: &mut Glyphs) {
    text::Text::new_color([0.15, 0.15, 0.2, 1.0], 32)
        .draw(
            "Sudoku Solver",
            glyphs,
            &c.draw_state,
            c.transform.trans((WINDOW_W / 2.0) - 130.0, WINDOW_H / 4.0),
            g,
        )
        .unwrap();
}

fn draw_grid(grid: &Grid, c: &Context, g: &mut G2d, glyphs: &mut Glyphs, app_state: &AppState) {
    let start_x = 50.0;
    let start_y = 115.0;

    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            let cell_x = start_x + x as f64 * CELL_SIZE;
            let cell_y = start_y + y as f64 * CELL_SIZE;

            let bg = if grid.original[y][x] {
                CELL_ORIGINAL
            } else {
                CELL_BG
            };

            rectangle(
                bg,
                [cell_x, cell_y, CELL_SIZE, CELL_SIZE],
                c.transform,
                g,
            );

            let value = grid.get_grid()[y][x];
            if value != 0 {
                let color = if grid.original[y][x] {
                    TEXT_ORIGINAL
                } else {
                    TEXT_SOLVED
                };
                let text_x = cell_x + CELL_SIZE * 0.5 - 8.0;
                let text_y = cell_y + CELL_SIZE / 2.0 + 13.0;

                text::Text::new_color(color, 28)
                    .draw(
                        &value.to_string(),
                        glyphs,
                        &c.draw_state,
                        c.transform.trans(text_x, text_y),
                        g,
                    )
                    .unwrap();
            }
        }
    }

    if let Some((sx, sy)) = app_state.selected_cell() {
        let x = start_x + sx as f64 * CELL_SIZE;
        let y = start_y + sy as f64 * CELL_SIZE;

        rectangle(
            [0.8, 0.85, 1.0, 0.6],
            [x, y, CELL_SIZE, CELL_SIZE],
            c.transform,
            g,
        );
    }
    draw_grid_lines(c, g, start_x, start_y);
}

fn draw_grid_lines(c: &Context, g: &mut G2d, start_x: f64, start_y: f64) {
    let size = CELL_SIZE * GRID_SIZE as f64;

    for i in 0..=GRID_SIZE {
        let (color, thickness) = if i % 3 == 0 {
            (GRID_LINE_BOLD, 2.5)
        } else {
            (GRID_LINE, 1.0)
        };

        let x = start_x + i as f64 * CELL_SIZE;
        line(color, thickness, [x, start_y, x, start_y + size], c.transform, g);

        let y = start_y + i as f64 * CELL_SIZE;
        line(color, thickness, [start_x, y, start_x + size, y], c.transform, g);
    }
}

pub(crate) fn read_file(path: &PathBuf) -> Result<Grid, String> {
    let contents = fs::read_to_string(path)
        .map_err(|e| e.to_string())?;

    let grid = parser::parser_file(&contents, Some('.'))
        .map_err(|e| e.to_string())?;
    let original = parser::parser_ori(&contents, Some('.'))
        .map_err(|e| e.to_string())?;

    let mut grid = Grid { grid, original };
    solver::is_valid(&mut grid, 0);

    Ok(grid)
}

pub(crate) fn read_file_play(path: String) -> Result<Grid, String> {
    let contents = fs::read_to_string(path)
        .map_err(|e| e.to_string())?;

    let grid = parser::parser_file(&contents, Some('.'))
        .map_err(|e| e.to_string())?;
    let original = parser::parser_ori(&contents, Some('.'))
        .map_err(|e| e.to_string())?;

    let grid = Grid { grid, original };
    Ok(grid)
}

pub(crate) fn get_cell_from_mouse(mouse: [f64; 2]) -> Option<(usize, usize)> {
    let start_x = 50.0;
    let start_y = 115.0;

    let mx = mouse[0];
    let my = mouse[1];

    let grid_w = GRID_SIZE as f64 * CELL_SIZE;
    let grid_h = GRID_SIZE as f64 * CELL_SIZE;

    if mx < start_x || mx >= start_x + grid_w ||
        my < start_y || my >= start_y + grid_h {
        return None;
    }

    let x = ((mx - start_x) / CELL_SIZE) as usize;
    let y = ((my - start_y) / CELL_SIZE) as usize;

    Some((x, y))
}
