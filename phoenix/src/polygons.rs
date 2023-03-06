use crate::color::RGBA;

#[derive(Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone)]
pub struct TriangleVertices {
    vertices: [Point; 3],
}

#[derive(Clone)]
pub struct Triangle {
    vertices: TriangleVertices,
    shader_program_id: u32,
    color: RGBA,
}

pub struct TriangleBuilder {
    vertices: TriangleVertices,
    shader_program_id: Option<u32>,
    color: Option<RGBA>,
}

pub struct TriangleManager {
    triangles: Vec<Triangle>,
}

impl TriangleVertices {
    pub fn new(points: &[Point; 3]) -> Self {
        let mut vertices = [Point::default(); 3];
        vertices.copy_from_slice(points);
        TriangleVertices { vertices }
    }
}

impl Triangle {
    pub fn new(vertices: TriangleVertices, shader_program_id: u32, color: RGBA) -> Self {
        Triangle {
            vertices,
            shader_program_id,
            color,
        }
    }
}

impl TriangleBuilder {
    pub fn new() -> Self {
        TriangleBuilder {
            vertices: TriangleVertices::default(),
            shader_program_id: None,
            color: None,
        }
    }

    pub fn build(&self) -> Triangle {
        let mut color = RGBA::default();
        if let Some(value) = self.color.as_ref() {
            color = value.clone();
        }

        Triangle::new(
            self.vertices.clone(),
            self.shader_program_id.unwrap_or(0),
            color,
        )
    }

    pub fn set_vertices(&mut self, vertices: &TriangleVertices) -> &mut Self {
        self.vertices = vertices.clone();
        self
    }

    pub fn set_shader_program(&mut self, shader_program_id: u32) -> &mut Self {
        self.shader_program_id = Some(shader_program_id);
        self
    }

    pub fn set_color(&mut self, color: RGBA) -> &mut Self {
        self.color = Some(color);
        self
    }
}

impl TriangleManager {
    pub fn new() -> Self {
        TriangleManager { triangles: vec![] }
    }

    pub fn add(&mut self, triangle: Triangle) {
        self.triangles.push(triangle)
    }
}

impl Default for Point {
    fn default() -> Self {
        Point {
            x: 0_f32,
            y: 0_f32,
            z: 0_f32,
        }
    }
}

impl Default for TriangleVertices {
    fn default() -> Self {
        TriangleVertices {
            vertices: [Point::default(); 3],
        }
    }
}

impl Default for Triangle {
    fn default() -> Self {
        Triangle {
            vertices: TriangleVertices::default(),
            shader_program_id: 0,
            color: RGBA::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //TODO!
}
