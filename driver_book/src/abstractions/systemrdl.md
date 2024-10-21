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
field my_custom_field {

};


// simple anonymous field
field {} ;
```

### Needed Tooling
- [ ] Conversion to IP-XACT xml
- [ ] Automatic documentation
- [ ] Strict SystemRDL {
   - makes sure user fills out every component property explicitly (or the intellisense fills things out automatially)
   - every instance is explicitly named (no implicit declarations and instantiations)
   - address comments are filled automatically (eg reg {} csr; // 0x400 - 0x408)
   - no parameter dependence
   - no dynamic assignments
   - no implicit declarations or instantiations
   - No "Arrays may be used as struct members, or in property or parameter declarations.s"
   - Where and how is it recommended to define custom properties?
   - "Effectively, multi-dimensional arrays are not supported. This limitation may be circumvented
by defining arrays of structs containing arrays." - this is not a limitation, design better!
   - remove this rule: "When expression is not specified, it is presumed the property_name is of type boolean and the default value
      is set to true" - it does not make sense eg 
      ```systemrdl
        field myField {
rclr;
// Bool property assign, set implicitly to true
woset = false;
// Bool property assign, set explicitly to false
name = “my field”; // string property assignment
sw = rw;
// accesstype property assignment
};
      ```
   - Dont allow this : "swwe and swwel have precedence over the software access property in determining its current
access state, e.g., if a field is declared as sw=rw, has a swwe property, and the value is currently
false, the effective software access property is sw=r."

  }
- [ ] Driver generation
- [ ] C header generation
- [ ] PAC-generator (with Rust style safety)
- [ ] Language server (for IDEs)
- [ ] VsCode systemRDL support
- [ ] Intellisense
- [ ] rust-based compiler
- [ ] nixos support


### Advantages
1. "NOTE—Any hardware-writable field is inherently volatile, which is important for verification and test purposes." - this will reduce the number of volatile reads in our PAC, it will allow the compiler to optimize things
2. Elaborate reads/write access descriptions : 
   1. description includes both hardware and software access specifications
   2. descriptions go beyond RW, RO, WOs
   3. description includes both read and write effects
   4. * does not explain register-register effect
   5. * does not explain timing costraints
3. fddf