.PHONY: run clean book build test lint ci

all: kernel user tests

# Kernel
KERNEL_MAIN := kernel/main.bin
kernel: $(KERNEL_MAIN)
kernel/main.bin: kernel/main.asm kernel/init.asm kernel/syscalls.asm
	customasm kernel/main.asm -o kernel/main.bin

# User space
USER_BINS := exit help hello ls sh
USER_TARGETS := $(USER_BINS:%=user/%.bin)
user: $(USER_TARGETS)
user/%.bin: user/%.asm $(KERNEL_MAIN)
	customasm $< -o $@

# Tests
TEST_ASM := $(wildcard kernel/tests/*.asm)
TEST_BINS := $(TEST_ASM:%.asm=%.bin)
tests: $(TEST_BINS)
kernel/tests/%.bin: kernel/tests/%.asm $(KERNEL_MAIN)
	customasm $< -o $@

run: $(KERNEL_MAIN) $(USER_TARGETS)
	cargo run --features desktop --bin cli-desktop -- run $^

clean:
	rm -f kernel/*.bin user/*.bin kernel/tests/*.bin

book:
	mdbook serve ./docs

build:
	cargo build --release

test:
	cargo test

lint:
	cargo fmt -- --check
	cargo clippy

ci: build test lint
