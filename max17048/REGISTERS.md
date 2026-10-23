# MAX17048 registers

Source: Maxim MAX17048/MAX17049, document 19-6171, **Rev. 7, November
2016**, pages 10-14, 17-18. [Manufacturer URL][datasheet],
[retrieved manufacturer PDF mirror][mirror]. The ADI product, datasheet,
and errata endpoints were inaccessible during implementation; a newer revision
or separate errata could not be verified. Recheck these before production.
Rev. 1 corrected byte ordering; Rev. 2 corrected swapped hibernate names.

Address is the first byte of a **big-endian 16-bit word**. Read both bytes in
one repeated-START transaction; write both bytes in one transaction. Single-byte
writes have no effect, except the documented TABLE lock byte sequence.
All ordinary reads have **no clearing or other side effects**. Undefined bits
must not be interpreted. The driver preserves them when updating registers.

| Address | Datasheet / Rust type | Bits (15 down to 0) | Access | Reset | Read requirement before writing |
|---|---|---|---|---|---|
| 0x02 | VCELL / `CellVoltage` | unsigned voltage count | R | measurement | Not writable |
| 0x04 | SOC / `StateOfCharge` | unsigned Q8.8 percent | R | calculated | Not writable |
| 0x06 | MODE / `Mode` | X:15, QuickStart:14, EnSleep:13, HibStat:12, X:11..0 | commands R/W; HibStat R | 0x0000 | Read-modify-write; preserve EnSleep for quick-start; never replay QuickStart |
| 0x08 | VERSION / `Version` | production version | R | 0x001_ | Not writable |
| 0x0A | HIBRT / `HibernateThreshold` | HibThr:15..8, ActThr:7..0 | R/W | 0x8030 | Read-modify-write to preserve the other threshold |
| 0x0C | CONFIG / `Configuration` | RCOMP:15..8, SLEEP:7, ALSC:6, ALRT:5, ATHD:4..0 | R/W; ALRT write-zero-clear | 0x971C | Read-modify-write; preserve unrelated fields and pending ALRT |
| 0x14 | VALRT / `VoltageAlert` | MIN:15..8, MAX:7..0 | R/W | 0x00FF | Read before replacing both limits; one word write |
| 0x16 | CRATE / `ChargeRate` | signed two's-complement rate | R | calculated | Not writable |
| 0x18 | VRESET/ID / `ResetVoltage` | threshold:15..9, Dis:8, ID:7..0 | upper byte R/W, ID R (factory OTP) | 0x96__ | Read-modify-write; preserve ID and comparator configuration |
| 0x1A | STATUS / `Status` | X:15, EnVR:14, SC:13, HD:12, VR:11, VL:10, VH:9, RI:8, X:7..0 | EnVR R/W; flags write-zero-clear | 0x01__ | Read-modify-write; write ones to flags not being cleared |
| 0x3E / 0x3F | TABLE access / `ModelAccess` | unlock 0x4A57; lock 0x0000 | W control | locked | No read; unlock writes 0x57 to 0x3F, then 0x4A to 0x3E; lock writes zero in the same order |
| 0x40, 0x42, ..., 0x7E | TABLE / `ModelTableWord` | 32 opaque 16-bit model words (64 bytes total) | W, unlocked only | factory model | No read; obtain validated model and programming instructions from manufacturer |
| 0xFE | CMD / `Command` | POR command 0x5400 | R/W command | 0xFFFF | No read; reset takes effect at final bit and does **not ACK** |

The TABLE address range includes every byte 0x40..0x7F. Its private field
semantics and full model qualification procedure are not published; the driver
only provides the documented raw transfer and relocking, not model generation
or a claim of battery-model validation.

MAX17048 is for **one cell**: VCELL count = 78.125 uV. MAX17049 uses two
cells and twice the VCELL scale; this crate intentionally implements MAX17048
only. Both share this map, but are not interchangeable at the voltage API.
HIBRT ActThr is 1.25 mV/count; HibThr and CRATE are 0.208 %/hour/count.
VALRT is 20 mV/count. VRESET is 40 mV/count in bits 15..9.

[datasheet]: https://www.analog.com/media/en/technical-documentation/data-sheets/max17048-max17049.pdf
[mirror]: https://cdn-learn.adafruit.com/assets/assets/000/114/607/original/MAX17048-MAX17049.pdf
