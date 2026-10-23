# MCP23017 / MCP23S17

Allocation-free, `no_std`, blocking `embedded-hal` 1.0 driver for Microchip's
16-bit GPIO expanders. Supports I2C (MCP23017) and SPI (MCP23S17), bilge-typed
registers, direction, pull-ups, polarity inversion, output latches, interrupt
configuration, source flags and captured input states. Optional `defmt` logging
reports initialization, invalid configurations and bus failures.

## I2C

```rust
use embedded_hal::i2c::I2c;
use mcp23017::{u3, DeviceError, Direction, MCP23017, Pin, Port};

fn example<I: I2c>(i2c: I) -> Result<(), DeviceError<I::Error>> {
    // Address pins A2:A0 = 000 => I2C address 0x20.
    let mut device = MCP23017::new_i2c(i2c, u3::new(0))?;
    device.set_output(Port::A, Pin::P0, true)?;
    device.set_direction(Port::A, Pin::P0, Direction::Output)?;
    device.set_pull_up(Port::B, Pin::P6, true)?;
    let _levels = device.read_ports()?;
    let _i2c = device.release().release();
    Ok(())
}
```

## SPI

Supply a configured `SpiDevice<u8>` (SPI mode 0 or 3) which manages CS, not a
bare `SpiBus`. Use a clock within the datasheet limit for your supply voltage.

```rust
use embedded_hal::spi::SpiDevice;
use mcp23017::{u3, DeviceError, MCP23S17};

fn example<S: SpiDevice<u8>>(spi: S) -> Result<(), DeviceError<S::Error>> {
    let mut device = MCP23S17::new_spi(spi, u3::new(5))?;
    device.write_outputs(0x1234)?;
    Ok(())
}
```

## Typed builder and registers

```rust
use embedded_hal::i2c::I2c;
use mcp23017::{
    builder::Builder, comproc::I2cInterface,
    register::{Configuration, InputDirection}, u3, DeviceError, Direction, Port,
};

fn example<I: I2c>(i2c: I) -> Result<(), DeviceError<I::Error>> {
    let mut device = Builder::builder()
        .interface(I2cInterface::new(i2c, u3::new(2)))
        .build()
        .connect()?;
    let mut direction = device.read_register::<InputDirection>(Port::A)?;
    direction.set_pin3(Direction::Output);
    device.write_register(Port::A, direction)?;
    let mut config = device.read_register::<Configuration>(Port::A)?;
    config.set_interrupt_open_drain(true);
    device.write_configuration(config)?;
    Ok(())
}
```

Select `SpiInterface::new(spi, address_pins)` instead for the SPI builder.
`u3` restricts address-pin values to 0-7; these are pin levels, **not** full I2C
addresses. `Pin` restricts pin indices to 0-7 within the selected `Port`.

## Struct and function connections

These diagrams cover every library struct and handwritten function, including
private helpers. Repeated bilge-generated register methods and typed-builder
methods are grouped explicitly. Test-only helpers and compiler-generated
formatting, copying and equality implementations are omitted.

Arrows show ownership, calls or data flow as labeled. Method names repeated
across diagrams refer to the same implementation. Register values are passed
through device methods, not cached as device fields.

### Device ownership and interface selection

```mermaid
flowchart TD
    device["MCP23017&lt;I&gt;<br/>device.rs"]
    builder["Builder&lt;I&gt;<br/>builder.rs"]
    interface["I: Interface<br/>sealed transport trait in comproc.rs"]
    i2c["I2cInterface&lt;I2C&gt;"]
    spi["SpiInterface&lt;SPI&gt;"]
    processing["PortValues<br/>private function.rs struct"]
    spiAlias["MCP23S17&lt;SPI&gt;<br/>type alias, not another struct"]
    errors["DeviceError&lt;I::Error&gt;<br/>error.rs enum"]
    errorAlias["Error&lt;E&gt;<br/>type alias"]
    port["Port enum<br/>A or B"]
    pin["Pin enum<br/>P0 through P7"]
    direction["Direction enum<br/>Input or Output"]

    builder -->|"owns interface until connect"| interface
    device -->|"owns interface field"| interface
    device -->|"owns functions field"| processing
    i2c -.->|"implements"| interface
    spi -.->|"implements"| interface
    i2c -->|"owns bus"| i2cHal["embedded_hal::i2c::I2c"]
    spi -->|"owns bus"| spiHal["embedded_hal::spi::SpiDevice"]
    spiAlias -.->|"MCP23017 with SpiInterface"| device
    device -.->|"fallible methods return"| errors
    builder -.->|"connect returns device or error"| errors
    errorAlias -.->|"same enum"| errors
    port -->|"register and pin operation argument"| device
    pin -->|"pin operation argument"| device
    direction -->|"set_direction argument"| device
```

