mod grid;
mod general_shapes;
mod helpers;
mod orbit_camera;

use macroquad::prelude::*;

use crate::grid::draw_graphing_area;
use crate::orbit_camera::OrbitCamera;

#[macroquad::main("Testerrrrrr")]
async fn main() {
    let radius = 35.0;

    let mut camera = OrbitCamera::new(
        vec3(0.0,0.0,0.0), 
        radius
    );

    loop {
        clear_background(WHITE);

        camera.update();
        set_camera(&camera.to_macro_camera());

        draw_graphing_area();

        set_default_camera();
        draw_text("I LOVE DANNYYYYY", screen_height() / 2.0 - 60.0, 20.0, 30.0, DARKGRAY);

        next_frame().await;
    }
}