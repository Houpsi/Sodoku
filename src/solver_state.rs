use piston_window::{Context, G2d, Glyphs};
use rfd::FileDialog;
use crate::button::ButtonRect;
use crate::app_state::AppState;
use crate::display::{State, BTN_BG, BTN_HOVER};

pub struct Solver {
    choose_file: ButtonRect,
    solver: ButtonRect,
    clear_grid: ButtonRect,
    back: ButtonRect,
}

impl Solver {
    pub fn new() ->Self {
        Self {
            choose_file: ButtonRect::flat(40.0, 60.0, 110.0, 38.0, "Load", [0.61, 0.30, 0.8, 1.0], [0.87, 0.66, 1.0, 1.0]),
            solver: ButtonRect::flat(160.0, 60.0, 110.0, 38.0, "Solve", [0.61, 0.30, 0.8, 1.0], [0.87, 0.66, 1.0, 1.0]),
            clear_grid: ButtonRect::flat(280.0, 60.0, 110.0, 38.0, "Clear", [0.61, 0.30, 0.8, 1.0], [0.87, 0.66, 1.0, 1.0]),
            back: ButtonRect::flat(10.0, 10.0, 40.0, 38.0, " < ", BTN_BG, BTN_HOVER),
        }
    }

    pub fn press_button_solver(&self, mouse: [f64; 2], app_state: &mut AppState, state: &mut State) {
        if self.choose_file.is_hovered(mouse) {
            let file = FileDialog::new().add_filter("text", &["txt"]).pick_file();
            app_state.set_file_chosen(file);
            app_state.set_click_on_file(true);
        }
        if self.solver.is_hovered(mouse) {
            if let Some(path) = app_state.get_file_chosen() {
                if let Ok(grid) = crate::display::read_file(path) {
                    app_state.set_grid(grid);
                }
            }
        }
        if self.clear_grid.is_hovered(mouse) {
            app_state.clear_grid();
            app_state.clear_grid_ori();
            app_state.set_click_on_file(false);
        }
        if self.back.is_hovered(mouse) {
            app_state.clear_selected_cell();
            app_state.clear_grid();
            app_state.clear_grid_ori();
            *state = State::Menu
        }
    }

    pub fn display_solver(&self,
        app_state: &AppState,
        c: &Context,
        g: &mut G2d,
        glyphs: &mut Glyphs,
    ) {
        self.choose_file.draw(c, g, glyphs, self.choose_file.is_hovered(app_state.get_mouse_pos()), 18);
        self.solver.draw(c, g, glyphs, self.solver.is_hovered(app_state.get_mouse_pos()), 18);
        self.clear_grid.draw(c, g, glyphs, self.clear_grid.is_hovered(app_state.get_mouse_pos()), 18);
        self.back.draw(c, g, glyphs, self.back.is_hovered(app_state.get_mouse_pos()), 18);
    }
}



