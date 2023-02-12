
/*https://ftp.gnu.org/old-gnu/Manuals/ld-2.9.1/html_node/ld_26.html*/

OUTPUT_ARCH( "riscv" ) /* Specify output machine architecture */

ENTRY( _start )

PHDRS {         /*Instructs the linker to only load the specified program headers - https://sourceware.org/binutils/docs/ld/PHDRS.html*/
  text PT_LOAD;
  rodata PT_LOAD;
  data PT_LOAD;
  bss PT_LOAD;
}
/* Short refresher on sections/segments used here:
  - text
  Code Section containing the programm instructions, read only
  - rodata
  Read-only data, contains initialized constants
  - data
  Contains initialized static variables
  - bss
  Contains both uninitialized static variables and static constants
*/
                          /*0x80000000 is the start of the DRAM*/
SECTIONS {                /*Specifies how the sections are merged and where they are placed*/ 
  . = 0x80000000;         /*Sections will be copied to this adress, '.' represents the location counter and is initially set to 0x0*/
  .text : {
    . = ALIGN(16);        /*Insert Padding Bytes until current location becomes align on 16-Byte boundary*/
    *(.text.init) *(.text .text.*)  /* Define that the .text.init section, the .text section and all subsections of */
    . = ALIGN(16);                  /* .text should be in the output files .text section in that order */
  }

/*The following is all analagous to the explanations of the text section */
  .rodata : {
    . = ALIGN(16);
    *(.rodata .rodata.*)
  }

  .data : {
    . = ALIGN(4096);  /* Align to Pages?*/
    *(.sdata .sdata.*) *(.data .data.*)
  }

  .bss :{
    . = ALIGN(16);
    *(.sbss .sbss.*) *(.bss .bss.*)
    PROVIDE(_bss_end = .); /* the symbol _bss_end is only defined if it is referenced but not defined*/
  }


  PROVIDE(_stack_start = _bss_end);
  PROVIDE(_stack_size = 0x80000); /*(524 KiB)*/
  PROVIDE(_stack_end = _stack_start + _stack_size);
}