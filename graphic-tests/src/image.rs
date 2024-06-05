use image::{error, io::Reader as ImageReader, GenericImageView};
use phoenix::window::{Resolution, Window};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    ReadImageError(#[from] std::io::Error),
    #[error("{0}")]
    ImageError(#[from] error::ImageError),
}

pub fn save_screen_as_img_png(window: &Window, image_name: &str) -> Result<()> {
    let resolution = window.get_resolution();
    let buffer = read_pixels_from_front_buffer(&resolution);
    let flipped_buffer = flipped_buffer(&buffer, &resolution);
    save_png(&flipped_buffer, &resolution, image_name)
}

fn read_pixels_from_front_buffer(resolution: &Resolution) -> Vec<u8> {
    let buffer_size = usize::from(resolution.width) * usize::from(resolution.height) * 3;
    let mut result: Vec<u8> = vec![0; buffer_size];

    unsafe {
        gl::PixelStorei(gl::PACK_ALIGNMENT, 4);
        gl::ReadBuffer(gl::FRONT);
        gl::ReadPixels(
            0,
            0,
            resolution.width.into(),
            resolution.height.into(),
            gl::RGB,
            gl::UNSIGNED_BYTE,
            result.as_mut_ptr().cast::<std::ffi::c_void>(),
        );
    }

    result
}

fn flipped_buffer(buffer: &[u8], resolution: &Resolution) -> Vec<u8> {
    let row_size = resolution.width * 3;
    let mut result = Vec::with_capacity(buffer.len());

    for row in buffer.chunks_exact(row_size.into()).rev() {
        result.extend_from_slice(row);
    }

    result
}

fn save_png(buffer: &[u8], resolution: &Resolution, image_name: &str) -> Result<()> {
    image::save_buffer(
        image_name,
        buffer,
        resolution.width.into(),
        resolution.height.into(),
        image::ColorType::Rgb8,
    )?;
    Ok(())
}

pub fn read_image_from_file(image_name: &str) -> Result<image::DynamicImage> {
    let img_result = ImageReader::open(image_name)?.decode();
    match img_result {
        Ok(img) => Ok(img),
        Err(err) => Err(Error::ImageError(err)),
    }
}

pub fn are_images_equal(first: &image::DynamicImage, second: &image::DynamicImage) -> bool {
    if first.dimensions() != second.dimensions() {
        return false;
    }

    for y in 0..first.height() {
        for x in 0..first.width() {
            if first.get_pixel(x, y) != second.get_pixel(x, y) {
                return false;
            }
        }
    }
    true
}
