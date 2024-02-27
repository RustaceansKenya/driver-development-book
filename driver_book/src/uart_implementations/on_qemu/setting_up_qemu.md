# Setting up the Riscv Virtual environment


We will be using the [Qemu RISC-V System emulator](https://www.qemu.org/docs/master/system/target-riscv.html) to emulate a RISCV-CPU microcontroller. 

### Installation
To install Qemu, input the following terminal commands  
```bash  
sudo apt install qemu-user
sudo apt install qemu-system-misc
```

#### Qemu Configurations  

*1. Machine to be emulated*  
For QEMU’s RISC-V system emulation, you must specify which board model you want to emulate with the `-M` or `--machine` Qemu-configuration option; there is no default machine selected.  
In our case, we will emulate the [‘virt’ Generic Virtual Platform ](https://www.qemu.org/docs/master/system/riscv/virt.html)as our target board model.  

*2. Booting mode*  
When using the sifive_u or virt machine there are three different firmware boot options: 
1. `-bios default` - This is the default behaviour if no -bios option is included. This option will load the default OpenSBI firmware automatically. The firmware is included with the QEMU release and no user interaction is required. All a user needs to do is specify the kernel they want to boot with the -kernel option 
2. `-bios none` - QEMU will not automatically load any firmware. It is up to the user to load all the images they need. 
3. `-bios <file>` - Tells QEMU to load the specified file as the firmware.


We will use the following Qemu configurations ;
```bash 
# let's define some variables 
QEMU=qemu-system-riscv64  # we are using the Riscv Qemu emulator. qemu-system-riscv64 is a variable containing the path to the QEMU executable
MACH=virt                 # we will target the Virt Riscv Machine 
CPU=rv64                  # we will use a 64-bit CPU
CPUS=4                    # The Board will have 4 CPUs... 4 HARTS
MEM=128M                  # The RAM memory will be 128 MBs
// # DRIVE=hdd.dsk             // This is the path to our virtual harddrive

# Let's substitute the above variables into Qemu-configuration options
$(QEMU) -machine $(MACH) 
        -cpu $(CPU)      
        -smp $(CPUS)     # specifies the number of CPUs to emulate
        -m $(MEM)        # specifies the amount of RAM in MBs 
        -nographic       # disables graphical output, so QEMU runs in a terminal window.
        -serial mon:stdio # connects the virtual machine motherboard's serial port to the host's system terminal. Ie, our Linux terminal. This enables us to use the terminal as a console to the virtual machine.
        -bios none       # we not depend on any firmware becaue our machine is virtual. We can just direclty load the kernel image to memory. 
        -kernel $(OUT)  # This specifies the path to the loader/driver/kernel image file
```


So whenever we run a Qemu emulation, we should run it with the above config files


### Template
There is no template for this subchapter. Try writing the qemu commands by hand before we integrate them to cargo.  