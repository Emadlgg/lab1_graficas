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

    // --- POLÍGONO 3 ---
    let polygon3 = Polygon::new(
        vec![
            Vector2::new(377.0, 249.0),
            Vector2::new(411.0, 197.0),
            Vector2::new(436.0, 249.0),
        ],
        Color::RED,
        Color::WHITE,
    );
    polygon3.draw(&mut framebuffer, background_color);

    let output_file = "output_polygon3.bmp";
    framebuffer.render_to_file(output_file);
}