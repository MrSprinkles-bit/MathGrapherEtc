use crate::{general_shapes::{self, apply_matrix}, utils::matrix_from_xyz};

use macroquad::prelude::*;
use std::f32::consts::PI;

static RANGE: i32 = 20;
static FRANGE: f32 = RANGE as f32;

pub fn draw_graphing_area() {
        // Draw grid... duh
        draw_grid(RANGE as u32, 1.0, DARKGRAY, LIGHTGRAY);

        let text_scale = 25.0;
        let steps = 2;
        for i in ((-RANGE/2)..(RANGE/2 + 1)).step_by(steps) {
          let matrix = matrix_from_xyz(
            vec3(i as f32 - 0.5, 0.0, 1.0), 
            vec3(PI/2.0,0.0,0.0), 
            vec3(1.0/text_scale, 1.0/text_scale, 1.0/text_scale)
          );
          apply_matrix(matrix, || {
            draw_text(i.to_string(), i as f32, 0.0, text_scale, RED);
          });
        }

        // Major axis arrows
        general_shapes::draw_arrow_rot(vec3(0.0, -FRANGE/2.0, 0.0), vec3(0.0, 0.0, 0.0), 0.08, 0.25, 20.0, 0.95, None, LIGHTGRAY);
        general_shapes::draw_arrow_rot(vec3(-FRANGE/2.0, 0.0, 0.0), vec3(0.0, 0.0, -PI / 2.0), 0.08, 0.25, 20.0, 0.95, None, LIGHTGRAY);
        general_shapes::draw_arrow_rot(vec3(0.0, 0.0, FRANGE/2.0), vec3(-PI / 2.0, 0.0, 0.0), 0.08, 0.25, 20.0, 0.95, None, LIGHTGRAY);
}