#!/usr/bin/make

all:
	gcc -c main.c -o objects/main.o
	nasm -f elf64 linked.s -o objects/linked.o
	ld -s -o linked.bin objects/main.o objects/linked.o -lc --dynamic-linker /lib/ld-linux-x86-64.so.2
