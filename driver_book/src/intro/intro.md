# Intro

This book is on driver development using Rust. You get to procedurally write a UART driver for a RISCV chip called [ESP32C3][ESP32C3-datasheet-link] and a [Qemu-riscv-virt board][qemu-riscv-virt-board].  

If you have no idea what a UART driver is, then you have nothing to worry about. You'll get a hang of it along the way.  

This is NOT a book on embedded-development but it will touch up on some embedded-development concepts here and there.  
To learn about Rust on embedded, you are better off reading [The Embedded Rust Book][the-embedded-rust-book].  


## Book phases, topics and general flow.  

The book will take you through 5 phases :  

### Phase 1:  
<hr>

Under phase 1, you get to build a UART driver for a qemu-riscv virtual board. This will be our first phase because it will take you through the fundamentals without having to deal with the intricacies of handling a physical board. You won't have to write flashing algorithms that suite specific hardware. You won't have to read hardware schematics.  

The code here will be suited for a general virtual environment.  

The resultant UART driver at the end of this phase will NOT be multi_thread-safe.


### Phase 2:  
<hr>

We will try to improve our UART driver code to conform to standard APIs like **PAC** and **HAL**. This phase will try to show devs how to make standard drivers that are more portable.   

If you have no idea what HAL and PACs are, you hav nothing to worry about. You'll learn about them along the way.  


### Phase 3:  
<hr>

Both Phase 1 and 2 focus on building a UART driver for a virtual riscv board, BUT phase 3 changes things up and focusses on porting that UART code to a physical board.  

We will modify the previously built UART driver so that it can run on an esp32 **physical** board. We'll set up code harnesses that assist in flashing, debugging, logging and testing the driver-code on the physical board.  

On normal circumstances, people use common pre-made and board-specific tools to do the above processes : ie testing, logging, debugging and flashing.   
For example, developers working with Espressif Boards usually use Espressif tools like [esptool.py](https://docs.espressif.com/projects/esptool/en/latest/esp32/).  

We will not use the standard esp-tools but we will use [probe-rs](https://probe.rs/), this is because esp-tools abstract away a lot of details that are important for driver-devs to master. Esp-rs tools are just too good to use in our use-case... it would be awesome if we write our own flashing algorithms and build or own logging module. Probe-rs is hack-able and allows one to do such bare activities.  

We will however imitate the esp-tools.  

The driver produced in this phase will still NOT be multi_thread-safe.  


### Phase 4: <hr> 
Under Phase 4, we start making our driver to be multi-thread safe. This will be first done in a qemu virtual environment to reduce the complexity. After we have figured our way out of the virtual threads, we will move on to implementing things on the physical board.  



### Phase 5:  
In phase 5, We'll do some brush-up on driver-security and performance testing.   
<hr>


## Why the UART?

The UART driver was chosen because it is simple and hard at the same time. Both a beginner and an experienced folk can learn a lot while writing it.  

For example, the beginner can write a minimal UART and concentrate on understanding the basics of driver development; No-std development,linking, flashing, logging, abstracting things in a standard way, interrupt and error-handling...  
The pseudo_expert on the other hand can write a fully functional concurrent driver while focusing on things like performance optimization,concurrency and parallelism.  

A dev can iteratively work on this one project for a long time while improving on it and still manage to find it challenging on each iteration. You keep on improving.  

Moreover, the UART is needed in almost all embedded devices that require some form of I/O; making it a necessary topic for driver developers.  


The main aim here is to teach, not to create the supreme UART driver ever seen in the multiverse.    

### What this book is NOT
This book does not explain driver development for a particular Operating System or Kernel. Be it Tock, RTOS, Windows or linux. This book assumes that you are building a generic driver for a bare-metal execution environment.  


### Quick links

To access the tutorial book, visit : [this link][driver-development-book-website]  
To access the source-code, visit [this repo's sub-folder][driver-code]  


This is an open-source book, if you feel like you want to adjust it...feel free to fork it and create a pull-request.  


[driver-development-book-website]: https://rustaceanskenya.github.io/driver-development-book/
[driver-code]: https://github.com/RustaceansKenya/driver-development-book/tree/master/driver_code
[ESP32C3-datasheet-link]: https://www.espressif.com/sites/default/files/documentation/esp32-c3_datasheet_en.pdf  
[qemu-riscv-virt-board]: https://www.qemu.org/docs/master/system/riscv/virt.html
[the-embedded-rust-book]: https://docs.rust-embedded.org/book/