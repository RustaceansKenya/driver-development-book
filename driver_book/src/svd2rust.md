# svd2rust

Once you read the datasheet, and understand the memory mappings, pin-layout and whatever else you wanted to get straight, you begin to safely abstract the board.  


### SVD files
An svd file is a file that describes the peripherals of a board using xml. So you could say that an svd file is a board abstracted as an xml template.  
SVD is the abbreviation for : System View Description.  


The svd file outlines :
 - The boards metadata eg boardname, board version, feature description, vendor name
 - Major component info : eg CPU_capabilities, Endianness, address_width, added cpu_extensions... 
 - all list of all the peripherals
   - the registers of each peripheral
   - the functions of each register
   - the memory address of each register
   - the read/write access of each register



You can find sample svd files [here][espressif_svd_file_samples], they are from the espressif organization.  
Here is the esp32C3 svd file that we will be using : [ESP32_C3 svd file][esp32c3_svd_file]  


Here is a snippet of a sample svd file : 
```xml
<?xml version="1.0" encoding="UTF-8"?>
<device schemaVersion="1.1" xmlns:xs="http://www.w3.org/2001/XMLSchema-instance" xs:noNamespaceSchemaLocation="CMSIS-SVD_Schema_1_1.xsd">
  <vendor>ESPRESSIF SYSTEMS (SHANGHAI) CO., LTD.</vendor>
  <vendorID>ESPRESSIF</vendorID>
  <name>ESP32-C3</name>
  <series>ESP32 C-Series</series>
  <version>17</version>
  <description>32-bit RISC-V MCU &amp; 2.4 GHz Wi-Fi &amp; Bluetooth 5 (LE)</description>

  <!-- snip snip snipped some lines -->

  <cpu>
    <name>RV32IMC</name>
    <revision>r0p0</revision>
    <endian>little</endian>
    <mpuPresent>false</mpuPresent>
    <fpuPresent>false</fpuPresent>
    <nvicPrioBits>0</nvicPrioBits>
    <vendorSystickConfig>false</vendorSystickConfig>
  </cpu>

  <!-- snip snip snipped some lines -->

   <peripherals>
   
   <!-- here is 1/32 peripherals -->
    <peripheral>
        <name>UART0</name>
      <description>UART (Universal Asynchronous Receiver-Transmitter) Controller 0</description>
      <groupName>UART</groupName>
      <baseAddress>0x60000000</baseAddress>
      <addressBlock>
        <offset>0x0</offset>
        <size>0x84</size>
        <usage>registers</usage>
      </addressBlock>
      <interrupt>
        <name>UART0</name>
        <value>21</value>
      </interrupt>
      <registers>
        <register>
          <name>FIFO</name>
          <description>FIFO data register</description>
          <addressOffset>0x0</addressOffset>
          <size>0x20</size>
          <fields>
            <field>
              <name>RXFIFO_RD_BYTE</name>
              <description>UART 0 accesses FIFO via this register.</description>
              <bitOffset>0</bitOffset>
              <bitWidth>8</bitWidth>
              <access>read-write</access>
            </field>
          </fields>
        </register>

        <!-- more registers -->
        <register>
        </register>
        </register>
        <!-- more registers -->
    <peripheral>

    <!-- snipped out the other 31 peripherals -->
   <peripherals>
```


### svd2Rust

This is a tool that takes in svd files and outputs Rust code that reflects the contents of the svd file.  

### Why use svd2rust instead of doing the abstraction manually?

Before we discuss whether you should do it manually or not. Let's settle out some facts first.  
A full-fledged board has many components. The datasheet reference is like >700 pages. These components are dependent on each other.  
You get some form of info overload. How can you create complete abstractions if you do not fully understand the board and how they are interdependent on each other? Enter headaches and suicidal thoughts.  

If you look at the esp32c3.svd file, you realize it is >35000 lines. But at-least the svd file provides a complete abstraction from the >700 page datasheet.  

#### When to do it manually
1. When you fully understand all the details about a peripheral
2. When you also fuly understand all the direct components that the target peripheral depends on.  
3. When you can comfortably abstract the peripheral and its dependents, in a safe way: critical sections, atomics and all that vodoo when accessing registers.  
4. When you do not need to abstract the whole board.  

#### When to do it automatically
1. When you dont mind abstracting all the peripherals
2. When you want a library to automatically implement the access-safety methods of accessing registers. You don't have to implement atomic vodoo on your own.
3. When you want to use a standard way of abstracting the board. Your whole team uses the same template. Everyone speaks the same language, everyone becomes happy.



### svd2rust

To understand svd2rust, let's :
1. read its docs
2. experiment with it a little
3. Do our abstractions manually without depending on svd2rust
4. Go back to svd2rust while fully appreciating all the manual work it does for us 



From the docs :  

"svd2rust supports Cortex-M, MSP430, RISCV and Xtensa LX6 microcontrollers." this means that ...

Taking Peripherals needs to be atomic. Some boards do not support atomics, so the best way is to use software-defined critical sections.  
The take function looks something like this :  
```rust

```



[espressif_svd_file_samples]: [https://github.com/espressif/svd/tree/main/svd]  
[esp32c3_svd_file]: [https://github.com/espressif/svd/tree/main/svd]

