# MMC5983MA processing functions

Source: MEMSIC MMC5983MA Rev A (2019-04-03), pages 2, 13-14 and 18.
`function.rs` is private. Default-derived processing structs accept only typed
register read results; acquisition, user choices and delays belong to `Mmc5983ma`.

| Processing struct | Inputs | Result / device connection |
|---|---|---|
| MagneticConversion | Seven `MagneticOutputRegisters` bytes at 00-06 | `read_magnetic_field`, `measure_magnetic_field`: raw 18-bit counts and nominal gauss |
| TemperatureConversion | `OutputByte` read from 07 | `read_temperature`, `measure_temperature`: degrees Celsius |
| OffsetCompensation | SET and RESET `MagneticOutputRegisters` | `measure_offset_compensated`: gauss with bridge offset removed |

For each axis:

$$N = (H \ll 10) \mathbin{|} (L \ll 2) \mathbin{|} b$$

where $b$ is X=06[7:6], Y=06[5:4], Z=06[3:2].

$$B_{\mathrm{G}} = \frac{N - 131072}{16384}$$

$$T_{\mathrm{C}} = -75 + 0.8 N_T$$

$$B_{\mathrm{G,compensated}} = \frac{N_{\mathrm{SET}}-N_{\mathrm{RESET}}}{2 \times 16384}$$

The compensation sequence is SET, measure, RESET, measure, restore SET.
Automatic S/R and continuous mode must be off for this sequence. Subtract
in signed arithmetic before division to avoid unsigned underflow and preserve
half-count differences. Raw temperature bytes above the specified operating
range are converted without clamping. These are nominal datasheet conversions,
not hard/soft-iron calibration, tilt compensation or a heading algorithm.
