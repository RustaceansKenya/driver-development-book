# Udev

Reference tutorial : https://opensource.com/article/18/11/udev

Udev stands for User-Device manager.  
It is one of linux's subsystems. You can find the other subsystems in the folder '/sys'.    

[undone: write a tutorial on udev]  
to remember : 
  - monitoring with 'udevadm monitor'. Looking out for verbs/actions associated with device. What is the order of events when device_event happens?
  - listing the available devices : lsusb, lsblk, lshw, 
  - Reading output from lsusb.
  - Reading output from lsblk.
  - Getting all associated attributes associated to a specific device and its parents, so as to get the right attributes to reference when writing rules
  - Writing rules.
  - Reloading rules
  - Logging