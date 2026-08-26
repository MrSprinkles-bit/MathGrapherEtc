mod grid;
mod general_shapes;
mod helpers;
mod orbit_camera;

use macroquad::prelude::*;

use crate::general_shapes::draw_n_vertex_line;
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
        return x;
    }

    let res = 10;

    loop {
        clear_background(WHITE);

        camera.update();
        set_camera(&camera.to_macro_camera());

        draw_graphing_area(20);

        let mut lines: Vec<Vertex> = Vec::new();

        for x in (-20*res/2)..=(20*res/2) {
            let xf = x as f32;
            let resf = res as f32;
            // y=f(x)
            let z = -func(xf/resf);
            //(x,f(x)) or i guess technically (x,0.,f(x))
            lines.push(
                Vertex::new2(vec3(xf/resf, 0.,z), vec2(0.,0.), RED)
            );
        }
        draw_n_vertex_line(&lines);

        set_default_camera();
        draw_text("I LOVE DANNYYYYY", screen_height() / 2.0 - 60.0, 20.0, 30.0, DARKGRAY);

        next_frame().await;
    }
}