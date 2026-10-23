# WS2812B-2020 wire procedure

Source: Worldsemi **WS2812B-2020**, 10-page manufacturer-authored datasheet
[currently hosted by LCSC](https://www.lcsc.com/datasheet/C965555.pdf)
([direct PDF](https://datasheet.lcsc.com/datasheet/pdf/fa8a6e6f798ba5cc95814978f8bc7219.pdf?productCode=C965555)),
pages 4-5, checked 2026-10-23. The PDF has no printed revision/date identifier.
The manufacturer's current catalog lists newer WS2812B variants, but its legacy
download URLs were unavailable; this is the latest accessible datasheet located
for the **2020 package**, not a claim that the entire WS2812 family is identical.
No separate public errata was located. The page-5 bit-order diagram accidentally
omits G2; the accompanying text and 24-bit requirement specify all eight bits.

There is no register interface, readback, acknowledgement, address, CRC or
software-detectable LED count. `REGISTERS.md` and `bilge` are not applicable.

## Timing

| Signal | Datasheet requirement | Four SPI bits at 3.2 MHz |
|---|---|---|
| 0 high | 220-380 ns | 312.5 ns |
| 0 low | 580-1000 ns | 937.5 ns |
| 1 high | 580-1000 ns | 625 ns |
| 1 low | 580-1000 ns | 625 ns |
| Bit period | At least 1.25 us | 1.25 us |
| Reset/latch low | Strictly greater than 280 us | 320 us (128 zero bytes) |

A zero bit is encoded as `1000`, a one as `1100`. At the nominal 3.2 MHz, this
is 800 kbit/s on DIN. The actual SPI clock must be **3.0-3.2 MHz inclusive**:
below 3 MHz the zero's low time exceeds 1 us; above 3.2 MHz the total bit time
is shorter than 1.25 us. Prefer a clock with tolerance/headroom inside that
range. `new` assumes nominal 3.2 MHz; `try_new` checks the caller-reported clock
against these bounds. Neither can configure or measure the HAL's clock.

1. Configure MSB-first, eight-bit SPI mode 0; route MOSI to DIN. SCK is not
   connected to the LEDs. CS must not gate the LED data line.
2. Wait for stable LED power according to the board/power design. The constructor
   does no I/O. Before the first frame, transmit 128 zero bytes, holding DIN low.
3. Encode every pixel into 12 bytes: green, red, blue; bit 7 before bit 0.
   Pack the earlier symbol into each byte's high nibble.
4. Transmit the entire encoded pixel slice in **one** SPI write operation.
   There are no per-pixel transactions or software delays inside this payload.
5. Transmit another 128 zero bytes to latch the colors before returning success.
   The prefix, payload and suffix are one `SpiDevice` transaction.

SPI HALs must not add out-of-spec pauses inside the payload or drive MOSI high
between operations. `embedded-hal` does not promise these electrical properties;
verify on a logic analyzer, including under interrupts, DMA and executor load.
The HAL must finish/flush the transaction before reporting success. Prefix/suffix
operation boundaries may pause only while MOSI remains low.

## Recovery and edge cases

An empty color array/slice is a no-op with no bus access. `latch` explicitly sends
128 zero bytes when only a reset/latch is wanted. A buffered frame requires at
least 12 bytes per pixel; insufficient space or arithmetic overflow is rejected
before modifying the scratch buffer or touching the bus. Unused scratch bytes
are preserved and not sent.

A failed transaction may already have changed a prefix of the chain or latched
a partial frame. The driver forwards the original transport error and does not
attempt cleanup that could hide it. Every subsequent nonempty frame starts with
a new reset prefix; after restoring the peripheral, resend the **complete**
chain. Async cancellation has the same partial-frame risk. `latch` can latch
incomplete data; it does not clear the chain to black.

Send one color for every physical pixel when replacing a whole chain. Omitted
pixels are not automatically cleared; `write_color` addresses the first pixel,
not an append operation. Reset pulses separate frames rather than advance to
the next pixel.
