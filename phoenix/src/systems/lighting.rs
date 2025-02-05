use crate::components::{transformer::Transformer, Shape};
use cgmath::{Vector3, Vector4};

pub fn calculate_light_pos(
    shape: &dyn Shape,
    transformation: Option<&Transformer>,
) -> Vector3<f32> {
    let vertices = shape.get_vertices();
    let result = Vector3::new(vertices[0], vertices[1], vertices[2]);
    if let Some(matrix) = transformation {
        let transformed = matrix.get_matrix() * Vector4::new(result.x, result.y, result.z, 1.0);
        return Vector3::new(transformed[0], transformed[1], transformed[2]);
    }

    result
}
