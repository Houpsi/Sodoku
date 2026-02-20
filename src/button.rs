use piston_window::{rectangle, text, G2d, Glyphs, Transformed};
use piston_window::types::{Color, FontSize};

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

    pub fn draw(&self, c: &piston_window::Context, g: &mut G2d, glyphs: &mut Glyphs, hovered: bool, size: FontSize) {
        let color = if hovered { self.color_hovered } else { self.color };
        let y_offset = if hovered { -2.0 } else { 0.0 };
        let by = self.y + y_offset;

        rectangle([0.0, 0.0, 0.0, 0.25], [self.x + 4.0, by + 4.0, self.w, self.h], c.transform, g);

        rectangle(color, [self.x, by, self.w, self.h], c.transform, g);

        piston_window::Rectangle::new_border([0.15, 0.15, 0.15, 1.0], 1.0)
            .draw([self.x, by, self.w, self.h], &c.draw_state, c.transform, g);

        let approx_text_width = self.label.len() as f64 * (size as f64 * 0.6);
        let text_x = self.x + (self.w - approx_text_width) / 2.0;
        let text_y = by + self.h / 2.0 + (size as f64 * 0.35);

        text::Text::new_color([1.0, 1.0, 1.0, 1.0], size)
            .draw(&self.label, glyphs, &c.draw_state, c.transform.trans(text_x, text_y), g)
            .unwrap();
    }

    pub fn flat(x: f64, y: f64, w: f64, h: f64, label: &str, color: Color, color_hovered: Color) -> Self {
        Self {
            x,
            y,
            w,
            h,
            label: label.to_string(),
            color,
            color_hovered,
        }
    }
}
