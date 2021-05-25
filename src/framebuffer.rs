use bootloader::boot_info::{FrameBuffer, FrameBufferInfo, PixelFormat};
use core::convert::TryFrom;
use core::ops::Range;
use conquer_once::spin::OnceCell;

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

static INFO: OnceCell<FrameBufferInfo> = OnceCell::uninit();
static WRITER: OnceCell<spin::Mutex<FrameBufferWriter>> = OnceCell::uninit();

pub fn init(fb: FrameBuffer) {
    INFO.try_init_once(|| fb.info()).expect("cannot initialize INFO");
    WRITER.try_init_once(||spin::Mutex::new(FrameBufferWriter::new(fb))).expect("cannot initialize INFO");
}

pub fn info() -> &'static FrameBufferInfo {
    INFO.try_get().expect("INFO is not initialized")
}

pub fn writer() -> spin::MutexGuard<'static, FrameBufferWriter> {
    WRITER.try_get().expect("WRITER is not initialized").lock()
}

pub struct FrameBufferWriter {
    inner: FrameBuffer,
}

impl FrameBufferWriter {
    fn new(inner: FrameBuffer) -> Self {
        Self { inner }
    }

    pub fn width(&self) -> Range<usize> {
        0..info().horizontal_resolution
    }

    pub fn height(&self) -> Range<usize> {
        0..info().vertical_resolution
    }

    pub fn pixel_index<T>(&self, p: Point<T>) -> Option<usize>
    where
        usize: TryFrom<T>,
    {
        let FrameBufferInfo {
            bytes_per_pixel,
            stride,
            ..
        } = info();

        let x = usize::try_from(p.x).ok()?;
        let y = usize::try_from(p.y).ok()?;
        if !self.width().contains(&x) || !self.height().contains(&y) {
            return None;
        }
        Some((y * stride + x) * bytes_per_pixel)
    }

    pub fn write_pixel<T>(&mut self, p: Point<T>, c: Color) -> bool
    where
        usize: TryFrom<T>,
    {
        let index = match self.pixel_index(p) {
            Some(p) => p,
            None => return false,
        };

        match info().pixel_format {
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
