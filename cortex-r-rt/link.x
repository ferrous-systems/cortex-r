/*
Basic Cortex-R linker script.

You must supply a file called `memory.x` which defines the memory regions 'CODE' and 'DATA'.

The stack pointer will be the top of the DATA region.
*/

INCLUDE memory.x

ENTRY(_vector_table);
EXTERN(_vector_table);

SECTIONS {
    .text : {
        /* The vector table must come first */
        *(.vector_table)
        /* This is our Fast Interrupt Request function - it sits right after the vector table */
        *(.text.fiq_handler)
        /* Now the rest of the code */
        *(.text .text*)
    } > CODE

    .rodata : {
        *(.rodata .rodata*)
    } > CODE

    .data : {
        *(.data .data*)
    } > DATA

    .bss : {
        *(.bss .bss* COMMON)
    } > DATA

    /DISCARD/ : {
        *(.note .note*)
    }

}

PROVIDE(_stack_top = ORIGIN(DATA) + LENGTH(DATA));
PROVIDE(_undefined_handler=_default_handler);
PROVIDE(_svc_handler=_default_handler);
PROVIDE(_prefetch_handler=_default_handler);
PROVIDE(_abort_handler=_default_handler);
PROVIDE(_irq_handler=_default_handler);
