# Setting up the Build automation tool

Our build tool will be cargo.  
We will not use third party build tools like Makefiles.  
It is better to not use 3rd parties.  


So create a .cargo folder withing the repo.  
Create a config.toml inside the folder

So you have : project/.cargo/config.toml.
Inside this file, paste the following configurations : 

```toml
[build]
target = "riscv64gc-unknown-none-elf"
rustflags = ['-Clink-arg=-Tsrc/lds/virt.lds']

[target.riscv64gc-unknown-none-elf]
runner = "qemu-system-riscv64 -machine virt -cpu rv64 -smp 4 -m 128M -serial mon:stdio -nographic -bios none -kernel "	
```

The [build] section has configs that affect the compilation process. We tell the compiler our target platform. And tell the linker the path to the linker script.

The [target.riscv64gc-unknown-none-elf] section has the configs that will be considered only if we are compiling for the riscv64gc-unknown-none-elf target.  
The "runner" specifies the cmd command that will be executed when we call "Cargo run". There is a space after -kernel. This is because cargo will automatically specify the executable, whose name is configured through Cargo.toml. 


### Template
You can view the template folder [here](https://github.com/RustaceansKenya/driver-development-book/tree/master/chapter_snapshots/_2_bare_with_runner).


