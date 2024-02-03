# driver-development-book


This book explains driver development using Rust. You get to procedurally write a UART driver for a RISCV chip called ESP32C3.  

You do not need a physical chip to get through... but having one is highly encouraged so as to garner a more hands-on experience. The chip costs less than 8 dollars. 


### Why the UART?

The UART driver was chosen because it is simple and hard at the same time. Both a beginner and an experienced folk can learn a lot while writing it.  
For example, the beginner can write a minimal UART and concentrate on understanding the basics of firmware development; linking, flashing, logging, abstracting things in a standard way...  
The pseudo_expert on the other hand can write a fully functional concurrent driver while focusing on things like concurrency and parallelism.  

A dev can iteratively work on this one project for a long time while improving on it and still manage to find it challenging on each iteration. You keep on improving.  

Moreover, the UART is needed in almost all embedded devices that require some form of I/O; making it a necessary topic for firmware writers.  


The main aim here is to teach, not to create the supreme UART driver ever seen in the multiverse.    

### Quick links

To access the tutorial book, visit : [this link][driver-development-book-website]  
To access the source-code, visit [this-repo/driver_code][driver-code]    
To access the mdbook source files, visit [this-repo/driver_book/src][driver-book-src] 
If you wish to contribute, skim through this : [book][contribution-book]


## Communication

You can air your thoughts or ask questions at the [dicussion section][discussion-section].  
If you find an issue in the DOCs or CODE, you can raise it [here][issues-section]  



## Notes to the contributors

To intrested contributors, you can go through this very short tiny microscopic brief informative [book][contribution-book].  
The book takes you through the project structure and explains how tasks get suggested & assigned.    
It also helps you troubleshoot any stalled pull-requests by explaining the pull-request requirement-checks.

It is the ["contribution book"][contribution-book]


<!-- hard-link -->
[driver-code]: https://github.com/RustaceansKenya/driver-development-book/tree/master/driver_code  

<!-- hard-link -->
[driver-book-src]: https://github.com/RustaceansKenya/driver-development-book/tree/master/driver_book/src  

<!-- hard-link -->
[discussion-section]: https://github.com/RustaceansKenya/driver-development-book/discussions

<!-- hard-link -->
[issues-section]: https://github.com/RustaceansKenya/driver-development-book/issues

<!-- hard link -->
[contribution-book]: https://kiarie404.github.io/tunnel/onboarding_book/book/index.html  

<!-- Hard link -->
[driver-development-book-website]: https://kiarie404.github.io/tunnel/driver_book/book/index.html

