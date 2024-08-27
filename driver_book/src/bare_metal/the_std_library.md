# The Standard Library

The standard library is a library like any other... it is just that it contains definitions of things that help in very essential tasks. Tasks that are expected to be found in almost every OS.  

For example, it may contain declarations & definitions of `file-handling functions`, `thread-handling functions`, `String struct definition`, ... etc  

You can find the documentation of the rust standard library [here](https://doc.rust-lang.org/std/index.html).  

Below is a story that explains more about standard libraries (disclaimer: the story does not even explain the actual modules of the standard library). 


#  Story time
You can skip this page if you already understand ...  
- What the standard library is
- Why it exists
- The different standards that may be followed


## System Interface Standards

Long ago ... once upon a time (in the 70s-80s), there were a lot of upcoming operating systems. Each operating system had its's own features. For example, some had graphical interfaces, some didn't. Some could run apps using multi-threading capabilities, others didn't. Some had file systems that contained like 100 functions... others had like 10 file-handling functions.

It was chaos everywhere. For example : the `open_file()` function might have had different names across many operting systems. So if you wrote an app for 3 OSes, you would have re-written your code 3 times just because the `open_file` function was not universal.  

It was a bad time to be an application developer. You either had to specialize in writing apps for one operating system OR sacrifice your sanity and learn the system functions of multiple Operating systems.  

To make matters worse... the individual operating systems were improving FAST, it was a period when there were operating system wars... each new weekend introduced breaking changes in the OS API...so function names were changing, file_handling routines were changing, graphical output commands were changing. CHAOS! EVERYWHERE.  


So developers decided that they needed some form of decorum for the sake of their sanity.  
They decided to create common rules and definitions on the three topics below : 
1. Basic definitions of words used in both kernel and application development
2. System interface definition
3. Shell and utilities.  

So what the hell are these three things?  

### 1. Basic definitions
Just as the title says, before the devs made rules, they had to first know that they were speaking the same language. I mean... how can you make rules about something that you don't even have a description for?  

They had to define the meaning of words. Eg "What is a `process`? What is an `integer`? What is a `file`? What is a `kernel` even?  
Defining things explicitly reduced confusion. 

They had to ...
1. Agree on the definition of things ie terminology. 
2. Agree on the exact representation of data-types and their behavior. This representation does not have to be the same as the ABI that you are using, you just have to make sure that your kernel interface *treats* data-types as defined here.  
3. Agree on the common constants : For example error_codes and port numbers of interest ...

### 2. System Interface  
As earlier mentioned, each kernel had different features and capabilities... some had dozens of advanced and uniquely named file-handling functions while others had like 2 poorly named and unique file-handling functions.  

This was a problem. It forced devs to have to re-write apps for each OS.  
So the devs sat down and created a list of well-named function signatures... and declared that kernel developers should implement kernels that us those exact signatures. They also explicitly defined the purpose of each of those functions. eg 
```bash
void _exit(int status); # A function that terminates a process

```
You can find the [full description of the `_exit` function](https://pubs.opengroup.org/onlinepubs/9699919799/functions/_exit.html) under POSIX.1-2017 and see how explicit the definitions were.  


This ensured that all kernels, no matter how different, had a similar interface. Now devs did not need to re-write apps for each OS. They no longer had to learn the interfaces of each OS. They just had to learn ONE interface only.  

These set of functions became known as the System interface.  
You can view the POSIX system interface [here](https://pubs.opengroup.org/onlinepubs/9699919799/functions/contents.html)


### 3. Shell and its utilities
The Operating system is more than just a kernel. You need the command line. You may even need a Graphic User Interface like a Desktop.  
In the 1980's, shells were the hit. So there were dozens of unique shells, each with their own commands and syntax.  

The devs sat down and declared the common commands that had to be implemented or availed by all shells eg `ls`, `mv`, `grep`, `cd`...  


As for the shell syntax... well... I don't know... the devs tried to write a formal syntax. It somehow worked, but people still introduced 
their own variations. Humanity does not really have a universal shell syntax. 

(which is good, bash syntax is horrifying... the author took years to get good at Rust/JS/C/C++, but they're sure they'll take their whole life to get comfortable with bash. Nushell to the rescue.)
<br><br><br><br>


There are a few standards that cover the above 3 specifications. Some of them are:  
1. [POSIX standard](https://en.wikipedia.org/wiki/POSIX) 
2. WASI (WebAssembly System Interface)  
3. Windows API (WinAPI)


## Entry of the standard library
Why is this 'System Interface Standards' story relevant?  

Well... because the functions found in the Rust standard library usually call Operating system functions in the background(i.e POSIX-like functions). In other words, the source-code for the standard library may call POSIX-system functions in the background.  
 
<!-- undone: draw image of levels of abstraction:  many oses // many Interfaces // POSIX interface // different_standard libraries -->



## POSIX compliance  
If you look at the [list of system functions specified by posix](https://pubs.opengroup.org/onlinepubs/9699919799/idx/functions.html), you might get a heart-attack. That list is so Long!!.  

What if I just wanted to create a small-specialized kernel that does not have a file-system or thread-management? Do I still have to define file-handling functions? Do I still have to define thread-management functions? - NO!, that would be a waste of everyone's time and RAM.  

So we end up having kernels that define only a subset of the posix system interfaces. So Kernel A may be more Posix-compliant than Kernel B just because kernel A implements more system interfaces than B... it is up to the developers to know which level of tolerance they are fine with.  

The level of tolerance is sometimes called **Posix Compliance level**. I find that name limiting, I prefer 'level of tolerance'.   

## C example
Read about The C standard library and its relation to System interfaces from [this wikipedia page](https://en.wikipedia.org/wiki/C_standard_library). 