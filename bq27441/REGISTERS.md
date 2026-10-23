# BQ27441-G1 registers

Primary specifications: TI **SLUSBH1C**, November 2013, revised December 2014,
and **SLUUAC9A**, December 2013, revised May 2015. These are the current revisions
linked from TI's product page when checked on 2026-10-23. The product-page date
for the TRM differs from the revision printed in the PDF; this document uses the
PDF revision. No separate silicon errata document is listed on that page.
Follow the datasheet communication restrictions even where the TRM examples use
multi-byte writes.

## Reading and writing

Fixed **7-bit I2C address 0x55**, not the 8-bit wire addresses 0xAA/0xAB.
Two-byte command values are **little-endian**, including Control responses.
Data-memory integers inside BlockData are **big-endian**. Reserved addresses
must not be accessed. No listed read acknowledges interrupts or clears flags;
flags change with gauge state, not simply because they are read.

All reads require valid power, startup communication delay (at least 250 ms),
bus pull-ups, and a controller supporting clock stretching. Each packet needs
at least 66 us bus-free time. Above 100 kHz each write must contain **one data
byte**; incremental reads remain supported. The driver always uses byte writes
and inserts the bus-free delay, including after failed transactions.

The datasheet section 9.5.4.3 limits each standard command to twice per second
and requires at least 2 s for read/write command results to update. The driver
conservatively delays 500 ms after every standard-word read and 2 s after
Control or Temperature writes. Do not bypass this pacing with another handle.

## Standard commands and extended word mirrors

`dynamic` means the reset value is not specified as a fixed measurement; it is
not a claim that the reset value is zero. All rows have no read-clear side
effect. Each type has its own source file under `src/register/`.

| Address | Rust register / TI command | Access | Reset | Field / read requirements |
|---|---|---|---|---|
| 00-01 | Control | R/W | Selected response | Write a subcommand before reading; no arbitrary read through `read_register` |
| 02-03 | Temperature | R/W | Dynamic | u16, 0.1 K; writes used only with OpConfig.TEMPS=1 |
| 04-05 | Voltage | R | Dynamic | u16 mV, 0-6000 |
| 06-07 | Flags | R | Dynamic; CFGUPMODE=0 | Read to poll mode; battery predictions require BAT_DET=1 |
| 08-09 | NominalAvailableCapacity | R | Dynamic | u16 mAh, uncompensated remaining capacity |
| 0A-0B | FullAvailableCapacity | R | Dynamic | u16 mAh, uncompensated full capacity |
| 0C-0D | RemainingCapacity | R | Dynamic | u16 mAh, filtered if OpConfigB.SMOOTHEN=1 |
| 0E-0F | FullChargeCapacity | R | Dynamic | u16 mAh, filtered if OpConfigB.SMOOTHEN=1 |
| 10-11 | AverageCurrent | R | Dynamic | i16 mA, two's complement; negative discharging |
| 12-13 | StandbyCurrent | R | Initial Standby (-3 mA default) | i16 mA, adaptive |
| 14-15 | MaximumLoadCurrent / MaxLoadCurrent | R | Initial MaxLoad (-200 mA default) | i16 mA, adaptive |
| 18-19 | AveragePower | R | Dynamic | i16 mW, negative discharging |
| 1C-1D | StateOfCharge | R | Dynamic | u16 percent, 0-100 |
| 1E-1F | InternalTemperature | R | Dynamic | u16, 0.1 K, always internal sensor |
| 20-21 | StateOfHealth | R | Dynamic | Low byte percent; high byte validity status |
| 28-29 | RemainingCapacityUnfiltered | R | Dynamic | u16 mAh |
| 2A-2B | RemainingCapacityFiltered | R | Dynamic | u16 mAh |
| 2C-2D | FullChargeCapacityUnfiltered | R | Dynamic | u16 mAh |
| 2E-2F | FullChargeCapacityFiltered | R | Dynamic | u16 mAh |
| 30-31 | StateOfChargeUnfiltered | R | Dynamic | u16 percent; TRM table 4-1's mAh label is a typo (see section 4.20) |
| 3A-3B | OperationConfiguration / OpConfig | R | 0x25F8 | Read-only mirror; modify via subclass 64, offset 0 |
| 3C-3D | DesignCapacity | R | G1A 1340, G1B 1000 mAh | Read-only mirror; modify via subclass 82, offset 10 |

Unlisted gaps in 00-3D are reserved, not readable registers in this API.

### Typed bit fields

