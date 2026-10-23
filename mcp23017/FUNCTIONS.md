# MCP23017 / MCP23S17 functions

Pure register-result processing lives in the private `function.rs` module.
`PortValues` derives `Default`; its inputs are exclusively the typed register
read results. The device owns it and performs every bus read before calling it.
All sixteen-bit results use **port A in bits 0-7, port B in bits 8-15**:

$$
V = A + 2^8 B
$$

| Internal function | Device method | Register reads | Post-processing / side effects |
|---|---|---|---|
| `PortValues::combine_input_levels` | `read_ports` | GPIOA, GPIOB | Combine current levels; IPOL already applied by hardware. Each read acknowledges that port's interrupt. |
| `PortValues::combine_captured_levels` | `read_capture` | INTCAPA, INTCAPB | Combine latched interrupt-time levels. Read INTF first; reading capture acknowledges the interrupt. |
| `PortValues::combine_interrupt_flags` | `read_interrupt_flags` | INTFA, INTFB | Combine interrupt-source flags; does not clear interrupts. |
| `PortValues::combine_output_latches` | `read_output_latches` | OLATA, OLATB | Combine output latches without sampling pins or clearing interrupts. |

## Device operations

User arguments belong to the device, not the processing functions. Every
read-modify-write reads hardware, preserving other bits without stale caches.
For a pin index $p$, $M=2^p$. Setting uses $R'=R\mathbin{\vert}M$;
clearing uses $R' = R \land (255 - M)$

| Method | Register / procedure | Requirements |
|---|---|---|
| `new_i2c` | Read IOCON at 0x0A; validate driver invariants. | Hardware reset/power-on BANK=0; I2C address = 0x20 + three-bit address-pin value. No configuration writes. |
| `new_spi` | Broadcast IOCON=0x08 using opcodes 0x40 and 0x48; then read IOCON at the selected address. | Hardware reset/power-on BANK=0. Handles DS80311A A2 erratum; enables HAEN on all reset devices sharing CS. |
| `Builder::connect` | Same initialization as the selected interface constructor. | A `typed-builder` connection specifies `I2cInterface` or `SpiInterface` and its address pins. |
| `read_register<R>` | Read the selected port's typed register. | Known BANK=0. GPIO/INTCAP side effects apply. |
| `write_register<R>` | Write the selected port's typed register. | Only writable types; INTF, INTCAP and IOCON are excluded. GPIO writes change OLAT. |
| `write_configuration` | Validate and write IOCON at 0x0A. | Reject BANK=1, reserved bit=1, and HAEN=0 on SPI before any I/O. Read current IOCON and modify it to preserve unrelated settings. |
| `set_direction` | Read-modify-write IODIR. | Input=1, output=0. Reject Input on MCP23017 GPA7/GPB7 (DS20001952D). Set desired OLAT before enabling output to avoid a transient level. |
| `set_pull_up` | Read-modify-write GPPU. | Pull-ups affect input pins. |
| `set_input_inverted` | Read-modify-write IPOL. | Affects GPIO input reporting, not output latch contents. |
| `set_interrupt_enabled` | Read-modify-write GPINTEN. | Configure input direction, DEFVAL/INTCON, pull-ups, IOCON before enabling. |
| `set_interrupt_comparison` | Read-modify-write INTCON. | True compares against DEFVAL, false against the previous input. |
| `set_interrupt_default` | Read-modify-write DEFVAL. | Relevant when INTCON=1 for that pin. |
| `set_output` | Read-modify-write OLAT. | Never reads GPIO, so input levels cannot overwrite unrelated latches or acknowledge interrupts. |
| `read_pin` | Read GPIO and mask the requested pin. | Acknowledges the whole port's interrupt. |
| `write_outputs` | Write low byte to OLATA, high byte to OLATB. | Not atomic. If B fails, A may already have changed. |
| `release` | Return the owned interface. | Interface `release` returns the underlying HAL bus. No hardware changes. |

All combined reads are two separate transactions and are **not atomic**.
If a second read fails, the first read's acknowledgement may already have
happened; the method returns an error, never a partially successful value.
There are no retries, CRC, arithmetic approximations, async API, software reset,
or per-pin HAL adapters. `SpiDevice` handles CS and bus sharing; an
`embedded-hal-bus` adapter can be supplied by the application.

## Error propagation

All fallible device methods and `Builder::connect` return
`DeviceError<I::Error>`. Every failed interface initialization, read or write is
wrapped as `DeviceError::Interface(original_error)` without discarding or
replacing its payload. Multi-step operations return immediately on failure,
including failures after earlier writes or interrupt acknowledgements.

Configuration validation reports `UnsupportedRegisterBank`,
`ReservedConfigurationBitSet`, or `HardwareAddressingDisabled`, in that order
if several conditions are invalid. Selecting input direction for MCP23017 pin 7
reports `UnsupportedInputPin`. Invalid configuration writes and unsupported
direction requests are rejected before bus I/O. Constructor validation follows
initialization and configuration readback. Optional logging never substitutes
for an error return.
