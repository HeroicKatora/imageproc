use image::{GenericImage, Pixel};

/// Stores an underlying image representation with special semantics for drawing onto a pixel.
pub trait PixelCanvas<P> {
    /// Draw onto the pixel at (x, y) with a specified color.
    fn draw_pixel(&mut self, x: u32, y: u32, color: P);
}

/// A canvas that simply overwrites existing pixel values when drawing
pub struct Overwrite<I>(pub I);

/// A canvas that blends pixels when drawing
pub struct Blend<I>(pub I);

impl<I: GenericImage> PixelCanvas<I::Pixel> for Overwrite<I> {
    fn draw_pixel(&mut self, x: u32, y: u32, color: I::Pixel) {
        self.0.put_pixel(x, y, color)
    }
}

impl<I: GenericImage> PixelCanvas<I::Pixel> for Blend<I> {
    fn draw_pixel(&mut self, x: u32, y: u32, color: I::Pixel) {
        self.0.get_pixel_mut(x, y).blend(&color)
    }
}
