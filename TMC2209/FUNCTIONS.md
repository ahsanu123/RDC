# TMC2209 functions

Reference: bundled manufacturer datasheet rev. 1.09, sections 4-5, 9 and 14.
The private `RegisterProcessing` struct derives `Default`. Its methods accept
only typed register read results. UART access, clock frequency, resistance and
other application arguments stay on `TMC2209`, not in these functions.

## Pure register processing

| Function | Read results | Device method / result |
|---|---|---|
| `microstep_count` | `ChopperConfiguration` | `read_microsteps`; 256 >> MRES. Encodings 9-15 return `InvalidMicrostepResolution`; device first checks that GCONF selects register-controlled microsteps. |
| `phase_currents` | `MicrostepCurrent` | `read_phase_currents`; sign-extend phase A/B nine-bit values, return A then B. These are table values, not amperes. |
| `pwm_correction` | `PwmScaling` | `read_pwm_correction`; sign-extend the nine-bit automatic correction. |
| `step_clock_period` | `StepInterval` | `measured_256_microstep_frequency_hz`; reject zero or 0xFFFFF (standstill/overflow). |
| `rms_sense_voltage` | `ChopperConfiguration`, `DriverStatus` | `estimated_rms_current_amperes`; compute nominal RMS voltage at the sense resistor. |
| `signed_nine_bits` (private helper) | Nine-bit fields extracted from read results | Shared sign extension for phase currents and PWM correction. |

For a nine-bit raw value $x$, signed decoding is:

$$
s(x) =
\begin{cases}
x, & x < 256 \\
x - 512, & x \geq 256
\end{cases}
$$

For MRES $m \in [0,8]$, the configured microsteps per full step are:

$$
N = \frac{256}{2^m}
$$

TSTEP always refers to 1/256 microsteps, independent of configured resolution.
Given a valid interval $T$:

$$
f_{256} = \frac{f_{\mathrm{CLK}}}{T}
$$

For external sense resistors with analog VREF scaling disabled, use the actual
current scale $CS$ read from DRV_STATUS (not a guessed write-only IRUN value):

$$
V_{\mathrm{RMS}} = \frac{CS+1}{32}\frac{V_{\mathrm{FS}}}{\sqrt{2}}
\qquad
I_{\mathrm{RMS}} = \frac{V_{\mathrm{RMS}}}{R_{\mathrm{SENSE}}+0.020\,\Omega}
$$

$V_{\mathrm{FS}}$ is nominally 0.325 V, or 0.180 V when VSENSE=1. The device
rejects nonfinite/nonpositive resistance, internal sense resistors and enabled
analog scaling rather than reporting an invalid current estimate. These are
nominal estimates, not ADC measurements; calibration and supply tolerances apply.
The constant $1/\sqrt{2}$ is available in `core`; no `libm` is needed.

## Device methods

All fallible methods return `DeviceError<UART::Error>`. Interface failures retain
the entire original HAL error. No write or multi-step operation is retried.

| Method | Registers / procedure | Requirements and effects |
|---|---|---|
| `new`, `Builder::connect` | Read IOIN and require VERSION=0x21. | Validate settings first; no configuration writes, bridge enabling or motor motion. Constructor failure consumes its resources (use borrowed/shared handles for recovery). |
| `read_register<R>` | Typed UART read, full header/CRC validation. | Only `ReadableRegister` types; write-only registers are compile-time errors. |
| `write_register<R>` | Validate write mask and constraints; read IFCNT, send word, read IFCNT again. | Complete words for write-only registers; no caches or fabricated read-modify-write. GSTAT writes are write-one-clear. |
| `configure_uart` | Read-modify-write GCONF.PDN_DISABLE; then write NODECONF.SENDDELAY. | Use delay >=2 on multi-node buses. Does not enable bridges; two writes are not atomic. |
| `set_currents` | Write all IHOLD/IRUN/IHOLDDELAY fields. | Current code 0 is 1/32, not zero motor current. Provide all three settings. |
| `set_microsteps` | Read CHOPCONF/GCONF; write MRES, then GCONF.MSTEP_REG_SELECT. | Hardware address pins no longer select resolution, but still select UART address. Non-atomic; chopper change can take effect before a later error. |
| `read_microsteps` | Read GCONF then CHOPCONF. | Reject pin-controlled resolution instead of falsely reporting MRES. |
| `enable` | Read-modify-write CHOPCONF.TOFF. | Caller supplies nonzero valid off time; hardware ENN/faults can still disable output. May energize the motor. |
| `disable` | Read-modify-write CHOPCONF.TOFF=0. | Bridges off, but VACTUAL and external STEP activity are not stopped. ENN remains application-owned. |
| `set_velocity` | Write signed VACTUAL. | Range -8,388,607 to +8,388,607. Nonzero can initiate motion when bridges are enabled; zero selects external STEP/DIR, not a guaranteed motor stop if STEP is active. |
| `set_reversed_direction` | Read-modify-write GCONF.SHAFT. | Preserve other flags; change direction only under application-controlled safe conditions. |
| `set_spread_cycle` | Read-modify-write GCONF.EN_SPREADCYCLE. | Follow standstill/tuning requirements; hardware SPREAD can invert selection. |
| `set_standstill_mode` | Read-modify-write PWMCONF.FREEWHEEL. | Effective for IHOLD=0 in StealthChop; preserve all unrelated PWM fields. |
| `clear_status_flags` | Write requested RESET/DRV_ERR bits to GSTAT. | No GSTAT read-modify-write. Cannot clear a persisting hardware fault; UV_CP is live. |
| `read_phase_currents`, `read_pwm_correction` | Read MSCURACT / PWM_SCALE then pure conversion. | Neither is a measured physical current. |
| `measured_256_microstep_frequency_hz` | Read TSTEP and divide configured device clock by period. | Error on no valid measurement; clock setting must match actual hardware. |
| `estimated_rms_current_amperes` | Read GCONF, CHOPCONF, DRV_STATUS. | External Rsense and internal reference only; see equations above. |
| `program_otp_bit` | Read OTP, verify bit clear; read IFCNT; issue OTP_PROG; wait >=10 ms; verify IFCNT; refresh OTP via 0x04; read back the programmed bit. | Irreversible, explicit operation only. Byte 0-2, bit 0-7; no retries. See PROCEDURE.md. |
| `recover_uart` | Flush TX, idle >=80 bit times, drain bounded RX bytes. | Explicit recovery, returns discarded byte count. Does not undo writes; requires exclusive physical bus ownership. |
| `release` | Return UART and delay provider. | No disable/stop command is sent. Caller retains responsibility for hardware state. |

Internal `validate_write` checks reserved/nonwritable bits, test mode, legal
MRES, TOFF=1 requiring TBL>=2, effective hysteresis end+start <=16, signed velocity
range, and incompatible double-edge STEP / input filtering (reading the other
configuration when necessary). `write_checked` and `verify_write_counter` require
exactly one accepted write, including IFCNT 255 -> 0 rollover. Concurrent writes
from another controller are unsupported.

For raw VACTUAL $v$, section 14 gives the pulse-generator rate:

$$
f_{\mu\mathrm{step}} = v\frac{f_{\mathrm{CLK}}}{2^{24}}
$$

UART provides no acceleration profile. The application must generate safe ramps
and translate microstep counts to mechanical units using its motor and gearing.
