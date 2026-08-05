#make_iso:
#	xorriso -as mkisofs -R -r -J -b limine-bios-cd.bin -no-emul-boot -boot-load-size 4 -boot-info-table -hfsplus -apm-block-size 2048 --efi-boot limine-uefi-cd.bin -efi-boot-part --efi-boot-image --protective-msdos-label iso_root -o image.iso

ifeq ($(PLATFORM),linux)
    copy = cp ./target/x86_64-unknown-none/debug/tiny-kernel ./iso_root/boot/tiny-kernel
	WSL = 
	OVMF_CODE = /usr/share/OVMF/OVMF_CODE.fd
	QEMU = qemu-system-x86_64
else
	copy = copy .\target\x86_64-unknown-none\debug\tiny-kernel .\iso_root\boot\tiny-kernel
    WSL = wsl
	OVMF_CODE = "C:\Users\Jzargo\Downloads\OVMF_CODE.fd"
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
	$(QEMU) -cdrom image.iso \
	-device qemu-xhci,id=xhci \
	-serial none -serial file:kernel_log.txt \
	-machine type=q35 \
    -cpu max \
    -pflash  $(OVMF_CODE) \
    -m 4G \
	-device ahci,id=ahci0 \
  	-device intel-hda,id=sound0 \
  	-device e1000,netdev=net0 \
  	-netdev user,id=net0 \