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

- Fundamental Features:
    These are core aspects of the ISA that define its behavior and capabilities.
        Memory Consistency: Specifies how memory accesses by different parts of a program are ordered and synchronized.
        Addressing Modes: Defines the various ways in which memory addresses can be specified in instructions (e.g., direct addressing, indirect addressing, indexed addressing).
        Virtual Memory: Defines how the processor interacts with virtual memory systems, including mechanisms for address translation between virtual and physical memory, page tables, and memory protection.

- Input/Output Model:
    This refers to how the processor communicates with peripheral devices such as keyboards, displays, storage devices, and network interfaces. ISA specifies the instructions and mechanisms for transferring data between the processor and these devices, often through dedicated I/O instructions or memory-mapped I/O.