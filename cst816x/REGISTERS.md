# CST816S register reference

Source: [`docs/CST816S_register_declaration.pdf`](docs/CST816S_register_declaration.pdf),
pages 1-5. The supplied document describes **CST816S**; it does not establish
compatibility with other CST816x variants. No document revision is stated.

The canonical typed modules are `src/register.rs` and `src/register/`.
`registers` remains a compatibility re-export. See [README.md](README.md) for
source authority, framing evidence, supported methods and unresolved limits.

## Registers

The table lists all 32 registers declared in the PDF, ordered by address. Each
register is one byte. Bit ranges below identify only documented fields; other
bits are unspecified, not confirmed reserved.

**Access and read requirements:** Hardware access permissions, read prerequisites
and read side effects are unknown for every address: the declaration supplies
fields, not an access matrix. `R` / `R/W` below is the driver's exposed access,
not a manufacturer's permission guarantee. Configuration writes are based on
the declaration's enable/setting descriptions. BPC and raw reference writes
are not exposed because their write permission/purpose is not established.
Use a ready, powered device; IRQ is the manufacturer's notification to read
valid touch data, but it is not a documented latch or read-clear mechanism.

**Write prerequisites:** No prerequisite read is specified by the manufacturer.
The driver reads first for `0xEC`, `0xFA`, and `0xFD` solely to preserve
unspecified bits; it overwrites all documented fields from the supplied value.
All other supported writes replace a complete byte without a preceding read,
after range validation. See README §Shared bus and §Partial failures.

**Defaults and byte order:** Values below are documented defaults, not guaranteed
reset values; the PDF does not separately specify reset values. For paired values,
the lower address holds the high byte and the next address holds the low byte.
The PDF does not specify whether multi-byte reads are atomic.

`Byte` means one byte, no endianness. `BE high/low` means high byte at the
lower address. Unknown side effects/prerequisites are deliberately not "none".

| Address | Register / fields / meaning | Default | Reset | Hardware access / driver access | Byte order | Read side effects | Read prerequisites | Declaration page |
|---------|-----------------------------|---------|-------|---------------------------------|------------|-------------------|--------------------|------------------|
| `0x01` | GestureID `[7:0]`, gesture table below | Unknown | Unknown | Unknown / R | Byte | Unknown | Unknown | 1 |
| `0x02` | FingerNum `[7:0]`: 0 no finger, 1 one finger | Unknown | Unknown | Unknown / R | Byte | Unknown | Unknown | 1 |
| `0x03` | XposH `[3:0]`: X[11:8], high nibble unspecified | Unknown | Unknown | Unknown / R | BE high | Unknown | Unknown | 1 |
| `0x04` | XposL `[7:0]`: X[7:0] | Unknown | Unknown | Unknown / R | BE low | Unknown | Unknown | 1 |
| `0x05` | YposH `[3:0]`: Y[11:8], high nibble unspecified | Unknown | Unknown | Unknown / R | BE high | Unknown | Unknown | 1 |
| `0x06` | YposL `[7:0]`: Y[7:0] | Unknown | Unknown | Unknown / R | BE low | Unknown | Unknown | 1 |
| `0xA7` | ChipID `[7:0]`; expected ID unspecified | Unknown | Unknown | Unknown / R | Byte | Unknown | Unknown | 2 |
| `0xA8` | ProjID `[7:0]`, project ID | Unknown | Unknown | Unknown / R | Byte | Unknown | Unknown | 2 |
| `0xA9` | FwVersion `[7:0]`, software version | Unknown | Unknown | Unknown / R | Byte | Unknown | Unknown | 2 |
| `0xB0` | BPC0H `[7:0]`, meaning unknown | Unknown | Unknown | Unknown / R | BE high | Unknown | Unknown | 1-2 |
| `0xB1` | BPC0L `[7:0]`, meaning unknown | Unknown | Unknown | Unknown / R | BE low | Unknown | Unknown | 2 |
| `0xB2` | BPC1H `[7:0]`, meaning unknown | Unknown | Unknown | Unknown / R | BE high | Unknown | Unknown | 2 |
| `0xB3` | BPC1L `[7:0]`, meaning unknown | Unknown | Unknown | Unknown / R | BE low | Unknown | Unknown | 2 |
| `0xEC` | MotionMask `[2] EnConLR`, `[1] EnConUD`, `[0] EnDClick`: continuous LR, UD, double-click enable | Unknown | Unknown | Unknown / R/W | Byte | Unknown | Unknown | 2 |
| `0xED` | IrqPluseWidth `[7:0]`: low-pulse width, 0.1 ms units, 1..=200 | 10 (1 ms) | Unknown | Unknown / R/W | Byte | Unknown | Unknown | 3 |
| `0xEE` | NorScanPer `[7:0]`: normal scan period, 10 ms units, 1..=30; affects auto-wake/sleep, coupling unknown | 1 (10 ms) | Unknown | Unknown / R/W | Byte | Unknown | Unknown | 3 |
| `0xEF` | MotionSlAngle `[7:0]`: tan(c) × 10, angle units/rounding unknown | Unknown | Unknown | Unknown / R/W | Byte | Unknown | Unknown | 3 |
| `0xF0` | LpScanRaw1H `[7:0]`, channel 1 reference | Unknown | Unknown | Unknown / R | BE high | Unknown | Unknown | 3 |
| `0xF1` | LpScanRaw1L `[7:0]`, channel 1 reference | Unknown | Unknown | Unknown / R | BE low | Unknown | Unknown | 3 |
| `0xF2` | LpScanRaw2H `[7:0]`, channel 2 reference | Unknown | Unknown | Unknown / R | BE high | Unknown | Unknown | 3 |
| `0xF3` | LpScanRaw2L `[7:0]`, channel 2 reference | Unknown | Unknown | Unknown / R | BE low | Unknown | Unknown | 3 |
| `0xF4` | LpAutoWakeTime `[7:0]`: low-power recalibration, 1 minute units, 1..=5 | 5 (5 minutes) | Unknown | Unknown / R/W | Byte | Unknown | Unknown | 3-4 |
| `0xF5` | LpScanTH `[7:0]`: wake threshold, 1..=255, smaller is more sensitive | 48 | Unknown | Unknown / R/W | Byte | Unknown | Unknown | 4 |
| `0xF6` | LpScanWin `[7:0]`: measurement range, 0..=3, larger increases sensitivity and power | 3 | Unknown | Unknown / R/W | Byte | Unknown | Unknown | 4 |
| `0xF7` | LpScanFreq `[7:0]`: frequency setting, 1..=255, smaller is more sensitive; unit unknown | 7 | Unknown | Unknown / R/W | Byte | Unknown | Unknown | 4 |
| `0xF8` | LpScanIdac `[7:0]`: current setting, 1..=255, smaller is more sensitive; unit unknown | Unknown | Unknown | Unknown / R/W | Byte | Unknown | Unknown | 4 |
| `0xF9` | AutoSleepTime `[7:0]`: idle time to low-power mode, seconds | 2 seconds | Unknown | Unknown / R/W | Byte | Unknown | Unknown | 4 |
| `0xFA` | IrqCtl: provisional `[7] EnTest`, `[6] EnTouch`, `[5] EnChange`, `[4] EnMotion`, `[0] OnceWLP`; conflict below | Unknown | Unknown | Unknown / R/W | Byte | Unknown | Unknown | 4-5 |
| `0xFB` | AutoReset `[7:0] DebounceTime`: reset after touch without gesture, seconds; 0 disables | Unknown (blank) | Unknown | Unknown / R/W | Byte | Unknown | Unknown | 5 |
| `0xFC` | LongPressTime `[7:0]`: reset after long press, seconds; 0 disables | 10 seconds | Unknown | Unknown / R/W | Byte | Unknown | Unknown | 5 |
| `0xFD` | IOCtl `[2] SOFT_RST`, `[1] IIC_OD`, `[0] En1v8`, details below | Partial: IIC_OD=0, En1v8=0; SOFT_RST unknown | Unknown | Unknown / R/W | Byte | Unknown | Unknown | 5 |
| `0xFE` | DisAutoSleep `[7:0]`: 0 enables auto-sleep, any nonzero disables | 0 | Unknown | Unknown / R/W | Byte | Unknown | Unknown | 5 |

