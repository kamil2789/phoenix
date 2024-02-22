

pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

pub struct Buffers {
    vao: u32,
    vbo: u32
}

impl Buffers {
    pub fn new(vao: u32, vbo: u32) -> Self {
        Buffers{vao, vbo}
    }

    pub fn get_vertex_array_object(&self) -> u32 {
        self.vao
    }

    pub fn get_vertex_buffer_object(&self) -> u32 {
        self.vbo
    }
}

impl Default for Buffers {
    fn default() -> Self {
        Self { vao: 0, vbo: 0 }
    }
}