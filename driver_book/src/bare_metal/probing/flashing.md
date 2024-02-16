# Flashing  

In embedded development, "flashing" typically refers to the process of writing or programming the firmware or software onto a microcontroller or other embedded system's non-volatile memory. This non-volatile memory can be flash memory, EEPROM (Electrically Erasable Programmable Read-Only Memory), or similar types of memory that retain data even when power is removed.  

We will use [the esp-flash tool][esp-flash-docs-for-esp32c3]. A tool purposefully built to flash programs into `esp` boards like our very own Esp32c3.  

We could have used other tools like OpenOCD and Probe-rs because they are more generic than espflash. Once you learn them, you can tweak them for different platforms. Espflash is specialized, readers are advised to learn other tools in order to have a holisti growth.  

We will use [espflash][esp-flash-docs-for-esp32c3] in order to escape the trouble of writing our own configs and flashing algorithm.  



## The honors of flashing... finally
The [esp-rs team][esp-rs-github] did a good job with [their docs][esp-rs-book]. Go through the [flashing page][esp-flash-docs-for-esp32c3] and do the honors. 




[esp-flash-docs-for-esp32c3]: https://esp-rs.github.io/book/tooling/espflash.html
[esp-rs-github]: https://github.com/esp-rs
[esp-rs-book]: https://esp-rs.github.io/book/
