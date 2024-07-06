# Intro to Drivers

This chapter is filled with definitions.  

And as you all know, there are NO right definitions in the software world. People still debate what 'kernel' means. People are okay using the word 'serverless' apps. What does AI even mean? It's chaos everywhere.  

So the definitions used here are constrained in the context of ***this*** book.  

### What's a driver?  What's firmware?  

In simple terms. Firmware is the code engrained on a device's motherboard. The driver is code that sits in between the kernel and the device(inclusive of the firmware).  

Firmware is a piece of software that typically gets engrained so close to the hardware and it controls the physical functions of the hardware.  
Okay, that was an exageration. Firmware usually gets stored in the ROM of a device. And yes, it may control the circuitry of the device, thereby controlling the physical device.  

A driver is a piece of software that abstracts away the device for other pieses of software like the kernel. 
So the first piece of software on top of a physical device is called a driver.  


A driver typically sits in-between a high-leve program and the physical device.  
The high level program could be a kernel in this case. And the physical device could be an SSD disk attached to the motherboard.  

The driver has 2 primary functions : 
1. Controll the underlying device. (the SSD)
2. Provide an interface for the kernel/higher-level program to interact with. The interface could contain things like public functions, data_structures and message passing endpoints that somehow manipulate how the driver controls the underlying device...

Here is a Bird's eye-view of the driver's ecosystem:  
![Alt text](img/birds_eye_view_upper.svg)

Demo : 
![Alt text](img/birds_eye_view.svg)

Let's break down the two main roles of the driver... 





