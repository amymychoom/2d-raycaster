use macroquad::prelude::*;

pub struct Boundary {
    pub point_a: Vec2,
    pub point_b: Vec2,
}

impl Boundary {
    pub fn new(point_a: Vec2, point_b: Vec2 ) -> Self {
        Boundary {
            point_a,
            point_b
        }
    }

    pub fn draw(&self) {
        draw_line(self.point_a.x, self.point_a.y, self.point_b.x, self.point_b.y, 2.5f32, WHITE);
    }
}