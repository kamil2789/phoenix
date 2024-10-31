use cgmath::{InnerSpace, Vector3};

use crate::components::Shape;

pub type ID = u32;

#[derive(Default)]
pub struct IdGarbageCollector {
    num_pool: u32,
    renewable_ids: Vec<ID>,
}

pub fn calculate_normal_vec_for_shape(shape: &dyn Shape) -> Vec<f32> {
    let mut result = Vec::with_capacity(shape.get_vertices().len() / 3);
    for triangle in shape.get_vertices().chunks(9) {
        let normal = calculate_normal_vec(triangle);
        result.extend_from_slice(&normal);
    }
    result
}

fn calculate_normal_vec(vertices: &[f32]) -> [f32; 3] {
    let vector_one = Vector3::new(vertices[0], vertices[1], vertices[2]);
    let vector_two = Vector3::new(vertices[3], vertices[4], vertices[5]);
    let vector_three = Vector3::new(vertices[6], vertices[7], vertices[8]);

    let edge_one = vector_two - vector_one;
    let edge_two = vector_three - vector_one;

    edge_one.cross(edge_two).normalize().into()
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
}
