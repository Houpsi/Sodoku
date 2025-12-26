// Display proprement la grid
// pourquoi pas faire de l'ui with piston

use std::fs;
use std::path::PathBuf;
use piston_window::{clear, text, Button, G2d, Glyphs, MouseCursorEvent, PistonWindow, PressEvent, Transformed, WindowSettings};
use piston_window::types::Color;
use crate::grid::Grid;
use crate::button::ButtonRect;
use rfd::FileDialog;
use crate::{parser, solver};

pub fn init_window() {
    let mut window: PistonWindow =
        WindowSettings::new("Sudoku Solver", [640, 480])
            .exit_on_esc(true)
            .build()
            .unwrap();
    let mut glyphs = window.load_font("font.ttf").unwrap();
    let mut pos_mousse: [f64; 2] = [0.0, 0.0];
    let mut grid= Grid {
        grid : [[0; 9]; 9],
    };
    let mut file_chosen: Option<PathBuf> = None;
    let choose_file = ButtonRect {
        x: 00.0,
        y: 20.0,
        w: 85.0,
        h: 35.0,
        label: "file".to_string(),
        color_hovered: [0.7, 0.7, 0.7, 1.0],
        color: [0.5, 0.5, 0.5, 1.0],
    };
    let solve_sodoku = ButtonRect {
        x: 100.0,
        y: 20.0,
        w: 100.0,
        h: 35.0,
        label: "Solve".to_string(),
        color_hovered: [1.0, 0.0, 0.0, 1.0],
        color: [0.0, 1.0, 0.0, 1.0],
    };

    while let Some(e) = window.next() {
        e.mouse_cursor(|pos| {
            pos_mousse = pos;
        });
        if let Some(Button::Mouse(_button)) = e.press_args() {
            if choose_file.is_hovered(pos_mousse) {
                let files = FileDialog::new()
                    .add_filter("text", &["txt"])
                    .set_directory("/home/heleneh/Documents")// TO DO change the path to a personalize one
                    .pick_file();
                print!("file choose : {:?}", files);
                file_chosen = files;
            }
            if solve_sodoku.is_hovered(pos_mousse)  {
                if let Some(new_grid) = read_file(file_chosen.clone()) {
                    grid = new_grid;
                }
            }
        }

        window.draw_2d(&e, |c, g, device| {
            clear([1.0; 4], g);

            display_grid_piston(&grid, &c, g, &mut glyphs);

            choose_file.draw(&c, g, &mut glyphs, choose_file.is_hovered(pos_mousse));
            solve_sodoku.draw(&c, g, &mut glyphs, solve_sodoku.is_hovered(pos_mousse));

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
    if !check_grid_not_empty(grid.get_grid()) {
        return;
    }
    let mut offset_y = 130.0;
    for (y, row) in grid.get_grid().iter().enumerate() {
        let mut offset_x = 230.0;
        for (x, cell) in row.iter().enumerate() {
            draw_text(
                &c,
                g,
                glyphs,
                [0.0, 0.0, 0.0, 1.0],
                [
                    (x as f64 * 20.0 + offset_x) as u32,
                    (y as f64 * 20.0 + offset_y) as u32,
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

pub fn read_file(file_path: Option<PathBuf>) -> Option<Grid> {
    let file_path = match file_path {
        Some(path) => path,
        None => {
            eprintln!("Aucun fichier sélectionné");
            return None;
        }
    };
    let contents = fs::read_to_string(file_path.clone())
        .expect("Should have been able to read the file");
    let mut my_grid = Grid {
        grid: [[0; 9]; 9],
    };

    match parser::parser_file(&contents, Some('.')) {
        Ok(grid) => {
            my_grid.grid = grid;
            solver::is_valid(&mut my_grid, 0);
        }
        Err(err) => {
            eprintln!("Error parsing : {}", err);
        }
    }

    Some(my_grid)
}

pub fn check_grid_not_empty(grid: [[u32; 9]; 9]) -> bool {
    for (_y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell != 0 {
                return true
            }
        }
    }
    false
}