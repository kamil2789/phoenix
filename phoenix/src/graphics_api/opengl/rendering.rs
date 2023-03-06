use crate::color::RGBA;
use gl;

pub fn draw_background(color: &RGBA) {
    let normalized_color = color.get_as_normalized_f32();
    unsafe {
        gl::ClearColor(
            normalized_color[0],
            normalized_color[1],
            normalized_color[2],
            normalized_color[3],
        );
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}