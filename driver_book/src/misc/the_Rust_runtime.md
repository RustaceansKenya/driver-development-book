# The Rust Runtime

undone chapter.  

I do not undertand exactly what the Rust-runtime does.  
Here is the source code for the Runtime : [link to page](https://github.com/rust-lang/rust/blob/master/library/std/src/rt.rs)  
Here is a page that tries to explain what the runtime does : The Rust Reference book: [The Rust runtime](https://doc.rust-lang.org/reference/runtime.html)  

I am currently assuming that the functions of the Runtime are...
1. Sets up threading and creates a new special thread for 'main'
2. Inserts `clean_up` code. This is code that **MAY** get executed at the end of the `main` function. It clears up memory.  
3. Sets up backtracing functions
4. Sets up panicking  behavior by locating and doing some linking magic on the function tagged with the `#[panic_handler]` function
5.  