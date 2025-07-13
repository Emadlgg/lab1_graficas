use raylib::prelude::*;

pub struct Framebuffer {
    image: Image,
    current_color: Color,
}

impl Framebuffer {
    pub fn new(width: i32, height: i32) -> Self {
        Framebuffer {
            image: Image::gen_image_color(width, height, Color::BLACK),
            current_color: Color::WHITE,
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.image = Image::gen_image_color(
            self.image.width,
            self.image.height,
            color
        );
    }

    pub fn clear(&mut self) {
        let bg_color = self.image.get_color(0, 0);
        self.image = Image::gen_image_color(
            self.image.width,
            self.image.height,
            bg_color
        );
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn draw_pixel(&mut self, x: i32, y: i32) {
        if x >= 0 && x < self.image.width && y >= 0 && y < self.image.height {
            self.image.draw_pixel(x, y, self.current_color);
        }
    }

    pub fn render_to_file(&self, filename: &str) {
        self.image.export_image(filename);
    }
}