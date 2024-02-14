ld vs lld

- Internally, LLD consists of several different linkers called flavours/ ports : ld.lld (Unix), ld64.lld (macOS), lld-link (Windows), wasm-ld (WebAssembly)
- Each flavour is tweaked to be optimized for producing certain object file formats. For example, the wasm-ld can produce an elf file, but you would have a smoother experience using the ld.lld linker port.  


- LLD accepts the same commands as the old gnu-ld. It also accepts linker scripts if need be.  
- unlike ld, lld does not always require a linker script. In many cases, LD.LLD can automatically generate linker scripts internally based on the target architecture, format, and other parameters specified during the linking process. This means that LD.LLD can handle the linking process without requiring an explicit linker script provided by the user.  

- ld is not always a cross-linker. LLD is always a cross-linker, meaning that it always supports all the above targets however it was built.  

Rust uses lld by default. You can tweak it to another... but why do that?   

