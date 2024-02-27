# The Bootloader

Since we are not booting the typical kernel, let us stop using the term bootloader. let's use the word `start-up code`.  

Calling it bootloader implies that it does esoteric functions such as performing a `power-on self test` and loading secondary loaders. So we will stick to the name `startup code`.  

The startup code does the following actions : 
1. It describes a function that helps us find the correct exception handlers. Something like an `switch` for exception-handling functions
2. Chooses a HART/CORE that will execute the rest of the activities below
3. Copies all initialized data from FLASH to RAM.  
4. Copies all un-initialized data from FLASH to RAM.
5. Zeroes-out all the un-initialized data
6. Sets up the stack pointer  
7. Calls the `main` function in our rust code


Our start-up code will be written in Riscv Assembly.  
We will embed those assembly files as part of our rust files.   


You can find books to help you learn riscv in the [reading_resources folder](https://github.com/RustaceansKenya/driver-development-book/tree/master/reading_resources)  


### Template  
You can view the template folder [here](https://github.com/RustaceansKenya/driver-development-book/tree/master/chapter_snapshots/_3_with_startup_code).



