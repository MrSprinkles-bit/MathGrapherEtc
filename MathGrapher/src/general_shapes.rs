use macroquad::prelude::*;

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
    let translation = Mat4::from_translation(position);
    let rotation = Mat4::from_quat(quaternion);
    let matrix = translation * rotation;
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
    let translation = Mat4::from_translation(position);
    let rotation_x = Mat4::from_rotation_x(rotation.x);
    let rotation_y = Mat4::from_rotation_y(rotation.y);
    let rotation_z = Mat4::from_rotation_z(rotation.z);
    let matrix = translation * rotation_x * rotation_y * rotation_z;
    draw_cylinder_matrix(matrix, radius_top, radius_bottom, height, texture, color);
}