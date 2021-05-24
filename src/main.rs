#![no_std]
#![no_main]

mod framebuffer;

use crate::framebuffer::{Color, FrameBufferWriter, Point};
use bootloader::boot_info::Optional;
use bootloader::{entry_point, BootInfo};
use core::{mem, panic::PanicInfo};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    if let Some(fb) = mem::replace(&mut boot_info.framebuffer, Optional::None).into() {
        let mut w = FrameBufferWriter::new(fb);
        for y in w.height() {
            for x in w.width() {
                w.write(Point::new(x, y), Color::WHITE);
            }
        }
        for y in 0..100 {
            for x in 0..200 {
                w.write(Point::new(x, y), Color::GREEN);
            }
        }
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
