#![no_std]
#![no_main]
#![feature(abi_efiapi)]
#![feature(exclusive_range_pattern)]
extern crate uefi;
extern crate uefi_services;

#[macro_use]
extern crate log;
#[macro_use]
extern crate alloc;

use uefi::prelude::*;
use uefi::table::boot::MemoryType;
use uefi::proto::loaded_image::LoadedImage;
use uefi::proto::media::fs::SimpleFileSystem;
use uefi::proto::media::file::{Directory, File, FileAttribute, FileMode, RegularFile};

const EFI_PAGE_SIZE: u64 = 0x1000;

fn save_memory_map(bt: &BootServices, mut file: RegularFile) {
    let map_size = bt.memory_map_size();
    info!("memory map size: {}", map_size);
    let buffer: &mut [u8] = &mut [0; (EFI_PAGE_SIZE as usize) * 4];
    let (_key, desc_iter) = bt.memory_map(buffer).expect_success("Failed to retrieve UEFI memory map");

    let header: &[u8] = "Index, Type, Type(name), PhysicalStart, NumberOfPages, Attribute\n".as_bytes();
    file.write(header).unwrap_success();

    for (i, desc) in desc_iter.enumerate() {
        let memory_type: u32 = match desc.ty {
            MemoryType::RESERVED => 0,
            MemoryType::LOADER_CODE => 1,
            MemoryType::LOADER_DATA => 2,
            MemoryType::BOOT_SERVICES_CODE => 3,
            MemoryType::BOOT_SERVICES_DATA => 4,
            MemoryType::RUNTIME_SERVICES_CODE => 5,
            MemoryType::RUNTIME_SERVICES_DATA => 6,
            MemoryType::CONVENTIONAL => 7,
            MemoryType::UNUSABLE => 8,
            MemoryType::ACPI_RECLAIM => 9,
            MemoryType::ACPI_NON_VOLATILE => 10,
            MemoryType::MMIO => 11,
            MemoryType::MMIO_PORT_SPACE => 12,
            MemoryType::PAL_CODE => 13,
            MemoryType::PERSISTENT_MEMORY => 14,
            _ => 0xffff_ffff,
        };
        let memory_type_name = match desc.ty {
            MemoryType::RESERVED => "EfiReservedMemoryType",
            MemoryType::LOADER_CODE => "EfiLoaderCode",
            MemoryType::LOADER_DATA => "EfiLoaderData",
            MemoryType::BOOT_SERVICES_CODE => "EfiBootServicesCode",
            MemoryType::BOOT_SERVICES_DATA => "EfiBootServicesData",
            MemoryType::RUNTIME_SERVICES_CODE => "EfiRuntimeServicesCode",
            MemoryType::RUNTIME_SERVICES_DATA => "EfiRuntimeServicesData",
            MemoryType::CONVENTIONAL => "EfiConventionalMemory",
            MemoryType::UNUSABLE => "EfiUnusableMemory",
            MemoryType::ACPI_RECLAIM => "EfiACPIReclaimMemory",
            MemoryType::ACPI_NON_VOLATILE => "EfiACPIMemoryNVS",
            MemoryType::MMIO => "EfiMemoryMappedIO",
            MemoryType::MMIO_PORT_SPACE => "EfiMemoryMappedIOPortSpace",
            MemoryType::PAL_CODE => "EfiPalCode",
            MemoryType::PERSISTENT_MEMORY => "EfiPersistentMemory",
            _ => "InvalidMemoryType",
        };

        let line = format!("{:02}, {:x}, {}, {:08x}, {:x}, {:x}\n", i, memory_type, memory_type_name, desc.phys_start, desc.page_count, desc.att.bits());
        file.write(line.as_bytes()).unwrap_success();
    }
    file.flush().unwrap_success();

    info!("All done");
}

fn open_root_dir(handle: Handle, bt: &BootServices) -> RegularFile {
    let loaded_image = bt
        .handle_protocol::<LoadedImage>(handle)
        .unwrap_success()
        .get();
    let device;
    unsafe {
        device = (*loaded_image).device();
    }
    let file_system = bt
        .handle_protocol::<SimpleFileSystem>(device)
        .unwrap_success()
        .get();
    let mut root_dir: Directory;
    unsafe {
        root_dir = (*file_system).open_volume().unwrap_success();
    }

    let memory_map_file_handle = root_dir
        .open("\\memmap", FileMode::CreateReadWrite, FileAttribute::empty())
        .unwrap_success();
    let memory_map_file: RegularFile;
    unsafe {
        memory_map_file = RegularFile::new(memory_map_file_handle);
    }
    memory_map_file
}

#[entry]
fn efi_main(handle: Handle, st: SystemTable<Boot>) -> Status {
    uefi_services::init(&st).expect_success("Failed to initialize utils");
    st.stdout().reset(false).expect_success("Failed to reset output buffer");

    {
        let rev = st.uefi_revision();
        let (major, minor) = (rev.major(), rev.minor());
        info!("UEFI {}.{}", major, minor);
    }

    let memory_map_file = open_root_dir(handle, st.boot_services());
    save_memory_map(st.boot_services(), memory_map_file);

    loop {}
}
