# MAX31865 registers

Source: Analog Devices / Maxim **19-6478, Rev 3, 7/15**, pages 13-17,
[official PDF](https://www.analog.com/media/en/technical-documentation/data-sheets/MAX31865.pdf),
[verified manufacturer-authored mirror](https://cdn-learn.adafruit.com/assets/assets/000/037/309/original/MAX31865.pdf).
Rev 3 is the latest publicly located revision as checked 2026-10-23; the official
download was unavailable from this environment, so the mirror was read directly.
No separate public errata was located. Rev 3 specifically updates Table 6
(threshold fields). AAP (SSOP) and ATP (TQFN-EP), including +T reel variants,
share the register interface; board wiring/pinouts differ.

Hexadecimal addresses below are read addresses; writes set address bit 7.
Multibyte values are big-endian, MSB first. SPI supports modes 1 and 3.

| Read / write | Register | POR | Access | Read / write requirements and side effects |
|---|---|---|---|---|
| 00 / 80 | Configuration | 00 | R/W | Read before modifying stable settings; do not replay one-shot, fault-cycle or fault-clear command bits |
| 01 / -- | RTD resistance MSB | 00 | R | Burst-read 01-02 after conversion; bits 14:7 of the 15-bit ratio |
| 02 / -- | RTD resistance LSB | 00 | R | Bits 7:1 contain ratio bits 6:0; bit 0 reports a fault; reading RTD data acknowledges DRDY |
| 03 / 83 | High threshold MSB | FF | R/W | No prior read for full replacement; upper byte of raw threshold word |
| 04 / 84 | High threshold LSB | FF | R/W | Bits 7:1 carry threshold bits 6:0; bit 0 is don't-care |
| 05 / 85 | Low threshold MSB | 00 | R/W | No prior read for full replacement; upper byte of raw threshold word |
| 06 / 86 | Low threshold LSB | 00 | R/W | Bits 7:1 carry threshold bits 6:0; bit 0 is don't-care |
| 07 / -- | Fault status | 00 | R | Latched bits, not clear-on-read; acknowledge using Configuration.D1 |

No other register addresses are defined. Threshold byte reads have no documented
side effects. Threshold write pairs and the legacy four-write update are not
atomic; partial failure can leave mixed old/new thresholds.

## Configuration fields

| Bits | Meaning | Requirements |
|---|---|---|
| 7 | Bias enabled | Enable before conversion; account for external RC settling |
| 6 | Automatic conversion | 0 = normally off, 1 = continuous; continuous mode keeps bias on |
| 5 | One-shot command | Self-clearing; valid in normally-off mode; conversion starts when CS rises |
| 4 | Three-wire compensation | 1 = three-wire; 0 = two- or four-wire, with matching physical wiring |
| 3:2 | Fault cycle | 00 none/finished; 01 automatic; 10 manual first stage; 11 manual finish |
| 1 | Clear fault status | Self-clearing; write only with bits 5, 3, 2 clear |
| 0 | Notch filter | 0 = 60 Hz, 1 = 50 Hz; do not change while automatic conversion is active |

Read-modify-write preserves stable bits (7, 6, 4, 0), clears transient commands,
then applies the requested operation. Changing notch frequency while automatic
mode is active stops automatic mode, changes the filter, then restores automatic
mode. A failure is reported immediately, without rollback.

## RTD and threshold words

`RtdResistance` is a 16-bit `bilge` struct with a `u15` count and a fault bit.
`FaultThreshold` uses the same layout, with don't-care bit 0 instead of a fault.
The legacy `set_threshold` and getters continue to take/return **raw shifted
16-bit words**, not unshifted `read_rtd()` counts. For example, ADC count 8192
corresponds to raw threshold 0x4000. The default upper threshold remains FFFF.

`Register` remains a compatibility bitflag API for existing low-level code.
Its historic address/mask mixing is not used for internal register processing;
new code should use `Address` and typed register structures. Raw access traits
can bypass the high-level sequencing guarantees.

## Fault status fields

| Bit | Fault |
|---|---|
| 7 | RTD count at or above high threshold |
| 6 | RTD count at or below low threshold |
| 5 | REFIN- greater than 0.85 times bias |
| 4 | REFIN- less than 0.85 times bias with FORCE- open |
| 3 | RTDIN- less than 0.85 times bias with FORCE- open |
| 2 | Overvoltage / undervoltage |
| 1:0 | Don't-care; ignored by fault evaluation |

Multiple simultaneous faults are retained in a typed status value, never treated
as success. The RTD fault flag without a specific status code is also an error.
