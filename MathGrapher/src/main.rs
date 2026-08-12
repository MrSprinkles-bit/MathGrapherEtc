mod general_shapes;

use std::f32::consts::PI;

use macroquad::prelude::*;

fn spherical_to_cartesian(radius: f32, theta: f32, phi: f32) -> Vec3 {
    return vec3(
        radius * theta.sin() * phi.cos(),
        radius * theta.sin() * phi.sin(),
        radius * theta.cos()
    )
}

#[macroquad::main("Testerrrrrr")]
async fn main() {
    let radius = 35.0;
    let theta = 1.0;
    let phi = 1.0;

    loop {
        clear_background(WHITE);

        set_camera(&Camera3D {
            position: spherical_to_cartesian(radius, theta, phi),
            up: vec3(0.0, 1.0, 0.0),
            target: vec3(0.0, 0.0, 0.0),
            ..Default::default()
        });

        // All temp. ASP this will all be a one call draw function.
        // Draw grid... duh
        draw_grid(20, 1.0, DARKGRAY, LIGHTGRAY);

        // Major axis arrows
        general_shapes::draw_arrow_rot(vec3(0.0, -10.0, 0.0), vec3(0.0, 0.0, 0.0), 0.08, 0.25, 20.0, 0.95, None, LIGHTGRAY);
        general_shapes::draw_arrow_rot(vec3(-10.0, 0.0, 0.0), vec3(0.0, 0.0, -PI / 2.0), 0.08, 0.25, 20.0, 0.95, None, LIGHTGRAY);
        general_shapes::draw_arrow_rot(vec3(0.0, 0.0, 10.0), vec3(-PI / 2.0, 0.0, 0.0), 0.08, 0.25, 20.0, 0.95, None, LIGHTGRAY);

        set_default_camera();
        draw_text("I LOVE DANNYYYY", screen_height() / 2.0 - 60.0, 20.0, 30.0, DARKGRAY);

        next_frame().await;
    }
}