# Types of Drivers

Classifications and fancy words do not matter, we go straight to the list :  

## Drivers classified by the level of 'how close to the metal?'
1. **Function drivers** : this drivers implement functions that directly manipulate registers. You could say that this are the OG drivers. 
 
2. **Filter drivers/ Processing drivers/ Wrapper drivers**: This drivers take input from the function drivers and process them into palatable input and functions for the kernel. They can be seen as 'adapters' between the function-driver and the kernel. They can even be used to implement additional security features. Point being, their main function is wrapping the function-driver.  

Oh look... this 👇🏻 is what we were talking about... thanks windows for your docs.  
![Alt text](img/types_of_drivers.png)  
This image was borrowed from the [Windows Driver development docs][Windows_driver_development_docs]  

**Note** : *A driver stack* is a collection of different drivers that work together to achieve a common goal. For example, you may use many function and filter drivers to control an integrated piece of hardware.  
Another example : You may use a couple of filter drivers when porting a function driver to a new kernel environment.   

## Drivers classified by function
- storage drivers : eg ssd drivers
- File System Drivers : Drivers above the file system.
- system drivers : used in motherboard components instead of peripherals
- Input Device Drivers
- Network Drivers
- Communication drivers
- Virtual drivers (Emulators)
- This list can be as long as one can imagine... but I hope you get the drift


[Windows_driver_development_docs]: https://learn.microsoft.com/en-us/windows-hardware/drivers/gettingstarted/what-is-a-driver-