OUTPUT_ARCH( "riscv" )

ENTRY( _start )


PHDRS
{
  text PT_LOAD;
  rodata PT_LOAD;
  data PT_LOAD;
  bss PT_LOAD;
}

SECTIONS
{
  . = 0x80300000;
  .text : {
    . = ALIGN(16);
    *(.text.init) *(.text .text.*)
    . = ALIGN(16);
  }

  .rodata : {
    . = ALIGN(16);
    *(.rodata .rodata.*)
  }

  .data : {
    . = ALIGN(4096);
    *(.sdata .sdata.*) *(.data .data.*)
  }

  .bss :{
    . = ALIGN(16);
    *(.sbss .sbss.*) *(.bss .bss.*)
    PROVIDE(_bss_end = .);
  }

  PROVIDE(_stack_start = _bss_end);
  PROVIDE(_stack_size = 0x80000); /*(524 KiB)*/
  PROVIDE(_stack_end = _stack_start + _stack_size);
}
