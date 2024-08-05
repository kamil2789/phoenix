use crate::components::{Shape, ShapeType};

use super::Point;


pub struct Cube {
    vertices: [f32; 108],
}

pub struct Sphere {
    vertices: Vec<f32>
}

impl Cube {
    #[must_use]
    pub fn new(side: f32, center_point: [f32; 3]) -> Self {
        let half_side = side / 2.0;
        let [cx, cy, cz] = center_point;

        let left = cx - half_side;
        let right = cx + half_side;
        let bottom = cy - half_side;
        let top = cy + half_side;
        let far = cz - half_side;
        let near = cz + half_side;

        let vertices = [
            left, bottom, far, right, bottom, far, right, top, far, right, top, far, left, top,
            far, left, bottom, far, left, bottom, near, right, bottom, near, right, top, near,
            right, top, near, left, top, near, left, bottom, near, left, top, near, left, top, far,
            left, bottom, far, left, bottom, far, left, bottom, near, left, top, near, right, top,
            near, right, top, far, right, bottom, far, right, bottom, far, right, bottom, near,
            right, top, near, left, bottom, far, right, bottom, far, right, bottom, near, right,
            bottom, near, left, bottom, near, left, bottom, far, left, top, far, right, top, far,
            right, top, near, right, top, near, left, top, near, left, top, far,
        ];

        Self { vertices }
    }
}

impl Sphere {
    #[must_use]
    pub fn new(center: &Point, radius: f32, precision: u16) -> Self {
        let mut vertices: Vec<Vec<Point>> = Vec::new();
        vertices.push(Sphere::calculate_upper_point(center, radius));
        vertices.extend(Sphere::calculate_sector_points(center, radius, precision));
        vertices.push(Sphere::calculate_bottom_point(center, radius));
        Self {
            vertices: Sphere::generate_geometry(&vertices, precision),
        }
    }

    fn calculate_upper_point(center: &Point, radius: f32) -> Vec<Point> {
        vec![Point {
            x: center.x,
            y: center.y,
            z: radius + center.z,
        }]
    }

    fn calculate_bottom_point(center: &Point, radius: f32) -> Vec<Point> {
        vec![Point {
            x: center.x,
            y: center.y,
            z: -radius + center.z,
        }]
    }

    fn calculate_sector_points(center: &Point, radius: f32, precision: u16) -> Vec<Vec<Point>> {
        let mut result: Vec<Vec<Point>> = Vec::new();
        let sector_angle_leap = 360_f32 / f32::from(precision * 3);
        let stack_angle_leap = 180_f32 / f32::from(precision);
        let mut stack_angle = 90_f32;
        for _ in 0..precision {
            let mut sector_points = Vec::new();
            let mut sector_angle = 0_f32;
            stack_angle -= stack_angle_leap;
            for _ in 0..(precision * 3) {
                sector_points.push(Sphere::calculate_point(
                    center,
                    radius,
                    sector_angle,
                    stack_angle,
                ));
                sector_angle += sector_angle_leap;
            }
            result.push(sector_points);
        }
        result
    }

    fn calculate_point(center: &Point, radius: f32, sector_angle: f32, stack_angle: f32) -> Point {
        let radian_sector_angle = f32::to_radians(sector_angle);
        let radian_stack_angle = f32::to_radians(stack_angle);
        let x = radius * f32::cos(radian_stack_angle) * f32::cos(radian_sector_angle);
        let y = radius * f32::cos(radian_stack_angle) * f32::sin(radian_sector_angle);
        let z = radius * f32::sin(radian_stack_angle);
        Point {
            x: x + center.x,
            y: y + center.y,
            z: z + center.z,
        }
    }

    fn generate_geometry(vertices: &[Vec<Point>], precision: u16) -> Vec<f32> {
        let mut result = Vec::with_capacity(Sphere::estimate_buffer_len(precision));
        for i in 0..usize::from(precision - 1) {
            let top = vertices[i + 1].clone();
            let bottom = vertices[i + 2].clone();

            for j in 0..usize::from(precision * 3) {
                result.push(bottom[j].x);
                result.push(bottom[j].y);
                result.push(bottom[j].z);

                let index = if j + 1 == bottom.len() { 0 } else { j + 1 };
                result.push(bottom[index].x);
                result.push(bottom[index].y);
                result.push(bottom[index].z);

                result.push(top[j].x);
                result.push(top[j].y);
                result.push(top[j].z);
            }

            for j in 0..usize::from(precision * 3) {
                result.push(top[j].x);
                result.push(top[j].y);
                result.push(top[j].z);

                let index = if j + 1 == bottom.len() { 0 } else { j + 1 };
                result.push(top[index].x);
                result.push(top[index].y);
                result.push(top[index].z);

                let tmp_index = if j + 1 == bottom.len() { 0 } else { j + 1 };
                result.push(bottom[tmp_index].x);
                result.push(bottom[tmp_index].y);
                result.push(bottom[tmp_index].z);
            }
        }

        //upper and bottom
        let top_point = vertices[0].clone();
        let bottom = vertices[1].clone();
        for i in 0..usize::from(precision * 3) {
            result.push(top_point[0].x);
            result.push(top_point[0].y);
            result.push(top_point[0].z);

            let index = if i + 1 == bottom.len() { 0 } else { i + 1 };
            result.push(bottom[index].x);
            result.push(bottom[index].y);
            result.push(bottom[index].z);

            result.push(bottom[i].x);
            result.push(bottom[i].y);
            result.push(bottom[i].z);
        }

        let bottom_point = vertices.last().unwrap();
        let last_stack = vertices[vertices.len() - 2].clone();
        for i in 0..usize::from(precision * 3) {
            result.push(bottom_point[0].x);
            result.push(bottom_point[0].y);
            result.push(bottom_point[0].z);

            let index = if i + 1 == last_stack.len() { 0 } else { i + 1 };
            result.push(last_stack[index].x);
            result.push(last_stack[index].y);
            result.push(last_stack[index].z);

            result.push(last_stack[i].x);
            result.push(last_stack[i].y);
            result.push(last_stack[i].z);
        }

        result
    }

    fn estimate_buffer_len(precision: u16) -> usize {
        let vertices_per_point = 9;
        let two_triangles_per_unit = 2;
        let sectors = usize::from(precision) * 3;
        usize::from(precision) * two_triangles_per_unit * vertices_per_point * sectors
    }
}

impl Shape for Cube {
    fn get_vertices(&self) -> &[f32] {
        &self.vertices
    }

    fn get_type(&self) -> ShapeType {
        ShapeType::Cube
    }
}

impl Shape for Sphere {
    fn get_vertices(&self) -> &[f32] {
        &self.vertices
    }

    fn get_type(&self) -> ShapeType {
        ShapeType::Sphere
    }
}
