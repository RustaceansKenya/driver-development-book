# Dependencies  

A library refers to a collection of precompiled routines, functions, classes, or modules that can be reused by various programs or applications to perform specific tasks or operations. Libraries are essentially packages of code that provide commonly used functionality, such as mathematical operations, file handling, networking, user interface components, and more.  

A dependency, on the other hand, refers to a specific software component or library that a project relies on to function properly.  


For example, the hello-world program below uses the `time` library as a dependency:  
```rust
use time; 

fn main(){
    println!("Hello world!!!");
}
```


## Default dependencies
By default, all rust programs use the [`std` library](https://doc.rust-lang.org/std/index.html) as a dependency. Even if you write a simple hello-world or an add-library, the contents of the [`std::prelude`](https://doc.rust-lang.org/std/prelude/index.html) library get included as part of your code as if you had written it as follows...  

```rust
use std::prelude::*; // this line is usually not there... but theoretically, 
                    // your compiler treats your code as if this line was explicitly declared here

fn main(){
    println!("Hello world!!!");
}
```


So ... what is the standard library? What is a prelude?  