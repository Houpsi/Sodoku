// Display proprement la grid
// pourquoi pas faire de l'ui with piston

use std::fs;
use std::path::PathBuf;
use piston_window::{clear, line, rectangle, text, Button, Context, G2d, Glyphs, MouseCursorEvent, PistonWindow, PressEvent, Transformed, WindowSettings};
use piston_window::types::Color;
use crate::grid::Grid;
use crate::button::ButtonRect;
use rfd::FileDialog;
use crate::{parser, solver, app_state};
use crate::app_state::AppState;

// const GRID_SIZE: usize = 9;
// const CELL_SIZE: f64 = 25.0;

const WINDOW_W: f64 = 640.0;
const WINDOW_H: f64 = 480.0;

const GRID_SIZE: usize = 9;
const CELL_SIZE: f64 = 40.0;

// Colors
const BG_COLOR: Color = [0.96, 0.97, 0.98, 1.0];
const GRID_LINE: Color = [0.7, 0.7, 0.7, 1.0];
const GRID_LINE_BOLD: Color = [0.4, 0.4, 0.4, 1.0];

const CELL_BG: Color = [1.0, 1.0, 1.0, 1.0];
const CELL_ORIGINAL: Color = [0.92, 0.92, 0.94, 1.0];

const TEXT_ORIGINAL: Color = [0.1, 0.1, 0.1, 1.0];
const TEXT_SOLVED: Color = [0.25, 0.45, 0.85, 1.0];


pub fn init_window() {
    let mut window: PistonWindow =
        WindowSettings::new("Sudoku Solver", [WINDOW_W as u32, WINDOW_H as u32])
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut glyphs = window.load_font("font.ttf").unwrap();
    let mut app_state = AppState::new();

    let choose_file = ButtonRect::flat(40.0, 60.0, 110.0, 38.0, "Load");
    let solve = ButtonRect::flat(160.0, 60.0, 110.0, 38.0, "Solve");
    let clear_btn = ButtonRect::flat(280.0, 60.0, 110.0, 38.0, "Clear");

    while let Some(e) = window.next() {
        e.mouse_cursor(|pos| app_state.set_mousse_pos(pos));

        if let Some(Button::Mouse(_)) = e.press_args() {
            let mouse = app_state.get_mousse_pos();

            if choose_file.is_hovered(mouse) {
                let file = FileDialog::new().add_filter("text", &["txt"]).pick_file();
                app_state.set_file_chosen(file);
                app_state.set_click_on_file(true);
            }

            if solve.is_hovered(mouse) {
                if let Some(path) = app_state.get_file_chosen() {
                    if let Ok(grid) = read_file(path) {
                        app_state.set_grid(grid);
                    }
                }
            }

            if clear_btn.is_hovered(mouse) {
                app_state.grid_mut().set_grid([[0; 9]; 9]);
                app_state.set_click_on_file(false);
            }
        }

        window.draw_2d(&e, |c, g, device| {
            clear(BG_COLOR, g);

            draw_title(&c, g, &mut glyphs);
            draw_grid(&app_state.get_grid(), &c, g, &mut glyphs);

            choose_file.draw(&c, g, &mut glyphs, choose_file.is_hovered(app_state.get_mousse_pos()));
            solve.draw(&c, g, &mut glyphs, solve.is_hovered(app_state.get_mousse_pos()));
            clear_btn.draw(&c, g, &mut glyphs, clear_btn.is_hovered(app_state.get_mousse_pos()));

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
            c.transform.trans(40.0, 35.0),
            g,
        )
        .unwrap();
}

fn draw_grid(grid: &Grid, c: &Context, g: &mut G2d, glyphs: &mut Glyphs) {
    // if !grid_has_values(&grid.get_grid()) {
    //     return;
    // }

    let grid_px = CELL_SIZE * GRID_SIZE as f64;
    let start_x = (WINDOW_W - grid_px) / 2.0;
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

pub fn draw_text(
    ctx: &piston_window::Context,
    graphics: &mut G2d,
    glyphs: &mut Glyphs,
    color: Color,
    pos: [u32; 2],
    text: &str,
) {
    text::Text::new_color(color, 20)
        .draw(text, glyphs, &ctx.draw_state, ctx.transform.trans(pos[0] as f64, pos[1] as f64), graphics)
        .unwrap();
}

fn read_file(path: &PathBuf) -> Result<Grid, String> {
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

fn grid_has_values(grid: &[[u32; 9]; 9]) -> bool {
    grid.iter().flatten().any(|&cell| cell != 0)
}
