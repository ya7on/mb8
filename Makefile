.PHONY: run

all: kernel users tests

# Kernel
KERNEL_MAIN := kernel/main.bin
kernel: $(KERNEL_MAIN)
kernel/main.bin: kernel/main.asm kernel/init.asm kernel/syscalls.asm
	customasm kernel/main.asm -o kernel/main.bin

# User space
USER_BINS := exit help hw ls sh
USER_TARGETS := $(USER_BINS:%=user/%.bin)
users: $(USER_TARGETS)
user/%.bin: user/%.asm KERNEL_MAIN
	customasm $< -o $@

# Tests
TEST_ASM := $(wildcard kernel/tests/*.asm)
TEST_BINS := $(TEST_ASM:%.asm=%.bin)
tests: $(TEST_BINS)
kernel/tests/%.bin: kernel/tests/%.asm KERNEL_MAIN
	customasm $< -o $@

run: $(KERNEL_MAIN) $(USER_TARGETS)
	cargo run --release --bin mb8-cli -- run $^
