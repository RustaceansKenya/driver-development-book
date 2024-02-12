INCLUDE "memory.x"


/* memory abstraction setion */
REGION_ALIAS("ROTEXT", IROM);
REGION_ALIAS("RODATA", DROM);

REGION_ALIAS("RWDATA", DRAM);
REGION_ALIAS("RWTEXT", IRAM);

REGION_ALIAS("RTC_FAST_RWTEXT", RTC_FAST);
REGION_ALIAS("RTC_FAST_RWDATA", RTC_FAST);

ENTRY(_start);

SECTIONS {
    
    .text : {
        *(text)
    } > RWTEXT

    .data : {
        *(data)
        *(COMMON)
    } > RWDATA

    .bss : {
        *(bss)
    }

    .rodata : ALIGN(4)
    {
        . = ALIGN (4);
        _rodata_start = ABSOLUTE(.);
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
        . = ALIGN(4);
        _rodata_end = ABSOLUTE(.);
    } > RODATA

    .rodata.wifi : ALIGN(4)
    {
        . = ALIGN(4);
        *( .rodata_wlog_*.* )
        . = ALIGN(4);
    } > RODATA

}