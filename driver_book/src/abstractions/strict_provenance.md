# Abstraction using strict provenance.  

Many Rust developers use Rust because of its inbuilt memory safety.  
Unsafe rust is not so safe. It tries hard, but it is named unsafe for a reason.  

Strict Provenance is a concept on how to make it harder to mess up pointer manipulation in Rust, even in unsafe code blocks.  
It is an unfinished project that you can check out in this [thread](https://github.com/rust-lang/rust/issues/95228).  


The rust compiler currently does not observe strict provenance but tools like [MIRI](https://github.com/rust-lang/miri) and [CHERI](https://www.cl.cam.ac.uk/research/security/ctsrd/cheri/) do.  
So running `cargo miri run` will catch more errors than the normal `cargo run`.  

(needed contribution: anyone with info on cheri can contribute to this page)

We really hope this project grows.  
This is a super-power loading.  


Being that this is a new topic, the author will just list the docs they read and provide tips along the way.  
Any developer with more experience is welcome to over-write all content concerning strict-provenance.   

### Direction
1. Start here : [initial draft by Aria Desires](https://faultlore.com/blah/fix-rust-pointers/#distinguish-pointers-and-addresses)  
2. If you need some practice with pointers, read ["Learn Rust by writing Entirely Too Many Linked Lists"](https://rust-unofficial.github.io/too-many-lists/).   
3. Reas up on [Stacked Borrows](https://plv.mpi-sws.org/rustbelt/stacked-borrows/)
4. Continuously read up on the concepts brought up in the [github thread](https://github.com/rust-lang/rust/issues/95228)


