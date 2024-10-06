# Critical Section

**What is a critical section?**     
I'd rather give you two definitions or the sake of clarity ...

1. A critical section refers to code that accesses shared resources (e.g., global variables, hardware registers) that must be protected from concurrent access by multiple execution contexts (such as threads or cores) to avoid race conditions. 

2. A critical section is a section of code that contains statements that require atomicity and isolation to prevent undefine behaviour caused by data-races. Most of the time, these activities involve modifying shared resources. eg a global mutable variable or a peripheral-registers.   
  


**The `critical-section` crate**   
The critical-section crate provides a critical-section management API above arbitrary environments.  
It is up to the environment creators to provide implementations of the functions listed in the API.  
For example

The `critical-section` crate in Rust is designed for managing critical sections. It provides a universal API for creating, acquiring and releasing critical sections across different environments. It ensures that only one execution-context(not necesarily a thread) can access the critical section at a time, usually by disabling interrupts in embedded systems or acquiring a lock in multi-threaded applications.  

You can read the [crates docs](https://crates.io/crates/critical-section) for better understanding.  


**Types of Critical Section implementations**  

1. If you are in a single-core bare metal environment, you can execute the critical section in isolation by disabling all interrupts coming from peripherals.  

2. If you are in a multi-core bare metal environment, you can execute a critical section in isolation by...
   1.  disabling all interrupts coming to the subject core AND...
   2.  Putting all the other cores on sleep (or a spinlock) whenever they want to execute a critical section that may affect the critical section that is currently getting exeuted. In short, no two cores should execute conflicting critical sections in parallel. Cores can however execute non-conflicting critical sections.  
  
3. For a bare metal environment where you have limited memory control, eg (if you are in priviledged mode instead of machine mode in riscv) - you can make syscalls that invoke machine mode functions mentioned above.(ie the above 2 paragraphs you just read)  

4. For hosted environments, you can either invoke library functions provided OR use the synchronization primitives provided(eg Mutex, SpinLock, Semaphores). Hosted environments are setups where a kernel or management runtime is avalilable.   


#### An internal look at the critical-section crate
Get the source code of critical-setion crate. (this book used version 1.1.3)  
I hold the bias that : "it is not enough to read the docs of a vital crate, you need to understand how it internally works by parsing through the code yourself" - this bias is very important especially for a firmware/driver dev who needs to control low-level aspects of their code. You need total/major control over your code.  

And it is our luck that `critical-section` has no dependencies and it has less than 700 lines of code. Parsing it would be somehow easy.  

### Cargo.toml
When you look at the cargo file,  you note that the crate has a couple of features named using the form `restore-state-[type_name]` eg `restore-state-bool`, `restore-state-u32`... as seen below : 
```toml

[features]
restore-state-bool = []
restore-state- none = []
restore-state-u16 = []
restore-state-u32 = []
restore-state-u64 = []
restore-state-u8 = []
restore-state-usize = []
std = ["restore-state-bool"]
```  

So, what is this `restore-state-*` thing?  
To find out the answer, we move on to the `lib.rs`, maybe we'll find answers there....  

Aaah... there's so much happening in `lib.rs`, I don't get it;  lets just visit `mutex.rs` first since it is one of `lib.rs` building mods. Bottom up understanding.  

`mutex.rs` depends on the `critical_section` struct. 
```rust
#[derive(Clone, Copy, Debug)]
pub struct CriticalSection<'cs> {
    _private: PhantomData<&'cs ()>,
}
```  

The designers of the crate hoped that : "if a thread wants to execute a critical section, it would first instantiate a `critical_section` struct that will be dropped by the thread once it has finished executing the critical section"  
This means that we have to find how they ensured that...
1. Each independent critical section got a unique `critical_section` struct instance. eg CriticalSection A is tied to `critical_section_struct_A` while  CriticalSection B is tied to `critical_section_struct_B` 
2. The number of instances of a `critical_section` struct that exist must not exceed one.   


A question for you... "Why use phantom data in the struct? As far as we can tell, the phantom data was unnecessary"

(undone chapter)

