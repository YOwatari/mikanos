.PHONY: build run clean target/x86_64-unknown-uefi/debug/rmikanos.efi

build: target/x86_64-unknown-uefi/debug/rmikanos.efi

target/x86_64-unknown-uefi/debug/rmikanos.efi:
	cargo build -Z build-std --target x86_64-unknown-uefi

run: /usr/share/OVMF/OVMF_CODE.fd OVMF_VARS.fd disk.img
	qemu-system-x86_64 \
    -drive if=pflash,format=raw,readonly,file=$< \
    -drive if=pflash,format=raw,file=OVMF_VARS.fd \
    -drive if=ide,index=0,media=disk,format=raw,file=disk.img

OVMF_VARS.fd: /usr/share/OVMF/OVMF_VARS.fd
	cp $< $@

disk.img: target/x86_64-unknown-uefi/debug/rmikanos.efi
	qemu-img create -f raw $@ 200M
	mkfs.fat -n 'MIKAN OS' -s 2 -f 2 -R 32 -F 32 $@
	mkdir mnt
	sudo mount -o loop $@ mnt
	sudo mkdir -p mnt/EFI/BOOT
	sudo cp $< mnt/EFI/BOOT/BOOTX64.EFI
	sleep 0.5
	sudo umount mnt
	rm -rf mnt

clean:
	-rm -rf disk.img
