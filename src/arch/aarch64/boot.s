.section ".text.boot"
.global _start

_start:
    // Check processor ID is zero (primary core)
    mrs     x1, mpidr_el1
    and     x1, x1, #3
    cbz     x1, 2f
1:  wfe
    b       1b
2:
    // Set stack before our code
    ldr     x1, =_start
    mov     sp, x1

    // Clear BSS
    ldr     x1, =__bss_start
    ldr     x2, =__bss_end
    sub     x2, x2, x1
1:  cbz     x2, 2f
    str     xzr, [x1], #8
    sub     x2, x2, #8
    b       1b
2:
    // Jump to Rust code
    bl      _kernel_main
    // Should never reach here
1:  wfe
    b       1b
