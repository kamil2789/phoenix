use crate::renderer::{Error, Result};

use super::common::get_last_error_code;
use super::Buffers;

const POSITION_LAYOUT: u32 = 0;
const NORMAL_LAYOUT: u32 = 1;
const COLOR_LAYOUT: u32 = 2;
const TEXTURE_LAYOUT: u32 = 3;

#[derive(Debug)]
struct VertexAttrPointerArgs {
    pub layout: u32,
    pub size: i32,
    pub stride: usize,
    pub offset: usize,
}

pub fn init_shape(
    vertices: &[f32],
    normals: Option<&[f32]>,
    color: Option<&[f32]>,
    texture: Option<&[f32]>,
) -> Result<Buffers> {
    let buffers = generate_buffers(vertices.len());
    bind_buffers(&buffers);

    allocate_gpu_buffer(vertices, normals, color, texture);
    send_data_to_gpu_buffer(vertices, normals, color, texture);

    create_vertex_attribute_pointer_argument_list(vertices, normals, color, texture)
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

fn create_vertex_attribute_pointer_argument_list(
    positions: &[f32],
    normals: Option<&[f32]>,
    color: Option<&[f32]>,
    texture: Option<&[f32]>,
) -> Vec<VertexAttrPointerArgs> {
    let mut result = Vec::new();
    let mut offset = 0;

    result.push(VertexAttrPointerArgs {
        layout: POSITION_LAYOUT,
        size: 3,
        stride: 3,
        offset,
    });
    offset += std::mem::size_of_val(positions);

    if let Some(val) = normals {
        result.push(VertexAttrPointerArgs {
            layout: NORMAL_LAYOUT,
            size: 3,
            stride: 3,
            offset,
        });
        offset += std::mem::size_of_val(val);
    }

    if let Some(val) = color {
        result.push(VertexAttrPointerArgs {
            layout: COLOR_LAYOUT,
            size: 4,
            stride: 4,
            offset,
        });
        offset += std::mem::size_of_val(val);
    }

    if texture.is_some() {
        result.push(VertexAttrPointerArgs {
            layout: TEXTURE_LAYOUT,
            size: 2,
            stride: 2,
            offset,
        });
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

fn allocate_gpu_buffer(
    positions: &[f32],
    normals: Option<&[f32]>,
    color: Option<&[f32]>,
    texture: Option<&[f32]>,
) {
    let mut size = std::mem::size_of_val(positions);
    if let Some(val) = normals {
        size += std::mem::size_of_val(val);
    }

    if let Some(val) = color {
        size += std::mem::size_of_val(val);
    }

    if let Some(val) = texture {
        size += std::mem::size_of_val(val);
    }

    unsafe {
        gl::BufferData(
            gl::ARRAY_BUFFER,
            size.try_into().unwrap(),
            std::ptr::null(),
            gl::STATIC_DRAW,
        );
    }
}

fn send_data_to_gpu_buffer(
    positions: &[f32],
    normals: Option<&[f32]>,
    color: Option<&[f32]>,
    texture: Option<&[f32]>,
) {
    let mut offset = 0;
    offset = copy_buffer_to_gpu(positions, offset);
    if let Some(normals) = normals {
        offset = copy_buffer_to_gpu(normals, offset);
    }

    if let Some(color) = color {
        offset = copy_buffer_to_gpu(color, offset);
    }

    if let Some(texture) = texture {
        copy_buffer_to_gpu(texture, offset);
    }
}

fn copy_buffer_to_gpu(buffer: &[f32], offset: isize) -> isize {
    let size: isize = std::mem::size_of_val(buffer).try_into().unwrap();
    unsafe {
        gl::BufferSubData(
            gl::ARRAY_BUFFER,
            offset,
            size,
            buffer.as_ptr().cast::<std::ffi::c_void>(),
        );
    }
    offset + size
}

fn set_vertex_attribute_pointer(args: &VertexAttrPointerArgs) {
    let stride = args.stride * std::mem::size_of::<f32>();
    unsafe {
        gl::VertexAttribPointer(
            args.layout,
            args.size,
            gl::FLOAT,
            gl::FALSE,
            stride.try_into().unwrap_or(0),
            args.offset as *const std::ffi::c_void,
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
    use crate::components::geometry::solid::Cube;
    use crate::components::Shape;
    use crate::renderer::opengl::OpenGL;
    use crate::{
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
        let buffers = super::init_shape(&vertices, None, None, None).unwrap();
        assert_ne!(buffers.vertex_array_object, 0);
        assert_ne!(buffers.vertex_buffer_object, 0);
    }

    #[test]
    #[serial]
    fn test_init_shape_with_texture() {
        setup_opengl!();

        let vertices = vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0];
        let texture = vec![0.0, 0.0, 1.0, 0.0, 0.5, 1.0];
        let buffers = super::init_shape(&vertices, None, Some(&texture), None).unwrap();
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

        let buffers = super::init_shape(&vertices, None, None, Some(&color)).unwrap();
        assert_ne!(buffers.vertex_array_object, 0);
        assert_ne!(buffers.vertex_buffer_object, 0);
    }

    #[test]
    #[serial]
    fn test_init_shape_3d_cube() {
        setup_opengl!();

        let cube = Cube::new(0.5, [0.0, 0.0, 0.0]);
        let texture = vec![];

        let buffers = super::init_shape(cube.get_vertices(), None, None, Some(&texture)).unwrap();
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

        let buffers = super::init_shape(&vertices, None, Some(&texture), Some(&color)).unwrap();
        assert_ne!(buffers.vertex_array_object, 0);
        assert_ne!(buffers.vertex_buffer_object, 0);
    }
}
