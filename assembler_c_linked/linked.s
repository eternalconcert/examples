section .text
global _start

_start:
    extern _hello_world
    call _hello_world

    mov eax, 1
    int 0x80
