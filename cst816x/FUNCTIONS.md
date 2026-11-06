# CST816S register-derived functions

Private `src/function.rs` owns `Default`-derived `TouchFunction` and
`TimingFunction`. Inputs are **only typed register read results**, never bus
handles or user configuration. Public device methods read then process; they
forward original interface/decoding errors without invoking processing on failure.

Source: `docs/CST816S_register_declaration.pdf`, no stated revision, page 1 for
touch, pages 3–4 for units. See README §Sources and evidence limits for framing
evidence and §Touch consistency for the absence of a hardware latch guarantee.

| Device method | Typed read results | Private processing | Meaning / result |
|---------------|--------------------|--------------------|------------------|
| `read_touch` | `GestureIdentifier`, `FingerNumber`, `XPosition`, `YPosition`, from sequential `0x01..=0x06` | `TouchFunction::process` | Return validated gesture and `None` if count=0; count=1 returns a 12-bit `TouchPosition`. No screen scaling or interpretation of unspecified high bits. |
| `read_interrupt_pulse_width_microseconds` | `InterruptPulseWidth` (`0xED`) | `TimingFunction::interrupt_pulse_microseconds` | Raw 0.1 ms units converted to `u32` microseconds. |
| `read_normal_scan_period_milliseconds` | `NormalScanPeriod` (`0xEE`) | `TimingFunction::normal_scan_milliseconds` | Raw 10 ms units converted to `u16` milliseconds. |
| `read_low_power_auto_wake_time_seconds` | `LowPowerAutoWakeTime` (`0xF4`) | `TimingFunction::auto_wake_seconds` | Raw minutes converted to `u16` seconds. |

## Equations and semantics

High/low bytes are assembled big-endian into lossless bilge pair values; typed
12-bit coordinate getters exclude each high nibble:

$$x = ((XposH \mathbin{\&} 15) \times 256) + XposL$$
$$y = ((YposH \mathbin{\&} 15) \times 256) + YposL$$

Coordinate bytes do not establish whether a finger is present: `FingerNum`
does. Count=0 suppresses possibly stale coordinates; it does not suppress the
gesture, which can describe a completed action. Count=1 at `(0,0)` is a real
reported position, not a no-finger sentinel. Undocumented gestures/counts are
rejected by register decoding before private processing, without nibble-masking
finger counts.

$$pulse_{\mu s} = IrqPluseWidth \times 100$$
$$scan_{ms} = NorScanPer \times 10$$
$$wake_s = LpAutoWakeTime \times 60$$

These integer multiplications are exact and fit even for raw byte 255:
25500 µs, 2550 ms, 15300 s. They do not validate current hardware settings;
lossless reads can return out-of-range values, while device writes reject them.
The declaration says normal scan period affects auto-wake/auto-sleep but supplies
no coupling equation, so these are nominal register units, not a timing model.

Raw register methods return typed bytes/pairs unchanged (apart from fallible
gesture/count decoding); they do not require another processing layer.
`MotionSlAngle` documents $Angle = \tan(c) \times 10$, but angle units and
rounding are unspecified: no angle conversion or guessed radians/degrees API
is provided. Frequency/current units and BPC meaning are likewise unknown.
