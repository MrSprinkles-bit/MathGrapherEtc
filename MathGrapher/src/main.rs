mod grid;
mod general_shapes;
mod helpers;

use std::f32::consts::PI;

use macroquad::prelude::*;

use crate::grid::draw_graphing_area;

fn camera_spherical_to_cartesian(radius: f32, mut pitch: f32, yaw: f32) -> Vec3 {
    pitch = clamp(pitch, 0.0 + 0.0001, PI - 0.0001);
    println!("{}", pitch);
    return vec3(
        radius * pitch.sin() * yaw.cos(),
        radius * pitch.sin() * yaw.sin(),
        radius * pitch.cos()
    )
}

fn to_xzy(vec: Vec3) -> Vec3 {
    return vec.xzy();
}

#[macroquad::main("Testerrrrrr")]
async fn main() {
    let mut radius = 35.0;
    let mut theta = 1.0;
    let mut phi = 1.0;

    loop {
        clear_background(WHITE);

        if mouse_wheel().1 != 0.0 {
            radius += 0.5 * -mouse_wheel().1.signum();
        }

        if is_mouse_button_down(MouseButton::Left) {
            phi -= mouse_delta_position().x;
            theta += mouse_delta_position().y;
        }

        set_camera(&Camera3D {
            position: to_xzy(camera_spherical_to_cartesian(radius, theta, phi)),
            up: vec3(0.0, 1.0, 0.0),
            target: vec3(0.0, 0.0, 0.0),
            ..Default::default()
        });

        draw_graphing_area();

        set_default_camera();
        draw_text("I LOVE DANNYYYYY", screen_height() / 2.0 - 60.0, 20.0, 30.0, DARKGRAY);

        next_frame().await;
    }
}