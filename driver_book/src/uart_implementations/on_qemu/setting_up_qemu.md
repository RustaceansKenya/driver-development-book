# Setting up the Riscv Virtual environment

[Crude](./theory_on_Qemu.md)  

We will be using the [Qemu RISC-V System emulator](https://www.qemu.org/docs/master/system/target-riscv.html) to emulate a RISCV-CPU microcontroller. 

#### How to install Qemu RISCV system Emulator on Linux-Mint
At the command type  
```bash  
sudo apt install qemu-user
sudo apt install qemu-system-misc
```

#### Qemu Configurations
For QEMU’s RISC-V system emulation, you must specify which board model you want to emulate with the -M or --machine option; there is no default. In our case we will emulate the [‘virt’ Generic Virtual Platform ](https://www.qemu.org/docs/master/system/riscv/virt.html)as our target board model

When using the sifive_u or virt machine there are three different firmware boot options: 
1. -bios default - This is the default behaviour if no -bios option is included. This option will load the default OpenSBI firmware automatically. The firmware is included with the QEMU release and no user interaction is required. All a user needs to do is specify the kernel they want to boot with the -kernel option 
2. -bios none - QEMU will not automatically load any firmware. It is up to the user to load all the images they need. 
3. -bios --file - Tells QEMU to load the specified file as the firmware.

We will use the following Qemu configurations ;
```bash 
// we define some variables 
QEMU=qemu-system-riscv64  // we are using the Riscv Qemu emulator. qemu-system-riscv64 is a variable containing the path to the QEMU executable
MACH=virt                 // we will target the Virt Riscv Machine 
CPU=rv64                  // we will use a 64-bit CPU
CPUS=4                    // The Board will have 4 CPUs... 4 HARTS
MEM=128M                  // The RAM memory will be 128 MBs
DRIVE=hdd.dsk             // This is the path to our virtual harddrive

$(QEMU) -machine $(MACH) 
        -cpu $(CPU) 
        -smp $(CPUS)     // specifies the number of CPUs to emulate
        -m $(MEM)        // specifies the amount of RAM in MBs 
        -nographic       // disables graphical output, so QEMU runs in a terminal window.
        -serial mon:stdio // connects the virtual machine motherboard's serial port to the host's system terminal. Ie, our Linux terminal. This enables us to use the terminal as a console to the virtual machine.
        -bios none       // we not depend on any firmware becaue our machine is virtual. We can just direclty load the OS image to memory. 
        -kernel $(OUT)  // This specifies the path to the kernel image file
        -drive if=none,format=raw,file=$(DRIVE),id=attic // explained below
        -device virtio-blk-device,scsi=off,drive=attic     // explained below
```

```bash
-drive if=none,format=raw,file=$(DRIVE),id=attic
```
**'if=none'** meant that Qemu should not create an interface between the hard drive and the Kernel image. An example of an interface is SATA interface.

'**format=raw**' means that the hard drive image should consist of raw bytes to represent data on the disk. The disk should no have extra metadata or compressions.
Other possible values for the format option include:

- qcow2: This is the default format for disk images in QEMU/KVM, and it supports features like compression, snapshots, and encryption.
- mdk: This is a format used by VMware virtualization software.
- vpc: This is a format used by Microsoft Virtual PC.
- raw: This is similar to format=raw, but it includes a 512-byte header that specifies the disk geometry and other information.

The choice of disk image format depends on the specific needs of your virtualization environment. For example, if you need to support snapshots or compression, you would likely choose qcow2. If you need to import or export the image to another virtualization platform, you may need to choose a format that is compatible with that platform.

```bash
-device virtio-blk-device,scsi=off,drive=attic
```

**'-device'** is a Qemu command for attaching new devices to the motherboard of the virtual machine.

**virtio-blk-device,scsi=off,drive=attic** implies that we are adding a block device that adheres to VIRTIO protocol. '**scsi=off**' disables the SCSI (Small Computer System Interface), this is because we intend to write a custom virtio block driver. '**drive=attic**' specifies the Identifier of the new device that is being attached.


#### Creating a virtual hard disk
In the configurations above, it was specified that a virtual hard disk would get attached to the motherboard. It was specified that its path would be ./hdd.dsk

To create this hard disk we use a tool called [Losetup](https://man7.org/linux/man-pages/man8/losetup.8.html). This tool converts a normal text file into a virtual block hard drive.

Losetup creates Loop devices. A loop device is a file that emulates a block device.

Losetup comes pre-installed in any standard linux distribution. To check its documentation, type this in the terminal:
```bash
man losetup
```

To create a virtual disk within your development working dierctory, write the following command in your terminal: 
```bash
dd if=/dev/zero of=hdd.dsk count=32 bs=1M  
``` 
where : 
- '**if=/dev/zero**: This option specifies the input file to use for the dd command. In this case, the input file is /dev/zero, which is a special file that produces an endless stream of zeroes when read.
- **of=hdd.dsk**: This option specifies the output file to create for the dd command. In this case, the output file is called hdd.dsk.
- **count=32**: This option specifies the number of blocks to copy from the input file to the output file. In this case, 32 blocks of data will be copied.
- **bs=1M**: This option specifies the block size to use for the dd command. In this case, the block size is 1 megabyte (1M).


An alternative set of commands would be :
```bash
fallocate --length 32M hdd.dsk  // create a new file called hdd.dsk and allocate to it 32 MB
sudo losetup /dev/loop0 hdd.dsk // convert hdd.dsk into a  virtual hard drive whose mount point is at /dev/loop0
```