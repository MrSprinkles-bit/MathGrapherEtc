use crate::general_shapes;

use macroquad::prelude::*;
use std::f32::consts::PI;

static RANGE: i32 = 20;
static FRANGE: f32 = RANGE as f32;

pub fn draw_graphing_area() {
        // Draw grid... duh
        draw_grid(RANGE as u32, 1.0, DARKGRAY, LIGHTGRAY);

        // add numbers later
        //for i in -RANGE..RANGE {
          //  draw_text(stringify!(i), i as f32, 0.0, 2.0, RED);
        //}
        let text_scale = 25.0;
        let matrix = Mat4 { x_axis: vec4(1.0/text_scale,0.0,0.0,0.0), y_axis: vec4(0.0, -1.0/text_scale, 0.0, 0.0), z_axis: vec4(0.0, 0.0, 0.0, 0.0), w_axis: vec4(0.0,0.0,0.0,1.0) };
        let gl = unsafe { get_internal_gl() };
        gl.quad_gl.push_model_matrix(matrix);
        draw_text("-20", 0.0, 0.0,text_scale, RED);
        gl.quad_gl.pop_model_matrix();

        // Major axis arrows
        general_shapes::draw_arrow_rot(vec3(0.0, -FRANGE/2.0, 0.0), vec3(0.0, 0.0, 0.0), 0.08, 0.25, 20.0, 0.95, None, LIGHTGRAY);
        general_shapes::draw_arrow_rot(vec3(-FRANGE/2.0, 0.0, 0.0), vec3(0.0, 0.0, -PI / 2.0), 0.08, 0.25, 20.0, 0.95, None, LIGHTGRAY);
        general_shapes::draw_arrow_rot(vec3(0.0, 0.0, FRANGE/2.0), vec3(-PI / 2.0, 0.0, 0.0), 0.08, 0.25, 20.0, 0.95, None, LIGHTGRAY);
}