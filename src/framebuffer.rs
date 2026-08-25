use raylib::prelude::*;

pub struct Framebuffer {
    pub buffer: Image,
    background_color: Color,
    current_color: Color,
}

impl Framebuffer {
    pub fn new(width: i32, height: i32) -> Self {
        let background_color = Color::BLACK;

        let buffer = Image::gen_image_color(
            width,
            height,
            background_color,
        );

        Framebuffer {
            buffer,
            background_color,
            current_color: Color::WHITE,
        }
    }

    pub fn clear(&mut self) {
        self.buffer.clear_background(self.background_color);
    }

    pub fn set_pixel(&mut self, x: i32, y: i32) {
        if x >= 0
            && y >= 0
            && x < self.buffer.width
            && y < self.buffer.height
        {
            self.buffer.draw_pixel(x, y, self.current_color);
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn render_to_file(&self, file_path: &str) {
        self.buffer.export_image(file_path);
    }
}