.section .text.boot
.global _start

_start:
    // Set stack pointer
    ldr     x1, =_start
    mov     sp, x1

    // Clear BSS
    ldr     x1, =__bss_start
    ldr     x2, =__bss_end
    sub     x2, x2, x1
    cbz     x2, 2f
1:  str     xzr, [x1], #8
    sub     x2, x2, #8
    cbnz    x2, 1b

2:  // Jump to Rust code
    bl      _kernel_main

    // Should never reach here
3:  wfe
    b       3b
