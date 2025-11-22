.PHONY: kernel tests all

all: kernel tests

kernel:
	customasm ./kernel/main.asm

tests:
	find ./kernel/tests -name '*.asm' -exec customasm {} \;
