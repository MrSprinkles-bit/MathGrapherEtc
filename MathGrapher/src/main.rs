mod grid;
mod general_shapes;
mod helpers;
mod orbit_camera;
mod function_grapher;

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

    fn func(x: f32) -> f32 {
        return x*x;
    }

    loop {
        clear_background(WHITE);

        camera.update();
        set_camera(&camera.to_macro_camera());

        draw_graphing_area(20);

        function_grapher::draw_graph_1d(10, 10, RED, func);

        set_default_camera();
        draw_text("I LOVE DANNYYYYY", screen_height() / 2.0 - 60.0, 20.0, 30.0, DARKGRAY);

        next_frame().await;
    }
}