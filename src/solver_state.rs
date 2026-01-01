use piston_window::{Context, G2d, Glyphs};
use rfd::FileDialog;
use crate::button::ButtonRect;
use crate::app_state::AppState;

pub fn init_solver_state() {

}

pub fn press_button_solver(choose_file: &ButtonRect, solve: &ButtonRect, clear_btn: &ButtonRect, mouse: [f64; 2], app_state: &mut AppState) {
    if choose_file.is_hovered(mouse) {
        let file = FileDialog::new().add_filter("text", &["txt"]).pick_file();
        app_state.set_file_chosen(file);
        app_state.set_click_on_file(true);
    }
    if solve.is_hovered(mouse) {
        if let Some(path) = app_state.get_file_chosen() {
            if let Ok(grid) = crate::display::read_file(path) {
                app_state.set_grid(grid);
            }
        }
    }
    if clear_btn.is_hovered(mouse) {
        app_state.grid_mut().set_grid([[0; 9]; 9]);
        app_state.grid_mut().set_grid_ori([[false; 9]; 9]);
        app_state.set_click_on_file(false);
    }
}

pub fn display_solver(
    choose_file: &ButtonRect,
    solve: &ButtonRect,
    clear_btn: &ButtonRect,
    app_state: &AppState,
    c: &Context,
    g: &mut G2d,
    glyphs: &mut Glyphs,
) {
    choose_file.draw(c, g, glyphs, choose_file.is_hovered(app_state.get_mousse_pos()));
    solve.draw(c, g, glyphs, solve.is_hovered(app_state.get_mousse_pos()));
    clear_btn.draw(c, g, glyphs, clear_btn.is_hovered(app_state.get_mousse_pos()));
}

