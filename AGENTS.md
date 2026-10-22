# Driver Creation Procedure

Main Guide to create driver for this project with rust.
Below are main dependencies can used for making driver, use latest version if there 
is any, driver must compliance and compitable with embedded-hal, 
use bilge for list all register as typed struct and enum, also use bilge 
to represent odd register like u3, u5, u7 etc. only add libm if driver need math operation 
that not available in nostd environment, always use defmt for debugging and showing info. 

```toml
[dependencies]
bilge = "0.5.0"
defmt = { version = "1.1.1", optional = true }
embedded-hal = "1.0.0"
embedded-hal-async = "1.0.0"
embedded-hal-bus = "0.3.0"
libm = "0.2.15"
typed_builder = "0.23.2"
```

## Preparation and Plan

Use latest manufacturer datasheet and errata; cite revision and check differences between device variants.

If Device is use register as main interface 

- list all register available in device 
- document it inside REGISTERS.md on root device folder, with tabular shape data
- on every register write requirement to read the register
- include reset values, access permissions, byte order and read side effects in REGISTERS.md.
- create register as new rust file, make sure to use bilge for typing each register and its field
- there is one main mod for register which is `register.rs` so every register is inside `register` folder
- after all register is complete to write and documented, add every function description inside FUNCTIONS.md, 
  write all function and how to retrieve it from device register and post process after it, if function 
  use equation add markdown compatible math equation. 
- then write all function inside `function.rs` file, if function is big, split it inside folder then expose it 
  from `function.rs` but if not just place all function in  function.rs 
- function is struct shaped with default derive, every function argument must only use register read result from `register.rs`
  then resulting function result based on data sheet. 
- create test for every function, make sure to use good mock data so test consistent, make sure to use consistent test name. 
- next device, device is the main struct exposed by library, it will use function and all register struct in its field, 
  device is have all main method to exposing function and register, so every register reading is happen on device method then passed 
  to function from inside same method, device is main interface for user library so if we need user argument inside device method 
  it should be on here not inside function.

If Device is use other type of interface, like PWM code or timing based operation 

- if device is not use standard interface, write all procedure requirement and step inside PROCEDURE.md
- then implement all step inside procedure.rs, if step is big, split it inside folder procedure/ but if not just place inside procedure.rs 
- always separate constant for clearness.
- then, follow same procedure as the register based device

Every protocoled device is got its own way to communicate, separate communication method into file comproc.rs if communication procedure is long 
split it into folder comproc/ but if not place it directly in comproc.rs

Make sure to use real device name for main library struct name. 

If device have multiple possible protocol interface create builder with type and protocol type and other possible builder argument,
place it inside builder.rs in root folder, use `typed_builder` https://docs.rs/typed-builder/latest/typed_builder/

Prefer to use long name struct or function instead short hard to understand name from datasheet, but always add comment above struct for datasheet reference.

Create device error enum containing specific device error and wrapped interface error, make sure never sink error instead forward it into device error.

Document reset/startup requirements, shared-bus effects and non-atomic partial failures in device README.md.

Add Mermaid diagrams in device README.md for struct ownership and function connections; group generated methods.

## Validation 

If device got validation requirement like crc or anything, place it inside validation.rs , if validation is many and long split it into file inside folder 
validation/ if not too long just place it inside validation. 

- mock-test bus framing, register preservation and error forwarding, including failures during multi-step operations.
- check formatting, clippy, docs and tests with/without optional features; build for a no_std target and state whether hardware was tested.

## Look for possible already implementation 

Look on internet for already implementation for same device in same language or other language 
but make sure to use it only for reference.

## Commit

Always commit changes on every important changes and on every completed step.

## Summary 

- use bilge as typed register 
- write device docs and registers
- split function and function is only intended for device struct usage not user use. 
- always create test on every testable operation. 
