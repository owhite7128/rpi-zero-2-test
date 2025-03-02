ENTRY(_start)

SECTIONS
{
    . = 0x80000;
    .text.boot : { 
        KEEP(*(.text._start))
        . = ALIGN(8);
    }

    .vector_table : 
    {
        . = ALIGN(2048);
        *(.vector_table)
    }

    .text :
    { 
        *(.text)
        . = ALIGN(8);
    }
    .rodata :
    {
        *(.rodata.*)
        . = ALIGN(16);
    }
    .data :
    {
        *(.data)
        . = ALIGN(16);
    }

    . = ALIGN(16);
    _stack_end = .;
    . = . + 0x80000;      /* Reserve 32KB stack */
    _stack_start = .;

    .bss : ALIGN(16) {
        *(.bss .bss.*)
    }
}