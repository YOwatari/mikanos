use bootloader::boot_info::{FrameBuffer, FrameBufferInfo, PixelFormat};
use crate::framebuffer::Color;

pub trait PixelWrite {
    fn write_pixel(&self, buf: &mut [u8], index: usize, c: Color) -> bool;
}

pub fn pixel_writer(format: PixelFormat) -> &'static (dyn PixelWrite + Send + Sync) {
    match format {
        PixelFormat::RGB => &RGB_PIXEL_WRITER as _,
        PixelFormat::BGR => &BGR_PIXEL_WRITER as _,
        PixelFormat::U8 => &U8_PIXEL_WRITER as _,
        _ => &UNSUPPORTED_PIXEL_WRITER as _,
    }
}

struct RGBPixelWriter;
static RGB_PIXEL_WRITER: RGBPixelWriter = RGBPixelWriter;
impl PixelWrite for RGBPixelWriter {
    fn write_pixel(&self, buf: &mut [u8], index: usize, c: Color) -> bool {
        buf[index + 0] = c.r;
        buf[index + 1] = c.g;
        buf[index + 2] = c.b;
        true
    }
}

struct BGRPixelWriter;
static BGR_PIXEL_WRITER: BGRPixelWriter = BGRPixelWriter;
impl PixelWrite for BGRPixelWriter {
    fn write_pixel(&self, buf: &mut [u8], index: usize, c: Color) -> bool {
        buf[index + 0] = c.b;
        buf[index + 1] = c.g;
        buf[index + 2] = c.r;
        true
    }
}

struct U8PixelWriter;
static U8_PIXEL_WRITER: U8PixelWriter = U8PixelWriter;
impl PixelWrite for U8PixelWriter {
    fn write_pixel(&self, buf: &mut [u8], index: usize, c: Color) -> bool {
        buf[index] = c.grayscale();
        true
    }
}

struct UnsupportedPixelWriter;
static UNSUPPORTED_PIXEL_WRITER: UnsupportedPixelWriter = UnsupportedPixelWriter;
impl PixelWrite for UnsupportedPixelWriter {
    fn write_pixel(&self, buf: &mut [u8], index: usize, c: Color) -> bool {
        false
    }
}
