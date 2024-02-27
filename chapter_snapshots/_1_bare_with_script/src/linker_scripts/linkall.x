/* This is a linker script specifically made to instruct the linker on .., */
/* ... how to build elf files for virt Riscv Machine on Qemu */

/* The explanation for this script has been given in the example in the subchapter */

OUTPUT_ARCH( "riscv" )


ENTRY( _start )  /* _start will be defined as part of the Boot code*/

MEMORY
{
  ram : ORIGIN = 0x80000000, LENGTH = 128M
}


PHDRS
{
  text PT_LOAD;
  data PT_LOAD;
  bss PT_LOAD;
}


SECTIONS
{
  . = ALIGN(4096);
  .text : {
	 

    PROVIDE(_text_start = .);

    *(.text.init)
	  *(.eh_frame)
	  *(.text .text.*)
    PROVIDE(_text_end = .);
	
  } >ram AT>ram :text
   
   PROVIDE(_global_pointer = .);

  
  . = ALIGN(4096);
  .rodata : {
    PROVIDE(_rodata_start = .);
    *(.rodata .rodata.*)
    PROVIDE(_rodata_end = .);

  } >ram AT>ram :text

  .data : {
	
    . = ALIGN(4096);
    PROVIDE(_data_start = .);

    *(.sdata .sdata.*) *(.data .data.*)
    PROVIDE(_data_end = .);
  } >ram AT>ram :data

  . = ALIGN(4096);
  .bss : {
    PROVIDE(_bss_start = .);
    *(.sbss .sbss.*) *(.bss .bss.*)
    PROVIDE(_bss_end = .);
  } >ram AT>ram :bss

  
  PROVIDE(_memory_start = ORIGIN(ram));
  
  . = _bss_end;
  . = ALIGN(4096);
  PROVIDE(_stack_start = .);

  . = _stack_start;
  . = . + 0x80000; /* The STACK is 512 KB */

  PROVIDE(_stack_end = .);
  PROVIDE(_memory_end = ORIGIN(ram) + LENGTH(ram));

  . = ALIGN(4096);
  PROVIDE(_heap_start = .);
  PROVIDE(_heap_size = _memory_end - _heap_start);
  PROVIDE(_heap_end = _memory_end - 1);  /* minusing 1 because the memory_end address is unusable. Let the last address be usable*/
  
}