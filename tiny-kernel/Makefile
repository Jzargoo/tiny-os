all: cargo cp make_iso qemu

cargo:
	cargo build --target x86_64-unknown-none
cp:
	cp ./target/x86_64-unknown-none/debug/tiny-kernel ./iso_root/boot/tiny-kernel
make_iso:
	xorriso -as mkisofs -R -r -J -b limine-bios-cd.bin -no-emul-boot -boot-load-size 4 -boot-info-table -hfsplus -apm-block-size 2048 --efi-boot limine-uefi-cd.bin -efi-boot-part --efi-boot-image --protective-msdos-label iso_root -o image.iso
qemu:
	qemu-system-x86_64 -cdrom image.iso -serial file:kernel_log.txt