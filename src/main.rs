use macroquad::prelude::*;
use ::rand::prelude::*;

mod boundary;
mod ray;
mod player;

//use ray::ray::*;
use boundary::boundary::*;
use player::player::*;

// Window Bounds: 800, 600
#[macroquad::main("Raycaster")]
async fn main() {

    let mut boundaries: Vec<Boundary> = vec![];
    for _ in 0..5 {
        boundaries.push(Boundary::new(
            Vec2::new(random::<f32>() * screen_width(), random::<f32>() * screen_height()),
            Vec2::new(random::<f32>() * screen_width(), random::<f32>() * screen_height())
        ));
    }


    let mut player = Player::new(
        Vec2::new(screen_width() / 2f32, screen_height() / 2f32)
    );
    

    loop {
        //r.set_direction(Vec2::new(mouse_position().0 - r.position.x, mouse_position().1 - r.position.y));

        clear_background(BLACK);

        //let cast_result = r.cast(&b);
        //match cast_result {
        //    Some(result) => draw_circle(result.x, result.y, 5f32, WHITE),
        //    None => {},
        //}

        player.cast_rays(&boundaries);
        player.update();

        player.draw();

        for b in &boundaries {
            b.draw();
        }

        next_frame().await
    }
}