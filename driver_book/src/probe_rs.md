# probe-rs

Probe-rs is an open-source debugging and flashing tool for microcontrollers.  
It provides a unified interface to interact with various hardware debug probes, enabling developers to debug and flash firmware on embedded systems.  

Probe-rs can be used both as a crate or a CLI app.  

It supports a wide range of microcontroller architectures and debug probe hardware, making it a versatile tool for embedded development

## Architecture

Probe-rs is built with a modular architecture, consisting of several key components:

- Probe Interface: This component interacts with the physical hardware debug probe, such as J-Link, ST-Link, or CMSIS-DAP.
- Debugging Backend: Handles the low-level communication with the target microcontroller's debug interface (e.g., JTAG, SWD).
- API Layer: Provides a high-level API for interacting with the debug probe and the target microcontroller. This API is used by higher-level tools and applications.
- Utilities and Extensions: Probe-rs offers various utilities and extensions for specific use cases, such as firmware flashing, debugging, and memory inspection

## probe-rs CLI commands and their importance
