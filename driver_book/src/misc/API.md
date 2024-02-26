
# APIs

## What is an API?  

Application Programming Interface is a set of rules (diguised as functions), protocols, and tools that allows different software applications to communicate and interact with each other.  

It does so by defining the methods and data formats that developers can use to request and exchange information between software components.  

So you can say an API is the interface for an application. It contains objects and functions that can be called by other apps.  

Here are some examples of different APIs .... 

### Example 1 (Library API):  


The simplest and common type of API is a **Library API**. Library APIs define the methods, classes, and data structures that developers can use when programming with the library.  

For example, If you are trying to write an CLI-app that prints a random number on screen, chances are that you might import and use the `rand` library/crate. Here is the 'rand' crate's [library API documentation][rand-API-documentation]: 


The [`rand`] library exposes [these functions][rand-crate-functions], [these traits][rand-crate-traits] and [this struct][rand-crate-structs] as its API. If you use the rand crate in your app, you can interact with it by bringing these exposed items in the scope of your app.  


### Example 2 (Kernel API): 

**Operating System APIs** are provided by operating systems to allow applications to interact with system resources such as files, processes, and devices. They provide a way for applications to access low-level functionality without needing to understand the underlying hardware or system architecture.  

Here is the [Linux Kernel API][linux-kernel-api].  
From the page, you can see it exposing datatypes and functions. For example, It exposes the [`Doubly Linked List`][doubly-linked-list] and all the `methods` associated with that doubly-linked-list.   

#### Example 3 (Database APIs): 


**Database APIs** provide a way for applications to interact with databases, allowing them to perform operations such as querying data, inserting records, and updating information.   
For example a relational database may expose functions that assist the programmer in creating, modifying and deleting tables.  


APIs enable interoperability between different software systems, allowing them to work together seamlessly. They abstract away the complexities of underlying systems and provide a standardized interface that developers can use to build applications.  




[rand-API-documentation]: https://docs.rs/rand/0.8.5/rand/
[rand-crate-functions]: https://docs.rs/rand/0.8.5/rand/#functions  
[rand-crate-traits]: https://docs.rs/rand/0.8.5/rand/#traits
[rand-crate-structs]: https://docs.rs/rand/0.8.5/rand/#structs  


[linux-kernel-api]: https://archive.kernel.org/oldlinux/htmldocs/kernel-api/
[doubly-linked-list]: https://archive.kernel.org/oldlinux/htmldocs/kernel-api/adt.html#id-1.3.2
