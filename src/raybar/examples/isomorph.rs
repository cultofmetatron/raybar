use image::{DynamicImage, GenericImageView, ImageBuffer, ImageResult, Pixel, Rgb};
use std::path::Path;

pub fn load_image<P>(path: P) -> ImageResult<DynamicImage>
where
    P: AsRef<Path>,
{
    image::open(path)
}

/*
    creates a new image buffer to
ImageBuffer<T::Pixel, Vec<<<T as image::GenericImageView>::Pixel as Trait>::Subpixel>
*/
pub fn mount_image<T: GenericImageView>(
    (x, y): (u32, u32),
    input: T,
    clear_value: T::Pixel,
) -> ImageBuffer<T::Pixel, Vec<<T::Pixel as Pixel>::Subpixel>>
where
    T::Pixel: 'static,
{
    let mut imageBuffer = ImageBuffer::new(x, y);
    for (x, y, pixel) in imageBuffer.enumerate_pixels_mut() {
        *pixel = clear_value.clone();
    }

    for (x, y, pixel) in input.pixels() {
        //let img_pixel = input.get_pixel(x, y);
        imageBuffer.put_pixel(x, y, pixel.clone());
    }

    imageBuffer
}
