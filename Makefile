install:
	rustup component add rust-src
	rustup component add llvm-tools-preview

run: target/x86_64-rmikanos/debug/boot-uefi-rmikanos.img /usr/share/OVMF/OVMF_CODE.fd
	qemu-system-x86_64 \
	  -bios /usr/share/OVMF/OVMF_CODE.fd \
	  -drive if=ide,index=0,media=disk,format=raw,file=$<

build: target/x86_64-rmikanos/debug/boot-uefi-rmikanos.img

target/x86_64-rmikanos/debug/boot-uefi-rmikanos.img: target/x86_64-rmikanos/debug/rmikanos
	cargo run -p boot

target/x86_64-rmikanos/debug/rmikanos: x86_64-rmikanos.json
	cargo build -p rmikanos \
	  --target $< \
	  -Z build-std=core,compiler_builtins \
	  -Z build-std-features=compiler-builtins-mem

.PHONY: target/x86_64-rmikanos/debug/boot-uefi-rmikanos.img target/x86_64-rmikanos/debug/rmikanos
