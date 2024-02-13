# No Std

Most rust programs depend on the standard library by default, including that simple 'hello world' you once wrote. The standard library on the other hand is dependent on the underlying operaring system or execution environment.   

Drivers provide an interface for the OS to use, meaning that the OS depends on drivers... as a result, you have to write the driver code without the help of the OS-dependent Standard Library. This paragraph sounds like a riddle ha ha... but you get the point... to write a driver, you have to forget about help from the typical std library. That std library depends on your driver code... that std library depends on you.     

When software does not depend on the standard library, it is said to be a bare-metal program. It can just be loaded to memory and the physical CPU will execute it as it is.  


Bare metal programming is the art of writing code that assumes zero or almost-zero hosted-environment. A hosted environment typically provides a language runtime + a system interface like POSIX.  

We will procedurally create a bare metal program in the next few sub-chapters.  


