use super::{Error, Result};
use crate::components::texture::{Filtering, MinFiltering, Mipmaps, Texture, Wrapping};

pub fn init_texture(texture: &Texture) -> Result<u32> {
    let id = generate_texture_buffer();
    bind_texture(id);

    let texture_config = texture.get_config();
    set_wrapping(
        texture_config.wrapping_horizontal,
        texture_config.wrapping_vertical,
    );
    set_filtering(texture_config.min_filtering, texture_config.max_filtering);
    generate_texture(texture)?;

    if is_mipmaps_set(texture_config.min_filtering) {
        generate_mipmaps();
    }

    Ok(id)
}

fn generate_texture_buffer() -> u32 {
    let mut texture = 0;
    unsafe { gl::GenTextures(1, &mut texture) };
    texture
}

fn bind_texture(texture: u32) {
    unsafe {
        gl::BindTexture(gl::TEXTURE_2D, texture);
    }
}

fn generate_mipmaps() {
    unsafe {
        gl::GenerateMipmap(gl::TEXTURE_2D);
    }
}

fn is_mipmaps_set(min_filtering: MinFiltering) -> bool {
    match min_filtering {
        MinFiltering::Mimpmap(_) => true,
        MinFiltering::Filtering(_) => false,
    }
}

fn generate_texture(texture: &Texture) -> Result<()> {
    let encode = if texture.is_alpha_channel() {
        gl::RGBA
    } else {
        gl::RGB
    };
    unsafe {
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB.try_into().unwrap_or(0),
            texture.get_width().try_into().unwrap_or(0),
            texture.get_height().try_into().unwrap_or(0),
            0,
            encode,
            gl::UNSIGNED_BYTE,
            texture.get_raw_data().as_ptr().cast::<std::ffi::c_void>(),
        );

        let error_code = gl::GetError();
        if error_code == gl::NO_ERROR {
            Ok(())
        } else {
            Err(Error::RenderingError(format!(
                "OpenGL error: 0x{error_code}"
            )))
        }
    }
}

fn set_wrapping(wrapping_horizontal: Wrapping, wrapping_vertical: Wrapping) {
    unsafe {
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_WRAP_S,
            match_wrapping(wrapping_horizontal).try_into().unwrap_or(0),
        );
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_WRAP_T,
            match_wrapping(wrapping_vertical).try_into().unwrap_or(0),
        );
    }
}

fn set_filtering(min_filtering: MinFiltering, max_filtering: Filtering) {
    unsafe {
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MIN_FILTER,
            match_min_filtering(min_filtering).try_into().unwrap_or(0),
        );
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MAG_FILTER,
            match_mag_filtering(max_filtering).try_into().unwrap_or(0),
        );
    }
}

fn match_mag_filtering(max_filtering: Filtering) -> u32 {
    match max_filtering {
        Filtering::Nearest => gl::NEAREST,
        Filtering::Linear => gl::LINEAR,
    }
}

fn match_min_filtering(min_filtering: MinFiltering) -> u32 {
    match min_filtering {
        MinFiltering::Mimpmap(Mipmaps::NearestMipmapNearest) => gl::NEAREST_MIPMAP_NEAREST,
        MinFiltering::Mimpmap(Mipmaps::NearestMipmapLinear) => gl::NEAREST_MIPMAP_LINEAR,
        MinFiltering::Mimpmap(Mipmaps::LinearMipmapNearest) => gl::LINEAR_MIPMAP_NEAREST,
        MinFiltering::Mimpmap(Mipmaps::LinearMipmapLinear) => gl::LINEAR_MIPMAP_LINEAR,
        MinFiltering::Filtering(Filtering::Nearest) => gl::NEAREST,
        MinFiltering::Filtering(Filtering::Linear) => gl::LINEAR,
    }
}

fn match_wrapping(wrapping: Wrapping) -> u32 {
    match wrapping {
        Wrapping::Repeat => gl::REPEAT,
        Wrapping::MirroredRepeat => gl::MIRRORED_REPEAT,
        Wrapping::ClampToEdge => gl::CLAMP_TO_EDGE,
        Wrapping::ClampToBorder => gl::CLAMP_TO_BORDER,
    }
}
