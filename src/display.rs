use std::fs;
use std::path::{PathBuf};
use piston_window::{clear, line, rectangle, text, Button, Context, Flip, G2d, Glyphs, MouseCursorEvent, PistonWindow, PressEvent, Texture, TextureSettings, Transformed, WindowSettings};
use piston_window::types::Color;
use crate::grid::Grid;
use crate::button::ButtonRect;
use crate::{parser, solver};
use crate::app_state::AppState;
use crate::play_state::{display_play, press_button_play, press_number_button, Number};
use crate::solver_state::{display_solver, press_button_solver};

pub(crate) const WINDOW_W: f64 = 800.0;
const WINDOW_H: f64 = 500.0;

const GRID_SIZE: usize = 9;
pub(crate) const CELL_SIZE: f64 = 40.0;

// Colors
const BG_COLOR: Color = [0.96, 0.97, 0.98, 1.0];
const GRID_LINE: Color = [0.7, 0.7, 0.7, 1.0];
const GRID_LINE_BOLD: Color = [0.4, 0.4, 0.4, 1.0];

// UI colors
const PRIMARY: Color = [0.18, 0.45, 0.95, 1.0];
const PRIMARY_HOVER: Color = [0.12, 0.38, 0.85, 1.0];
const PRIMARY_ACTIVE: Color = [0.10, 0.30, 0.70, 1.0];

pub const BTN_BG: Color = [0.95, 0.96, 0.98, 1.0];
pub const BTN_HOVER: Color = [0.88, 0.91, 0.97, 1.0];

const BTN_TEXT: Color = [0.15, 0.15, 0.2, 1.0];


const CELL_BG: Color = [1.0, 1.0, 1.0, 1.0];
const CELL_ORIGINAL: Color = [0.92, 0.92, 0.94, 1.0];

const TEXT_ORIGINAL: Color = [0.1, 0.1, 0.1, 1.0];
const TEXT_SOLVED: Color = [0.25, 0.45, 0.85, 1.0];

#[derive(PartialEq)]
enum State {
    Menu,
    Solver,
    Play,
}

pub fn init_window() {
    let mut state = State::Menu;
    let mut window: PistonWindow =
        WindowSettings::new("Sudoku Solver", [WINDOW_W as u32, WINDOW_H as u32])
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut glyphs = window.load_font("font.ttf").unwrap();
    let mut app_state = AppState::new();

    let choose_file = ButtonRect::flat(40.0, 60.0, 110.0, 38.0, "Load", [0.61, 0.30, 0.8, 1.0], [0.87, 0.66, 1.0, 1.0]);
    let solve = ButtonRect::flat(160.0, 60.0, 110.0, 38.0, "Solve", [0.61, 0.30, 0.8, 1.0], [0.87, 0.66, 1.0, 1.0]);
    let clear_btn = ButtonRect::flat(280.0, 60.0, 110.0, 38.0, "Clear", [0.61, 0.30, 0.8, 1.0], [0.87, 0.66, 1.0, 1.0]);

    let chose_solver = ButtonRect::flat((WINDOW_W / 2.0) - 75.0, (WINDOW_H / 2.0) - 50.0, 150.0, 38.0, "Solve Sudoku", [0.61, 0.30, 0.8, 1.0], [0.87, 0.66, 1.0, 1.0]);
    let chose_play = ButtonRect::flat((WINDOW_W / 2.0) - 55.0, (WINDOW_H / 2.0) + 10.0, 110.0, 38.0, "Play", [0.61, 0.30, 0.8, 1.0], [0.87, 0.66, 1.0, 1.0]);

    let new_sudoku = ButtonRect::flat((WINDOW_W / 1.3), (WINDOW_H / 15.0), 130.0, 38.0, "New sudoku", [0.61, 0.30, 0.8, 1.0], [0.87, 0.66, 1.0, 1.0]);

    let mut numbers = Number::new();
    numbers.fill_vector();

    let mut life: u32 = 3;
    let texture = Texture::from_path(
        &mut window.create_texture_context(),
        "assets/life.png",
        Flip::None,
        &TextureSettings::new(),
    ).expect("Impossible de charger l'image");


    while let Some(e) = window.next() {
        e.mouse_cursor(|pos| app_state.set_mousse_pos(pos));

        if let Some(Button::Mouse(_)) = e.press_args() {
            let mouse = app_state.get_mousse_pos();

            match state {
                State::Menu => {
                    if chose_play.is_hovered(mouse) {
                        state = State::Play
                    }
                    if chose_solver.is_hovered(mouse) {
                        state = State::Solver
                    }
                }
                State::Solver => {
                    press_button_solver(&choose_file, &solve, &clear_btn, mouse, &mut app_state);
                }
                State::Play => {
                    if let Some((x, y)) = get_cell_from_mouse(mouse) {
                        if !app_state.get_grid().get_grid_ori()[y][x] {
                            app_state.set_selected_cell(x, y);
                        }
                    }
                    if app_state.selected_cell().is_some() {
                        press_number_button(&numbers, mouse, &mut app_state, &mut life);
                    }
                    press_button_play(mouse, &new_sudoku, &mut app_state);
                }
            }
        }

        window.draw_2d(&e, |c, g, device| {
            clear(BG_COLOR, g);

            if state == State::Menu {
                draw_title(&c, g, &mut glyphs);
                chose_play.draw(&c, g, &mut glyphs, chose_play.is_hovered(app_state.get_mousse_pos()), 18);
                chose_solver.draw(&c, g, &mut glyphs, chose_solver.is_hovered(app_state.get_mousse_pos()), 18);
            }

            if state == State::Solver {
                display_solver(&choose_file, &solve, &clear_btn, &mut app_state, &c, g, &mut glyphs);
            }
            if state == State::Play {
                display_play(&numbers,&mut app_state, &c, g, &mut glyphs, &new_sudoku, &texture, life);
            }

            if state == State::Play || state == State::Solver {
                draw_grid(&app_state.get_grid(), &c, g, &mut glyphs, &app_state);
            }
            glyphs.factory.encoder.flush(device);
        });
    }
}

fn draw_title(c: &Context, g: &mut G2d, glyphs: &mut Glyphs) {
    text::Text::new_color([0.15, 0.15, 0.2, 1.0], 32)
        .draw(
            "Sudoku Solver",
            glyphs,
            &c.draw_state,
            c.transform.trans((WINDOW_W / 2.0) - 130.0, 35.0),
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

fn get_cell_from_mouse(mouse: [f64; 2]) -> Option<(usize, usize)> {
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
