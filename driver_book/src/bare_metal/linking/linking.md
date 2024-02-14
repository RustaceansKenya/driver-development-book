# Linking


**SOLID CHOICES** :  

Linking is a VERY fundamental topic.  

It is best to learn it **slowly** and **in full** from the [docs](#note-worthy-docs). It will be worth it; You will save yourself hundreds of hours in the future if you make the first read intentional. Be patient with yourself, restrain from skimming through the docs if it's your first time.  

For this reason, this book will not spoil or water-down the purity of the [linking docs](#note-worthy-docs).  

This book will however :
  - assume that you have read [the docs](#note-worthy-docs).  
  - Briefly explain LLD linker usage in Rust targets 
  - Demonstrate how to fix the no-std linking error encountered in the previous chapter 
  - Demonstrate how to build a full linker script for the Esp32c3 board. (found in a much ater chapter)



### Note-worthy docs 
1. Start with this 3-minute video demonstrating the role of the linker from a high level.  
2. Then move to this [doc][gentle-first-docs]. It is gentle, covers the basics and its short.  
3. And finally finish it with this [more detailed docs][more-detailed-second-docs]. The two most important pages there are on [memory description][memory-description] and [memory abstraction][memory-abstraction].


[linker-video]: https://www.youtube.com/watch?v=cJDRShqtTbk
[gentle-first-docs]: https://users.informatik.haw-hamburg.de/~krabat/FH-Labor/gnupro/5_GNUPro_Utilities/c_Using_LD/ldLinker_scripts.html
[more-detailed-second-docs]: https://sourceware.org/binutils/docs/ld/Scripts.html
[memory-description]: https://sourceware.org/binutils/docs/ld/MEMORY.html
[memory-abstraction]: https://sourceware.org/binutils/docs/ld/REGION_005fALIAS.html