### Gesture IDs (`0x01`)

| Value | Gesture |
|-------|---------|
| `0x00` | No gesture |
| `0x01` | Slide up |
| `0x02` | Slide down |
| `0x03` | Slide left |
| `0x04` | Slide right |
| `0x05` | Single click |
| `0x0B` | Double click |
| `0x0C` | Long press |

### Interrupt control (`0xFA`)

| Bit | Field | Meaning when enabled |
|-----|-------|----------------------|
| 7 | `EnTest` | Periodically generate low pulses on the interrupt pin for testing. |
| 6 | `EnTouch` | Generate low pulses when touch is detected. |
| 5 | `EnChange` | Generate low pulses when touch changes. |
| 4 | `EnMotion` | Generate low pulses when a gesture is detected. |
| 0 | `OnceWLP` | Generate only one low pulse for a long-press gesture. |

**Source inconsistency:** The `IrqCtl` bit-layout drawing on page 4 places
`OnceWLP` under bit 4 and omits `EnMotion`. The field descriptions on pages 4-5
instead assign `EnMotion` to bit 4 and `OnceWLP` to bit 0. The tables above follow
the field descriptions, also corroborated independently by InfiniTime. The
newest located manufacturer datasheet V1.3 has a blank register-specification
section and does not resolve the conflict. Verify this provisional mapping
against your module/firmware before configuring hardware.

### I/O control (`0xFD`)

| Bit | Field | Value `0` | Value `1` | Documented default |
|-----|-------|-----------|-----------|--------------------|
| 2 | `SOFT_RST` | Disable soft reset through IRQ. | Enable soft reset by having the main controller pull IRQ low. | Not stated |
| 1 | `IIC_OD` | I2C pin pull-up resistor mode. | I2C pin open-drain mode. | `0` |
| 0 | `En1v8` | I2C and IRQ levels follow VDD. | I2C and IRQ levels use 1.8 V. | `0` |
