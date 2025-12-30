use piston_window::{rectangle, text, G2d, Glyphs, Transformed};
use piston_window::types::Color;

pub struct ButtonRect {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
    pub label: String,
    pub color_hovered : Color,
    pub color : Color,
}

impl ButtonRect {
    pub fn is_hovered(&self, mouse: [f64; 2]) -> bool {
        mouse[0] >= self.x
            && mouse[0] <= self.x + self.w
            && mouse[1] >= self.y
            && mouse[1] <= self.y + self.h
    }

    pub fn draw(&self, c: &piston_window::Context, g: &mut G2d, glyphs: &mut Glyphs, hovered: bool) {
        let color: Color = if hovered {
            self.color_hovered
        } else {
            self.color
        };
        rectangle(color, [self.x, self.y, self.w, self.h], c.transform, g);

        text::Text::new_color([0.0, 0.0, 0.0, 1.0], 18)
            .draw(
                &self.label,
                glyphs,
                &c.draw_state,
                c.transform.trans(self.x + 10.0, self.y + self.h / 2.0 + 6.0),
                g,
            )
            .unwrap();
    }

    pub fn flat(x: f64, y: f64, w: f64, h: f64, label: &str) -> Self {
        Self {
            x,
            y,
            w,
            h,
            label: label.to_string(),
            color: [0.61, 0.30, 0.8, 1.0],
            color_hovered: [0.87, 0.66, 1.0, 1.0],
        }
    }
}
