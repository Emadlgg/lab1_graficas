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

    // --- POLÍGONO 1 ---
    let polygon1 = Polygon::new(
        vec![
            Vector2::new(165.0, 380.0),
            Vector2::new(185.0, 360.0),
            Vector2::new(180.0, 330.0),
            Vector2::new(207.0, 345.0),
            Vector2::new(233.0, 330.0),
            Vector2::new(230.0, 360.0),
            Vector2::new(250.0, 380.0),
            Vector2::new(220.0, 385.0),
            Vector2::new(205.0, 410.0),
            Vector2::new(193.0, 383.0),
        ],
        Color::YELLOW,
        Color::WHITE,
    );
    polygon1.draw(&mut framebuffer, background_color);

    let output_file = "output.bmp";
    framebuffer.render_to_file(output_file);
}