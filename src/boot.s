.section .text.boot
.global _start

_start:
    // Save boot parameters
    mov x20, x0
    mov x21, x1
    mov x22, x2
    mov x23, x3

    // Set up stack
    ldr x0, =_start
    mov sp, x0

    // Clear BSS
    ldr x0, =__bss_start
    ldr x1, =__bss_end
    sub x1, x1, x0
    bl memzero

    // Jump to kernel
    mov x0, x20
    mov x1, x21
    mov x2, x22
    mov x3, x23
    bl _kernel_main

    // Should never reach here
1:  wfe
    b 1b

memzero:
    str xzr, [x0], #8
    subs x1, x1, #8
    b.gt memzero
    ret

.section .bss
.align 16
stack_bottom:
.skip 4096 * 4
stack_top:
