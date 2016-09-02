#!/bin/make

all:
	nasm -g -f elf64 hello.asm
	ld hello.o -o hello

clean:
	rm hello.o
	rm hello
