use std::f32::consts::PI;

use macroquad::prelude::*;

use crate::helpers;

pub struct OrbitCamera {
    target: Vec3,
    distance: f32,
    pitch: f32,
    yaw: f32,
    min_pitch: f32,
    max_pitch: f32,
    sensitivity: f32,
    scroll_sens: f32,
    fov: f32
}

impl OrbitCamera {
    pub fn new(target: Vec3, distance: f32) -> Self {
        // defaults
        Self {
            target,
            distance,
            pitch: 1.0,
            yaw: 1.0,
            min_pitch: 0.0 + 0.0001,
            max_pitch: PI - 0.0001,
            sensitivity: 1.0,
            scroll_sens: 0.5,
            fov: (45.0_f32).to_radians()
        }
    }

    pub fn update(&mut self) {
        if mouse_wheel().1 != 0.0 {
            self.distance += self.scroll_sens * -mouse_wheel().1.signum();
        }
        
        if is_mouse_button_down(MouseButton::Left) {
            self.yaw -= mouse_delta_position().x * self.sensitivity;
            self.pitch += mouse_delta_position().y * self.sensitivity;

            self.pitch = self.pitch.clamp(self.min_pitch, self.max_pitch);
        }
    }

    pub fn to_macro_camera(&self) -> Camera3D {
        let pos = helpers::to_spherical(
            self.distance, 
            self.yaw, 
            self.pitch
        );

        return Camera3D {
            position: helpers::to_xzy(self.target + pos),
            target: helpers::to_xzy(self.target),
            up: vec3(0.0,1.0,0.0),
            fovy: self.fov,
            ..Default::default()
        };
    }
}