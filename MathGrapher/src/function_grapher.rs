use macroquad::prelude::*;

use crate::general_shapes::draw_n_vertex_line;

/// Graphs a 1 dimensional function
/// e.g. y=x, y=e^x, y=x^3-x^2-sin(x)+1
pub fn draw_graph_1d<F>(range: i32, resolution: i32, color: Color, func: F) where F: Fn(f32) -> f32 {
    let mut lines: Vec<Vertex> = Vec::new();

    for x in (-range*resolution)..=(range*resolution) {
        let xf = x as f32;
        let resf = resolution as f32;

        let z = -func(xf/resf);
        lines.push(
            Vertex::new2(vec3(xf/resf, 0.,z), vec2(0.,0.), color)
        );
        // add if statement so if z is greater than range we have to go do a different thing.
        // also i need to make sure i can handle discontinuous jumps.
    }

    draw_n_vertex_line(&lines);
}