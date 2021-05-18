use bootloader_locator::locate_bootloader;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let kernel_binary_path = PathBuf::from("target/x86_64-rmikanos/debug/rmikanos").canonicalize().unwrap();
    let bootloader_manifest_path = locate_bootloader("bootloader").unwrap();
    let kernel_manifest_path = locate_cargo_manifest::locate_manifest().unwrap();

    let mut build_cmd = Command::new(env!("CARGO"));
    build_cmd.current_dir(bootloader_manifest_path.parent().unwrap());
    build_cmd.arg("builder");
    build_cmd.arg("--kernel-manifest").arg(&kernel_manifest_path);
    build_cmd.arg("--kernel-binary").arg(&kernel_binary_path);
    build_cmd.arg("--target-dir").arg(kernel_manifest_path.parent().unwrap().join("target"));
    build_cmd.arg("--out-dir").arg(kernel_binary_path.parent().unwrap());

    if !build_cmd.status().unwrap().success() {
        panic!("build failed");
    }
}
