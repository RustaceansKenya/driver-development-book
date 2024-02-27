# The startup code does the following actions : 
# 1. It describes a function that helps us find the correct exception handlers. Something like an `switch` for exception-handling functions
# 2. Chooses a HART/CORE that will execute the rest of the activities below
# 3. Copies all initialized data from FLASH to RAM.  
# 4. Copies all un-initialized data from FLASH to RAM.
# 5. Zeroes-out all the un-initialized data
# 6. Sets up the stack pointer  
# 7. Calls the `main` function in our rust code




# notify the assembler that we will not be using Riscv Compressed instructions
# we need simplicity and predictability more than we need memory efficient code
.option norvc # this is an assembler directive

# this is where we will store global initialized variables
# and we have no global data yet
.section .data

# this is code that will get called before the kmain function
# .text.init sections typically store startup code that sets up the environment for the rest of the code
.section .text.init 

# _start is declared as a global symbol so that the linker gets to detect it 
# This will be the entry point of the bootloader
.global _start
_start:
    j   _choose_bootloading_HART
    
# The gp register currently contains the gp_memory address of the loader.
# We need to update it to point to the kernel's gp
# We numb out optimizations to make sure the update happens explicitly
_fetch_kernel_global_pointer:
    .option push    # save and disable all current assembler directives
    .option norelax # disable code optimization, this is a delicate operation; we need no surprises
    la gp, _global_pointer # load the address of _global_pointer label into the gp register
    .option pop  # restore previous assembler directives
    j _clear_BSS_section


_choose_bootloading_HART:
    # fetch the ID of the current Hardware Thread (HART) and store it in the temporary register t1
    csrr t1, mhartid 
    bnez t1, _make_HART_sleep # If HART ID is not ZERO, make that HART sleep.
                             # If HART IS is zero, _fetch_kernel_global_pointer
    j   _fetch_kernel_global_pointer  # after choosing the HART, we move on to configure essential registers 
                                      # [gp, sp, ]
    

# this does not completely shut down the HART
_make_HART_sleep:
    wfi                 # power off and wait for an interrupt
    j _make_HART_sleep  # continuously make HART sleep, we are running a single_core OS

# the bootloader needs to make sure that all uninitialized dlobal values of...
# ...the kernel are ZEROED out
_clear_BSS_section:
    la a1, _bss_start
    la a2, _bss_end
    j _clear_BSS_section_loop

_clear_BSS_section_loop:
    sd      zero, (a1)                          # store z mepc, mieero in the 64bit memory space referenced by a1
    addi    a1, a1, 8                           # increment the address by 64 bits. (8 bytes)
    bltu    a1, a2, _clear_BSS_section_loop     # loop until we reach the last address of the bss section
    j       _initialize_registers_for_kmain     # if we have zerord out the BSS section, _initialize_registers_for_kmain

_initialize_registers_for_kmain:
    la		sp, _stack_end                          # setup the stack pointer
    li		t0, (0b11 << 11) | (1 << 7) | (1 << 3)  # Set MPP field to 11 (Machine Mode), 
                                                    # Bit 7, sets MPIE bit to 1 ; meaning interrupts from lower levels can get handled by machine mode if invoked
                                                    # Bit 3, Sets the MIE bit to 1 ; meaning the CPU can receive interrups while in machine mode
    csrw	mstatus, t0

    # set kmain to be the value that will be pasted tp the PC counter after calling mret
    la		t1, kmain
	csrw	mepc, t1   

    #set the Machine trap vector
    la		t2, asm_trap_vector
	csrw	mtvec, t2  

    # allow specific interrupts
    # 3 == Software Interrupts, 7 == Timer Interrupts, 11 == External Interrupts
    li		t3, (1 << 1) | (1 << 3) | (1 << 5) | (1 << 7) | (1 << 9) | (1 << 11) 
	csrw	mie, t3

    #  set kmain return address to _make_HART_sleep (a shutdown)
    la      ra, _make_HART_sleep

    # call kmain (indirectly, this is because mret will make the cpu program counter to point to the value in mepc(kmain))
    mret

