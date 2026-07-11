#make_iso:
#	xorriso -as mkisofs -R -r -J -b limine-bios-cd.bin -no-emul-boot -boot-load-size 4 -boot-info-table -hfsplus -apm-block-size 2048 --efi-boot limine-uefi-cd.bin -efi-boot-part --efi-boot-image --protective-msdos-label iso_root -o image.iso

ifeq ($(PLATFORM),linux)
    copy = cp ./target/x86_64-unknown-none/debug/tiny-kernel ./iso_root/boot/tiny-kernel
	WSL = 
	QEMU = qemu-system-x86_64
else
	copy = copy .\target\x86_64-unknown-none\debug\tiny-kernel .\iso_root\boot\tiny-kernel
    WSL = wsl
	QEMU = "C:\Program Files\qemu\qemu-system-x86_64.exe"
endif

all: cargo cp make_iso qemu

cargo:
	cargo build --target x86_64-unknown-none

cp:
	$(copy)

make_iso:
	$(WSL) xorriso -as mkisofs -R -r -J -b limine-bios-cd.bin -no-emul-boot -boot-load-size 4 -boot-info-table -hfsplus -apm-block-size 2048 --efi-boot limine-uefi-cd.bin -efi-boot-part --efi-boot-image --protective-msdos-label iso_root -o image.iso

qemu:
	$(QEMU) -cdrom image.iso -device qemu-xhci,id=xhci -serial file:kernel_log.txt 
