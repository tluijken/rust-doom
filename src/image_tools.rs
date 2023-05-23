use crate::WIDTH;
use image::{DynamicImage, GenericImageView};
pub fn scale_image(img: DynamicImage) -> DynamicImage {
    img.resize(
        (img.width() as f32 * crate::SCALE) as u32,
        (img.height() as f32 * crate::SCALE) as u32,
        image::imageops::FilterType::Nearest,
    )
}

fn is_black(pixel: &image::Rgba<u8>) -> bool {
    pixel[0] == 0 && pixel[1] == 0 && pixel[2] == 0
}

pub fn render_image(img: &DynamicImage, x_pos: usize, y_pos: usize, buffer: &mut [u32]) {
    for (x, y, pixel) in img.pixels() {
        if is_black(&pixel) {
            continue;
        }
        let rgba = pixel.0;
        // Convert the pixel's color channels from u8 to u32, and arrange them into an ARGB format
        let color = ((rgba[3] as u32) << 24)
            | ((rgba[0] as u32) << 16)
            | ((rgba[1] as u32) << 8)
            | rgba[2] as u32;
        buffer[((y as usize + y_pos) * WIDTH) + x as usize + x_pos] = color;
    }
}
