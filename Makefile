.PHONY: kernel user tests all

all: kernel user tests

kernel:
	customasm ./kernel/main.asm

user:
	find ./user -name '*.asm' -exec customasm {} \;

tests:
	find ./kernel/tests -name '*.asm' -exec customasm {} \;