### Construction and resource release

`connect` is private to the crate; applications use a constructor or the builder.
Its numbered edges show initialization order. Releasing a device returns its
interface; releasing that interface is a separate application call.

```mermaid
flowchart TD
    newI2c["MCP23017::new_i2c"]
    newSpi["MCP23S17::new_spi<br/>same impl as MCP23017::new_spi"]
    interfaceI2cNew["I2cInterface::new"]
    interfaceSpiNew["SpiInterface::new"]
    builderStart["Builder::builder<br/>generated"]
    builderInterface["interface<br/>generated builder setter"]
    builderBuild["build<br/>generated, returns Builder"]
    builderConnect["Builder::connect"]
    connect["MCP23017::connect"]
    initialize["Interface::initialize"]
    read["MCP23017::read_register&lt;Configuration&gt;"]
    check["MCP23017::check_configuration"]

    newI2c -->|"constructs transport"| interfaceI2cNew
    newSpi -->|"constructs transport"| interfaceSpiNew
    newI2c -->|"passes transport"| connect
    newSpi -->|"passes transport"| connect
    builderStart -->|"returns builder state"| builderInterface
    interfaceI2cNew -->|"value supplied by caller"| builderInterface
    interfaceSpiNew -->|"value supplied by caller"| builderInterface
    builderInterface -->|"next builder call"| builderBuild
    builderBuild -->|"caller invokes"| builderConnect
    builderConnect -->|"passes owned transport"| connect
    connect -->|"1: prepare transport"| initialize
    connect -->|"2: read port A configuration"| read
    connect -->|"3: validate readback"| check
    connect -->|"success constructs device and unit PortValues"| ready["MCP23017&lt;I&gt;"]

    release["MCP23017::release"]
    release -->|"returns I2C interface"| releasedI2c["I2cInterface"]
    release -->|"or returns SPI interface"| releasedSpi["SpiInterface"]
    releasedI2c -->|"caller may invoke"| releaseI2c["I2cInterface::release"]
    releasedSpi -->|"caller may invoke"| releaseSpi["SpiInterface::release"]
    releaseI2c -->|"returns owned bus"| i2cBus["I2C"]
    releaseSpi -->|"returns owned bus"| spiBus["SPI"]
    ready -->|"consumed by caller's release call"| release
```

### Pin operations, configuration and private helpers

Edge labels on setters identify the register type used by `update_bit`.
Read-modify-write preserves neighboring bits and returns immediately on error.
Output updates use `OutputLatch`, never a GPIO read.

