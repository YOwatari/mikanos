use bootloader::boot_info::{FrameBuffer, FrameBufferInfo, PixelFormat};
use core::convert::TryFrom;
use core::ops::Range;

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn grayscale(self) -> u8 {
        u8::try_from(
            (f32::from(self.r) * 0.3 + f32::from(self.g) * 0.59 + f32::from(self.b) * 0.11)
                .to_bits(),
        )
        .unwrap()
    }

    pub const WHITE: Self = Color::new(255, 255, 255);
    pub const GREEN: Self = Color::new(0, 255, 0);
}

pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

pub struct FrameBufferWriter {
    inner: FrameBuffer,
}

impl FrameBufferWriter {
    pub fn new(inner: FrameBuffer) -> Self {
        Self { inner }
    }

    pub fn info(&self) -> FrameBufferInfo {
        self.inner.info()
    }

    pub fn width(&self) -> Range<usize> {
        0..self.info().horizontal_resolution
    }

    pub fn height(&self) -> Range<usize> {
        0..self.info().vertical_resolution
    }

    pub fn pixel_index<T>(&self, p: Point<T>) -> Option<usize>
    where
        usize: TryFrom<T>,
    {
        let FrameBufferInfo {
            bytes_per_pixel,
            stride,
            ..
        } = self.info();

        let x = usize::try_from(p.x).ok()?;
        let y = usize::try_from(p.y).ok()?;
        if !self.width().contains(&x) || !self.height().contains(&y) {
            return None;
        }
        Some((y * stride + x) * bytes_per_pixel)
    }

    pub fn write<T>(&mut self, p: Point<T>, c: Color) -> bool
    where
        usize: TryFrom<T>,
    {
        let index = match self.pixel_index(p) {
            Some(p) => p,
            None => return false,
        };

        match self.info().pixel_format {
            PixelFormat::RGB => {
                self.inner.buffer_mut()[index + 0] = c.r;
                self.inner.buffer_mut()[index + 1] = c.g;
                self.inner.buffer_mut()[index + 2] = c.b;
            }
            PixelFormat::BGR => {
                self.inner.buffer_mut()[index + 0] = c.b;
                self.inner.buffer_mut()[index + 1] = c.g;
                self.inner.buffer_mut()[index + 2] = c.r;
            }
            PixelFormat::U8 => {
                self.inner.buffer_mut()[index] = c.grayscale();
            }
            _ => return false,
        }
        true
    }
}
