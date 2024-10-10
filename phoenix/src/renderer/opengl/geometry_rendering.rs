use crate::renderer::{Error, Result};

use super::common::get_last_error_code;
use super::Buffers;

struct VertexAttrPointerArgs {
    pub layout: u32,
    pub size: i32,
    pub stride: usize,
    pub offset: usize,
}

pub fn init_shape(
    vertices: &[f32],
    color: Option<&[f32]>,
    texture: Option<&[f32]>,
) -> Result<Buffers> {
    let buffers = generate_buffers(vertices.len());
    bind_buffers(&buffers);

    send_data_to_gpu_buffer(&combine_vertices(vertices, color, texture));

    create_vertex_attribute_pointer_argument_list(color.is_some(), texture.is_some())
        .into_iter()
        .for_each(|args| set_vertex_attribute_pointer(&args));

    unbind_buffers();

    if let Some(err_code) = get_last_error_code(true) {
        Err(Error::RenderingError(format!(
            "OpenGL error code {err_code}"
        )))
    } else {
        Ok(buffers)
    }
}

fn combine_vertices(vertices: &[f32], color: Option<&[f32]>, texture: Option<&[f32]>) -> Vec<f32> {
    if let Some(color) = color {
        if let Some(texture) = texture {
            return combine_position_with_color_and_texture(vertices, color, texture);
        }
        return combine_position_with_color(vertices, color);
    }

    if let Some(texture) = texture {
        if vertices.len() > 100 {
            return combine_position_with_texture_for_3d_cube(vertices);
        }
        return combine_position_with_texture(vertices, texture);
    }

    vertices.to_vec()
}

fn create_vertex_attribute_pointer_argument_list(
    is_color: bool,
    is_texture: bool,
) -> Vec<VertexAttrPointerArgs> {
    let mut result = Vec::new();
    let mut stride = 3;
    if is_color {
        stride += 4;
    }

    if is_texture {
        stride += 2;
    }

    result.push(VertexAttrPointerArgs {
        layout: 0,
        size: 3,
        stride,
        offset: 0,
    });

    if is_color && is_texture {
        result.push(VertexAttrPointerArgs {
            layout: 1,
            size: 4,
            stride,
            offset: 3,
        });
        result.push(VertexAttrPointerArgs {
            layout: 2,
            size: 2,
            stride,
            offset: 7,
        });
    } else if is_texture {
        result.push(VertexAttrPointerArgs {
            layout: 2,
            size: 2,
            stride,
            offset: 3,
        });
    } else if is_color {
        result.push(VertexAttrPointerArgs {
            layout: 1,
            size: 4,
            stride,
            offset: 3,
        });
    }

    result
}

fn combine_position_with_texture_for_3d_cube(position: &[f32]) -> Vec<f32> {
    let texture_vertices: [f32; 72] = [
        0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0,
        1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0,
        1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0,
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    ];

    let mut result = Vec::with_capacity(position.len() + texture_vertices.len());
    let pos_size = 3;
    let texture_size = 2;
    let iter = position
        .chunks(pos_size)
        .zip(texture_vertices.chunks(texture_size));

    for (pos, tex) in iter {
        result.extend_from_slice(pos);
        result.extend_from_slice(tex);
    }
    result
}

fn combine_position_with_color(position: &[f32], color: &[f32]) -> Vec<f32> {
    let mut result = Vec::with_capacity(position.len() + color.len());
    let pos_stride = 3;
    let color_stride = 4;
    let iter = position.chunks(pos_stride).zip(color.chunks(color_stride));

    for (pos, col) in iter {
        result.extend_from_slice(pos);
        result.extend_from_slice(col);
    }
    result
}

fn combine_position_with_texture(position: &[f32], texture: &[f32]) -> Vec<f32> {
    let mut result = Vec::with_capacity(position.len() + texture.len());
    let pos_size = 3;
    let texture_size = 2;
    let iter = position.chunks(pos_size).zip(texture.chunks(texture_size));

    for (pos, tex) in iter {
        result.extend_from_slice(pos);
        result.extend_from_slice(tex);
    }
    result
}

