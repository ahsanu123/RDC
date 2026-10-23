# MCP23017 / MCP23S17 registers

Source: Microchip [DS20001952D](https://ww1.microchip.com/downloads/aemDocuments/documents/APID/ProductDocuments/DataSheets/MCP23017-Data-Sheet-DS20001952.pdf),
sections 3.4-3.6. The driver uses **BANK=0** exclusively. All registers are eight
bits; pin fields are ordered pin0 (LSB) through pin7 (MSB). A and B share the same
Rust type but are selected with `Port`. `Configuration` is one global register with two
aliases. This table lists all 22 addresses, not just writable registers.

Revision D specifies that **MCP23017 GPA7/GPB7 are output-only**, despite their
input-direction reset bits. Do not attach input signals to those pins. This
restriction does not apply to the SPI MCP23S17. Raw register access exposes the
actual register layout and does not hide those bits.

| Datasheet name | Descriptive Rust type | Datasheet section |
|---|---|---|
| IODIR | `InputDirection` | 3.5.1 |
| IPOL | `InputPolarity` | 3.5.2 |
| GPINTEN | `InterruptEnable` | 3.5.3 |
| DEFVAL | `InterruptDefaultValue` | 3.5.4 |
| INTCON | `InterruptControl` | 3.5.5 |
| IOCON | `Configuration` | 3.5.6 |
| GPPU | `PullUpResistors` | 3.5.7 |
| INTF | `InterruptFlags` | 3.5.8 |
| INTCAP | `InterruptCapture` | 3.5.9 |
| GPIO | `GeneralPurposeInputOutput` | 3.5.10 |
| OLAT | `OutputLatch` | 3.5.11 |

| Register | BANK=0 A / B | BANK=1 A / B (reference only) | Access | Reset | Fields and read requirements / side effects |
|---|---|---|---|---|---|
| IODIR | 0x00 / 0x01 | 0x00 / 0x10 | R/W | 0xFF | Eight `Direction` fields: 1=input, 0=output. Read anytime; no side effects. |
| IPOL | 0x02 / 0x03 | 0x01 / 0x11 | R/W | 0x00 | Eight inversion bits, affecting inputs in GPIO. Read anytime; no side effects. |
| GPINTEN | 0x04 / 0x05 | 0x02 / 0x12 | R/W | 0x00 | Eight interrupt-enable bits. Read anytime; inputs must be configured before enabling interrupts. |
| DEFVAL | 0x06 / 0x07 | 0x03 / 0x13 | R/W | 0x00 | Eight interrupt comparison levels, used when INTCON bit is 1. Read anytime; no side effects. |
| INTCON | 0x08 / 0x09 | 0x04 / 0x14 | R/W | 0x00 | Eight comparison-mode bits: 1=DEFVAL, 0=previous input. Read anytime; no side effects. |
| IOCON | 0x0A / 0x0B | 0x05 / 0x15 | R/W | 0x00 | Global control (below). Read either alias in the known bank; no side effects. |
| GPPU | 0x0C / 0x0D | 0x06 / 0x16 | R/W | 0x00 | Eight weak-pull-up enable bits for input pins. Read anytime; no side effects. |
| INTF | 0x0E / 0x0F | 0x07 / 0x17 | R | 0x00 | Eight interrupt-source flags. Read before INTCAP/GPIO when servicing interrupts; reading INTF does not clear them. |
| INTCAP | 0x10 / 0x11 | 0x08 / 0x18 | R | 0x00 | Eight captured levels, meaningful after an interrupt. Reading acknowledges that port's interrupt; capture is held until GPIO or INTCAP is read. |
| GPIO | 0x12 / 0x13 | 0x09 / 0x19 | R/W | 0x00 | Eight current levels (subject to IPOL). Reading acknowledges the port interrupt and can discard its capture. Writes update OLAT. Actual read levels depend on external signals. |
| OLAT | 0x14 / 0x15 | 0x0A / 0x1A | R/W | 0x00 | Eight output-latch bits. Read anytime without acknowledging interrupts; use for output read-modify-write, not GPIO. |

## IOCON fields

| Bit | Typed field | Meaning |
|---|---|---|
| 7 | `separate_register_banks` | 0=interleaved map; 1=separate banks (rejected by driver). |
| 6 | `mirror_interrupts` | OR both interrupt sources onto both INT pins. |
| 5 | `sequential_operation_disabled` | Disable address auto-increment. Both settings work with single-byte transfers. |
| 4 | `slew_rate_disabled` | Disable I2C SDA slew-rate control. |
| 3 | `hardware_address_enable` | Enable SPI address pins. Must remain enabled on MCP23S17; not applicable to MCP23017, whose address pins are always enabled. |
| 2 | `interrupt_open_drain` | Open-drain INT outputs; overrides polarity. |
| 1 | `interrupt_active_high` | Active-high push-pull INT outputs when open-drain is disabled. |
| 0 | `unimplemented_bit` | Unimplemented, reads zero. Driver rejects writes of one. |

All types are bilge bitfields; `InputDirection` resets to 0xFF and all other register types
reset to 0x00. `InterruptFlags` and `InterruptCapture` cannot be written. `Configuration` is writable only
through `write_configuration`, which checks BANK, reserved bits and SPI HAEN.
No driver method changes BANK. A hardware reset/power-on and exclusive ownership
of the register configuration are prerequisites; there is no software reset.
