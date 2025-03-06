TARGET := target/riscv64gc-unknown-none-elf/release/os
BOOTLOADER := ../bootloader/rustsbi-qemu.bin
QEMU := qemu-system-riscv64
LOG ?= INFO

# 默认目标
all: build

# 构建项目
build:
	cargo build --release

# 运行项目
run: build
	$(QEMU) -machine virt \
		-nographic \
		-bios $(BOOTLOADER) \
		-device loader,file=$(TARGET),addr=0x80200000

# 调试模式运行
debug: build
	$(QEMU) -machine virt \
		-nographic \
		-bios $(BOOTLOADER) \
		-device loader,file=$(TARGET),addr=0x80200000 \
		-s -S

# 清理项目
clean:
	cargo clean

.PHONY: all build run debug clean