fn combine_position_with_color_and_texture(
    position: &[f32],
    color: &[f32],
    texture: &[f32],
) -> Vec<f32> {
    let mut result = Vec::with_capacity(position.len() + color.len() + texture.len());
    let pos_size = 3;
    let color_size = 4;
    let texture_size = 2;
    let iter = position
        .chunks(pos_size)
        .zip(color.chunks(color_size))
        .zip(texture.chunks(texture_size));

    for ((pos, col), tex) in iter {
        result.extend_from_slice(pos);
        result.extend_from_slice(col);
        result.extend_from_slice(tex);
    }
    result
}

fn generate_buffers(vertices_count: usize) -> Buffers {
    let mut vertex_array_object = 0;
    let mut vertex_buffer_object = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vertex_array_object);
        gl::GenBuffers(1, &mut vertex_buffer_object);
    }

    Buffers::new(
        vertex_array_object,
        vertex_buffer_object,
        u16::try_from(vertices_count / 3).unwrap_or(0),
    )
}

fn bind_buffers(buffers: &Buffers) {
    unsafe {
        gl::BindVertexArray(buffers.vertex_array_object);
        gl::BindBuffer(gl::ARRAY_BUFFER, buffers.vertex_buffer_object);
    }
}

fn send_data_to_gpu_buffer(vertices: &[f32]) {
    let size = std::mem::size_of_val(vertices);
    unsafe {
        gl::BufferData(
            gl::ARRAY_BUFFER,
            size.try_into().unwrap(),
            vertices.as_ptr().cast::<std::ffi::c_void>(),
            gl::STATIC_DRAW,
        );
    }
}

fn set_vertex_attribute_pointer(args: &VertexAttrPointerArgs) {
    let stride = args.stride * std::mem::size_of::<f32>();
    let offset = args.offset * std::mem::size_of::<f32>();
    unsafe {
        gl::VertexAttribPointer(
            args.layout,
            args.size,
            gl::FLOAT,
            gl::FALSE,
            stride.try_into().unwrap_or(0),
            offset as *const std::ffi::c_void,
        );
        gl::EnableVertexAttribArray(args.layout);
    }
}

fn unbind_buffers() {
    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }
}

#[cfg(test)]
mod tests {
    use super::combine_position_with_color;
    use crate::components::geometry::solid::Cube;
    use crate::components::Shape;
    use crate::renderer::opengl::OpenGL;
    use crate::{
        renderer::opengl::geometry_rendering::{
            combine_position_with_color_and_texture, combine_position_with_texture,
        },
        testing::setup_opengl,
        window::{GlfwConfig, Resolution},
    };
    use serial_test::serial;
    use std::rc::Rc;

