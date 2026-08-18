use crate::{general_shapes::{self, apply_matrix}, helpers::matrix_from_xyz};

use macroquad::prelude::*;
use std::f32::consts::PI;

const  RANGE: i32 = 20;
const  FRANGE: f32 = RANGE as f32;

// Center Alignment, kinda

// should probably turn x and y into easier helper functions
fn draw_x_labels(text_scale: f32, steps: usize) {
  const ZERO_LABEL_OFFSET: f32 = -0.5;

  let font_size = text_scale as u16;
  let font_scale = 1.0 / text_scale;

  // To account for the offset given by the character "-".
  // This way we can have the number be the center.
  let minus_width = 
    measure_text("-", Default::default(), font_size, font_scale).width;

  for i in ((-RANGE/2)..=(RANGE/2)).step_by(steps) {
    let text = i.to_string();

    let dimensions = 
      measure_text(&text, Default::default(), font_size, font_scale);

    let x = match i {
      0 => ZERO_LABEL_OFFSET,
      i if i < 0 => i as f32 - (dimensions.width + minus_width) / 2.0,
      _ => i as f32 - dimensions.width / 2.0
    };

    let translation = vec3(
      x,
      0.0,
      0.5 - dimensions.height / 2.0 + dimensions.offset_y
    );

    let matrix = matrix_from_xyz(
      translation, 
      vec3(PI/2.0,0.0,0.0), 
      vec3(font_scale, font_scale, font_scale)
    );

    apply_matrix(matrix, || {
      draw_text(text, 0.0, 0.0, text_scale, RED);
    });
  }
}

// Right Alignment
fn draw_y_labels(text_scale: f32, steps: usize) {
  const SLIGHT_X_OFFSET: f32 = 0.05;

  let font_size = text_scale as u16;
  let font_scale = 1.0 / text_scale;

  for i in ((-RANGE/2)..(RANGE/2)).step_by(steps) {
    if i == 0 {
      continue;
    }

    let text = i.to_string();

    let dimensions = 
      measure_text(&text, Default::default(), font_size, font_scale);

    let translation = vec3(
      -dimensions.width - SLIGHT_X_OFFSET,
      0.0,
      -i as f32 + dimensions.height / 2.0
    );

    let matrix = matrix_from_xyz(
      translation, 
      vec3(PI/2.0,0.0,0.0), 
      vec3(font_scale, font_scale, font_scale)
    );

    apply_matrix(matrix, || {
      draw_text(text, 0.0, 0.0, text_scale, RED);
    });
  }
}

pub fn draw_graphing_area() {
  // Draw grid... duh
  draw_grid(RANGE as u32, 1.0, DARKGRAY, LIGHTGRAY);

  // Draw number labels
  let text_scale = 25.0;
  let steps = 2;
  draw_x_labels(text_scale, steps);
  draw_y_labels(text_scale, steps);

  // Major axis arrows
  general_shapes::draw_arrow_rot(vec3(0.0, -FRANGE/2.0, 0.0), vec3(0.0, 0.0, 0.0), 0.08, 0.25, 20.0, 0.95, None, LIGHTGRAY);
  general_shapes::draw_arrow_rot(vec3(-FRANGE/2.0, 0.0, 0.0), vec3(0.0, 0.0, -PI / 2.0), 0.08, 0.25, 20.0, 0.95, None, LIGHTGRAY);
  general_shapes::draw_arrow_rot(vec3(0.0, 0.0, FRANGE/2.0), vec3(-PI / 2.0, 0.0, 0.0), 0.08, 0.25, 20.0, 0.95, None, LIGHTGRAY);
}