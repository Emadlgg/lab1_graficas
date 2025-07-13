use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use crate::line::line;

pub struct Polygon {
    points: Vec<Vector2>,
    holes: Vec<Vec<Vector2>>,
    fill_color: Color,
    border_color: Color,
}

impl Polygon {
    pub fn new(points: Vec<Vector2>, fill_color: Color, border_color: Color) -> Self {
        Polygon {
            points,
            holes: Vec::new(),
            fill_color,
            border_color,
        }
    }

    pub fn add_hole(&mut self, hole_points: Vec<Vector2>) {
        self.holes.push(hole_points);
    }

    pub fn draw(&self, framebuffer: &mut Framebuffer, background_color: Color) {
        self.draw_fill(framebuffer, background_color);
        self.draw_borders(framebuffer);
    }

    fn draw_fill(&self, framebuffer: &mut Framebuffer, background_color: Color) {
        if self.points.len() < 3 {
            return;
        }

        let (min_y, max_y) = self.find_vertical_bounds();

        for y in min_y..=max_y {
            let mut intersections = Vec::new();

            // Intersecciones con el polígono principal
            self.find_polygon_intersections(y as f32, &self.points, &mut intersections);

            // Intersecciones con los agujeros (restaremos estas áreas)
            for hole in &self.holes {
                self.find_polygon_intersections(y as f32, hole, &mut intersections);
            }

            intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());

            let mut inside = false;
            let mut prev_x = -1;

            for x in intersections {
                if inside {
                    // Dibujamos desde prev_x hasta x
                    let color = if self.is_in_hole(prev_x as f32, y as f32) {
                        background_color
                    } else {
                        self.fill_color
                    };
                    
                    framebuffer.set_current_color(color);
                    for px in prev_x..=x as i32 {
                        framebuffer.draw_pixel(px, y);
                    }
                }
                inside = !inside;
                prev_x = x as i32;
            }
        }
    }

    fn is_in_hole(&self, x: f32, y: f32) -> bool {
        for hole in &self.holes {
            if self.point_in_polygon(x, y, hole) {
                return true;
            }
        }
        false
    }

    fn point_in_polygon(&self, x: f32, y: f32, polygon: &[Vector2]) -> bool {
        let mut inside = false;
        let n = polygon.len();
        
        for i in 0..n {
            let j = (i + 1) % n;
            let xi = polygon[i].x;
            let yi = polygon[i].y;
            let xj = polygon[j].x;
            let yj = polygon[j].y;

            let intersect = ((yi > y) != (yj > y)) &&
                (x < (xj - xi) * (y - yi) / (yj - yi) + xi);
            
            if intersect {
                inside = !inside;
            }
        }
        
        inside
    }

    fn find_polygon_intersections(
        &self,
        scanline_y: f32,
        polygon: &[Vector2],
        intersections: &mut Vec<f32>,
    ) {
        for i in 0..polygon.len() {
            let j = (i + 1) % polygon.len();
            let start = polygon[i];
            let end = polygon[j];

            if (start.y > scanline_y && end.y > scanline_y) || 
               (start.y < scanline_y && end.y < scanline_y) {
                continue;
            }

            if start.y == end.y {
                continue;
            }

            let x_intersect = start.x + (scanline_y - start.y) * (end.x - start.x) / (end.y - start.y);
            
            if (x_intersect >= start.x.min(end.x)) && (x_intersect <= start.x.max(end.x)) {
                intersections.push(x_intersect);
            }
        }
    }

    fn find_vertical_bounds(&self) -> (i32, i32) {
        let mut min_y = self.points[0].y as i32;
        let mut max_y = self.points[0].y as i32;

        for point in &self.points {
            let y = point.y as i32;
            min_y = min_y.min(y);
            max_y = max_y.max(y);
        }

        (min_y, max_y)
    }

    fn draw_borders(&self, framebuffer: &mut Framebuffer) {
        framebuffer.set_current_color(self.border_color);
        
        // Dibujar borde principal
        for i in 0..self.points.len() {
            let start = self.points[i];
            let end = self.points[(i + 1) % self.points.len()];
            line(framebuffer, start, end);
        }
        
        // Dibujar bordes de agujeros
        for hole in &self.holes {
            for i in 0..hole.len() {
                let start = hole[i];
                let end = hole[(i + 1) % hole.len()];
                line(framebuffer, start, end);
            }
        }
    }
}