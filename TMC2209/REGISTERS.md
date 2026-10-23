# TMC2209 registers

Source: bundled [manufacturer datasheet rev. 1.09, 2023-02-16](documentation/TMC2209_datasheet_rev1.09.pdf),
sections 4-5, 9 and 14. The manufacturer's online download could not be reached
while implementing this driver, so a newer revision/errata could not be verified.
This map is for **TMC2209**, not TMC2208 or TMC2225. `IOIN.VERSION` must be 0x21.

Every UART data word is **32 bits, most-significant byte first**. Bitfield
declarations list fields from bit zero upwards. `R`, `W` and `R/W` below describe
actual hardware access, not a software shadow. Reserved/unimplemented write bits
must be zero. `None` in `Register::RESET` means live, OTP-dependent or unspecified;
no blanket `Default` is invented for those registers.

| Address | Datasheet | Rust struct | Access | Reset / read requirements and side effects |
|---|---|---|---|---|
| 0x00 | GCONF | `GeneralConfiguration` | R/W | OTP-dependent, analog scaling=1 and step filter=1. Ordinary read; TEST_MODE must remain zero. Read before modifying unrelated bits. |
| 0x01 | GSTAT | `GlobalStatus` | R + write-one-clear | Live/reset flags. Read status, write ones only for RESET/DRV_ERR to clear. UV_CP is live, not latched. Never use generic read-modify-write to acknowledge flags. |
| 0x02 | IFCNT | `InterfaceCounter` | R | 0. Ordinary reads do not increment it; each accepted UART write increments modulo 256. Used before/after every driver write. |
| 0x03 | NODECONF / SLAVECONF | `NodeConfiguration` | W | 0. No ordinary read; SENDDELAY is bits 8-11. Use at least 2 for multiple nodes. |
| 0x04 | OTP_PROG | `OtpProgramming` | programming command | No ordinary read/write API. Reading this address refreshes OTP after programming; see `program_otp_bit` and PROCEDURE.md. Programming is irreversible. |
| 0x05 | OTP_READ | `OtpConfiguration` | R | Factory-trimmed/OTP-dependent. Read OTP contents, refreshing via 0x04 after programming. No read acknowledgement of driver faults. |
| 0x06 | IOIN | `InputStatus` | R | Live pin levels plus VERSION=0x21. Used for connection identity; no side effects. |
| 0x07 | FACTORY_CONF | `FactoryConfiguration` | R/W | OTP clock/temperature trims. Ordinary read; preserve factory calibration unless deliberately calibrating. |
| 0x10 | IHOLD_IRUN | `CurrentControl` | W | OTP hold/delay, run=31. Cannot read back; supply a complete typed value, never synthesize read-modify-write from zero. |
| 0x11 | TPOWERDOWN | `PowerDownDelay` | W | 20. Cannot read back; units are 2^18 clock cycles. At least 2 for StealthChop automatic tuning. |
| 0x12 | TSTEP | `StepInterval` | R | Live measurement. All 20 bits set means overflow/standstill, not a valid finite frequency measurement. |
| 0x13 | TPWMTHRS | `PwmVelocityThreshold` | W | OTP-dependent. Cannot read back; zero disables velocity-based switching. |
| 0x14 | TCOOLTHRS | `SmartCurrentVelocityThreshold` | W | 0. Cannot read back; threshold in TSTEP units. |
| 0x22 | VACTUAL | `TargetVelocity` | W | 0. Cannot read back. Signed 24-bit velocity; zero selects external STEP/DIR pulses. |
| 0x40 | SGTHRS | `StallDetectionThreshold` | W | 0. Cannot read back. Stall criterion SG_RESULT <= 2*SGTHRS. |
| 0x41 | SG_RESULT | `StallDetectionResult` | R | Live load estimate for StealthChop; bits 9 and 0 always zero. No side effects. |
| 0x42 | COOLCONF | `SmartCurrentConfiguration` | W | 0. Cannot read back. Complete configuration required; minimum current constraints depend on IRUN. |
| 0x6A | MSCNT | `MicrostepCounter` | R | Live 0-1023 sequencer position; not an absolute mechanical position. |
| 0x6B | MSCURACT | `MicrostepCurrent` | R | Live signed phase B [8:0], phase A [24:16]; table values before current scaling. |
| 0x6C | CHOPCONF | `ChopperConfiguration` | R/W | OTP-dependent (table reference 0x10000053). Read before changing MRES/TOFF. TOFF=0 switches bridges off. |
| 0x6F | DRV_STATUS | `DriverStatus` | R | Live faults/current/standstill state. Reading does not clear short-circuit latches; disabling bridges clears short flags. Open-load flags can be misleading at standstill/high speed. |
| 0x70 | PWMCONF | `PwmConfiguration` | R/W | OTP-dependent (table reference 0xC10D0024). Read before modifying unrelated settings. |
| 0x71 | PWM_SCALE | `PwmScaling` | R | Live amplitude and signed 9-bit correction; no side effects. |
| 0x72 | PWM_AUTO | `AutomaticPwmValues` | R | Learned offset/gradient; require manufacturer tuning conditions before treating these as calibrated values. |

