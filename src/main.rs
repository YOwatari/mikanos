#![no_std]
#![no_main]
#![feature(asm)]
#![feature(abi_efiapi)]

extern crate uefi;

use uefi::prelude::*;
use core::panic::PanicInfo;
use core::fmt::Write;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[entry]
fn efi_main(_handle: Handle, st: SystemTable<Boot>) -> Status {
    writeln!(st.stdout(), "Yo!").unwrap();

    loop {}
}
