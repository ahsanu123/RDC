# TMC2209 communication and operation procedures

Reference: bundled manufacturer datasheet rev. 1.09, sections 4, 6, 9, 14, 16-18.
Communication is in `comproc.rs`; CRC and reply validation are in `validation.rs`.
The device methods orchestrate these steps, so a separate `procedure.rs` would
duplicate the same logic and is intentionally not added.

## Wiring and UART

1. Configure a real, non-inverted UART for **8 data bits, no parity, one stop bit**.
   The settings struct describes an already-configured peripheral; it cannot
   change peripheral baud/pin muxing. Use matching VCC_IO logic levels and GND.
1. Connect PDN_UART using the manufacturer's circuit. With TX through the
   recommended series resistor and RX on the shared line, enable echo handling.
   An isolated/half-duplex adapter which suppresses TX echo needs `EchoMode::Disabled`
   and must handle direction/turnaround itself. A bare tied push-pull TX/RX is
   not an acceptable substitute.
1. Bias MS1/MS2 to select address 0-3; ensure every node sharing the bus has a
   unique address. There is only UART register access, not an SPI/I2C protocol.
1. Baud must be >=9000 and <=fCLK/16. Default settings assume 115200 baud and
   the nominal 12 MHz internal oscillator. Match the real clock, including any
   external clock or deliberate factory-trim changes.
1. Exclusively own the physical bus for **entire** operations, not merely bytes.
   To share it, move/borrow the UART between device lifetimes or provide an
   external transaction-level lock. Do not interleave IFCNT-verified writes.

The transport waits at least 16 bit times before each datagram. TX sends bytes
in order, consumes each expected echo before sending the next byte (avoiding
small RX FIFO overrun), then flushes through the last stop bit. Read replies
must have sync 0x05, master address 0xFF, the requested register address and a
valid CRC; data is big-endian.

Every `WouldBlock` loop has a per-operation polling budget and a delay between
attempts. Defaults allow up to 10,000 attempts separated by 10 us (about 100 ms,
plus HAL execution time) per byte/flush. Set a budget long enough for the UART
byte time and maximum SENDDELAY (up to 120 bit times). The HAL must be genuinely
nonblocking; scheduling pauses can still break the device's 63-bit inter-byte
limit and will be surfaced as a failed transaction, never successful fallback.

## Initialization and ordinary writes

1. Keep hardware ENN high and STEP inactive while selecting safe current and
   chopper settings. The chip can energize its motor at power-on if ENN is low,
   independently of this driver.
1. Connect using `new` or the typed builder. Identity reads do not reset or
   configure the motor. No software reset exists.
1. Call `configure_uart`; this disables the PDN pin's standstill function.
   Use reply delay >=2 for all nodes on a multi-node bus.
1. Set complete current settings and desired microstep resolution. Read existing
   readable configuration before changing individual fields; write-only
   registers require complete typed values rather than zero-based shadow updates.
1. Every ordinary write is bracketed by IFCNT reads. A write error can mean
   the device accepted the write but the echo, flush or counter read failed.
   Do not automatically retry; inspect hardware/counter and application state.
1. ENN handling, STEP pulses, safe acceleration, current/temperature limits,
   StealthChop tuning and appropriate stall calibration remain application work.
   Switching modes may require standstill. Never treat open-load flags as
   definitive in operating conditions excluded by the datasheet.

Multi-register changes are not atomic. An error on a later command does not
roll back earlier current, chopper or interface changes. No `Drop` action disables
the motor, and releasing resources does not stop motion.

## Error recovery

After an incomplete frame, echo mismatch or invalid reply, do not assume RX
contains the start of a new reply. `recover_uart` flushes TX, waits 80 bit times
(more than the 63-bit communication reset plus 12-bit idle recovery), and drains
queued bytes until RX would block. It returns the number discarded, forwards
real UART errors and reports a bounded-drain failure instead of spinning forever.
Only perform this while exclusively owning the bus. Retry decisions belong to
the application; uncertain writes require state reconciliation.

## OTP programming

`program_otp_bit` is intentionally separate from ordinary typed writes.
Programming only sets bits to one and cannot erase them. It is never part of
connection, initialization or tests against real hardware.

1. Read all of manufacturer sections 5.1.1 and 16 before selecting a bit.
   Validate power, current, wiring and calibration requirements on real hardware;
   OTP is not a replacement for ordinary runtime setup.
1. Select byte 0, 1 or 2 and bit 0-7, checking the dual PWM/chopper meanings in
   OTP_READ. Factory oscillator trim bits must not be changed casually.
1. The driver checks that the selected bit is clear, records IFCNT and sends
   the OTP_PROG command with magic 0xBD.
1. Wait at least 10 ms, check the successful-write counter, read address 0x04
   to refresh OTP and confirm the selected bit in OTP_READ.
1. On failure the driver returns a specific error or the original interface
   error. It does not retry irreversible operations, including the datasheet's
   optional longer retry procedure. A failed readback is not proof the bit
   remained unprogrammed; inspect before deciding any further action.
