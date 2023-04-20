use macroquad::prelude::*;

use crate::ray::ray::*;
use crate::boundary::boundary::*;

pub struct Player {
    position: Vec2,
    rays: Vec<Ray>
}

impl Player {
    pub fn new(position: Vec2) -> Self {
        let mut p = Player { 
            position,
            rays: vec![]
        };
        for angle in (0..360).step_by(10) {
            p.rays.push(Ray::new(
                p.position,
                angle as f32
            ));
        }
        p
    }

    pub fn update(&mut self) {
        self.position = Vec2::new(mouse_position().0, mouse_position().1);
    }

    pub fn draw(&self) {
        draw_circle(self.position.x, self.position.y, 15f32, WHITE);
    }

    pub fn cast_rays(&mut self, boundaries: &[Boundary]) {
        for mut ray in &mut self.rays {
            let mut closest = self.position;
            let mut _best_dist = 999f32; 
            for wall in boundaries {   
                ray.position = self.position;
                let intersect = ray.cast(wall);
                match intersect {
                    Some(point) => {
                        let dist = self.position.distance(point);

                        if dist < _best_dist {
                            _best_dist = dist;
                            closest = point;
                        }
                    },
                    None => {}
                }
            }
            draw_circle(closest.x, closest.y, 5f32, WHITE);
            draw_line(self.position.x, self.position.y, closest.x, closest.y, 0.5f32, WHITE);
        }
    }
}