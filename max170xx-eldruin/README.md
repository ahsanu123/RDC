# Rust MAX170xx 1-Cell/2-Cell Fuel Gauge for Lithium-ion (Li+) Batteries Driver

[![crates.io](https://img.shields.io/crates/v/max170xx.svg)](https://crates.io/crates/max170xx)
[![Docs](https://docs.rs/max170xx/badge.svg)](https://docs.rs/max170xx)
[![Build Status](https://github.com/eldruin/max170xx-rs/workflows/Build/badge.svg)](https://github.com/eldruin/max170xx-rs/actions?query=workflow%3ABuild)
[![Coverage Status](https://coveralls.io/repos/github/eldruin/max170xx-rs/badge.svg?branch=master)](https://coveralls.io/github/eldruin/max170xx-rs?branch=master)
![Minimum Supported Rust Version](https://img.shields.io/badge/rustc-1.62+-blue.svg)

This is a platform agnostic Rust driver for the ultra-compact, low-cost,
host-side fuel-gauge systems for lithium-ion (Li+) batteries in handheld
and portable equipment using the [`embedded-hal`] traits.

It is compatible with MAX17043, MAX17044, MAX17048, MAX17049, MAX17058 and MAX17059.

This driver allows you to:
- Get state of charge. See: `soc()`.
- Get battery voltage. See: `voltage()`.
- Software reset. See: `reset()`.
- Quickstart. See: `quickstart()`.
- Get IC version. See: `version()`.
- Only on MAX17048/MAX17049:
    - Get charge/discharge rate. See: `charge_rate()`.
- Only on MAX17048/MAX17049/MAX17058/MAX17059:
    - Set table registers. See: `set_table()`.

<!-- TODO
[Introductory blog post]()
-->

## The devices
The devices are ultra-compact, low-cost, host-side fuel-gauge systems
for lithium-ion (Li+) batteries in handheld and portable equipment.
There are models configured to operate with a single or dual lithium
cell pack.

The devices use a sophisticated Li+ battery-modeling scheme, called
ModelGauge(TM) to track the battery's relative state-of-charge (SOC)
continuously over a widely varying charge/discharge profile. Unlike
traditional fuel gauges, the ModelGauge algorithm eliminates the need
for battery relearn cycles and an external current-sense resistor.
Temperature compensation is possible in the application with minimal
interaction between a μC and the device.

The communication is done through an I2C interface.

Datasheets: [MAX17043/MAX17044](https://datasheets.maximintegrated.com/en/ds/MAX17043-MAX17044.pdf),
[MAX17048/MAX17049](https://datasheets.maximintegrated.com/en/ds/MAX17048-MAX17049.pdf),
[MAX17058/MAX17059](https://datasheets.maximintegrated.com/en/ds/MAX17058-MAX17059.pdf)

## Usage

To use this driver, import this crate and an `embedded_hal` implementation,
then instantiate the device.

Please find additional examples using hardware in this repository: [driver-examples]

[driver-examples]: https://github.com/eldruin/driver-examples

```rust
use linux_embedded_hal::I2cdev;
use max170xx::Max17043;

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut sensor = Max17043::new(dev);
    loop {
        let soc = sensor.soc().unwrap();
        let voltage = sensor.voltage().unwrap();
        println!("Charge: {:.2}%", soc);
        println!("Voltage: {:.2}V", voltage);
    }
}
```

## Support

For questions, issues, feature requests, and other changes, please file an
[issue in the github project](https://github.com/eldruin/max170xx-rs/issues).

## Minimum Supported Rust Version (MSRV)

This crate is guaranteed to compile on stable Rust 1.62 and up. It *might*
compile with older versions but that may change in any new patch release.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   <http://www.apache.org/licenses/LICENSE-2.0>)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   <http://opensource.org/licenses/MIT>)

at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
