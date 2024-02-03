# Why Embedded Rust

## Memory Safety:  
Rust's ownership system and borrow checker ensure memory safety without the need for a garbage collector. This helps prevent common issues like null pointer dereferences, buffer overflows, and data races.  

## Concurrency and Parallelism:
Advantage: Rust provides ownership-based concurrency control, allowing developers to write concurrent code without the risk of data races. The language's emphasis on zero-cost abstractions enables efficient parallelism.  

## Nice integration with C and C++... and their respective tools
- Rust has a robust FFI that allows seamless integration with C and C++ code.  
- Cargo integrates well with tools that are popular in the embedded world, so a C developer needs not learn ALL NEW things. For example the default toolchain components are extended LLVM or GNU components. You can integrate C library and build tools in a seamless manner in your project.   

## Ergonomics
- Tools are considerably documented.  
- Helpful community   
- many helpful tools & crates... especially the compiler itself. 


Naive but somehow true perspective : Rust enables you to write complex software (even as a junior), your implementation is not 100% dependent on your experience level.  




