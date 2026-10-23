# BQ27441-G1

Allocation-free, `no_std`, blocking `embedded-hal` 1.0 I2C driver for TI's
BQ27441-G1A and BQ27441-G1B single-cell fuel gauges. Bilge-typed registers and
optional `defmt` diagnostics. See [REGISTERS.md](REGISTERS.md) for the complete
command and data-memory map.

## References

- TI [BQ27441-G1 product page](https://www.ti.com/product/BQ27441-G1), including
  the current documentation list (checked 2026-10-23; no separate errata listed).
- [Datasheet SLUSBH1C](https://www.ti.com/lit/ds/symlink/bq27441-g1.pdf),
  revised December 2014.
- [Technical Reference Manual SLUUAC9A](https://www.ti.com/lit/ug/sluuac9a/sluuac9a.pdf),
  revised May 2015.
- Existing Rust [`bq27441`](https://docs.rs/bq27441/latest/bq27441/) consulted
  for API comparison only; implementation follows the TI specifications.

G1A is for 4.2 V LiCoO2 cells (CHEM_ID 0x0128, DM_CODE 0x48).
G1B is for 4.3/4.35 V cells (CHEM_ID 0x0312, DM_CODE 0x58).
Both report DEVICE_TYPE 0x0421; that identifier alone is not variant detection.
Chemistry profiles are fixed in ROM and cannot be changed by configuration.
