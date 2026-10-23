# BQ27441-G1 functions

Algorithms follow SLUUAC9A chapters 3--6 and SLUSBH1C section 9.5.4.
`function.rs` is private: its default-derived `Measurements` struct only accepts
typed register read results. It performs no I2C or user-configuration handling.
User arguments and device sequencing belong to `device.rs`.

## Pure measurement processing

| Function | Register argument | Result / processing |
|---|---|---|
| `temperature_centicelsius` | Temperature | i32 centi-degrees C |
| `internal_temperature_centicelsius` | InternalTemperature | i32 centi-degrees C |
| `average_current_milliamps` | AverageCurrent | i16, reinterpret two's-complement bits |
| `standby_current_milliamps` | StandbyCurrent | i16, reinterpret two's-complement bits |
| `maximum_load_current_milliamps` | MaximumLoadCurrent | i16, reinterpret two's-complement bits |
| `average_power_milliwatts` | AveragePower | i16, reinterpret two's-complement bits |

Temperature conversion is exact integer arithmetic, with no `libm`:

$$T_{\mathrm{centiC}} = 10 T_{\mathrm{deciK}} - 27315$$

Signed measurement conversion, for raw unsigned word $x$:

$$s(x) = \begin{cases}x & x < 32768\\x - 65536 & x \ge 32768\end{cases}$$

Other measurement getters return the bilge register's already-scaled u16 field:
`voltage_millivolts`, `nominal_available_capacity_milliamp_hours`,
`full_available_capacity_milliamp_hours`, `remaining_capacity_milliamp_hours`,
`full_charge_capacity_milliamp_hours`, `state_of_charge_percent`,
`remaining_capacity_unfiltered_milliamp_hours`,
`remaining_capacity_filtered_milliamp_hours`,
`full_charge_capacity_unfiltered_milliamp_hours`,
`full_charge_capacity_filtered_milliamp_hours`,
`state_of_charge_unfiltered_percent`, `design_capacity_milliamp_hours`.
They each read the corresponding register exactly once. No clamping or
fabricated fallback is applied to a raw device result.

`flags`, `operation_configuration`, and `state_of_health` return the typed
register, retaining all bits. The caller must examine StateOfHealth.status:
invalid or reserved statuses are not evidence of a valid health percentage.
`read_register<R>` reads any sealed-trait standard register or extended word
mirror, without permitting writes to read-only locations.

## Transport (`comproc.rs`)

`I2cInterface` privately owns the bus and `DelayNs`.
`new` waits 250 ms for power-up communication.
`read` uses register-pointer write + repeated-start read, followed by 66 us.
`write_byte` sends register + one data byte, followed by 66 us.
`read_word` decodes little-endian and paces standard commands by 500 ms.
`write_word` writes low then high at consecutive addresses and waits 2 s.
Delays apply after failures too; a first-byte failure stops the second write.
`release` returns both resources.

## Device operations (`device.rs`)

| Method | Sequence / preconditions |
|---|---|
| `new`, `release` | Own bus + delay; no constructor I2C traffic; release both |
| `identify` | Query DEVICE_TYPE, DM_CODE, CHEM_ID; accept only consistent G1A/G1B triples |
| `read_control` | Validate query, write code, wait, read selected Control word |
| `control_status` | Query Status, convert to ControlStatus |
| `send_command` | Reject queries; check unsealed where required; check shutdown enable for Shutdown; write action |
| `unseal` | Write fixed ROM key twice without interleaving; poll SS clear |
| `seal` | Send Seal; poll SS set |
| `enter_configuration_update` | Check unsealed; send SET_CFGUPDATE; poll Flags.CFGUPMODE set |
| `exit_configuration_update` | Send SOFT_RESET; poll Flags.CFGUPMODE clear |
| `write_temperature` | Check OpConfig.TEMPS; write Temperature word, wait 2 s |
| `read_data_block` | Check unsealed; enable BlockData; select class/page; read 32 bytes and checksum; validate |
| `write_data_block` | Check unsealed and CFGUPMODE; select; write 32 bytes, commit checksum, reselect/read/validate full block |
| `configure_battery` | Validate arguments before I/O; preserve seal state; enter CFGUPDATE; read State page 0; replace four fields; write/verify; soft reset, poll exit; reseal if originally sealed, otherwise query SS and unseal if auto-resealed |

`require_unsealed`, `wait_for_sealed` and `wait_for_configuration_update` are
private state checks. Polls are bounded at four attempts. Control-status queries
include the conservative 2 s write wait; Flags reads include 500 ms pacing.
`select_data_block` writes 61=0, 3E=class, 3F=page, then waits 1 ms.
`check_data_block` rejects page indices beyond the documented subclass length.
`read_selected_block` reads 40-5F and 60 and validates checksum.
`write_selected_block` writes the block bytewise, writes 60, waits 10 ms for
commit, reselects and verifies. Selection and commit margins supplement, not
replace, the mandatory packet delay.

`BatteryConfiguration` arguments:

| Field | State offset | Unit | Accepted range |
|---|---|---|---|
| `design_capacity_milliamp_hours` | 10-11 | mAh | 200-8000 |
| `design_energy_milliwatt_hours` | 12-13 | mWh | 1-32767 |
| `terminate_voltage_millivolts` | 16-17 | mV | 2500-3700 |
| `taper_rate` | 27-28 | 0.1-hour rate | 1-2000 |

These values are big-endian within the block. `configure_battery` preserves all
other bytes, including reserved fields and Update Status. Applications derive
energy and taper rate from their actual cell and charger, rounding to supported
integer values:

$$E_{\mathrm{mWh}} = C_{\mathrm{mAh}} V_{\mathrm{nominal,mV}} / 1000$$
$$\mathrm{TaperRate} = 10 C_{\mathrm{mAh}} / I_{\mathrm{taper,mA}}$$

## Validation (`validation.rs`)

`checksum(BlockData)` returns a typed BlockDataChecksum:

$$c = 255 - \left(\sum_{i=0}^{31} b_i \bmod 256\right)$$

`validate_checksum` returns the expected/received mismatch explicitly.
Bus errors retain the original HAL error payload. Failed multi-step operations
stop at the failing step, do not hide errors with cleanup, and can leave the
device unsealed, in CFGUPDATE, partly staged, or already committed; see README.
