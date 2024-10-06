# Abstracting a Peripheral  

Instead of writing paragraphs, let's just build peripheral drivers and hope to learn along the way.  

We will build PACs above the following peripherals:  
1. The SystemTimer for a cortex-m board
2. The SystemTimer for an ESP32 board
3. The 16550A UART found in the Qemu board
4. The UART found in the ESP32 board.  


### Why is the list long?
The list is long because we want to learn through practice. If you are in a hurry, I suggest you look at the PAC for the UART found in the ESP32 board.  

### Why those peripherals?  
1. The cortex-m timer is very basic. It has 4 registers only. For this reason, it was chosen as the first practice point.  
2. The ESP32 system timer is not as basic as the cortex-m timer, but it is less complex in comparison to the UART. For this reason, it was chosen as an intermediate step.  
3. A virtual UART gives us room to make mistakes
4. I don't think we have to explain why the last peripheral was chosen.  


Off we go....