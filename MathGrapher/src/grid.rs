use crate::general_shapes;

use macroquad::prelude::*;
use std::f32::consts::PI;

static RANGE: u32 = 20;
static FRANGE: f32 = RANGE as f32;

pub fn draw_graphing_area() {
        // Draw grid... duh
        draw_grid(RANGE, 1.0, DARKGRAY, LIGHTGRAY);

        // add numbers later

        // Major axis arrows
        general_shapes::draw_arrow_rot(vec3(0.0, -FRANGE/2.0, 0.0), vec3(0.0, 0.0, 0.0), 0.08, 0.25, 20.0, 0.95, None, LIGHTGRAY);
        general_shapes::draw_arrow_rot(vec3(-FRANGE/2.0, 0.0, 0.0), vec3(0.0, 0.0, -PI / 2.0), 0.08, 0.25, 20.0, 0.95, None, LIGHTGRAY);
        general_shapes::draw_arrow_rot(vec3(0.0, 0.0, FRANGE/2.0), vec3(-PI / 2.0, 0.0, 0.0), 0.08, 0.25, 20.0, 0.95, None, LIGHTGRAY);
}