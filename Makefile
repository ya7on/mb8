.PHONY: run clean book build test lint ci

all: kernel user

# Kernel
KERNEL_MAIN := kernel/main.bin
kernel: $(KERNEL_MAIN)
kernel/main.bin: kernel/main.asm kernel/init.asm kernel/syscalls.asm
	cargo run --quiet -p asm -- build kernel/main.asm -o kernel/main.bin

# User space
USER_BINS := exit help ls sh pxl pong
USER_TARGETS := $(USER_BINS:%=user/%.bin)
ASM_SOURCES := kernel/main.asm $(USER_BINS:%=user/%.asm)
user: $(USER_TARGETS)
user/%.bin: user/%.asm $(KERNEL_MAIN)
	cargo run --quiet -p asm -- build $< -o $@

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
	@set -e; for source in $(ASM_SOURCES); do \
		cargo run --quiet -p asm -- check "$$source"; \
	done

ci: build test lint
