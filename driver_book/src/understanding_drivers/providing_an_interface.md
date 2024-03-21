# Role 2: Providing an Interface

The driver acts as an interface between the physical device and the kernel.  
An interface consists of :  
- callable functions exported by the driver
- exported structs
- exported traits
- exported macros
- ... some glue code or whatever

It is up to the programmer to make the Interface to be : 
1. Clear and Intuitive
2. Complete and correct 
4. Simple
5. Modular (well structured)
6. Well documented


We will cover these concepts in a practical manner while writing our driver. So don't stress about finding the right guide or definition for writing 'the perfect API'. There is no perfect API. You just try your best to fulfill the above 6 vague goals and with time your APIs will get better and better.  


Oh... here are the definitions of the above 6 goals.  
1. Clear and Intuitive : the names of the Interface elements should be meaningful. They should be easy to undersand. For example, a function that switches off the device should have a name like `switch_off_device`. 

2. Complete and correct : 
   - A complete API is an interface that abstracts ALL the possible functions of the underlying device. Can you imagine an SSD driver that does not provide the `read` function?  
   - A correct API does not expose functions that are wrong and sometimes produce undefined behavior. 

3. Simple : Be simple... Dumb things down.

4. Modular : A modular API is structured in such a way that a user can intuitively guess where things have been bundled up. For example, put error-codes in one module, put read functions in onother module... 

5. Well documented.




