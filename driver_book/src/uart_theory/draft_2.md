# UART Registers  

There are many UART designs and implementations each with different tweaks. So we will stick to the NS16550a UART because it was the one that the Qemu-riscv-virt machine emulates.  

NS16550a UART is also kinda generic. It is an old design, but very simple. It gets the job done without clutter.  
The NS16550a also has two 16-byte FIFO buffers for input and output  

If you want to check out the other different designs and implementations, go through this [table][other_uart_designs_and_implementations].  

## References and Docs
Here are a few guiding docs that will help you learn more about the UART registers and configs.  
1. [A blog-like explanation by Lammert Bies][simple-uart-blog] (start with this, it explains the most important bits without the electrical-engineering jargon)
2. [The 16550A datasheet][the-16550A-datasheet] (use this as a reference. It comes with electrical-references that wil come in handy if you are writing a driver for a physical UART)


The software that interacts with the UART 16550A can...
1. Adjust the speed of both the receiver and transmitter
2. Configure character width
3. Configure data-frame format
4. Activate both the transmitter-buffer and receiver-buffer
5. Set the 16-byte buffers to either FIFO or non_FIFO


There are two 16-byte buffers that come with the circuitry. These buffers are not part of the control registers. They are also NOT part of the data-buffer register(which is only one-byte long). They are independent physical buffers that can be activated or deactiated by the software during the initial configuration of the UART.  



[other_uart_designs_and_implementations]: https://en.wikipedia.org/wiki/Universal_asynchronous_receiver-transmitter#UART_models  
[simple-uart-blog]: https://www.lammertbies.nl/comm/info/serial-uart  
[the-16550A-datasheet]: http://caro.su/msx/ocm_de1/16550.pdf


8 bytes of mmio needed to access the 12 registers of the UART this is because one register can be used to access mutiple registers o certain conditions.   
DLAB switch?  
Behaves differenctly when certain registors are set in a certain way
