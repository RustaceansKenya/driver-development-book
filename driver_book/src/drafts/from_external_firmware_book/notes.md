# What is this  
these are notes from the book : "Firmware Development, a guide to specialized Systemic Knowledge by Vincent Zimmer and Subrata Banik"  

I liked the book because 
- it was detailed, it was written & reviewed by some pretty content-rich people. Check out the pages before the foreword of the book. 
- it is open-source focused. This is a big thing in the firmware world.  
- 'safety-focused' - the constant vibe that you get. I liked this.   
- It had simple explanations which often left me asking, 'so that's it? only that? I thought is was magic?'  
- Future focused - the book did not just explain 'how people usually write firmware'. It explained 'How we should write firmware' in a way that fits future expectations of security, portability, performance and maintenance.  


Firmware development and driver development are tightly intertwined, so it makes sense to have a chapter on Firmware development.  

## interesting software/crates/tools/Interfaces
1. Coreboot
2. OreBoot - Rust-based coreboot
3. Extensible Firmware Interface
4. UEFI
5. u-boot
6. LinuxBoot
7. EDKII

## Communities / Projects
1. Open Source Firmware Foundation
2. 9elements
3. Open Source Firmware Conference
4. Institute for Security and Technology  

## People
1. Vincent Zimmer : Author, lead at UEFI security team
2. Subrata Banik
3. Ron Minnich
4. Christian Walter
5. Kumar N. Dwarakanath : ch5 on firmware security


## Random buzz-words and questions I asked myself
1. `...Firmware Architectures like coreboot, UEFI and SlimBootloader...` - I didnt understand this sentence.  
2. `Subrata has extensive experience on platform enablement, which led to working for all the leading PC makers’ products` - what the hell is `platform enablement`?    
3. `OEMs, and ODMs` the hell are these initials?


## Preface
Firmware is the first piece of code that runs on any target hardware. Its functions can be summarized into 2 steps: 
1. Hardware unitialization
2. Handing off control to higher level software eg Kernels and UserInterfaced Applications