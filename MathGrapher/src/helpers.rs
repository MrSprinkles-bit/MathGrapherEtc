use macroquad::{math::EulerRot::XYZ, prelude::*};

pub fn matrix_from_xyz(translation: Vec3, rotation: Vec3, scale: Vec3) -> Mat4 {
    let t_matrix = Mat4::from_translation(translation);
    let r_matrix = Mat4::from_euler(XYZ, rotation.x, rotation.y, rotation.z);
    let s_matrix = Mat4::from_scale(scale);
    let matrix = t_matrix * r_matrix * s_matrix;
    return matrix;
}

pub fn matrix_from_quat(translation: Vec3, quaternion: Quat, scale: Vec3) -> Mat4 {
    let matrix = Mat4::from_scale_rotation_translation(scale, quaternion, translation);
    return matrix
}