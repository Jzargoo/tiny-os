cargo:
	cargo build --release
cp:
	cp target/x86_64-tiny-kernel/release/tiny-kernel ./iso_root/boot/tiny-kernel.elf
make_iso:
	xorriso -as mkisofs -R -r -J -b limine-bios-cd.bin -no-emul-boot -boot-load-size 4 -boot-info-table -hfsplus -apm-block-size 2048 --efi-boot limine-uefi-cd.bin -efi-boot-part --efi-boot-image --protective-msdos-label iso_root -o image.iso
qemu:
	qemu-system-x86_64 -cdrom image.iso