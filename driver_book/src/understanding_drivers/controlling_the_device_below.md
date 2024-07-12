# Role 1 : Controlling the Physical device below

> **TLDR** :   
>Software controls the hardware below by either **Direct Register Programming** or **Memory Mapped Programming**. This can be done using Assembly language, low-level languages like C/Rust, or a mixture of both.  


In the previous page, we concluded that BOTH firmware and drivers control hardware.  
So, what does controlling hardware mean? How tf is that possible?  
What is Hardware??



Hardware in this case is a meaningful combination of many electronic circuits. 

### A Hard-disk example
For example, a Hard Disk is made up of circuits that store data in form magnetic pockets, handle data-retrival, handle heat throttling, data encryption... all kinds of magic. 

In fact, let's try to make an imaginary-over-simplified DIY hard-disk.  
Here's a sketch...  
<figure>
  <img src="./img/DIY_disk.png" alt="DIY Hard-disk sketch">
  <figcaption>A DIY Hard-disk Sketch.</figcaption>
</figure>

Here is a break-down of the above image.  

1. **The External interface** 
   The external interface is the port that connects the Hard-disk to the host computer.   
   This interface has 2 functions: 
   - The interface receives an instruction, a memory address and data from the host computer and respectively stores them in the `Instruction Buffer`, `Memory Address Buffer` and `Data Buffer` found within the hard-disk. The acceptable instructions are only two: `READ_FROM_ADDRESS_X` and `WRITE_TO_ADDRESS_X`. <br>
   The `Memory Address Buffer` contains the address in memory where the host needs to either *read from* or *write to*.   <br>
   The `Data Buffer` contains the data that has either *been fetched from the disk* or *is waiting to written to the disk*. <br> 
   - The interface also receives data from within the hard-disk and transmits them to the Host computer
  
1. The ROM contains the SSD's firmware. The SSD firmware contains code that ...
   - handles heat throttling
   - handles the READ and WRITE function of the Actuator Arm
   - handles the movement of the Actuator Arm
   - handles the spinning speed of the disks

2. A **small IC or processor** that implements the SSD's firmware.  

   The micro-processor continuously does a fetch-execute cycle on the `Instruction Buffer`. If the instruction fetched is `READ_FROM_ADDRESS_X`, the processor begins the READ operation. If the instruction fetched is `WRITE_TO_ADDRESS_X`, the processor begins the WRITE operation.  

   **During a READ operation...**  
   - The processor fetches the target memory address from the `Memory Address Buffer`.  
   - The processor fetches firmware code to spin the disks accordingly in order to facilitate a read from the target address.  
   - The processor fetches firmware code to move the Actuator Arm to facilitate a read.  
   - After the read, the processor puts the fetched data in the `Data Buffer`
   - The External Interface fetches data from the `Data Buffer` and transmits it to the Host.  
   - The processor clears the `Instruction Buffer`, `Memory Address Buffer` and `Data Buffer`.
   - The Read operation ends.  

   **During a WRITE operation...**  
   - The processor fetches the target memory address from the `Memory Address Buffer`. 
   - The prcessor fetches the data that is meant to be written from the `Data Buffer`.  
   - The processor fetches firmware code to spin the disks accordingly in order to facilitate a write to the target address.  
   - The processor fetches firmware code to move the Actuator Arm to facilitate a write. 
   - The processor clears the `Instruction Buffer`, `Memory Address Buffer` and `Data Buffer`.
   - The Write operation ends.


### A manual driver?  
If we were in a zombie apocalypse and we had no access to a computer for us to plug in a hard-drive, how would we have stored data into the hard-disk?  

We could store data directly without using a driver-clad Operating system. All we have to do is to supply meaningful electric signals to the external interface of the Hard-disk. You could do this using wires that you collected from the car you just stripped for parts. We are in an apocalypse, remember?

For example, to store the decimal number 10 into the address 0b0101, we could do this... 

Strip, the external interface off and access the 3 registers directly: The `Data Buffer`, `Instruction Buffer` & `Memory Address Buffer`.  
From there, we could supply the electrical signals as follows...  

<figure>
  <img src="./img/DIY_disk_manual_driver.png" alt="Apocalypse Driver">
  <figcaption>Apocalypse Driver, manual signal manipulation.</figcaption>
</figure>


Of-course, this experiment is very hard to do in real life. But come on, we are in an apocalypse and we have just build ourselves an 8-bit DIY hard-disk. Kindly understand.  

###  Programming
We are developers, we automate everything... especially when it is unnecessary.  
So how do we automate this manual manipulation of Hard-disk registers? How??   

Let us imagine that in the middle of the apocalypse, we found a host computer where we can plug in our DIY hard-disk.  

***Solution 1: Direct Register Programming***  

Now that we have access to a Host computer with a CPU, we can attach all the 3 registers DIRECTLY to the CPU of the host computer as shown in the figure below.  

To control which signals reach the individual bits of the 3 registers, we can write some assembly code to change the values of the native CPU registers. It is our assumption that the electrical signals will find their way to the attached registers... (basic electricity)  

This solution gets the job done.  

<figure>
  <img src="./img/DIY_disk_direct_register_programming_redone.png" alt="DIY_disk_direct_register_programming">
  <figcaption>DIY_disk direct register programming.</figcaption>
</figure>



***Solution 2: Memory Mapped Programming***  
The CPU has a limited number of registers. The RAM exists because of this exact reason.  
So instead of directly connecting the Hard-disk registers to the limited CPU registers, we could attach them to the RAM instead.  

We could then write some assembly code to manipulate RAM addresses instead of the CPU register addresses, hence indirectly manipulating the values of the hard-disk registers. This is called Memory-mapped I/O programming (**mmio programming**).  

This is the method that we will stick to because it is more practical.  

You could however use Direct Register Programming when building things like pace-makers, nanobots or some divine machine that is highly specialized and requires 100% performance.  



### Summary
The driver controls the hardware below by either **Direct Register Programming** or **Memory Mapped Programming**. This can be done in Assembly, low-level languages like C/Rust, or a mixture of both.  
