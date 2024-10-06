# System Timer - Part 1

The system timer described in the [espc3 reference datasheet](https://www.espressif.com/sites/default/files/documentation/esp32-c3_technical_reference_manual_en.pdf) is more complex than the system timer in the [ARM reference manual](http://infocenter.arm.com/help/topic/com.arm.doc.dui0553a/Babieigh.html). For this reason, we will experiment with the ARM system timer before moving on to the espc3 system timer.  

So what's the PAC-creation process like?  
1. Read the concerned datasheet segment of the [ARM reference manual](http://infocenter.arm.com/help/topic/com.arm.doc.dui0553a/Babieigh.html) (6 pages inclusive of diagrams). You need to understand the little details, you are a low-level programmer. Read the full specs or you'll fuck up a user somewhere in the world.  

2. Outline all the registers concerned. Outline the fields within each register. List all the allowed values of each register field. This info will be useful to you when you'll be creating a state diagram of the state changes in the register values.   
   
3. List all functionalities achievable by the peripheral. This list MUST be EXHAUSTIVE. This list will help you design a better state digram, a better API and exhaustive tests.  

4. Create a functional state diagram for the entire peripheral.  
5. Create a functional-test document. Just write the exhaustive tests( dont get hang up on trying to implement them, you just write them for future implementation.) Arrange them by priority level so that in case you dont implement all of them, you would have at-least implemented the essential tests.

6. Write your abstractions in accordance to the functional-state-diagram and then write your tests.  

7. Refine your API in accordance to other common PACs

<br><br>

Step 1,2,3,4 and 5 are all about creating a System View Description. This is because they are focussed on describing the structure and functions of the hardware.    

Describing peripherals and their memory mapped registers on paper is a messy affair, that's why developers came up with different standards/formats of writing such descriptions.  

In this book, we will stick to the [SVD format](https://arm-software.github.io/CMSIS_5/SVD/html/svd_Format_pg.html) because of the existence of the [svd2rust crate](https://docs.rs/svd2rust/latest/svd2rust/).  

SVD files are usually provided by the microprocessor manufacturer. For example, the svd files for the esp32c3 can be found [here    ](https://github.com/espressif/svd/blob/main/svd/esp32c3.svd)  

But before we use the svd files from the manufacturer, let's try to create our own descriptive files.( our own custom format that does not follow the SVD rules)

## Non-SVD System Descriptions
The limitation of svd files is that... 
1. They do not programmatically describe functionalies of the different peripherals and their respective registers.  SVD files describe the fuctionalities of the registers in the form of `<descriptions> bla bla bla <description/>` which usually get treated like comments - they are not parsable. It would probably be better to have register-functions described programmaticaly (i.e in a way that can be easily parsed into code. This will encourage meta-programming and automatic testing just from the svd file)
2. They do not describe the timimg constraints of the register changes and their side-effects. eg
   1. "Reading from the DATA register might automatically clear a "Receive Data Ready" flag."
   2. "Writing to the UART1 DATA register might require waiting for a "Transmit Ready" flag in a status register"
3. They do not describe the inter-dependence between the peripherals eg If you change register `x` on device `Y`, the register `a` on device `B` will get toggled automatically. Real example: Changing the system clock frequency (usually done through a Clock Control peripheral) might affect the baud rate settings of UART peripherals.  

The above points make the svd format unsuitable for automatic generation of functionally-correct code. SVD is good at describing memory maps but it relies too much on the memory and understanding of the firmware/driver developer to constrain functionalities and timing details.

There are more thorough device description formats out there that describe more about the hardware than SVD files. For example:  
1. [IP-XACT (IEEE 1685)](https://accellera.org/downloads/standards/ip-xact). You can find its [user guide](https://accellera.org/images/downloads/standards/ip-xact/IPXACT-2022_user_guide.pdf) here. 
2. [SystemRDL](https://www.accellera.org/downloads/standards/systemrdl)
3. [UVM RAL](https://verificationguide.com/uvm-ral/introduction-to-uvm-ral/)
4. JSpec - a register description format used within Juniper Networks  

Tooling is expected to work in : 
1. Header generation'
2. Driver generation
3. SystemRDL-ipxact-svd translation
4. Auto-documentation
5. Auto-testing





