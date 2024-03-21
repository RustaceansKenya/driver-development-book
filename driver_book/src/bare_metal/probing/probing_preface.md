# Probing  

Probing is the act of interacting with the microcotroller with the aim of doing at least one of the following... 
1. Flashing a compiled program onto the RAM or ROM of the microcontroller. ie. writing your code into the RAM or ROM... breathing life into the machine.  

2. Performing some In-system programming ie. literally manipulating the values found in the processor's registers and occasionaly reading and writing to memory.  
 
3. Debugging the running program : Observing the how the program state changes in a step by step fashion.  

4. Testing the functionality of the microcrontroller

This chapter walks through the theory behind first 3 tasks while assuming that...  
- Your host machine is a linux box  
- Your target machine is an esp32c3 SoC.  

The practicals will be covered in a later chapter.  
<!-- undone: Specify the actua chapter -->