.global keyboard_vector
.global mouse_vector

keyboard_vector:
    // Save registers
    stp x29, x30, [sp, #-16]!
    stp x0, x1, [sp, #-16]!

    // Call keyboard handler
    bl keyboard_handler

    // Restore registers
    ldp x0, x1, [sp], #16
    ldp x29, x30, [sp], #16
    eret

mouse_vector:
    // Save registers
    stp x29, x30, [sp, #-16]!
    stp x0, x1, [sp, #-16]!
    stp x2, x3, [sp, #-16]!

    // Call mouse handler
    bl mouse_handler

    // Restore registers
    ldp x2, x3, [sp], #16
    ldp x0, x1, [sp], #16
    ldp x29, x30, [sp], #16
    eret
