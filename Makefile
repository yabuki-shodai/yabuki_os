.DEFAULT_GOAL := all
.DELETE_ON_ERROR:

BUILD := build
KERNEL := $(BUILD)/kernel.elf
USB := $(BUILD)/yabuki-os-0.0.1-usb.img
ISO := $(BUILD)/yabuki-os-0.0.1.iso
LIMINE_VERSION := 12.9.6
LIMINE_SHA256 := b38aabb042b0c0f9f562b7741be3c1204838d7bbacfa78bb411c674bd5b984ee
LIMINE_ARCHIVE := $(BUILD)/limine-binary-$(LIMINE_VERSION).tar.gz
LIMINE := $(BUILD)/limine
OVMF_CODE ?= /usr/share/OVMF/OVMF_CODE_4M.fd
OVMF_VARS ?= /usr/share/OVMF/OVMF_VARS_4M.fd
RUSTC ?= rustc
RUSTFLAGS := --edition=2021 --target x86_64-unknown-none -O -C panic=abort \
    -C relocation-model=static -C code-model=kernel -C no-redzone=yes \
    -C linker=ld.lld -C link-arg=-Tlinker.ld \
    -C link-arg=-z -C link-arg=max-page-size=0x1000 \
    -C link-arg=--build-id=none

.PHONY: all run run-uefi run-usb usb-image iso clean
all: $(KERNEL)

$(BUILD):
	mkdir -p $@

RUST_SOURCES := $(shell find src -name '*.rs' -type f)

$(KERNEL): $(RUST_SOURCES) linker.ld | $(BUILD)
	$(RUSTC) $(RUSTFLAGS) src/main.rs -o $@

$(LIMINE)/BOOTX64.EFI: $(LIMINE_ARCHIVE)
	@printf '%s  %s\n' '$(LIMINE_SHA256)' '$(LIMINE_ARCHIVE)' | sha256sum -c -
	mkdir -p $(LIMINE)
	tar -xzf $(LIMINE_ARCHIVE) -C $(LIMINE) --strip-components=1
	test -s $@
	touch $@

$(LIMINE_ARCHIVE): | $(BUILD)
	@echo 'Missing $(LIMINE_ARCHIVE). Download it manually; see README.md.' >&2
	@false

usb-image: $(USB)

$(USB): $(KERNEL) $(LIMINE)/BOOTX64.EFI limine.conf
	@command -v sgdisk >/dev/null || { echo 'sgdisk is required (gdisk package)' >&2; exit 1; }
	@command -v mformat >/dev/null || { echo 'mtools is required' >&2; exit 1; }
	truncate -s 64M $@
	sgdisk -o -n 1:2048:129023 -t 1:ef00 -c 1:'YABUKI EFI' $@
	mformat -i $(USB)@@1048576 -T 126976 -F -v YABUKI ::
	mmd -i $(USB)@@1048576 ::/EFI ::/EFI/BOOT ::/boot ::/boot/limine
	mcopy -i $(USB)@@1048576 $(LIMINE)/BOOTX64.EFI ::/EFI/BOOT/BOOTX64.EFI
	mcopy -i $(USB)@@1048576 limine.conf ::/boot/limine/limine.conf
	mcopy -i $(USB)@@1048576 $(KERNEL) ::/boot/kernel.elf

iso: $(ISO)

$(ISO): $(KERNEL) $(LIMINE)/BOOTX64.EFI limine.conf
	@command -v xorriso >/dev/null || { echo 'xorriso is required' >&2; exit 1; }
	@test -s $(LIMINE)/limine-uefi-cd.bin || { echo 'Limine UEFI CD image missing' >&2; exit 1; }
	mkdir -p $(BUILD)/iso_root/EFI/BOOT $(BUILD)/iso_root/boot/limine
	cp $(LIMINE)/BOOTX64.EFI $(BUILD)/iso_root/EFI/BOOT/
	cp $(LIMINE)/limine-uefi-cd.bin limine.conf $(BUILD)/iso_root/boot/limine/
	cp $(KERNEL) $(BUILD)/iso_root/boot/
	xorriso -as mkisofs -R -J --efi-boot boot/limine/limine-uefi-cd.bin \
	    -efi-boot-part --efi-boot-image --protective-msdos-label \
	    -o $@ $(BUILD)/iso_root

$(BUILD)/OVMF_VARS.fd: | $(BUILD)
	@test -r '$(OVMF_VARS)' || { echo 'Set OVMF_VARS to a readable OVMF variable template' >&2; exit 1; }
	cp '$(OVMF_VARS)' $@

run: run-uefi
run-uefi: $(USB) $(BUILD)/OVMF_VARS.fd
	@test -r '$(OVMF_CODE)' || { echo 'Set OVMF_CODE to a readable OVMF code image' >&2; exit 1; }
	qemu-system-x86_64 -machine q35 -m 512M -no-reboot -no-shutdown \
	    -drive if=pflash,format=raw,readonly=on,file='$(OVMF_CODE)' \
	    -drive if=pflash,format=raw,file=$(BUILD)/OVMF_VARS.fd \
	    -device qemu-xhci -drive if=none,id=yabuki_usb,format=raw,file=$(USB),readonly=on \
	    -device usb-storage,drive=yabuki_usb \
	    -serial stdio

run-usb: run-uefi

clean:
	rm -rf $(BUILD)
