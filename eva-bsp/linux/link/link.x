PHDRS
{
    headers PT_PHDR PHDRS;
    rom PT_LOAD FILEHDR FLAGS(5);
    ram PT_LOAD FLAGS(6);
}

EXTERN(__eva_default_init);
PROVIDE(_init = __eva_default_init);
EXTERN(__eva_default_fini);
PROVIDE(_fini = __eva_default_fini);

EXTERN(_start)
ENTRY(_start)

SECTIONS
{
    . = 0x400000;
    __executable_start = .;
    . += SIZEOF_HEADERS;
    
    .text : 
    {
        __stext = .;
        *(.text)
        *(.text.*)
        __etext = .;
    } :rom

    .rodata : ALIGN(16)
    {
        *(.rodata)
        *(.rodata.*)
        
        . = ALIGN(8);
        __preinit_array_start = .;
        KEEP (*(.preinit_array))
        __preinit_array_end = .;

        . = ALIGN(8);
        __init_array_start = .;
        KEEP (*(SORT(.init_array.*)))
        KEEP (*(.init_array))
        __init_array_end = .;

        . = ALIGN(8);
        __fini_array_start = .;
        KEEP (*(.fini_array))
        KEEP (*(SORT(.fini_array.*)))
        __fini_array_end = .; 
    } :rom

    .eh_frame : ALIGN(8) { 
        __eh_frame = .;
        KEEP (*(.eh_frame)) 
        *(.eh_frame.*) 
    } :rom

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