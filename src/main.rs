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

    // --- POLÍGONO 2  ---
    let polygon2 = Polygon::new(
        vec![
            Vector2::new(321.0, 335.0),
            Vector2::new(288.0, 286.0),
            Vector2::new(339.0, 251.0),
            Vector2::new(374.0, 302.0),
        ],
        Color::BLUE,
        Color::WHITE,
    );
    polygon2.draw(&mut framebuffer, background_color);

    let output_file = "outputp_polygon2.bmp";
    framebuffer.render_to_file(output_file);
}