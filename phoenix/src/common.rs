use cgmath::{InnerSpace, Vector3, Zero};

use crate::components::Shape;

pub type ID = u32;

#[derive(Default)]
pub struct IdGarbageCollector {
    num_pool: u32,
    renewable_ids: Vec<ID>,
}

pub fn calculate_normal_vec_for_shape(shape: &dyn Shape) -> Vec<f32> {
    let mut result = Vec::with_capacity(shape.get_vertices().len());

    let centroid = calculate_centroid(shape.get_vertices());

    for triangle in shape.get_vertices().chunks(9) {
        let vector_one = Vector3::new(triangle[0], triangle[1], triangle[2]);
        let vector_two = Vector3::new(triangle[3], triangle[4], triangle[5]);
        let vector_three = Vector3::new(triangle[6], triangle[7], triangle[8]);

        let normal = calculate_normal_vec(&vector_one, &vector_two, &vector_three);
        let slice: [f32; 3] =
            check_normal_direction(vector_one, vector_two, vector_three, centroid, normal).into();
        result.extend_from_slice(&slice);
        result.extend_from_slice(&slice);
        result.extend_from_slice(&slice);
    }
    result
}

fn calculate_normal_vec(a: &Vector3<f32>, b: &Vector3<f32>, c: &Vector3<f32>) -> Vector3<f32> {
    let edge_one = b - a;
    let edge_two = c - a;

    edge_one.cross(edge_two)
}

#[allow(clippy::cast_precision_loss)]
fn calculate_centroid(vertices: &[f32]) -> Vector3<f32> {
    let mut result = Vector3::zero();
    for triangle in vertices.chunks(3) {
        result.x += triangle[0];
        result.y += triangle[1];
        result.z += triangle[2];
    }

    result / vertices.len() as f32
}

fn check_normal_direction(
    a: Vector3<f32>,
    b: Vector3<f32>,
    c: Vector3<f32>,
    centroid: Vector3<f32>,
    normal: Vector3<f32>,
) -> Vector3<f32> {
    let face_center = (a + b + c) / 3.0;
    let direction_to_face = face_center - centroid;
    let dot_product = normal.dot(direction_to_face);

    if dot_product > 0.0 {
        normal
    } else {
        -normal
    }
}

impl IdGarbageCollector {
    pub fn create_id(&mut self) -> ID {
        if let Some(id) = self.renewable_ids.pop() {
            id
        } else {
            self.num_pool += 1;
            self.num_pool
        }
    }

    #[must_use]
    #[allow(dead_code)]
    pub fn get_num_pool(&self) -> u32 {
        self.num_pool
    }

    #[must_use]
    #[allow(dead_code)]
    pub fn get_renewable_ids_num(&self) -> usize {
        self.renewable_ids.len()
    }

    pub fn remove_id(&mut self, id: ID) {
        self.renewable_ids.push(id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_garbage_collector() {
        let mut id_gc = IdGarbageCollector::default();
        let id1 = id_gc.create_id();
        let id2 = id_gc.create_id();
        let id3 = id_gc.create_id();

        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
        assert_eq!(id3, 3);
        assert_eq!(id_gc.get_num_pool(), 3);
        assert_eq!(id_gc.get_renewable_ids_num(), 0);

        id_gc.remove_id(id2);
        assert_eq!(id_gc.get_num_pool(), 3);
        assert_eq!(id_gc.get_renewable_ids_num(), 1);

        let id4 = id_gc.create_id();
        assert_eq!(id4, id2);
        assert_eq!(id_gc.get_num_pool(), 3);
        assert_eq!(id_gc.get_renewable_ids_num(), 0);
    }

    #[test]
    fn test_calculate_centroid() {
        let vertices = vec![
            1.0, 1.0, 1.0, 1.0, -1.0, -1.0, -1.0, 1.0, -1.0, -1.0, -1.0, 1.0,
        ];
        assert_eq!(calculate_centroid(&vertices), Vector3::new(0.0, 0.0, 0.0));
    }
}
