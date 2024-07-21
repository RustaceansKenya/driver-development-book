# LLVM

read the docs at the [LLVM's main website.](https://llvm.org/)  

Modern modular Toolchain components: Compilers, optimizers, linkers, assemblers, code-generators.  


Originally began as a Compiler with the following novel qualities :  
1. Used **Single Static Assignment strategy (SSA strategy)**. Ie To make optimizations easier, any variable in the intermediate optimization could only be assigned once.  
2. The compiler could be tweaked to compile any language.  

But now it has transformed from being just a compiler, to being a toolchain comprising of many components with different functions.  
The unique features across the board are that :  
1. The components can be tweaked to suit different languages and execution environments.  s
2. The components are independent of each other.  

### LLVM components
1. **LLVM core libraries** : the core libraries include an **IR-optimizer** and a **couple of pre-made code generators** for popular CPUs. This module does not include an IR-to-machinecode code-generator.  

2. **Clang**: this is a compiler front-end for C, C++ and Objective C. It is not a full-fledged compiler. It converts source code into an AST, does semantic analysis and typechecking before converting it into LLVM-IR. Clang DOES NOT do optimization, code generation or linking.  

3. **LLDB**: this is the LLVM Debugger  

4. **LLD**: this is the LLVM linker

5. **libclc**: an implementation of the OpenCL standard library. (OpenCL == Open Computer Language)


### Learning.  

The best way to get a hang of LLVM is to go through [this tutorial](https://llvm.org/docs/tutorial/), trust me bro.  
- Rust tutorial that goes all the way but is not working : https://github.com/jauhien/iron-kaleidoscope
- Rust tutorial that cuts out the frontend part and assumes you know LLVM. (uses pest and inkwell libraries) : https://createlang.rs/intro.html  
- llvm IR explained by McYoung : https://mcyoung.xyz/2023/08/01/llvm-ir/