# Firmware versus Drivers

Like we said, definitions in tech are somehow subjective.    

## Firmware
We had earlier defined Firmware as... 
>**Firmware** is software that *majorly* controls the hardware. It *typically* gets stored in the ROM of the hardware 

There are many things that control hardware, even your browser controls hardware. So does that make it firmware? The kernel controls hardware, does that make it firmware? - No.  

If I decide to store a picture of an Anime-cat-girl in the ROM of a device, does that make it firmware? If I decide to store a small OS in the ROM, does it make it firmware? - No.  

What if I store the code used to control hardware in a seperate hard-disk instead of the ROM? Does that make the code lose its status of being called firmware? - NO.  

In the end, it is up to you, to decide if you want to call your code firmware.  


## Drivers

We had earlier defined  drivers as ...  
> a **Driver** is software that controls the hardware AND provides a higher level interface for something like a kernel or a runtime.  

### Drivers and hardware control
A driver controls hardware, but it is not as intimate to the machine as firmware.  
This is because the firmware is made specifically for the external device's motherboard, its code references specific registers **within** the external device. The driver on the other hand is more generic, it is not really made for the specific external device's motherboard, its code mostly references the registers of the host computer and the devices external registers. The driver rarely references the internal registers found in the external device.  
Drivers even use firmware as dependencies.  
The main point here is that the driver is a higher level of abstraction that is dependent on firmware as its building block.  
So in as much as it controls hardware, it is not as intimate as how the firmware does it.  

Here is a situation, A hard-disk driver may initiate a read-operation that will result in the spinning of disks within the external hard-disk. But its the firmware that will control how those disks spin.  


### The thin line
What if I write a driver that references the internal registers of an external device, does that make my driver to also be Firmware?  

What if I create firmware that exposes a mature & generic API that can be referenced to by the Kernel/Runtime code. Does it make that firmware to also be a driver?  


I hope that this section has caused more confusion. Thank you and Bye.  