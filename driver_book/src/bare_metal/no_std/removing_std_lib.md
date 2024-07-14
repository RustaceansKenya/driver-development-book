### Disabling the Standard Library


A earlier mentioned, when you write drivers, you cannot use the standard library. But you can use the [core-library][lib-core-documentation].  

So what is this core library?

Even before we discuss what the core library entails, lets answer this first:  
How is it possible that the core library can get used as a dependency by bare-metal apps while the `std` library cannot get used as a dependency? How are we able to use the core library on bare metal?  

well...Lib-core functions can be **directly** compiled to pure assembly and machine code without having to depend on pre-compiled OS-system binary files. Lib-core is dependency-free.  

Lib-core is lean. It is a subset of the std library. This means that you lose a lot of functionalities if you decide to use lib-core ONLY.  

Losing the std library's support means you forget about thread managemnt, handling the file system, heap memory allocation, the network, random numbers, standard output, or any other features requiring OS abstractions or specific hardware. If you need them, you have to implement them yourself. The table below summarizes what you lose...  


| feature                                                   | no\_std | std |
|-----------------------------------------------------------|--------|-----|
| heap (dynamic memory)                                     |   *    |  ✓  |
| collections (Vec, BTreeMap, etc)                          |  **    |  ✓  |
| stack overflow protection                                 |   ✘    |  ✓  |
| runs init code before main                                |   ✘    |  ✓  |
| libstd available                                          |   ✘    |  ✓  |
| libcore available                                         |   ✓    |  ✓  |

\* Only if you use the `alloc` crate and use a suitable allocator like [alloc-cortex-m].

\** Only if you use the `collections` crate and configure a global default allocator.

\** HashMap and HashSet are not available due to a lack of a secure random number generator.  


You can find lib-core's documentation [here][lib-core-documentation]  
You can find the standard library's documentation [here][std-lib-docs]  




[alloc-cortex-m]: https://github.com/rust-embedded/alloc-cortex-m  
[lib-core-documentation]: https://doc.rust-lang.org/core/  
[std-lib-docs]: https://doc.rust-lang.org/std/index.html










