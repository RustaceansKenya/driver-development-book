### The Standard Library  

The standard library is a group of common function declarations that get called by applications that run on top of an OS.  
So each OS needs to provide implementations for all those common functions.   

For example, the standard library declares the thread_spawn function. Linux OS provides an implementation of that function that is different from the Windows implementation... provided they all do the same thing.  

So when you write drivers, you cannot use the standard library. But you can use the [core-library][lib-core-documentation].  

How's that possible? How are we able to use the core library on bare metal?  
well...Lib-core functions can be **directly** compiled to assembly and machine code without having to depend on OS-system files. Lib-core is dependency-free.  


Losing the std library means you forget about threads, files, heap memory, the network, random numbers, standard output, or any other features requiring OS abstractions or specific hardware. If you need them, you have to implement them yourself. The table below summarizes what you lose...  


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
[lib-core-documentation]: (https://doc.rust-lang.org/core/)  
[std-lib-docs]: https://doc.rust-lang.org/std/index.html










