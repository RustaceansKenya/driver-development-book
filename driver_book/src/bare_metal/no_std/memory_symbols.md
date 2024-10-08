# Memory Symbols


The memory symbols required by the core library are six: 
- `memcopy`, 
- `memmove`, 
- `memset`, 
- `memcmp`, 
- `bcmp`, 
- `strlen`. 

Rust codegen backends generate code that usually reference the above six functions.  

It is up to the programmer to provide definitions of the above six functions. You need to provide them in a file containing the assembly code or machine code for your specific processor.  

The definitions of memcpy, memmove, memset, memcmp, bcmp, and strlen are listed as requirements because they represent commonly used memory manipulation and string operation functions that are expected to be available for use by generated code in certain contexts. While they are not strictly required for every Rust program, they are often relied upon by code generated by Rust's compiler, especially in situations where low-level memory manipulation or string operations are necessary.

Here are some reasons why these definitions are listed as requirements:

- Interoperability with C Code: Rust often needs to interoperate with existing C libraries or codebases. These C libraries commonly use functions like memcpy, memset, and strlen. Therefore, ensuring that Rust code can call these functions or be called from C code requires that their definitions are available.

- Compiler Optimizations: Even if a Rust program doesn't explicitly call these functions, the Rust compiler may internally use them as part of optimization passes. For example, when optimizing memory accesses or string manipulations in Rust code, the compiler may choose to use these functions or their equivalents to generate more efficient machine code.




> Disclaimer: The author is not completely versed with the internals of panic. Improvement contributions are highly welcome, you can edit this page however you wish. (undone)

here is a snippet from [`core` documentation](https://doc.rust-lang.org/core/#how-to-use-the-core-library) in concern of memory symbols:  


>*`memcpy`, `memmove`, `memset`, `memcmp`, `bcmp`, `strlen` - These are core memory routines
which are generated by Rust codegen backends. Additionally, this library(core lib) can make explicit
calls to `strlen`. Their signatures are the same as found in C, but there are extra
assumptions about their semantics: For `memcpy`, `memmove`, `memset`, `memcmp`, and `bcmp`, if
the `n` parameter is 0, the function is assumed to not be UB, even if the pointers are NULL or
dangling. (Note that making extra assumptions about these functions is common among compilers:
[clang](https://reviews.llvm.org/D86993) and [GCC](https://gcc.gnu.org/onlinedocs/gcc/Standards.html#C-Language) do the same.)
These functions are often provided by the system libc, but can also be provided by the
[compiler-builtins crate](https://crates.io/crates/compiler_builtins).
Note that the library does not guarantee that it will always make these assumptions, so Rust
user code directly calling the C functions should follow the C specification! The advice for
Rust user code is to call the functions provided by this library instead (such as
`ptr::copy`).*