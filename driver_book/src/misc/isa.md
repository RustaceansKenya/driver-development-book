# Instruction Set Architectures (ISAs)


An ISA specification is a piece of document that elaborates on how a certain processor functions. It does so by explaining the things listed below :  

- Supported Instructions:
    This refers to the set of operations or commands that a processor can understand and execute. Instructions could include arithmetic operations (addition, subtraction, multiplication, division), logical operations (AND, OR, NOT), control flow operations (branches, jumps, calls), and others specific to the architecture.

- Data Types:
    ISA specifies the types of data that can be manipulated by the processor. This might include integer types (such as 8-bit, 16-bit, 32-bit, or 64-bit integers), floating-point types (single precision, double precision), and sometimes vector or SIMD (Single Instruction, Multiple Data) types for parallel processing.

- Registers:
    Registers are small, fast storage locations within the processor that hold data temporarily during processing. ISA defines the number of registers, their sizes, and their intended purposes (e.g., general-purpose registers, special-purpose registers for specific tasks like storing the program counter or stack pointer).

- Hardware Support for Managing Main Memory:
    ISA specifies how the processor interacts with main memory (RAM). This includes mechanisms for loading and storing data from/to memory, handling memory access permissions, cache management, and mechanisms for memory protection to prevent unauthorized access.


Think of the ISA as a manual for your processor.  
There are no rules as to what an ISA should entail, the creator chooses how to write the manual.  
If you build a nano-processor that uses DNA-magic, it will be up to you define your own unique manual.  

For reference, here are the [Riscv ISA specifications](https://riscv.org/technical/specifications/)
