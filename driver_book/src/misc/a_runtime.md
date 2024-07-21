# Runtime

What is the C runtime? What is a Rust runtime? What is a runtime?  


## 100 Meanings
Well, the word runtime is a dense word, with multiple meanings that vary with context. Here are some of the definitions :  

### Meaning 1 :  
Runtime refers to the duration of time consumed when a program was executing.  
For example, if you played a videogame for 12 hours, you could say that that videogame had a 12-hour runtime.   

### Meaning 2 :  
Runtime refers to a piece of software that continuously tends to the needs of another running program.  

For example :  

1. In the case of programs written with languages with garbage-collection, you could say that those programs depend on a runtime-software to continuously collect the dead variables as a background service. In this case, the garbage-collector is part of the runtime.  

2. In the case of Intepreted languages, you could say that the intepreter itself is the runtime service. This is because the intepreter needs to continuously compile and execute the program as it runs.  


### Meaning 3 :  
Programs usually don't start getting executed just like that. There has to be code that makes the CPU to point to and fetch the right lines of code, make sure that there is enough stack space, make sure that some CPU registers have the expected default values...  

Point is, there is some code that gets executed before the actual program ie. Init code or control code.  

In this context, Runtime means init-code. Runtime means control code.  
Some parts of the runtime code get executed once while other parts get executed continuously & dynamically.  

For example, control functions like *overlay control*,*stack overflow protection* and *Exception Handling* get executed continuously & dynamically. Functions like program-stack initilization gt executed only once at the start of the program.  

Some compilers can be set to statically insert such control code in every place they are needed at compile time. Other compilers allow you to reference them dynamically.  

If you are short on space, you can go the dynamic path.  
If you are not short on space and you require performance, it's better to go the static path.  
