global start
extern rust_main

section .text
bits 32

start:
    call rust_main

.hang:
    cli
    hlt
    jmp .hang