    #[test]
    #[serial]
    fn test_init_shape() {
        setup_opengl!();

        let vertices = vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0];
        let buffers = super::init_shape(&vertices, None, None).unwrap();
        assert_ne!(buffers.vertex_array_object, 0);
        assert_ne!(buffers.vertex_buffer_object, 0);
    }

    #[test]
    #[serial]
    fn test_init_shape_with_texture() {
        setup_opengl!();

        let vertices = vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0];
        let texture = vec![0.0, 0.0, 1.0, 0.0, 0.5, 1.0];
        let buffers = super::init_shape(&vertices, Some(&texture), None).unwrap();
        assert_ne!(buffers.vertex_array_object, 0);
        assert_ne!(buffers.vertex_buffer_object, 0);
    }

    #[test]
    #[serial]
    fn test_init_shape_with_color() {
        setup_opengl!();

        let vertices = vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0];
        let color = vec![
            0.5_f32, 0.5_f32, 0.5_f32, 0.5_f32, 0.5_f32, 0.5_f32, 0.5_f32, 0.5_f32, 0.5_f32,
            0.5_f32, 0.5_f32, 0.5_f32,
        ];

        let buffers = super::init_shape(&vertices, None, Some(&color)).unwrap();
        assert_ne!(buffers.vertex_array_object, 0);
        assert_ne!(buffers.vertex_buffer_object, 0);
    }

    #[test]
    #[serial]
    fn test_init_shape_3d_cube() {
        setup_opengl!();

        let cube = Cube::new(0.5, [0.0, 0.0, 0.0]);
        let texture = vec![];

        let buffers = super::init_shape(cube.get_vertices(), None, Some(&texture)).unwrap();
        assert_ne!(buffers.vertex_array_object, 0);
        assert_ne!(buffers.vertex_buffer_object, 0);
    }

    #[test]
    #[serial]
    fn test_init_shape_with_color_and_texture() {
        setup_opengl!();

        let vertices = vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0];
        let color = vec![
            0.5_f32, 0.5_f32, 0.5_f32, 0.5_f32, 0.5_f32, 0.5_f32, 0.5_f32, 0.5_f32, 0.5_f32,
            0.5_f32, 0.5_f32, 0.5_f32,
        ];
        let texture = vec![0.0, 0.0, 1.0, 0.0, 0.5, 1.0];

        let buffers = super::init_shape(&vertices, Some(&texture), Some(&color)).unwrap();
        assert_ne!(buffers.vertex_array_object, 0);
        assert_ne!(buffers.vertex_buffer_object, 0);
    }

    #[test]
    fn test_combine_position_with_color() {
        let position = vec![
            1_f32, 2_f32, 3_f32, 4_f32, 5_f32, 6_f32, 7_f32, 8_f32, 9_f32,
        ];

        let color = vec![
            0.5_f32, 0.5_f32, 0.5_f32, 0.5_f32, 0.5_f32, 0.5_f32, 0.5_f32, 0.5_f32, 0.5_f32,
            0.5_f32, 0.5_f32, 0.5_f32,
        ];

        let result = combine_position_with_color(&position, &color);
        assert_eq!(
            result,
            vec![
                1_f32, 2_f32, 3_f32, 0.5_f32, 0.5_f32, 0.5_f32, 0.5_f32, 4_f32, 5_f32, 6_f32,
                0.5_f32, 0.5_f32, 0.5_f32, 0.5_f32, 7_f32, 8_f32, 9_f32, 0.5_f32, 0.5_f32, 0.5_f32,
                0.5_f32
            ]
        );
    }

    #[test]
    fn test_combine_position_with_texture() {
        let position = vec![
            1_f32, 2_f32, 3_f32, 4_f32, 5_f32, 6_f32, 7_f32, 8_f32, 9_f32,
        ];

        let texture = vec![0.0, 0.0, 1.0, 0.0, 0.5, 1.0];

        let result = combine_position_with_texture(&position, &texture);
        assert_eq!(
            result,
            vec![
                1_f32, 2_f32, 3_f32, 0.0, 0.0, 4_f32, 5_f32, 6_f32, 1.0, 0.0, 7_f32, 8_f32, 9_f32,
                0.5, 1.0
            ]
        );
    }

    #[test]
    fn test_combine_position_with_color_and_texture() {
        let position = vec![
            1_f32, 2_f32, 3_f32, 4_f32, 5_f32, 6_f32, 7_f32, 8_f32, 9_f32,
        ];
        let color = vec![
            0.5_f32, 0.5_f32, 0.5_f32, 0.5_f32, 0.5_f32, 0.5_f32, 0.5_f32, 0.5_f32, 0.5_f32,
            0.5_f32, 0.5_f32, 0.5_f32,
        ];

        let texture = vec![0.0, 0.0, 1.0, 0.0, 0.5, 1.0];

        let result = combine_position_with_color_and_texture(&position, &color, &texture);

        assert_eq!(
            result,
            vec![
                1_f32, 2_f32, 3_f32, 0.5_f32, 0.5_f32, 0.5_f32, 0.5_f32, 0.0, 0.0, 4_f32, 5_f32,
                6_f32, 0.5_f32, 0.5_f32, 0.5_f32, 0.5_f32, 1.0, 0.0, 7_f32, 8_f32, 9_f32, 0.5_f32,
                0.5_f32, 0.5_f32, 0.5_f32, 0.5, 1.0
            ]
        );
    }
}
