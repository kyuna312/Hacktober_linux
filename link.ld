ENTRY(_start)

SECTIONS
{
    . = 0x40080000;
    .text.boot : {
        *(.text.boot)
    }
    .text : {
        *(.text*)
    }
    .rodata : {
        *(.rodata*)
    }
    .data : {
        . = ALIGN(16);
        *(.data*)
    }
    .bss : {
        . = ALIGN(16);
        __bss_start = .;
        *(.bss*)
        *(COMMON)
        __bss_end = .;
    }
    /DISCARD/ : {
        *(.eh_frame*)
        *(.comment)
        *(.note*)
    }
}
