# SystemRDL

[Official SystemRDL docs](https://www.accellera.org/downloads/standards/systemrdl)   


different views of presenting hardware:
1. Human description (eg look at the hardware reference manual) - outlines functional and architectural specifications in human language
2. Schematics (still falls under human description)
3. Behavioural code (describe hardware as an algorithm - inputs, outputs and expected behaviour)
4. Register description cpde
5. RTL code
6. Gate-level code
7. Transistor-level code

A regiter description  code captures:  
1. The behaviour of the individual register
2. The organization of the registers in memory
3. the organization of the registers into register files,


Various variety of register functions: 
1. Simple storage elements
2. storage elements with special read/write behavior (e.g., ‘write 1 to clear’)
3. interrupts
4. counters


SystemRDL is intended for
- RTL generation
- RTL verification
- SystemC generation
- Documentation
- Pass through material for other tools, e.g., debuggers
- Software development


"SystemRDL was created to minimize problems encountered in describing and managing registers" - what does `managing` mean?  

SystemRDL was meant to provide a single source describing registers. This description will be constant throughout the design, manufacture, testing, abstraction and usage of the piece of hardware.  

Process involved in building hardware: 
1. Human functional and architectural description
2. Test/casual Designs (schematics)
3. Formalizing (coming up with formal spec views and designs (RTL, GTL, TL, RDL, Headers, PACs, MACs, HALs, ))
4. Design Testing/Verification
5. Design Implementation/Manufacture
6. Post-Testing/Verification  



HumanSpec ==> Schematics|Behavioural Code|SystemRDL ==> {RTL, C, Verilog, Rust, Clash, SpinalHDL}


## Access Modes
## Properties per component

## Compiler
- Which versions of SystemRDL does it support?
- Compatibility to future versions? 

### Examples

```systemerdl

// simple explicit/definitive field
field 
```

### Needed Tooling
- [ ] Conversion to IP-XACT xml
- [ ] Automatic documentation
- [ ] Driver generation
- [ ] C header generation
- [ ] PAC-generator (with Rust style safety)
- [ ] Language server (for IDEs)
- [ ] VsCode systemRDL support
- [ ] Intellisense
- [ ] rust-based compiler