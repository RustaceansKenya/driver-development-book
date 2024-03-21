# Intro to Drivers

This chapter is filled with definitions.  

And as you all know, there are NO right definitions in the software world. People still debate what 'kernel' means. People are okay using the word 'serverless' apps. What does AI even mean? It's chaos everywhere.  

So the definitions used here are constrained in the context of this book.  

### What's a driver?

A driver is the **first** piece of software that abstracts the circuitry a physical device. We have used the word **first** because technically... even the browser indirectly abstracts hardware.  

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





