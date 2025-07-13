mod framebuffer;
mod line;
mod polygon;

use raylib::prelude::*;
use framebuffer::Framebuffer;
use polygon::Polygon;

fn main() {
    let width = 800;
    let height = 600;
    let mut framebuffer = Framebuffer::new(width, height);

    let background_color = Color::new(50, 50, 100, 255);
    framebuffer.set_background_color(background_color);
    framebuffer.clear();



    let output_file = "output.bmp";
    framebuffer.render_to_file(output_file);
}