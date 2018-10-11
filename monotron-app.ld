MEMORY
{
    SRAM (rwx) : ORIGIN = 0x20000DF4, LENGTH = 0x00006000
}

SECTIONS
{
    .text :
    {
        KEEP(*(.entry_point))
        *    (.text .text.*)
        *    (.init)
        *    (.fini)
        *    (.rodata .rodata.*)
        *    (.data .data.*)
    }

    .bss (NOLOAD) :
    {
        *    (.bss .bss.*)
    }
}