```mermaid
flowchart TD
    direction["MCP23017::set_direction"]
    pullUp["MCP23017::set_pull_up"]
    polarity["MCP23017::set_input_inverted"]
    interruptEnable["MCP23017::set_interrupt_enabled"]
    comparison["MCP23017::set_interrupt_comparison"]
    defaultLevel["MCP23017::set_interrupt_default"]
    output["MCP23017::set_output"]
    update["MCP23017::update_bit&lt;R&gt;<br/>private"]
    read["MCP23017::read_register&lt;R&gt;"]
    write["MCP23017::write_register&lt;R&gt;"]
    mask["Pin::mask<br/>private, returns one-bit u8 mask"]
    readPin["MCP23017::read_pin"]
    configuration["MCP23017::write_configuration"]
    check["MCP23017::check_configuration<br/>private"]

    direction -->|"InputDirection"| update
    pullUp -->|"PullUpResistors"| update
    polarity -->|"InputPolarity"| update
    interruptEnable -->|"InterruptEnable"| update
    comparison -->|"InterruptControl"| update
    defaultLevel -->|"InterruptDefaultValue"| update
    output -->|"OutputLatch"| update
    direction -->|"MCP23017 P7 input rejected before I/O"| unsupported["DeviceError::UnsupportedInputPin"]
    update -->|"1: previous register value"| read
    update -->|"2: select bit"| mask
    update -->|"3: modified register value"| write
    mask -->|"converts Pin to its three-bit index"| index["u3::from(Pin).value"]
    readPin -->|"GeneralPurposeInputOutput"| read
    readPin -->|"mask selected level"| mask
    configuration -->|"1: validate requested Configuration"| check
    configuration -->|"2: write IOCON at 0x0A"| interfaceWrite["Interface::write"]
    check -->|"reads three generated field getters"| fields["Configuration::separate_register_banks<br/>Configuration::unimplemented_bit<br/>Configuration::hardware_address_enable"]
    check -->|"BANK=1"| bank["DeviceError::UnsupportedRegisterBank"]
    check -->|"reserved bit=1"| reserved["DeviceError::ReservedConfigurationBitSet"]
    check -->|"SPI HAEN=0"| addressing["DeviceError::HardwareAddressingDisabled"]
```

### Combined port operations and register-result processing

The device reads A then B before calling `PortValues`; each conversion puts A
in the low byte and B in the high byte. `PortValues` performs no bus I/O.

```mermaid
flowchart LR
    readPorts["MCP23017::read_ports"]
    readCapture["MCP23017::read_capture"]
    readFlags["MCP23017::read_interrupt_flags"]
    readLatches["MCP23017::read_output_latches"]
    read["MCP23017::read_register&lt;R&gt;<br/>separate A and B reads"]
    processing["PortValues"]
    inputLevels["PortValues::combine_input_levels"]
    capturedLevels["PortValues::combine_captured_levels"]
    interruptFlags["PortValues::combine_interrupt_flags"]
    outputLatches["PortValues::combine_output_latches"]
    result["u16::from_le_bytes<br/>A in bits 0-7; B in bits 8-15"]

    readPorts -->|"GeneralPurposeInputOutput A and B"| read
    readCapture -->|"InterruptCapture A and B"| read
    readFlags -->|"InterruptFlags A and B"| read
    readLatches -->|"OutputLatch A and B"| read
    readPorts -->|"passes typed read results"| inputLevels
    readCapture -->|"passes typed read results"| capturedLevels
    readFlags -->|"passes typed read results"| interruptFlags
    readLatches -->|"passes typed read results"| outputLatches
    processing -->|"method"| inputLevels
    processing -->|"method"| capturedLevels
    processing -->|"method"| interruptFlags
    processing -->|"method"| outputLatches
    processing -.->|"derived, not called by device construction"| processingDefault["PortValues::default"]
    inputLevels --> result
    capturedLevels --> result
    interruptFlags --> result
    outputLatches --> result

    outputs["MCP23017::write_outputs"] -->|"u16::to_le_bytes"| bytes["A low byte, B high byte"]
    bytes -->|"OutputLatch A then B"| write["MCP23017::write_register&lt;OutputLatch&gt;"]
```

### Register transactions and error forwarding

The `Interface` methods dispatch statically to the selected transport. Each
device boundary wraps an interface failure using `DeviceError::from`; `?`
forwards that error through the calling operation without replacing its payload.
I2C initialization is a no-op; SPI initialization performs two startup writes.

