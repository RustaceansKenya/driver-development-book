Vcell is one of the crates that PAC depends on. In order to understand the PAC, we need to understand : 
- What vcell is and how works
- Understand how vcell fits into the PAC crate


### What is a cell? Why are we using them?

To wholly understand what a cell is, read up on [core::cell][core_cell_documentation].  
I'll try my best to explain a watered-down version of it.  

In Rust, core::cell is a module that provides a set of types for interior mutability. **Interior mutability** allows you to mutate data even when you only have an immutable reference to it. This is useful in situations where you need mutability but also need to adhere to Rust's ownership and borrowing rules.  

So temporary unrustmanhip.  

The types provided are : 
- `Cell<T>`
- `RefCell<T>`
- `OneCell<T>`
- Other helper structs/types

These celll enclose types and allow that type to ...
- have multiple `&mut T` and `&T` at the same time
- be mutated through both `&mut T` and `&T` references  

But it is not all chaos... there are rules... some rules on how all these mutiple references work together.  
1. These types are not multi-thread-safe. They are guaranteed to work ONLY in a single-threaded program. (If you need to do aliasing and mutation among multiple threads, `Mutex<T>`, `RwLock<T>`, `OnceLock<T>` or atomic types are the correct data structures to do so).
2. Under Cell<T>
   - a `&mut T` reference to the inner value can NEVER be obtained.
   - The inner value `T` cannot just be obtained like that. You have to replace it with another value.  
   - 
3. Under RefCell<T>


(this topic is undone)


[core_cell_documentation]: undone_link