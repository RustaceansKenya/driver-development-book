A token tree is different from an AST. We can have leaf-token-trees and non-leaf-token-trees.  
Token tree generation happens before the AST generation happens. This is because the AST is constructed from the token tree.  

Macro processing happens after the AST tree has already been generated. So macro invocation must follow a certain syntax  

There are 4 syntax ways of invoking a macro: 
  1. `# [$arg]`  # where `$arg` is a token tree.  eg `#[test]`
  2. `#! [$arg]` # where `$arg` is a token tree.  eg `#![no_std]`
  3. `$name ! $arg`  # where `$arg` is a **non-leaf** token tree.  eg `println!("sdkfjskdf")`
  4. `$name ! $arg0 $arg1`  # where `$arg` is a token tree. eg `macro_rules! custom_name { // magic_lines_go_here }`


Macros can be of differennt types. Here they are :  
1. Attribute macros
2. Function-like macros

### Attribute Macros
Attribute macros are used to either tag things OR to derive things.  

The `#[test]` attribute macro is an example of an attribute used in tagging. It is used to tag/annotate that a certain function is a test. These tags are relevant to the compiler.  

The `#[derive(Degug)]` attribute macro is an example of a macro used to derive things. It is used to automate the implementation of the `Debug` trait for any tagged object.  


So if we decide to classify attribute macros based on fuctionality, we would have two classes :  
1. Attributes used for tagging purposes.
2. Attributes used for automatic derivations.  

Attribute macros can be declared and defined through a couple of ways. 
1. They can be hardcoded as part of the compiler code. 
2. They can be defined using [proc_macro crate](https://doc.rust-lang.org/proc_macro/index.html). This crate provides for you functions that can manipulate the AST of rust code. With this crate, you can create macros that manipulate the AST ie `procedural macros`.
3. They can be defined using normal rust code that uses the `macro_rules!` declaration syntax. This method allows you to define how your `custom syntax` gets expanded. ie How a string of syntax gets translated into a token tree. This macro-declaration method is called `Macro-by-example` or `Macro-by-declaration`. Weird names. I guess its because you explicitly declare how your syntax gets expanded.  

From the above info, you are right to think that you can write your own `derive` attributes and `tagging` attributes.  

So if we decide to classify attributes based on how they were created instead of their functionality, we would have 3 classes : 
1. Built-in attributes
2. Proc-macro attributes
3. Macro-rules attributes

To read more about attributes read the [Attributes chapter](https://doc.rust-lang.org/reference/attributes.html) from the [Rust-reference book](https://doc.rust-lang.org/reference/).  

### Function like macros
These are macros that have functions underneath them. For example `println!`, `format!`, `assert!`.  
These macros can be defined just like the Attribute macros ie. You can either use the `proc_macro crate` or the `macro_rules!` declaration syntax.  


### What can be represented by a syntax extension (ie a macro)?
Macros are allowed to expand into the following :  
- An item or items
- A statement or statements
- A type
- A pattern
- An expression

The macro expansion is not a textual expansion, It is an AST expansion. Expansions are treated as AST nodes. 

Expansion happens in "passes"; as many as is needed to completely expand all invocations.  
The compiler imposes an upper limit on the number of such recursive passes it is willing to run before giving up. This is known as the syntax extension recursion limit and defaults to 128.  
This limit can be raised using the `#![recursion_limit="…"]` attribute.  


### Macro Hygiene.  
We have seen that a macro gets expanded as an AST node.  


A macro is said to be unhygienic if : 
1. It interacts with identifiers and paths that were not defined within the macro
2. It exposes its local paths and identifiers to be accessible by code outside the macro

A macro is said to be hygienic if : 
1. It does not interact with Identifiers and Paths that were not defined within it
2. It does not expose its identifiers and paths to be accessible by code outside the macro definition.


### Debugging Macros
You can preview how your macros got expanded using `rustc`'s  `-Zunpretty=expanded` flag. This flag is curently available in rust-nightly only.  
like so :  
```bash
rustc +nigthly -Z unpretty=expanded hello.rs
```


### Macro Rules

the rules of the macro follow this syntax : 
```rust
(bunch of token-trees) => {  expanded token-tree structure }
```

You can use metavariables that have types attached. Read the "little book of macros" to understand this.  
You can also supply tokens to the matcher like a regex expression or something.  
Repetitions are shown using the syntax : `$ ( ...token-trees that need to be repeated... ) sep rep` where

