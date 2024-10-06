# svd2rust

Once you read the datasheet and understand the memory mappings, pin-layout and whatever else you wanted to get straight, you may begin to safely abstract the board.  


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


### svd2rust

`svd2rust` is a tool that takes in svd files and outputs Rust code that reflects the contents of the svd file. It automatically creates PACs over peripherals of a board.  

### Why use svd2rust instead of doing the abstraction manually?

Is this even a question?  

Dear reader, love yourself. Love yourself. The world is so big and there is so much to do.  
Whenever you find a tool that can convert +1000 pages worth of documentation into working code, do not hesitate to use that tool - then go ahead and build more abstractions.  
You are free to read the docs of that tool and replicate what it does... you are free to learn from it by re-inventing its wheels, you can even improve its wheels... but please - Do not do the same EXACT work manually.  

But anyway, let's answer the question... ignore whatever rubbish I just talked about.  

#### When to abstract manually and ignore automation tools like svd2rust
1. When you just want to learn and you want to go as low-level as possible and do things by your own intuition.  
2. When you fully understand all the details about a peripheral and you are confident that you'll make better abstractions than the automation tool
3. When you fully understand all the direct components that the target peripheral depends on.  
4. When you can comfortably abstract the peripheral and its dependents, in a safe way: Automation tools like svd2rust easily integrate with critical section control across many boards.  
5. When you just want to have fun.  

#### When to do it automatically
1. When you mind abstracting all the dozens of peripherals by hand (2000+ lines of pure xml)
2. When you want a library to automatically implement the access-safety methods of accessing registers. You don't have to implement atomic vodoo on your own.
3. When you want to use a standard way of abstracting the board for the sake of source-code portability. If your whole team uses the same abstraction template, then everyone will feel like they are speaking the same language and everyone will be happy. The world would then become this peaceful place where there are no wars and people try to understand each other before nuking each other.  
4. When you take no pride or fun in writing boiler-plate code.   



## Assignment
Reader, go read the svd2rust docs and experiment with it.  
Come back after you are done!  
Bye.  


For those who just want a summary, here is a watered down look into the svd2rust crate. [Cheatsheet](./abstractions/svd2rust/further_explanations.md)


