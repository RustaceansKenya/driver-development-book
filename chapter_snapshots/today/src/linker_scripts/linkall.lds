
ENTRY (kmain);  

SECTIONS {

    /* Text section */
    .text : { *(.text) }

    /* data section */
    .data : { *(.data) }

    /* bss section */
    .bss : { *(.bss) }

    /* bss section */
    .rodata : { *(.rodata) }
}

MEMORY {
    /* name [( attr )] : ORIGIN = origin , LENGTH = len */
    ram : ORIGIN = 0x80000000, LEN = 128M
}