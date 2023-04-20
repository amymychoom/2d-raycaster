use macroquad::prelude::*;

use crate::boundary::boundary::*;

pub struct Ray {
    pub position: Vec2,
    pub direction: Vec2,
}

impl Ray {
    pub fn new(position: Vec2, angle: f32) -> Self {
        Ray {
            position,
            direction: Vec2::from_angle(f32::to_radians(angle)),
        }
    }

    pub fn _draw(&self) {
        draw_line(self.position.x, self.position.y, self.position.x + self.direction.x * 25f32, self.position.y + self.direction.y * 25f32, 0.5f32, WHITE);
    }

    //pub fn set_direction(&mut self, direction: Vec2) {
    //    self.direction = direction.normalize();
    //}

    pub fn cast(&self, boundary: &Boundary) -> Option<Vec2> {
        // ref: https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection
        // In short, we must calculate a t value and a u value, the values of which
        // will tell us whether or not our two lines are intersecting. 

        // Once we have that figured out, we can find their intersection point.

        let x_1 = boundary.point_a.x;
        let y_1 = boundary.point_a.y;
        let x_2 = boundary.point_b.x;
        let y_2 = boundary.point_b.y;

        let x_3 = self.position.x;
        let y_3 = self.position.y;
        let x_4 = self.position.x + self.direction.x;
        let y_4 = self.position.y + self.direction.y;

        let denominator = (x_1 - x_2) * (y_3 - y_4) - (y_1 - y_2) * (x_3 - x_4);

        // If the denominator is zero, then the lines are parallel
        if denominator == 0.0 {
            None
        } else {
            let t = ((x_1 - x_3) * (y_3 - y_4) - (y_1 - y_3) * (x_3 - x_4)) / denominator;
            let u = -((x_1 - x_2) * (y_1 - y_3) - (y_1 - y_2) * (x_1 - x_3)) / denominator;

            // The formula technically stipulates an intersection means 0 < t < 1 & 0 < u < 1,
            // however, for our purposes we only care about the positive end of the ray, so we can discard
            // the upper bound of u (the value which corresponds to the second line segment i.e. the ray)
            // video explanation: https://youtu.be/TOEi6T2mtHo?t=683
            if (t > 0.0) && (t < 1.0) && (u > 0.0) {
                let intersection_point = Vec2::new(
                    x_1 + t * (x_2 - x_1),
                    y_1 + t * (y_2 - y_1)
                );
                Some(intersection_point)
            } else {
                None
            }
        }
    }
}