| Register | Bits (least significant first) |
|---|---|
| Flags | 0 DSG, 1 SOCF, 2 SOC1, 3 BAT_DET, 4 CFGUPMODE, 5 ITPOR, 6 reserved, 7 OCVTAKEN, 8 CHG, 9 FC, 10-13 reserved (`u4`), 14 UT, 15 OT |
| ControlStatus | 0 reserved, 1 VOK, 2 RUP_DIS, 3 LDMD, 4 SLEEP, 5 reserved, 6 HIBERNATE, 7 INITCOMP, 8 RES_UP, 9 QMAX_UP, 10 BCA, 11 CCA, 12 CALMODE, 13 SS, 14 WDRESET, 15 SHUTDOWNEN |
| OperationConfiguration | 0 TEMPS, 1 reserved=0, 2 BATLOWEN, 3 reserved=1, 4 RMFCC, 5 SLEEP, 6-8 reserved=1, 9 reserved=0, 10 reserved=1, 11 GPIOPOL, 12 BI_PU_EN, 13 BIE, 14-15 reserved=0 |
| StateOfHealth | 0-7 percentage, 8-15 HealthStatus: 0 invalid, 1 instant, 2 initial/default Qmax, 3 learned Qmax, 4-255 reserved (preserved) |

Reserved bits are retained by bilge `FromBits`. Generated setters alter only
in-memory values; they do not make a read-only register writable on the device.
No generic register-write API permits accidentally writing measurements.

## Control subcommands

All codes are sent low byte first at 00, then high byte at 01. Query rows return
a selected word at 00-01 after the write delay. Action rows must not be used as
queries (`InvalidControlQuery`). `send_command` rejects query commands
(`InvalidControlAction`). Other than the documented mode changes, reads have no
side effects. ControlStatus is a separate bilge response type.

| Code | ControlCommand | Allowed sealed | Result / effect |
|---|---|---|---|
| 0000 | Status | Yes | ControlStatus |
| 0001 | DeviceType | Yes | 0x0421 (not sufficient alone to identify variant) |
| 0002 | FirmwareVersion | Yes | Firmware word |
| 0004 | DataMemoryCode | Yes | Low byte: G1A 0x48, G1B 0x58 |
| 0007 | PreviousCommand | Yes | Previous MAC command below 0x0015 |
| 0008 | ChemistryIdentifier | Yes | G1A 0x0128, G1B 0x0312; chemistry cannot be reprogrammed |
| 000C | BatteryInsert | Yes | Set BAT_DET and start gauging, only if BIE=0 |
| 000D | BatteryRemove | Yes | Clear BAT_DET, only if BIE=0 |
| 0011 | SetHibernate | Yes | Request hibernate after sleep |
| 0012 | ClearHibernate | Yes | Clear request / leave hibernate |
| 0013 | SetConfigurationUpdate | No | Poll Flags.CFGUPMODE until set, up to 1 s |
| 001B | EnableShutdown | No | Enable shutdown |
| 001C | Shutdown | No | Requires shutdown enabled; GPOUT wake pulse needed to exit |
| 0020 | Seal | No (already sealed is harmless) | Set SS and Update Status bit 7 |
| 0023 | PulseStateOfChargeInterrupt | Yes | Pulse GPOUT ~1 ms; requires BATLOWEN=0 (table also calls this TOGGLE_GPOUT) |
| 0041 | Reset | No | Full reset, restores ROM defaults and sets ITPOR |
| 0042 | SoftReset | No | OCV measurement, resimulation, clears ITPOR/CFGUPMODE |
| 0043 | ExitConfigurationUpdate | No | Exit without OCV or resimulation |
| 0044 | ExitConfigurationUpdateWithResimulation | No | Exit with resimulation, no new OCV |
| 8000 twice | `unseal()` (fixed ROM key) | Yes | Consecutive words with no intervening Control write; verify SS clears |

Soft reset and both exit commands can automatically reseal if Update Status
bit 7 is set. The driver never assumes sealing state is cached.

## Extended byte and block registers

There is no documented fixed reset value for these selectors/buffers. Read
requirements below apply to every byte in each row. Reads do not clear state.
All are represented by bilge structs; BlockData contains 32 bilge byte structs.

| Address | Type | Access | Read/write requirements |
|---|---|---|---|
| 3E | DataClass | R/W unsealed | Select documented DataSubclass; no data access while sealed |
| 3F | DataBlock | R/W unsealed | Page = floor(subclass offset / 32) |
| 40-5F | BlockData | R/W unsealed | First enable 61=0, select class/page; byte index = offset mod 32; writes require CONFIG UPDATE |
| 60 | BlockDataChecksum | R/W unsealed | Validate all 32 bytes on read; correct checksum commits writes to RAM |
| 61 | BlockDataControl | W/R unsealed | Write zero before selecting a page |
| 62-7F | Reserved | Do not access | Datasheet additionally disallows reads above 6B; no API exposes these |

Checksum = `255 - (sum(all 32 bytes) mod 256)`. Selection changes shared device
state. A block read consists of selection, 32-byte read and checksum read, not
one atomic I2C transaction. A block write sends 32 individual bytes, commits,
then reselects and verifies all bytes and checksum. Errors leave partial state;
do not attempt to continue with an assumed selection.

## Data-memory map

