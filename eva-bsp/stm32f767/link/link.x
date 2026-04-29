
MEMORY
{
    XRAM(wx)  : ORIGIN = 0xd0000000, LENGTH =  32M
    SRAM(wx)  : ORIGIN = 0x20020000, LENGTH = 384K
    DTCM(wx)  : ORIGIN = 0x20000000, LENGTH = 128K
    FLASH(rx) : ORIGIN = 0x08000000, LENGTH =   2M
}

INCLUDE "exceptions.x"

EXTERN(__eva_default_init);
PROVIDE(_init = __eva_default_init);
EXTERN(__eva_default_fini);
PROVIDE(_fini = __eva_default_fini);

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

        . = ALIGN(4);
        __preinit_array_start = .;
        KEEP (*(.preinit_array))
        __preinit_array_end = .;

        . = ALIGN(4);
        __init_array_start = .;
        KEEP (*(SORT(.init_array.*)))
        KEEP (*(.init_array))
        __init_array_end = .;

        . = ALIGN(4);
        __fini_array_start = .;
        KEEP (*(.fini_array))
        KEEP (*(SORT(.fini_array.*)))
        __fini_array_end = .;      
    } > FLASH

    __exidx_start = .;
    .ARM.exidx :
    {
        *(.ARM.exidx* .gnu.linkonce.armexidx.*)
    } > FLASH
    __exidx_end = .;

    .data : ALIGN(8) 
    {
        _data = .;
        __sdata = .;
        *(.data)
        *(.data.*)
        *(.gnu.linkonce.d.*)
        __edata = .;
    } > SRAM AT > FLASH
    __sidata = LOADADDR(.data);

    .bss (NOLOAD) : ALIGN(8)
    {
        __sbss = .;
        *(.bss)
        *(.bss.*)
        *(.gnu.linkonce.b.*)
        *(COMMON)
        . = ALIGN(8);
        __ebss = .;
    } > SRAM

    __irq_stack_size = 64K;

    .heap (NOLOAD) :
    {
        __heap_start = .;
        . = ORIGIN(SRAM) + LENGTH(SRAM) - __irq_stack_size;
        __heap_end = .;
    } > SRAM

    .irq_stack (NOLOAD) :
    {
        __irq_stack_bottom = .;
        . += __irq_stack_size;
        __irq_stack_top = .;
    } > SRAM

}