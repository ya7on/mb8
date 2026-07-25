.PHONY: run clean book build test lint ci

all: kernel user

# Kernel
KERNEL_MAIN := kernel/main.bin
kernel: $(KERNEL_MAIN)
kernel/main.bin: kernel/main.asm kernel/init.asm kernel/syscalls.asm
	cargo run --quiet -p asm -- kernel/main.asm -o kernel/main.bin

# User space
USER_BINS := exit help ls sh pxl pong
USER_TARGETS := $(USER_BINS:%=user/%.bin)
user: $(USER_TARGETS)
user/%.bin: user/%.asm $(KERNEL_MAIN)
	cargo run --quiet -p asm -- $< -o $@

run: $(KERNEL_MAIN) $(USER_TARGETS)
	cargo run --features desktop --bin cli-desktop -- run $^

debug: $(KERNEL_MAIN) $(USER_TARGETS)
	cargo run --features desktop --bin cli-desktop -- run --debug $^

clean:
	rm -f kernel/*.bin user/*.bin

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