```mermaid
flowchart TD
    readRegister["MCP23017::read_register&lt;R&gt;"]
    writeRegister["MCP23017::write_register&lt;R&gt;"]
    address["Register::address<br/>ADDRESS_A plus Port B offset"]
    fromByte["R::from(u8)<br/>bilge-generated conversion"]
    intoByte["u8::from(R) / R::into<br/>bilge-generated conversion"]
    initialize["Interface::initialize"]
    read["Interface::read"]
    write["Interface::write"]
    i2cInitialize["I2cInterface::initialize"]
    spiInitialize["SpiInterface::initialize"]
    i2cRead["I2cInterface::read"]
    spiRead["SpiInterface::read"]
    i2cWrite["I2cInterface::write"]
    spiWrite["SpiInterface::write"]

    readRegister -->|"select register address"| address
    readRegister -->|"read one byte"| read
    readRegister -->|"convert successful byte to typed register"| fromByte
    writeRegister -->|"select register address"| address
    writeRegister -->|"serialize register"| intoByte
    writeRegister -->|"write one byte"| write
    initialize --> i2cInitialize
    initialize --> spiInitialize
    i2cInitialize -->|"no bus I/O"| success["Ok(())"]
    spiInitialize -->|"opcodes 0x40 and 0x48, IOCON=0x08"| startup["SpiDevice::write twice"]
    read --> i2cRead
    read --> spiRead
    write --> i2cWrite
    write --> spiWrite
    i2cRead -->|"register address then one byte"| i2cWriteRead["I2c::write_read"]
    i2cWrite -->|"register address and value"| i2cHalWrite["I2c::write"]
    spiRead -->|"Write header and Read byte under one CS"| spiTransaction["SpiDevice::transaction"]
    spiWrite -->|"opcode, register address, value"| spiHalWrite["SpiDevice::write"]
    initialize -.->|"Err at device connect boundary"| wrap["DeviceError::from"]
    read -.->|"Err at device read boundary"| wrap
    write -.->|"Err at device write boundary"| wrap
    wrap -->|"owns unchanged interface error"| error["DeviceError::Interface(original_error)"]
```

### Every typed register and generated methods

All eleven register structs implement `Register`, with addresses and reset
values supplied by the local `register!` macro. Only the eight types connected
to `WritableRegister` support `write_register`; `Configuration` instead requires
the checked `write_configuration` method. Both traits are sealed.

For repeated methods below, `N` expands to each index from 0 through 7 and
`field` expands to each named configuration field listed in the final node.
Generated setters modify an in-memory value, even for read-only register types;
they do not grant permission to write that value to hardware.

```mermaid
flowchart TD
    subgraph registers["Register structs in register/"]
        inputDirection["InputDirection"]
        inputPolarity["InputPolarity"]
        interruptEnable["InterruptEnable"]
        interruptDefault["InterruptDefaultValue"]
        interruptControl["InterruptControl"]
        configuration["Configuration"]
        pullUps["PullUpResistors"]
        flags["InterruptFlags"]
        capture["InterruptCapture"]
        gpio["GeneralPurposeInputOutput"]
        latch["OutputLatch"]
    end

    registers -.->|"all implement"| readable["Register"]
    readable -->|"provided method"| address["Register::address(Port)"]
    readable -->|"required conversions"| conversion["R::from(u8) and u8::from(R) / R::into"]
    registers -.->|"register! implements"| defaults["R::default"]
    defaults -->|"RESET value"| conversion
    registers -.->|"bilge generates"| constructor["R::new<br/>typed field arguments"]
    registers -.->|"bilge generates for all except Configuration"| accessors["pinN / set_pinN<br/>for all ten per-pin structs"]
    configuration -.->|"uses named fields instead of pinN"| configAccessors["field / set_field<br/>unimplemented_bit<br/>interrupt_active_high<br/>interrupt_open_drain<br/>hardware_address_enable<br/>slew_rate_disabled<br/>sequential_operation_disabled<br/>mirror_interrupts<br/>separate_register_banks"]
    inputDirection -->|"getter/setter field type"| direction["Direction enum<br/>From and Into u1"]
    inputPolarity -.-> writable["WritableRegister"]
    inputDirection -.-> writable
    interruptEnable -.-> writable
    interruptDefault -.-> writable
    interruptControl -.-> writable
    pullUps -.-> writable
    gpio -.-> writable
    latch -.-> writable
    writable -->|"extends"| readable
    writable -->|"bound for"| write["MCP23017::write_register"]
    readable -->|"bound for"| read["MCP23017::read_register"]
    configuration -->|"checked hardware write only"| checked["MCP23017::write_configuration"]
```

## Hardware and ownership requirements

- DS20001952D specifies **GPA7 and GPB7 are output-only on MCP23017**.
  Use only pins 0-6 on each port for inputs, pull-ups and interrupt sources.
  `set_direction` rejects `Input` for these two pins. Raw typed-register writes
  remain low-level and must respect this restriction. MCP23S17 has no such limit.