## Field map

Every unlisted bit is represented by a bilge `unused_*` field, is ignored on
reads and rejected if set in writes. Fields use bool or exact-width bilge
integers; `MicrostepResolution` and `StandstillMode` give descriptive enums.

| Struct | Implemented fields (bit positions) |
|---|---|
| `GeneralConfiguration` | analog_current_scaling 0; internal_sense_resistors 1; spread_cycle_enabled 2; reversed_direction 3; index_temperature_warning 4; index_step_pulses 5; power_down_pin_disabled 6; register_microstep_selection 7; step_filter_enabled 8; test_mode 9 |
| `GlobalStatus` | reset_detected 0; driver_error 1; charge_pump_undervoltage 2 (read-only) |
| `InterfaceCounter` | count [7:0] |
| `NodeConfiguration` | reply_delay [11:8] |
| `OtpProgramming` | bit_index [2:0]; byte_index [5:4]; programming_key [15:8], must be 0xBD |
| `OtpConfiguration` | clock_trim [4:0]; temperature_trim 5; internal_sense_resistors 6; blank_time 7; pwm_gradient_or_off_time [11:8]; automatic_gradient_or_hysteresis_start 12; pwm_threshold_or_hysteresis [15:13]; pwm_offset_or_hysteresis_end 16; regulation_speed 17; pwm_frequency 18; hold_delay [20:19]; hold_current [22:21]; spread_cycle_default 23 |
| `InputStatus` | hardware_disabled 0; address_pin_zero 2; address_pin_one 3; diagnostic 4; uart_pin 6; step_pin 7; spread_cycle_pin 8; direction_pin 9; version [31:24] |
| `FactoryConfiguration` | clock_trim [4:0]; temperature_trim [9:8] |
| `CurrentControl` | hold_current [4:0]; run_current [12:8]; hold_delay [19:16] |
| `PowerDownDelay` | delay [7:0] |
| `StepInterval`, `PwmVelocityThreshold`, `SmartCurrentVelocityThreshold` | clock_cycles [19:0] |
| `TargetVelocity` | signed_velocity_bits [23:0], two's complement |
| `StallDetectionThreshold` | threshold [7:0] |
| `StallDetectionResult` | result [9:0] |
| `SmartCurrentConfiguration` | minimum_load [3:0]; current_increment [6:5]; load_hysteresis [11:8]; current_decrease_speed [14:13]; quarter_minimum_current 15 |
| `MicrostepCounter` | position [9:0] |
| `MicrostepCurrent` | phase_b_bits [8:0]; phase_a_bits [24:16], both two's complement |
| `ChopperConfiguration` | off_time [3:0]; hysteresis_start [6:4]; hysteresis_end [10:7]; blank_time [16:15]; high_sense_sensitivity 17; microstep_resolution [27:24]; interpolation_enabled 28; double_edge_steps 29; ground_short_protection_disabled 30; low_side_short_protection_disabled 31 |
| `DriverStatus` | temperature_warning 0; temperature_shutdown 1; ground_short_phase_a/b 2/3; low_side_short_phase_a/b 4/5; open_load_phase_a/b 6/7; temperature_above_120/143/150/157 8/9/10/11; actual_current_scale [20:16]; stealth_chop_active 30; standstill 31 |
| `PwmConfiguration` | amplitude_offset [7:0]; amplitude_gradient [15:8]; frequency [17:16]; automatic_amplitude 18; automatic_gradient 19; standstill_mode [21:20]; regulation_speed [27:24]; amplitude_limit [31:28] |
| `PwmScaling` | amplitude [7:0]; automatic_correction_bits [24:16], two's complement |
| `AutomaticPwmValues` | amplitude_offset [7:0]; amplitude_gradient [23:16] |

OTP shared fields use PWM meanings when spread_cycle_default=0 and chopper
meanings when it is 1; the exact power-up lookup tables are in section 5.1.1.
There is no register-bank selection on this part. Typed reads reject write-only
registers at compile time; ordinary typed writes exclude OTP programming.
