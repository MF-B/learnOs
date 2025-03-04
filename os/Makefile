TARGET := target/riscv64gc-unknown-none-elf/release/os
BOOTLOADER := ../bootloader/rustsbi-qemu.bin
QEMU := qemu-system-riscv64
LOG ?= INFO

# 根据 LOG 变量设置正确的特性名称
ifeq ($(LOG), TRACE)
LOG_FEATURE := --features log_trace
else ifeq ($(LOG), DEBUG)
LOG_FEATURE := --features log_debug
else ifeq ($(LOG), INFO)
LOG_FEATURE := --features log_info
else ifeq ($(LOG), WARN)
LOG_FEATURE := --features log_warn
else ifeq ($(LOG), ERROR)
LOG_FEATURE := --features log_error
else ifeq ($(LOG), DISABLE)
LOG_FEATURE := --features log_disable
else
LOG_FEATURE := --features log_info
endif

# 默认目标
all: build

# 构建项目
build:
	cargo build --release $(LOG_FEATURE)

# 运行项目
run: build
	$(QEMU) -machine virt \
		-nographic \
		-bios $(BOOTLOADER) \
		-device loader,file=$(TARGET),addr=0x80200000 \
		-s

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