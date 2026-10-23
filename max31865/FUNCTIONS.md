# MAX31865 processing functions

Source: MAX31865 **19-6478, Rev 3, 7/15**, especially Table 5 on page 15;
see [REGISTERS.md](REGISTERS.md) for links.

`function.rs` is private. Processing structs derive `Default` and accept only
typed register read results. User resistor values are validated on the device
constructor and stored in its private conversion struct, not passed to public
processing methods.

| Private processing struct | Register input | Device methods |
|---|---|---|
| `RtdConversion` | `RtdResistance` burst from 01-02 | `temperature`, `get_ratio`, `get_resistance`, `get_debug_value` |
| `FaultEvaluation` | `FaultStatus` from 07 | `read_fault`; fault checking within `read_rtd` |

`read_rtd` removes the fault bit and returns the unshifted 15-bit count $N$.
Normalized ratio and resistance are:

$$q = \frac{N}{32768}, \qquad R_{\mathrm{RTD}} = q R_{\mathrm{REF}}$$

The existing positive-temperature Callendar-Van Dusen inversion is preserved:

$$T = \frac{\sqrt{A^2 - 4B + \frac{4B}{R_0}R_{\mathrm{RTD}}} - A}{2B}$$

where $A = 3.9083 \times 10^{-3}$, $B = -5.775 \times 10^{-7}$ and $R_0$
is nominal RTD resistance at 0 degrees Celsius. `libm::sqrtf` is retained because
this square root is needed in `no_std`.

For negative temperatures, the existing approximation is preserved. Let
$r = 100R_{\mathrm{RTD}}/R_0$:

$$T = -242.02 + 2.2228r + 2.5859 \times 10^{-3}r^2
 - 4.8260 \times 10^{-6}r^3 - 2.8183 \times 10^{-8}r^4
 + 1.5243 \times 10^{-10}r^5$$

This is the pre-existing driver algorithm, not a claim that the MAX31865 itself
returns temperature or prescribes these polynomial coefficients. It assumes a
platinum RTD with the corresponding curve. Calibration, lead resistance and
self-heating remain application responsibilities.

Each public measurement method performs one acquisition. `get_debug_value`
computes temperature, resistance and ratio from the **same** sample. Ratio and
resistance getters do not require the temperature polynomial to be valid.

## Intentional corrections

The old `get_ratio`, `get_resistance` and corresponding debug fields incorrectly
multiplied ADC counts by 32768. They now divide by 32768, matching Table 5 and the
existing temperature path. Temperature arithmetic and the successful default
60 Hz bus sequence remain covered by regression tests.

Fault evaluation preserves individual legacy error variants and retains a
combined typed status for simultaneous faults. Nonfinite conversion results,
invalid nominal/reference resistors, and transport errors are explicit errors.
