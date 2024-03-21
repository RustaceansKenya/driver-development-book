# Intro

This book is on driver development using Rust. You get to procedurally write a UART driver for a RISCV chip called ESP32C3 and a Qemu-riscv-virt board.  

It is not a book on embedded-development but it will touch up on some embedded-concepts here and there.  


## Book phases, topics, flow.  

The book will take you through 5 phases :  
**Phase 1**:  
You get to build a UART driver for a qemu-riscv virtual board. This will be our first phase because it will take you through the fundamentals without having to deal with the intricacies of handling a physical board or writing flashing algorithms.  
This UART driver will NOT be multi_thread-safe.


**Phase 2**:  
We improve the UART driver using standard abstractions: PAC, HAL and other crates. This phase will try to show devs how to make standard drivers that are more portable.   


**Phase 3**:  
You get to modify the previously built UART driver so that it could run on the esp32 physical board. We set up flashing, debugging, logging and testing on the physical board.  

We will not use the standard esp-tools but we will try to modify probe-rs, this is because esp-tools abstract away a lot of details that are important for driver-devs to master. Esp-rs tools are just too good to use... it would be awesome if we tried to write our own flashing algorithms and build or own logging module.  

We will however imitate the esp-tools.  

The driver produced in this phase will still NOT be multi_thread-safe.  


**Phase 4**:  
We do some brush-up on driver-security and performance testing.   


**Phase 5**:  
We start making our driver to be multi-thread safe. This will be first done in a qemu virtual environment to reduce the complexity. After we have figured our way out of the virtual threads, we will move on to implementing things on the physical board

### Why the UART?

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