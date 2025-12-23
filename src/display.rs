// Display proprement la grid
// pourquoi pas faire de l'ui with piston

use piston_window::{clear, rectangle, text, Button, G2d, Glyphs, MouseCursorEvent, MouseRelativeEvent, PistonWindow, PressEvent, Transformed, WindowSettings};
use piston_window::types::Color;
use crate::grid::Grid;
use crate::button::ButtonRect;
use rfd::FileDialog;

pub fn init_window(grid: &Grid) {
    let mut window: PistonWindow =
        WindowSettings::new("Sudoku Solver", [640, 480])
            .exit_on_esc(true)
            .build()
            .unwrap();
    let mut glyphs = window.load_font("font.ttf").unwrap();
    let mut pos_mousse: [f64; 2] = [0.0, 0.0];
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
        if let Some(Button::Mouse(button)) = e.press_args() {
            if choose_file.is_hovered(pos_mousse) {
                let files = FileDialog::new()
                    .add_filter("text", &["txt"])
                    .set_directory("/home/heleneh")// TO DO change the path to a personalize one
                    .pick_file();
                print!("file choose : {:?}", files);
            }
            if solve_sodoku.is_hovered(pos_mousse) {

            }
        }

        window.draw_2d(&e, |c, g, device| {
            clear([1.0; 4], g);

            // draw_text(
            //     &c,
            //     g,
            //     &mut glyphs,
            //     [0.0, 0.0, 0.0, 1.0],
            //     [3, 20],
            //     "bonjour",
            // );
            display_grid_piston(grid, &c, g, &mut glyphs);
            choose_file.draw(&c, g, &mut glyphs, choose_file.is_hovered(pos_mousse));
            solve_sodoku.draw(&c, g, &mut glyphs, choose_file.is_hovered(pos_mousse));

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

