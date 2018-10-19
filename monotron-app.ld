MEMORY
{
    SRAM (rwx) : ORIGIN = 0x20000DF4, LENGTH = 0x00006000
}

ENTRY(entry_point);
EXTERN(entry_point);

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
