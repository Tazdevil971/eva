PHDRS
{
    rom PT_LOAD FLAGS(5);
    ram PT_LOAD FLAGS(6);
}

EXTERN(_start)
ENTRY(_start)

SECTIONS
{
    . = 0x400000;
    .text : 
    {
        __executable_start = .;
        __stext = .;
        *(.text)
        *(.text.*)
        __etext = .;
    } :rom

    .rodata : ALIGN(16)
    {
        *(.rodata)
        *(.rodata.*)
    } :rom

    .eh_frame_hdr : { 
        __GNU_EH_FRAME_HDR = .;
        KEEP(*(.eh_frame_hdr)) 
    } :rom
    .eh_frame : { KEEP(*(.eh_frame)) } :rom

    /* Align to a sane boundary */
    . = ALIGN(CONSTANT(MAXPAGESIZE));

    .data : ALIGN(16) 
    {
        __sdata = .;
        *(.data)
        *(.data.*)
        __edata = .;
    } :ram

    .bss (NOLOAD) : ALIGN(16)
    {
        __sbss = .;
        *(.bss)
        *(.bss.*)
        . = ALIGN(8);
        __ebss = .;
    } :ram

    __irq_stack_size = 64K;
    __heap_size = 8M;

    .heap (NOLOAD) :
    {
        __heap_start = .;
        . += __heap_size;
        __heap_end = .;
    } :ram

    .irq_stack (NOLOAD) : ALIGN(16)
    {
        __irq_stack_bottom = .;
        . += __irq_stack_size;
        __irq_stack_top = .;
    } :ram
}