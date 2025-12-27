// Display proprement la grid
// pourquoi pas faire de l'ui with piston

use std::fs;
use std::path::PathBuf;
use piston_window::{clear, line, text, Button, G2d, Glyphs, MouseCursorEvent, PistonWindow, PressEvent, Transformed, WindowSettings};
use piston_window::types::Color;
use crate::grid::Grid;
use crate::button::ButtonRect;
use rfd::FileDialog;
use crate::{parser, solver, app_state};
use crate::app_state::AppState;

const GRID_SIZE: usize = 9;
const CELL_SIZE: f64 = 25.0;

pub fn init_window() {
    // let mut click_on_file: bool = false;
    let mut window: PistonWindow =
        WindowSettings::new("Sudoku Solver", [640, 480])
            .exit_on_esc(true)
            .build()
            .unwrap();
    let mut glyphs = window.load_font("font.ttf").unwrap();

    let mut app_state = AppState::new();
    let choose_file = ButtonRect {
        x: 5.0,
        y: 20.0,
        w: 85.0,
        h: 35.0,
        label: "file".to_string(),
        color_hovered: [0.7, 0.7, 0.7, 1.0],
        color: [0.5, 0.5, 0.5, 1.0],
    };
    let solve_sudoku = ButtonRect {
        x: 100.0,
        y: 20.0,
        w: 100.0,
        h: 35.0,
        label: "Solve".to_string(),
        color_hovered: [1.0, 0.0, 0.0, 1.0],
        color: [0.0, 1.0, 0.0, 1.0],
    };
    let clear_grid = ButtonRect {
        x: 200.0,
        y: 20.0,
        w: 100.0,
        h: 35.0,
        label: "Clear".to_string(),
        color_hovered: [1.0, 1.0, 0.0, 1.0],
        color: [0.0, 1.0, 1.0, 1.0],
    };

    while let Some(e) = window.next() {
        e.mouse_cursor(|pos| {
            app_state.set_mousse_pos(pos)
        });
        if let Some(Button::Mouse(_button)) = e.press_args() {
            if choose_file.is_hovered(app_state.get_mousse_pos()) {
                let files = FileDialog::new()
                    .add_filter("text", &["txt"])
                    .set_directory("/home/heleneh/Documents")// TO DO change the path to a personalize one
                    .pick_file();
                print!("file choose : {:?}", files);
                app_state.set_file_chosen(files);
                app_state.set_click_on_file(true);
            }
            if solve_sudoku.is_hovered(app_state.get_mousse_pos())  {
                if let Some(path) = &app_state.get_file_chosen() {
                    match read_file(path) {
                        Ok(new_grid) => app_state.set_grid(new_grid),
                        Err(err) => eprintln!("{}", err),
                    }
                }
            }
            if clear_grid.is_hovered(app_state.get_mousse_pos())  {
                app_state.grid_mut().set_grid([[0; 9]; 9]);
                app_state.set_click_on_file(false);
            }
        }

        window.draw_2d(&e, |c, g, device| {
            clear([1.0; 4], g);

            if !grid_has_values(&app_state.get_grid().get_grid()) && app_state.get_file_chosen().is_some() && app_state.get_click_on_file() {
                draw_text(
                    &c,
                    g,
                    &mut glyphs,
                    [1.0, 0.0, 0.0, 1.0],
                    [
                        200,
                        50,
                    ],
                    "The file is not in the correct format",
                );
            }
            draw_grid_lines(&c, g);
            display_grid_piston(&app_state.get_grid(), &c, g, &mut glyphs);

            choose_file.draw(&c, g, &mut glyphs, choose_file.is_hovered(app_state.get_mousse_pos()));
            solve_sudoku.draw(&c, g, &mut glyphs, solve_sudoku.is_hovered(app_state.get_mousse_pos()));
            clear_grid.draw(&c, g, &mut glyphs, clear_grid.is_hovered(app_state.get_mousse_pos()));

            glyphs.factory.encoder.flush(device);
        });
    }
}

pub fn display_grid(grid: [[u32; 9]; 9]) {
    for x in 0..9 {
        for y in 0..9 {
            print!("{} ", grid[x][y]);
            if y == 2 || y == 5 {
                print!("| ")
            }
        }
        print!("\n");
        if x == 2 || x == 5 {
            print!("---------------------\n")
        }
    }
}

pub fn display_grid_piston(grid: &Grid, c: &piston_window::Context,  g: &mut G2d, glyphs: &mut Glyphs,) {
    if !grid_has_values(&grid.get_grid()) {
        return;
    }
    let mut offset_y = 130.0;
    for (y, row) in grid.get_grid().iter().enumerate() {
        let mut offset_x = 230.0;
        for (x, cell) in row.iter().enumerate() {
            let color = if grid.original[y][x] {
                [0.0, 0.0, 0.0, 1.0] 
            } else {
                [0.2, 0.2, 0.8, 1.0]
            };

            draw_text(
                &c,
                g,
                glyphs,
                color,
                [
                    (x as f64 * 23.0 + offset_x) as u32,
                    (y as f64 * 22.0 + offset_y) as u32,
                ],
                &*cell.to_string(),
            );
            if x == 2 || x == 5 {
                offset_x += 10.0;
            }
        }
        if y == 2 || y == 5 {
            offset_y += 10.0;
        }
    }
}
fn draw_grid_lines(c: &piston_window::Context, g: &mut G2d) {
    let start_x = 225.0;
    let start_y = 109.0;
    let grid_size = CELL_SIZE * GRID_SIZE as f64;

    for i in 0..=GRID_SIZE {
        let thickness = if i % 3 == 0 { 2.0 } else { 1.0 };

        // Vertical lines
        let x = start_x + i as f64 * CELL_SIZE;
        line(
            [0.0, 0.0, 0.0, 1.0],
            thickness,
            [x, start_y, x, start_y + grid_size],
            c.transform,
            g,
        );

        // Horizontal lines
        let y = start_y + i as f64 * CELL_SIZE;
        line(
            [0.0, 0.0, 0.0, 1.0],
            thickness,
            [start_x, y, start_x + grid_size, y],
            c.transform,
            g,
        );
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
