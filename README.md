# driver-development-book
This book explains driver development using Rust. You get to procedurally write a UART driver for a RISCV chip.  
You do not need a physical chip to get through... but having one in the later chapters is highly encouraged so as to garner a more hands-on experience.  


### Why the UART?

The UART driver was chosen because it is simple and hard at the same time. Both a beginner and an expert can learn a lot while writing it. For example, the beginner can write a minimal UART while the expert can write a fully functional concurrent driver. Because of this reason, a dev can iteratively work on this one project for a long time while improving on it and still manage to find it challenging on each iteration.  

Moreover, the UART is needed in almost all embedded devices that require some form of I/O; making it a necessary topic for firmware writers.  


The main aim here is to teach, not to create the supreme UART driver ever seen in the multiverse.    

### Quick links

To access the tutorial website, visit : [this link][driver-development-book-website]  
To access the code, visit [this-repo/driver_code][driver-code]    
To access the mdbook source files, visit [this-repo/driver_book/src][driver-book-src] 

You do not need a physical board to go through this tutorial.  

## Communication

You can air your thoughts or ask questions at the [dicussion section][discussion-section].  
If you find an issue in the DOCs or CODE, you can raise it [here][issues-section]  



## Notes to the contributors

To intrested contributors, you can go through this very short tiny microscopic brief informative [book][onboarding-book].  
The book takes you through the project structure and explains how tasks get suggested & assigned.    
It also helps you troubleshoot any stalled pull-requests by explaining the pull-request checks.

It is the ["onboarding book"][onboarding-book]


<!-- hard-link -->
[driver-code]: https://github.com/RustaceansKenya/driver-development-book/tree/master/driver_code  

<!-- hard-link -->
[driver-book-src]: https://github.com/RustaceansKenya/driver-development-book/tree/master/driver_book/src  

<!-- hard-link -->
[discussion-section]: https://github.com/RustaceansKenya/driver-development-book/discussions

<!-- hard-link -->
[issues-section]: https://github.com/RustaceansKenya/driver-development-book/issues

<!-- hard link -->
[onboarding-book]: https://kiarie404.github.io/tunnel/onboarding_book/book/index.html  

<!-- Hard link -->
[driver-development-book-website]: https://kiarie404.github.io/tunnel/driver_book/book/index.html

