# Building the UART on qemu

This chapter elaborates on how to write a UART driver for a virtual UART emulated by the [Qemu machine emulator][qemu-emulator-front-page].  
The UART in this chapter is naive. It does not use standard & safe abstractions. It is also blocking in nature.  

The UART covered in the next-next chapter will be an improved of the UART covered in this chapter.  


## Qemu

QEMU is a generic and open source machine emulator and virtualizer.  

A machine emulator is a software program that simulates the behaviour of another computer or another computing system. For example you may simulate the behavior of a quantum computer on a convetional computer.

A virtualizer is a program that abstracts away an underlying system. The underlying system can be anything : Bare metal cpu, a hard disk, an operating system... anything.

QEMU can be used in several different ways. The most common is for System Emulation, where it provides a virtual model of an entire machine (CPU, memory and emulated devices) to run a guest OS. In this mode the CPU may be fully emulated, or it may work with a hypervisor such as KVM, Xen, Hax or Hypervisor.Framework to allow the guest to run directly on the host CPU.

The second supported way to use QEMU is User Mode Emulation, where QEMU can launch processes compiled for one CPU on another CPU. In this mode the CPU is always emulated.

In our project, we will use Qemu as a [Riscv System Emulator](https://www.qemu.org/docs/master/system/target-riscv.html).   


## Templates (hints)
The next few chapters are going to be about setting things up. At the end of each sub-chapter, you will see a link to a finished template containing a cargo project that has been modified in accordance to the concerned sub-chapter.  

The templates are not guaranteed to be always compile-worthy. They are meant to act as **Hints** -- Not copyable short-cuts. Try to understand them before moving on. Examine them, things will click with time.  



























[qemu-emulator-front-page]: https://www.qemu.org/  
[qemu-machine-emulator-explained]: ../../uart_implementations/on_qemu/qemu_emulator_explained/intro.md