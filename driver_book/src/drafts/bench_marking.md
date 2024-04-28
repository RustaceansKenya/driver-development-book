# Bench_marking


- benchmarking profile
- benchmarking commands
- file layout of bences 


Closely related to benchmarking in Rust is the rust-testing harness :

A testing harness is a framework or set of tools used to automate the process of running tests on software code.  
In the context of Rust, the testing harness refers to the infrastructure provided by the libtest library, which is included automatically when compiling tests using cargo test or rustc --test.  

The rust testing harness does the following :  
- Collects and compiles all the objects/functions marked with the `#[test]` attribute.  
- Executes the chosen test functions
- It provides utilities/libraries/frameworks for writing tests. For example, when you are in a no-std environment, you may need a special testing harness to suit that environment
- At the moment, the default rust testing harness can also handle bench-mark functions. ie : (provide bench-framework*, compile `#[bench]` functions, selectively execute `#[bench]` functions)


#### The harness field in cargo.toml manifest file
The harness field gets defined under both `[[test]]` and `[[bench]]` tables.  
If the harness field is set to true, then the compiler will automatically include the test-harness as part of the compilation process and it will also include a `main()` function whose body executes the chosen `benches` and `tests`.  

f the harness field is set to false, then Cargo will not include the default Rust test harness automatically. However, you are not required to provide your own harness and main file. Instead, you can write your own main() function to handle the execution of tests or benchmarks as you see fit. If you choose to provide your own harness, you can do so, but it's not mandatory.


#### Unanswered questions
- what is the default rust test harness? What can it do? What can it not do that is found in other established harnesses?  
- Are there other established 3rd party std and no-std test and bench-mark harnesses.  
- Is there a standrd test/bench-mark harness out there? possibly from the C/C++ world?


