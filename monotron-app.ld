MEMORY
{
    SRAM (rwx) : ORIGIN = 0x20002000, LENGTH = 0x00006000
}

EXTERN(ENTRY_POINT);

SECTIONS
{
    .entry ORIGIN(SRAM) :
    {
        KEEP(*(.entry_point))
    } > SRAM

    .text :
    {
        *    (.text .text.*)
        *    (.init)
        *    (.fini)
    } > SRAM

    .rodata : ALIGN(4)
    {
        *(.rodata .rodata.*);
        /* 4-byte align the end (VMA) of this section.
           This is required by LLD to ensure the LMA of the following .data
           section will have the correct alignment. */
        . = ALIGN(4);
    } > SRAM

    .data : ALIGN(4)
    {
        *    (.data .data.*)
        . = ALIGN(4);
    }  > SRAM

    .bss (NOLOAD) : ALIGN(4)
    {
        *    (.bss .bss.*)
        . = ALIGN(4);
    } > SRAM

    /* ## .got */
    /* Dynamic relocations are unsupported. This section is only used to detect relocatable code in
     the input files and raise an error if relocatable code is found */
    .got (NOLOAD) :
    {
        KEEP(*(.got .got.*));
    }

    /* ## Discarded sections */
    /DISCARD/ :
    {
        /* Unused exception related info that only wastes space */
        *(.ARM.exidx.*);
    }
}