These are **subclass offsets, not I2C addresses** (SLUUAC9A tables 6-2--6-6).
Access through `read_data_block` / `write_data_block`; the latter is a low-level
API, and the caller must validate each parameter against the TRM. Defaults are
ROM initialization values. Unlisted offsets are reserved: preserve them.
I1/I2 = signed 8/16 bit, U1/U2 = unsigned, H = raw hexadecimal.
F4 is TI's calibration floating format, **not an IEEE float cast**.

| Subclass | Offset: parameter (type; default) |
|---|---|
| 2 Safety | 0: Over Temp (I2; 550), 2: Under Temp (I2; 0), 4: Temp Hys (U1; 50), in 0.1 C |
| 36 Charge termination | 3: TCA Set % (I1; 99), 4: TCA Clear % (I1; 95), 5: FC Set % (I1; -1), 6: FC Clear % (I1; 98), 7: DODatEOC Delta T (I2; 50) |
| 48 Data | 2: Initial Standby (I1; -3 mA), 3: Initial MaxLoad (I2; -200 mA) |
| 49 Discharge | 0: SOC1 Set (U1; 10), 1: SOC1 Clear (U1; 15), 2: SOCF Set (U1; 2), 3: SOCF Clear (U1; 5), all percent |
| 64 Registers | 0: OpConfig (H2; 25F8), 2: OpConfigB (H1; 0F) |
| 68 Power | 7: Hibernate I (I2; 3 mA), 9: Hibernate V (I2; 2200 mV) |
| 80 IT Cfg | 22: Ra Filter (U2; 800), 35: Fast Qmax Start DOD % (U1; 92), 36: End DOD % (U1; 96), 37: Start Volt Delta (I2; 125), 39: Current Threshold (U2; 4), 41: Min Points (U1; 3), 45: Max Qmax Change (U1; 20), 46: Qmax Max Delta % (U1; 10), 47: Max % Default Qmax (U1; 120), 48: Qmax Filter (U1; 96), 50: ResRelax Time (U2; 500), 52: User Rate-mA (I2; 0), 54: User Rate-mW (I2; 0), 61: Max Sim Rate (U1; 1), 62: Min Sim Rate (U1; 20), 63: Ra Max Delta (U2; 11, spans pages), 72: Min Delta Voltage (I2; 0), 74: Max Delta Voltage (I2; 200), 76: DeltaV Max dV (I2; 100), 78: TermV Valid t (U1; 2) |
| 81 Current thresholds | 0: Dsg Current Threshold (I2; 167), 2: Chg Current Threshold (I2; 100), 4: Quit Current (I2; 250), 6: Dsg Relax Time (U2; 60), 8: Chg Relax Time (U1; 60), 9: Quit Relax Time (U1; 1), 12: Max IR Correct (U2; 400) |
| 82 State | 0: Qmax Cell 0 (I2; 16384), 2: Update Status (H1; 00), 3: Reserve Cap-mAh (I2; 0), 5: Load Select/Mode (H1; 81), 6: Q Invalid MaxV (I2; A 3803/B 3814), 8: Q Invalid MinV (I2; A 3752/B 3748), 10: Design Capacity (I2; A 1340/B 1000), 12: Design Energy (I2; A 4960/B 3800), 14: Default Design Cap (I2; A 1340/B 5580), 16: Terminate Voltage (I2; 3200), 22: T Rise (I2; 20), 24: T Time Constant (I2; 1000), 26: SOCI Delta (U1; 1), 27: Taper Rate (I2; 100), 29: Taper Voltage (I2; A 4100/B 4200), 31: Sleep Current (I2; 10, spans pages), 33: V at Chg Term (I2; A 4190/B 4290), 35: Avg I Last Run (I2; -50), 37: Avg P Last Run (I2; -50), 39: Delta Voltage (I2; 1) |
| 89 R_a RAM | Offsets 0,2,...,28: R_a0 0..14 (I2). G1A defaults: 102,102,99,107,72,59,62,63,53,47,60,70,140,369,588. G1B: 16,17,20,24,20,18,20,20,21,22,24,31,49,98,375 |
| 104 Calibration data | 0: Board Offset (I1; 0), 1: Int Temp Offset (I1; 0), 2: Pack V Offset (I1; 0) |
| 105 CC Cal | 0: CC Offset (I2; 0), 2: CC Cal Temp (I2; 2982), 4: CC Gain (F4; 0.672785), 8: CC Delta (F4; 799341.14) |
| 107 Current | 1: Deadband (U1; 5 mA) |
| 112 Codes | 0: Sealed to Unsealed (H4; 80008000), ROM, read-only |

`configure_battery` updates only State offsets 10, 12, 16 and 27 in page 0,
preserving all other bytes. It rejects capacity outside 200-8000 mAh (datasheet
supported range), energy outside 1-32767 mWh, terminate voltage outside
2500-3700 mV and taper rate outside 1-2000. Raw page writes do not grant
permission to modify ROM fields or reserved offsets.
Page indices above the last documented page are rejected (`InvalidDataBlock`).
SecurityCodes page writes are rejected (`ReadOnlyDataMemory`).