- Start after hardware reset/power-on, with BANK=0; no software reset exists.
  Constructors do not recover arbitrary pre-existing BANK=1 configurations.
  Do not reset or reconfigure the chip outside the driver while it is in use.
- Keep RESET deasserted and address pins externally biased. Respect voltage,
  current, startup/reset timing and bus pull-up requirements from the datasheet.
- SPI initialization writes IOCON=0x08 using both 0x40 and 0x48 startup opcodes
  to cover the A2 silicon erratum. This enables HAEN on **all reset devices
  sharing CS** and resets their IOCON settings; initialize the group before
  customizing IOCON on any member. Other I/O registers are not reset.
- All register transfers are one byte, so either SEQOP setting is supported.
  BANK=1, a set reserved IOCON bit, or disabling SPI HAEN is rejected explicitly.
- Port A is the low byte of sixteen-bit values. A/B access is sequential, not
  atomic; a failure on B can leave A written or its interrupt acknowledged.
- Reading GPIO or INTCAP acknowledges that port's interrupt. Read INTF before
  capture and avoid unrelated GPIO reads while servicing interrupts.
  Output updates read OLAT, not GPIO. Set the latch before changing direction.
- Errors preserve the underlying bus error; there is no silent fallback/retry.
  A failed constructor consumes the supplied interface. Keep a recoverable
  shared-bus handle in the application if reconnection is required.

See [REGISTERS.md](REGISTERS.md) for every address, field and read requirement,
and [FUNCTIONS.md](FUNCTIONS.md) for processing and device operations.

## Device errors

Every fallible device operation, including constructors and builder connections,
returns `DeviceError<I::Error>`, defined in `error.rs` and exported at the crate
root. `Error<E>` remains a short alias for the same enum.

| Variant | Meaning |
|---|---|
| `Interface(error)` | Original HAL interface error, including any device-specific payload owned by that error. |
| `UnsupportedRegisterBank` | IOCON requests BANK=1 instead of the supported BANK=0 map. |
| `ReservedConfigurationBitSet` | IOCON's unimplemented bit is set. |
| `HardwareAddressingDisabled` | MCP23S17 hardware addressing is disabled. |
| `UnsupportedInputPin` | MCP23017 GPA7 or GPB7 was requested as an input. |

Interface failures are forwarded unchanged, not reduced to `ErrorKind`, replaced
with a configuration error, logged and discarded, or converted to success.
Optional `defmt` logging supplements error returns; it never replaces them.
`From<I::Error>` wraps the original value as `DeviceError::Interface(error)`.
When multiple configuration bits are invalid, validation reports BANK first,
then the reserved bit, then disabled SPI addressing, before performing writes.

Code matching the previous `Error::Bus(error)` variant should now match
`DeviceError::Interface(error)`. The former generic `InvalidConfiguration` is
replaced by the three specific configuration variants above.

## Validation

```text
cargo test --manifest-path mcp23017/Cargo.toml
cargo test --manifest-path mcp23017/Cargo.toml --all-features
cargo clippy --manifest-path mcp23017/Cargo.toml --all-targets --all-features -- -D warnings
```

Tests use `embedded-hal-mock` for bus framing, errors, register preservation,
interrupt reads and SPI initialization. No physical hardware validation is
claimed.

## References

- Microchip [MCP23017/MCP23S17 datasheet DS20001952D](https://ww1.microchip.com/downloads/aemDocuments/documents/APID/ProductDocuments/DataSheets/MCP23017-Data-Sheet-DS20001952.pdf),
  especially sections 3.2-3.6.
- Microchip [MCP23S17 revision A errata DS80311A](https://ww1.microchip.com/downloads/aemDocuments/documents/OTH/ProductDocuments/Errata/80311a.pdf).
- Existing Rust [`mcp23017` driver](https://github.com/circuitry-maker/mcp23017)
  and [its register-access implementation](https://docs.rs/mcp23017/latest/src/mcp23017/lib.rs.html)
  consulted as references only. This driver is independently implemented with
  embedded-hal 1.0, typed registers and explicit port-byte ordering.
