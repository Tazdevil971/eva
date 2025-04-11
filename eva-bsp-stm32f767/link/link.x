
MEMORY
{
    XRAM(wx)  : ORIGIN = 0xd0000000, LENGTH =  32M
    SRAM(wx)  : ORIGIN = 0x20020000, LENGTH = 384K
    DTCM(wx)  : ORIGIN = 0x20000000, LENGTH = 128K
    FLASH(rx) : ORIGIN = 0x08000000, LENGTH =   2M
}

INCLUDE "exceptions.x"

EXTERN(Reset);
ENTRY(Reset);

SECTIONS
{
    . = 0;

    .isr_vector : 
    {
        /* First place the boot stack at the end of SRAM */
        LONG(__irq_stack_top);
        KEEP(*(.isr_vector.exceptions));
        KEEP(*(.isr_vector.interrupts));
    } > FLASH

    .text : 
    {
        __executable_start = .;
        __stext = .;
        *(.text)
        *(.text.*)
        *(.glue_7)
        *(.glue_7t)
        __etext = .;
    } > FLASH

    .rodata : ALIGN(4)
    {
        *(.rodata)
        *(.rodata.*)
    } > FLASH

    .data : ALIGN(8) 
    {
        __sdata = .;
        *(.data)
        *(.data.*)
        __edata = .;
    } > SRAM AT > FLASH
    __sidata = LOADADDR(.data);

    .bss : ALIGN(8)
    {
        __sbss = .;
        *(.bss)
        *(.bss.*)
        . = ALIGN(8);
        __ebss = .;
    } > SRAM

    __heap_start = .;
    

    /* We reserve 16KB for the IRQ stack */
    __irq_stack_size = 16 * 1024;

    __heap_end = ORIGIN(SRAM) + LENGTH(SRAM) - __irq_stack_size;
    __irq_stack_top = ORIGIN(SRAM) + LENGTH(SRAM);
}