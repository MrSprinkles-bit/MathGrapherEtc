use std::vec;

use macroquad::prelude::*;

use crate::helpers::{matrix_from_quat, matrix_from_xyz};

// This was genuinely the hardest thing i've implimented yet - Sam 8/11/2026 12:41AM
pub trait MeshRot {
    fn apply_mesh_matrix(&self, matrix: Mat4);
}

impl MeshRot for macroquad::models::Mesh {

    /// Applies given transformation matrix to the mesh.
    /// Does not work for shapes, nor after the draw call.
    fn apply_mesh_matrix(&self, matrix: Mat4) {
        let gl = unsafe { get_internal_gl() };
        gl.quad_gl.push_model_matrix(matrix);
        gl.quad_gl.geometry(&self.vertices, &self.indices);
        gl.quad_gl.pop_model_matrix();
    }
}

/// Applies given transformation matrix to the shape.
pub fn apply_matrix<F>(matrix: Mat4, draw: F) where F: FnOnce() {
    let gl = unsafe { get_internal_gl() };
    gl.quad_gl.push_model_matrix(matrix);
    draw();
    gl.quad_gl.pop_model_matrix();
}

/// Draws a cylinder according to the given transformation matrix.
pub fn draw_cylinder_matrix(
    matrix: Mat4, 
    radius_top: f32, 
    radius_bottom: f32, 
    height: f32, 
    texture: Option<&Texture2D>, 
    color: Color
) {
    apply_matrix(matrix, || {draw_cylinder(
        vec3(0.0, 0.0, 0.0), 
        radius_top, 
        radius_bottom, 
        height, 
        texture, 
        color
    )});
}

/// Draws a cylinder at the given position, rotated by the given quaternion.
pub fn draw_cylinder_quat(
    position: Vec3, 
    quaternion: Quat, 
    radius_top: f32, 
    radius_bottom: f32, 
    height: f32, 
    texture: Option<&Texture2D>, 
    color: Color
) {
    let matrix = matrix_from_quat(position, quaternion, vec3(1.0, 1.0, 1.0));
    draw_cylinder_matrix(matrix, radius_top, radius_bottom, height, texture, color);
}

/// Draws a cylinder at the given position, rotated by the given radians.
pub fn draw_cylinder_rot(
    position: Vec3, 
    rotation: Vec3, 
    radius_top: f32, 
    radius_bottom: f32, 
    height: f32, 
    texture: Option<&Texture2D>, 
    color: Color
) {
    let matrix = matrix_from_xyz(position, rotation, vec3(1.0, 1.0, 1.0));
    draw_cylinder_matrix(matrix, radius_top, radius_bottom, height, texture, color);
}

pub fn draw_cone_matrix(
    matrix: Mat4,
    radius: f32,
    height: f32,
    texture: Option<&Texture2D>,
    color: Color
) {
    draw_cylinder_matrix(matrix, 0.0, radius, height, texture, color);
}

pub fn draw_cone_quat(
    position: Vec3,
    quaternion: Quat,
    radius: f32,
    height: f32,
    texture: Option<&Texture2D>,
    color: Color
) {
    draw_cylinder_quat(position, quaternion, 0.0, radius, height, texture, color);
}

pub fn draw_cone_rot(
    position: Vec3,
    rotation: Vec3,
    radius: f32,
    height: f32,
    texture: Option<&Texture2D>,
    color: Color
) {
    draw_cylinder_rot(position, rotation, 0.0, radius, height, texture, color);
}

pub fn draw_arrow_matrix(
    matrix: Mat4,
    radius_shaft: f32,
    radius_tip: f32,
    length: f32,
    ratio: f32,
    texture: Option<&Texture2D>,
    color: Color
) {
    let shaft_height = ratio * length;
    draw_cylinder_matrix(matrix, radius_shaft, radius_shaft, shaft_height, texture, color);

    let translation_matrix = mat4(
        vec4(1.0, 0.0, 0.0, 0.0),
        vec4(0.0, 1.0, 0.0, 0.0),
        vec4(0.0, 0.0, 1.0, 0.0),
        vec4(0.0, shaft_height, 0.0, 1.0)
    );
    let new_matrix = matrix * translation_matrix;
    let tip_height = (1.0 - ratio) * length;
    draw_cone_matrix(new_matrix, radius_tip, tip_height, texture, color);
}

pub fn draw_arrow_quat(
    position: Vec3,
    quaternion: Quat,
    radius_shaft: f32,
    radius_tip: f32,
    length: f32,
    ratio: f32,
    texture: Option<&Texture2D>,
    color: Color
) {
    let matrix = matrix_from_quat(position, quaternion, vec3(1.0, 1.0, 1.0));
    draw_arrow_matrix(matrix, radius_shaft, radius_tip, length, ratio, texture, color);
}

pub fn draw_arrow_rot(
    position: Vec3,
    rotation: Vec3,
    radius_shaft: f32,
    radius_tip: f32,
    length: f32,
    ratio: f32,
    texture: Option<&Texture2D>,
    color: Color
) {
    let matrix = matrix_from_xyz(position, rotation, vec3(1.0, 1.0, 1.0));
    draw_arrow_matrix(matrix, radius_shaft, radius_tip, length, ratio, texture, color);
}

/// Vertex indices go left to right
/// Num of vertices must be >2
pub fn draw_n_vertex_line(
    vertices: &[Vertex]
) {
    // num vertices
    let n = vertices.len();
    assert!(n>2, "Number of vertices must be larger than 2");
    // [0,1, 1,2, 2,3, ...]
    let indices: Vec<u16> = (0..n-1).flat_map(|x| [x as u16, (x + 1) as u16]).collect();
    println!("{:?}", n);
    let gl = unsafe { get_internal_gl() };
    gl.quad_gl.texture(None);
    gl.quad_gl.draw_mode(DrawMode::Lines);
    gl.quad_gl.geometry(&vertices, &indices);
}

pub fn draw_triangle(
    v1: Vec3,
    v2: Vec3,
    v3: Vec3,
    color: Color
) {
    let vertices = [
        Vertex::new2(v1,vec2(0.,0.), color),
        Vertex::new2(v2,vec2(0.,0.), color),
        Vertex::new2(v3,vec2(0.,0.), color),
    ];
    let indices = [0,1,2];

    let gl = unsafe { get_internal_gl() };

    gl.quad_gl.texture(None);
    gl.quad_gl.draw_mode(DrawMode::Triangles);
    gl.quad_gl.geometry(&vertices, &indices);
}

pub fn draw_quad(
    v1: Vertex,
    v2: Vertex,
    v3: Vertex,
    v4: Vertex,
) {
    let vertices = [
        v1,
        v2,
        v3,
        v4
    ];
    let indices = [0,1,2,0,2,3];

    let gl = unsafe { get_internal_gl() };

    gl.quad_gl.texture(None);
    gl.quad_gl.draw_mode(DrawMode::Triangles);
    gl.quad_gl.geometry(&vertices, &indices);
}