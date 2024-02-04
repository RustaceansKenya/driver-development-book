# Intro

This book is about driver development using Rust. You get to procedurally write a UART driver for a RISCV chip called ESP32C3.  

You do not need a physical chip to get through... but having one is highly encouraged so as to garner a more hands-on experience. The chip costs less than 8 dollars. 


### Why the UART?

The UART driver was chosen because it is simple and hard at the same time. Both a beginner and an experienced folk can learn a lot while writing it.  
For example, the beginner can write a minimal UART and concentrate on understanding the basics of firmware development; linking, flashing, logging, abstracting things in a standard way...  
The pseudo_expert on the other hand can write a fully functional concurrent driver while focusing on things like concurrency and parallelism.  

A dev can iteratively work on this one project for a long time while improving on it and still manage to find it challenging on each iteration. You keep on improving.  

Moreover, the UART is needed in almost all embedded devices that require some form of I/O; making it a necessary topic for driver developers.  


The main aim here is to teach, not to create the supreme UART driver ever seen in the multiverse.    

### Quick links

To access the tutorial book, visit : [this link][driver-development-book-website] [undone: broken link]
To access the source-code, visit [this-repo/driver_code][driver-code] [undone: broken link]