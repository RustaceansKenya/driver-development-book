# Machine code  


From your Computer architecture class, you learnt that the processor is a bunch of gates cramped up together in a meaningful way. A circuit of some sort.  

You also learnt that each processor implements an [ISA (Instruction Set Architecture)](../misc/isa.md).  
As long as you can compile high level code into machine code that is within the bounds of the ISA, that CPU will gladly execute your code. You don't even have to follow a certain [ABI](../misc/abi.md) in order for that code to run.  

The main point here is that : "Machine code that follows a certain ISA can run on any processor that implements that ISA." This is because the processor is a DIRECT implementation of the ISA specifications.  

So the processor that you built in your room can run Rust code. As long as that rust code gets compiled into machine code that follows the ISA specifications of your custom processor.  


