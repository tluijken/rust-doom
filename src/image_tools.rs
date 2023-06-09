use crate::WIDTH;
use image::{DynamicImage, GenericImageView};

/// Returns true if the pixel is black
/// # Examples
/// ```
/// use image::Rgba;
/// use image_tools::is_black;
/// assert_eq!(is_black(&Rgba([0, 0, 0, 255])), true);
/// assert_eq!(is_black(&Rgba([255, 255, 255, 255])), false);
/// ```
///
fn is_black(pixel: &image::Rgba<u8>) -> bool {
    pixel[0] == 0 && pixel[1] == 0 && pixel[2] == 0
}

/// Renders an image to the screen buffer
/// # Examples
/// ```
/// use image::DynamicImage;
/// use image_tools::render_image;
/// let img = DynamicImage::new_rgb8(100, 100);
/// let mut buffer = vec![0; 100 * 100];
/// render_image(&img, 0, 0, &mut buffer);
/// assert_ne!(buffer[0], 0);
/// ```
pub fn render_image(img: &DynamicImage, x_pos: usize, y_pos: usize, buffer: &mut [u32]) {
    let length = &buffer.len();
    img.pixels()
        // filter out black pixels
        .filter(|(_, _, pixel)| !is_black(&pixel))
        // map x and y to the position in the buffer
        .map(|(x, y, pixel)| (((y as usize + y_pos) * WIDTH) + (x as usize + x_pos), pixel))
        // filter out pixels that are outside the buffer
        .filter(|(pos, _)| &pos < &length)
        // convert the pixel to a color and write it to the buffer
        .for_each(|(pos, pixel)| buffer[pos] = convert_to_color(&pixel));
}

/// Converts a pixel to a color
/// # Arguments
/// * `pixel` - The pixel to convert as an image::Rgba<u8>.
/// # Returns
/// The color of the pixel as a u32
fn convert_to_color(pixel: &image::Rgba<u8>) -> u32 {
    let rgba = pixel.0;
    ((rgba[3] as u32) << 24) | ((rgba[0] as u32) << 16) | ((rgba[1] as u32) << 8) | rgba[2] as u32
}
