# Loaders and Bootloaders  

Now that we have written a compile-worthy no-std binary... what next?  

We cannot just run a driver on metal like that. We need to have a program that boots up the machine before running our driver(our no-std file).  

And for this purpose, we introduce two new parties : **Loaders** and **Bootloaders**.  
Our firmware needs to get loaded into memory by either the Loader or the Bootloader.  


## Difference between a loader and a bootloader.  

A loader and a bootloader are both involved in the process of loading software into memory(**RAM*) for execution, but they serve different purposes and operate at different stages of the system startup process.  

### Bootloader:  

A bootloader is a small program that is executed when a computer system is powered on or restarted. Its primary function is to initialize the hardware, perform basic system checks, and load the operating system into memory for execution.  

Bootloaders are typically stored in a specific location on the storage device (e.g., the Master Boot Record on a hard disk drive or in the boot ROM of an embedded system).  

The bootloader is responsible for locating the operating system kernel, loading it into memory, and transferring control to the kernel to begin the boot process.  
Examples of bootloaders include GRUB (Grand Unified Bootloader) and U-Boot (Universal Bootloader), which are commonly used in Linux systems.  

But in our case, since we do not have a kernel in sight, the bootloader will load ***our no-std file***. Our no-std file will act as a temporary kernel... or rather, it will act as an execution runtime that can call the UART driver whenever it is needed.  


### Loader:  

A loader, also known as a program loader, is a component of an operating system that loads executable files from storage(eg SSD) into memory(eg RAM) and prepares them for execution.  

Loaders operate after the operating system kernel has been loaded and initialized by the bootloader. They are responsible for loading user-space programs, shared libraries, and other executable files as needed during the runtime of the system.  

Loaders perform tasks such as resolving external references, allocating memory for program code and data, setting up the program's execution environment, and transferring control to the entry point of the program.  
In some cases, the term "loader" may also refer to a component of a development toolchain responsible for generating executable files from source code by linking together various object files and libraries.  

So in our case, the loader will a part of the execution runtime (ie our no-std file that was acting as a minimal kernel)  

The loader will have the following functions :  
- listen for loading & unloading orders from the minimal-kernel
- execute the the loading and unloading.  

Loading a program involves things such as ; 
- copying the Program's loadable-elf-sections from ROM/HDD/SDD and putting them in the RAM.
- adjusting the necessary CPU registers. For example, making the Program counter to point to the entry point of the program that needs to be loaded.
- Setup stack-protection (if necessary)
- Ensuring that the metadata for the program is available for the minimal-kernel.  

Unloading a program involves things such as :  
- cleaning the program stack and zeroing out any 'confidential' program sections to avoid data-stealing.
- adjusting the necessary CPU registers. For example, making the Program counter to point back to the minimal kernel  


## Bootloaders in Qemu-Riscv Virt machine

When using the sifive_u or virt machine in Qemu, there are three different firmware boot options:


```bash
-bios default # option 1
```  
This is the default behaviour if no -bios option is included. This option will load the default OpenSBI firmware automatically. The firmware is included with the QEMU release and no user interaction is required. All a user needs to do is specify the kernel they want to boot with the -kernel option.  
<br>

```bash
-bios none    # option 2
```  
QEMU will not automatically load any firmware. It is up to the user to load all the images they need.  
<br>


```bash
-bios <file>  # option 3
```  
Tells QEMU to load the specified file as the firmware.
