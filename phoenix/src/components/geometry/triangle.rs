use super::basic::{Buffers, Point};


pub struct Triangle {
    vertices: [f32; 9],
    buffers: Buffers
}

impl Triangle {
    pub fn new(vertices: [f32; 9]) -> Self {
        Triangle{vertices, buffers: Buffers::default()}
    }

    pub fn set_buffers(&mut self, buffers: Buffers) {
        self.buffers = buffers;
    }

    pub fn get_vertices(&self) -> &[f32; 9] {
        &self.vertices
    }

    pub fn get_vao(&self) -> u32 {
        self.buffers.get_vertex_array_object()
    }
}