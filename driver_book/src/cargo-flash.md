# cargo-flash

This is a tool that tries to make the flashing process easier.  
It already has many supported chips. And if you have a custom chip, you can tweak it to suit your needs.  


The cargo flash in crates.io is outdated (indicates version 0.13), cargo-flash supports more than just ARM chips.  
Just to get the better version, install cargo-flash by installing probe-rs as a whole.  


### Understanding cargo-flash commands

To view the possible commands, run the following comand in the terminal.  
```bash
cargo flash --help
```

You get something like this, some lines have been hidden... but you can unhide them by toggling the 'eye' button below :  
```bash
Common options when flashing a target device

Usage: cargo flash [OPTIONS]

Options:
      --reset-halt
          Use this flag to reset and halt (instead of just a reset) the attached core after flashing the target
      --log <level>
          Use this flag to set the log level
      --path <path>
          The path to the file to be flashed
      --work-dir <directory>
          The work directory from which cargo-flash should operate from
      --chip <CHIP>
          [env: PROBE_RS_CHIP=]
      --chip-description-path <chip description file path>
          
      --connect-under-reset
          Use this flag to assert the nreset & ntrst pins during attaching the probe to the chip
      --dry-run
          
      --allow-erase-all
          Use this flag to allow all memory, including security keys and 3rd party firmware, to be erased even when it has read-only protection
      --disable-progressbars
          
      --disable-double-buffering
          Use this flag to disable double-buffering when downloading flash data. If download fails during programming with timeout errors, try this option
      --restore-unwritten
          Enable this flag to restore all bytes erased in the sector erase but not overwritten by any page
      --flash-layout <filename>
          Requests the flash builder to output the layout into the given file in SVG format
      --verify
          After flashing, read back all the flashed data to verify it has been written correctly
      --format <FORMAT>
          If a format is provided, use it. If a target has a preferred format, we use that. Finally, if neither of the above cases are true, we default to ELF
      --base-address <BASE_ADDRESS>
          The address in memory where the binary will be put at. This is only considered when `bin` is selected as the format
      --skip <SKIP>
          The number of bytes to skip at the start of the binary file. This is only considered when `bin` is selected as the format [default: 0]
      --idf-bootloader <IDF_BOOTLOADER>
          The idf bootloader path
      --idf-partition-table <IDF_PARTITION_TABLE>
          The idf partition table path
  -h, --help
          Print help (see more with '--help')
  -V, --version
          Print version

PROBE CONFIGURATION:
      --protocol <PROTOCOL>     Protocol used to connect to chip. Possible options: [swd, jtag]
      --probe <PROBE_SELECTOR>  Use this flag to select a specific probe in the list
      --speed <SPEED>           The protocol speed in kHz


// # CARGO BUILD OPTIONS:

// #     The following options are forwarded to 'cargo build':

// #         --bin
// #         --example
// #     -p, --package
// #         --release
// #         --target
// #         --manifest-path
// #         --no-default-features
// #         --all-features
// #         --features

// #     Additionally, all options passed after a sentinel '--'
// #     are also forwarded.

// #     For example, if you run the command 'cargo flash --release -- \
// #     --some-cargo-flag', this means that 'cargo build \
// #     --release --some-cargo-flag' will be called.
```


####  Meaning and Usage of the commands

```bash
--reset-halt
          Use this flag to reset and halt (instead of just a reset) the attached core after flashing the target
```
Resetting in the context of embedded systems typically refers to the act of restarting or reinitializing the microcontroller or microprocessor to a known state. This process can involve several steps, including clearing memory, resetting peripheral devices, and initializing hardware registers to their default values.  

Halting the core means stopping its execution, which can be useful for debugging or ensuring a clean state before starting execution.



```bash
--log <level>:
# for example :  
# cargo flash --log error
# cargo flash --log debug
``` 
Sets the log level for the operation. Logging levels control the verbosity of the output messages, allowing users to specify how much detail they want to see during the flashing process.  

Certainly! Let's consider an example of how the --log option can be used in a cargo-flash command:

Suppose you're flashing firmware onto a microcontroller using cargo-flash, and you want to control the amount of detail shown in the output messages during the flashing process. You can use the --log option to specify the desired logging level.

For instance, if you want to see only critical errors and warnings during the flashing process, you can set the log level to "error" or "warn". Here's how you would do it:

```bash
cargo flash --log error
```

With this command, cargo-flash will only display error messages, such as critical failures or warnings indicating potential issues that need attention. This minimal logging level can be useful when you're primarily interested in identifying and addressing critical problems during flashing.

On the other hand, if you want more detailed information during the flashing process, you can set the log level to "info" or "debug". Here's how you would do it:

```bash
cargo flash --log debug
```

With this command, cargo-flash will provide additional information, such as progress updates, debugging messages, and other details relevant to the flashing operation. This higher logging level can be helpful when you're troubleshooting issues or monitoring the flashing process closely.


```bash
--path <path>:
# for example : cargo flash --path ./target/debug/executable_file
``` 
Specifies the path to the file to be flashed. This is the binary file containing the firmware that will be programmed onto the target device.  
If you don't specify this, cargo will automatically assume that its the target binary of that particular cargo project.  


```bash
--chip <CHIP>:  
# for example : cargo flash --chip esp32c3
```  
Specifies the target chip/device. This option is crucial for cargo-flash to know the specific hardware it is flashing firmware onto. This command only works with supported chips. If your chip is not supported by default, you can add it by providing the chip description(SVD) and tweaking some configs [undone: more explanation is needed here]  

```bash
--chip-description-path <chip description file path>: 

```
Specifies the path to the file containing the description of the target chip. This file provides information about the memory layout, peripherals, and other details necessary for flashing. This file is helpful when you are adding a new chip to be supported by cargo flash and probe-rs.  


```bash
--connect-under-reset: 
# For example : cargo-flash --chip esp32c3 --connect-under-reset
```
Sets the states of the nreset & ntrst pins during attaching the probe to the chip. This option can ensure a more reliable connection between the programming tool and the target device.  
Setting the nreset pin resets the state of the whole micro-controller.  
setting the ntrst pin resets the chip debug/programming interface only.  


```bash
--dry-run`:
``` 
Performs a trial run without actually flashing the firmware; it simulates the flashing process without actually writing any data to the target device. Instead of performing the real flashing operation, it only goes through the motions of the process.  
This is useful for testing the command and verifying the parameters without making any changes to the target device. It's important for ensuring that the flashing process is configured correctly before performing the actual operation.  




