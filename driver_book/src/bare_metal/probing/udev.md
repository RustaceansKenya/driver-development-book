# Udev

Reference tutorial : https://opensource.com/article/18/11/udev

Udev stands for User-Device manager.  
It is one of linux's subsystems. You can find the other subsystems in the root folder '/sys'.    
Udev is the Linux subsystem that supplies your computer with device events.  


When you plug in a device to your linux pc...
 - The device gets detected by some of udev submodules.
 - The device gets abstracted as a file stored in the `/dev` root directory.  
 - Udev starts and continues to act as a message proxy between the kernel and device. 
 - Udev continuously listens and detects events induced by the external device and relays this info to the kernel.  
 - The kernel on the other hand, returns action responses to udev. Udev then invokes the necessary action given by the kernel responses.  

The above description is somehow inaccurate and is given to provide a high-level overview of what happens. To read on the exact order and definitions of things, consult official udev docs.  

## Listing attached devices  
The process of detecting and abstracting attached devices as files in the `/dev` directory happens automatically.  
To view attached devices, you can browse through the `/dev` directory OR use the following cmd commands :  

```bash
lsusb# List all the devices that have been attached to the USB-controllers
lsblk # List all block devives

lscpu # List all CPUs
lspci # Lists all PCI devices
```

### Reading the results from the cmd commands

Here is an example of a reading :  
```bash
> lsusb
Bus 001 Device 002: ID 8087:8001 Intel Corp. Integrated Hub
Bus 001 Device 001: ID 1d6b:0002 Linux Foundation 2.0 root hub
Bus 003 Device 002: ID 0424:5534 Microchip Technology, Inc. (formerly SMSC) Hub
Bus 003 Device 001: ID 1d6b:0003 Linux Foundation 3.0 root hub
Bus 002 Device 004: ID 05c8:0374 Cheng Uei Precision Industry Co., Ltd (Foxlink) HP EliteBook integrated HD Webcam
Bus 002 Device 003: ID 8087:0a2a Intel Corp. Bluetooth wireless interface
Bus 002 Device 005: ID 0461:4d22 Primax Electronics, Ltd USB Optical Mouse
Bus 002 Device 002: ID 0424:2134 Microchip Technology, Inc. (formerly SMSC) Hub
Bus 002 Device 038: ID 04e8:6860 Samsung Electronics Co., Ltd Galaxy A5 (MTP)
Bus 002 Device 001: ID 1d6b:0002 Linux Foundation 2.0 root hub

```  

The above output depicts that...
- There are at least three USB controllers denoted by the term BUS [1 - 3]. Note that a USB-port is not synonymous to USB-controller.  
- The `Ltd USB Optical Mouse` has a device ID of `4d22`
- The `Ltd USB Optical Mouse` has a Vendor ID of `0461`
- The `Ltd USB Optical Mouse` has been attached USB-port 005 of the second USB-controller
- The file that abstracts `Ltd USB Optical Mouse` is `/dev/bus/usb/002/005`.

## Udev's Real-time monitoring of device-events
With the udevadm monitor command, you can tap into udev in real time and see what it sees when you plug in different devices. Become root and try it.  

```bash
> sudo -i
> udevadm monitor
```

The monitor function prints received events for:

- UDEV: the event udev sends out after rule processing
- KERNEL: the kernel uevent

With udevadm monitor running, plug in a thumb drive and watch as all kinds of information is spewed out onto your screen. Notice that the type of event is an ADD event. That's a good way to identify what type of event you want.  

## Udev's info snooping 
You can view the info for a particular device by using the command : `udevadm info [OPTIONS] [DEVPATH|FILE]`.  
For example : 
```bash
# suppose the lsusb command had the folowing output...
# Bus 002 Device 005: ID 0461:4d22 Primax Electronics, Ltd USB Optical Mouse
# Then you would get info about the mouse by...
udevadm info /dev/bus/usb/002/005
```  

## Udev scripts
You can write scripts for udev. For example, you may automatically make your flash-drive trigger the execution of a bash script whenever it gets plugged in a specific usb-port.  

In normal scripting/programming, you usually identify an object using something like a variable name. However, in udev scripting, you identify devices based on a set of attributes. The more specific the atributes, the more you narrow your reference to a specific device.  

Here is a rough format of an instruction : 
```bash
# This is an exerpt from a udev script
# The following statement means ....
#  Whenever a USB device gets ADDed (plugged-in), the 'thumb' script gets ran
SUBSYSTEM=="usb", DRIVER=="usb", ACTION=="add", RUN+="/usr/local/bin/thumb
```

```bash
# But this rule is more specific... it does not specify just a usb device, 
# it specifies the product ID and vendor ID of the device
SUBSYSTEM=="usb", DRIVER=="usb", ATTR{idProduct}=="4d22",  ATTR{idVendor}=="0461", ACTION=="add", RUN+="/usr/local/bin/thumb_for_mouse
```

To find the specific attributes of a device and all of its parents, use the command below :  
```bash
udevadm info -a /dev/bus/usb/002/005
```

Udev fetches its scripts from `/etc/udev/rules.d/`.  
The script files end with the a `.rules` extension.  
The script files usually begin with a number to show the order in which the scripts get parsed by udev. For example `80-usb-script.rules` will get executed before `81-usb-extra-script.rules`.  

## So why were we learning about Udev?  
Well, two reasons...
1. It's essential knowledge, especially if you'll be handling devices using the linux kernel.  
2. We will need to interact with the device files. The problem is that the device files are only writable by `root`. So we need to make the files that we need accessible to our normal development account.  


# Pracs

- Attach your Esp32c3 to your computer using a usb cable.  
- You will notice that 2 new files get generated under the `/dev` directory. One under tty (eg ttyACM0) and one under `/dev/bus`. Figure out where and why. These 2 files are there to abstract your device and the USB connection. 
- Write a script to make these two files accessible to your linux account.  

Done!!


<!-- undone, finish this udev tutorial -->
<!-- [undone: write a tutorial on udev]  
to remember : 
  - [ ] monitoring with 'udevadm monitor'. Looking out for verbs/actions associated with device. What is the order of events when device_event happens?
  - [ ] listing the available devices : lsusb, lsblk, lshw, 
  - [ ] Reading output from lsusb.
  - [ ] Reading output from lsblk.
  - [ ] Getting all associated attributes associated to a specific device and its parents, so as to get the right attributes to reference when writing rules
  - [ ] Writing rules.
  - [ ] Reloading rules
  - [ ] Logging -->
