# MCP23017 / MCP23S17

Allocation-free, `no_std`, blocking `embedded-hal` 1.0 driver for Microchip's
16-bit GPIO expanders. Supports I2C (MCP23017) and SPI (MCP23S17), bilge-typed
registers, direction, pull-ups, polarity inversion, output latches, interrupt
configuration, source flags and captured input states. Optional `defmt` logging
reports initialization, invalid configurations and bus failures.

## I2C

```rust
use embedded_hal::i2c::I2c;
use mcp23017::{u3, Direction, Error, MCP23017, Pin, Port};

fn example<I: I2c>(i2c: I) -> Result<(), Error<I::Error>> {
    // Address pins A2:A0 = 000 => I2C address 0x20.
    let mut device = MCP23017::new_i2c(i2c, u3::new(0))?;
    device.set_output(Port::A, Pin::P0, true)?;
    device.set_direction(Port::A, Pin::P0, Direction::Output)?;
    device.set_pull_up(Port::B, Pin::P7, true)?;
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
use mcp23017::{u3, Error, MCP23S17};

fn example<S: SpiDevice<u8>>(spi: S) -> Result<(), Error<S::Error>> {
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
    register::{Configuration, InputDirection}, u3, Direction, Error, Port,
};

fn example<I: I2c>(i2c: I) -> Result<(), Error<I::Error>> {
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

## Hardware and ownership requirements

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

- Microchip [MCP23017/MCP23S17 datasheet DS20001952C](https://ww1.microchip.com/downloads/en/DeviceDoc/20001952C.pdf),
  especially sections 3.2-3.6.
- Microchip [MCP23S17 revision A errata DS80311A](https://ww1.microchip.com/downloads/aemDocuments/documents/OTH/ProductDocuments/Errata/80311a.pdf).
- Existing Rust [`mcp23017` driver](https://github.com/circuitry-maker/mcp23017)
  and [its register-access implementation](https://docs.rs/mcp23017/latest/src/mcp23017/lib.rs.html)
  consulted as references only. This driver is independently implemented with
  embedded-hal 1.0, typed registers and explicit port-byte ordering.
