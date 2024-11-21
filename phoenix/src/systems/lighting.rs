use crate::{
    components::{
        color::{Color, RGBA},
        transformer::Transformer,
        Shape,
    },
    entities::entity::View,
};
use cgmath::{Vector3, Vector4};

use super::{Error, Result};

pub struct LightConfig {
    pub camera_pos: Vector3<f32>,
    pub light_pos: Vector3<f32>,
    pub light_color: RGBA,
}

/// # Errors
///
/// Will return `Err` when shape or color is not defined. Color should be also in RGBA format
pub fn calculate_light(entity: &View, camera_pos: Vector3<f32>) -> Result<LightConfig> {
    let light_pos;
    let light_color;

    if let Some(shape) = entity.shape {
        light_pos = get_light_pos(shape, entity.transformer);
    } else {
        return Err(Error::LightingError(
            "Entity with no shape, cannot calculate light position vector".into(),
        ));
    }

    if let Some(color) = Color::unpack_rgba(entity.color) {
        light_color = color.clone();
    } else {
        return Err(Error::LightingError(
            "Entity with no color, cannot assign light color".into(),
        ));
    }

    Ok(LightConfig {
        camera_pos,
        light_pos,
        light_color,
    })
}

fn get_light_pos(shape: &dyn Shape, transformation: Option<&Transformer>) -> Vector3<f32> {
    let vertices = shape.get_vertices();
    let result = Vector3::new(vertices[0], vertices[1], vertices[2]);
    if let Some(matrix) = transformation {
        let transformed = matrix.get_matrix() * Vector4::new(result.x, result.y, result.z, 1.0);
        return Vector3::new(transformed[0], transformed[1], transformed[2]);
    }

    result
}

/*
let camera_pos = self.camera.as_ref().map_or_else(
    || Vector3::new(0.0, 0.0, 0.0),
    |cam| cam.get_camera_vec_pos(),
);

*/
