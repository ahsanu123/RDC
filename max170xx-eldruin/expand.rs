#![feature(prelude_import)]
//! This is a platform agnostic Rust driver for the ultra-compact, low-cost,
//! host-side fuel-gauge systems for lithium-ion (Li+) batteries in handheld
//! and portable equipment using the [`embedded-hal`] traits.
//!
//! It is compatible with MAX17043, MAX17044, MAX17048, MAX17049, MAX17058 and MAX17059.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! This driver allows you to:
//! - Get state of charge. See: [`soc()`].
//! - Get battery voltage. See: [`voltage()`].
//! - Software reset. See: [`reset()`].
//! - Quickstart. See: [`quickstart()`].
//! - Get IC version. See: [`version()`].
//! - Only on MAX17048/MAX17049:
//!     - Get charge/discharge rate. See: [`charge_rate()`].
//! - Only on MAX17048/MAX17049/MAX17058/MAX17059:
//!     - Set table registers. See: [`set_table()`].
//!
//! [`soc()`]: struct.Max17043.html#method.soc
//! [`voltage()`]: struct.Max17043.html#method.voltage
//! [`reset()`]: struct.Max17043.html#method.reset
//! [`quickstart()`]: struct.Max17043.html#method.quickstart
//! [`version()`]: struct.Max17043.html#method.version
//! [`charge_rate()`]: struct.Max17048.html#method.charge_rate
//! [`set_table()`]: struct.Max17048.html#method.set_table
//!
//! <!-- TODO
//! [Introductory blog post]()
//! -->
//!
//! ## The devices
//! The devices are ultra-compact, low-cost, host-side fuel-gauge systems
//! for lithium-ion (Li+) batteries in handheld and portable equipment.
//! There are models configured to operate with a single or dual lithium
//! cell pack.
//!
//! The devices use a sophisticated Li+ battery-modeling scheme, called
//! ModelGauge(TM) to track the battery's relative state-of-charge (SOC)
//! continuously over a widely varying charge/discharge profile. Unlike
//! traditional fuel gauges, the ModelGauge algorithm eliminates the need
//! for battery relearn cycles and an external current-sense resistor.
//! Temperature compensation is possible in the application with minimal
//! interaction between a μC and the device.
//!
//! The communication is done through an I2C interface.
//!
//! Datasheets: [MAX17043/MAX17044](https://datasheets.maximintegrated.com/en/ds/MAX17043-MAX17044.pdf),
//! [MAX17048/MAX17049](https://datasheets.maximintegrated.com/en/ds/MAX17048-MAX17049.pdf),
//! [MAX17058/MAX17059](https://datasheets.maximintegrated.com/en/ds/MAX17058-MAX17059.pdf)
//!
//! ## Usage examples (see also examples folder)
//!
//! To use this driver, import this crate and an `embedded_hal` implementation,
//! then instantiate the device.
//!
//! Please find additional examples using hardware in this repository: [driver-examples]
//!
//! [driver-examples]: https://github.com/eldruin/driver-examples
//!
//! ### Read state of charge and cell voltage
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use max170xx::Max17043;
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Max17043::new(dev);
//! let soc = sensor.soc().unwrap();
//! let voltage = sensor.voltage().unwrap();
//! println!("Charge: {:.2}%", soc);
//! println!("Voltage: {:.2}V", voltage);
//! ```
//!
//! ### Trigger software reset
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use max170xx::Max17043;
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Max17043::new(dev);
//! sensor.reset().unwrap();
//! ```
//!
//! ### Quick start
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use max170xx::Max17043;
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Max17043::new(dev);
//! // ... noisy power-up ...
//! sensor.quickstart().unwrap();
//! ```
//!
//! ### Read charge/discharge rate
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use max170xx::Max17048;
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Max17048::new(dev);
//! let rate = sensor.charge_rate().unwrap();
//! println!("Charge rate: {:.2}%/h", rate);
//! ```
//!
#![deny(unsafe_code, missing_docs)]
#![no_std]
#[prelude_import]
use core::prelude::rust_2021::*;
#[macro_use]
extern crate core;
extern crate compiler_builtins as _;
mod types {
    /// All possible errors in this crate
    pub enum Error<E> {
        /// I²C communication error
        I2C(E),
        /// Invalid input data provided
        InvalidInputData,
    }
    #[automatically_derived]
    impl<E: ::core::fmt::Debug> ::core::fmt::Debug for Error<E> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Error::I2C(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "I2C",
                        &__self_0,
                    )
                }
                Error::InvalidInputData => {
                    ::core::fmt::Formatter::write_str(f, "InvalidInputData")
                }
            }
        }
    }
}
pub use crate::types::Error;
#[macro_use]
mod common {
    //! Common methods
}
#[macro_use]
mod register_access {
    pub const ADDR: u8 = 0b011_0110;
    pub struct Register;
    impl Register {
        pub const VCELL: u8 = 0x02;
        pub const SOC: u8 = 0x04;
        pub const MODE: u8 = 0x06;
        pub const VERSION: u8 = 0x08;
        pub const CRATE: u8 = 0x16;
        pub const COMMAND: u8 = 0xFE;
    }
    pub struct Command;
    impl Command {
        pub const POR_43_44: u16 = 0x0054;
        pub const POR_X8_X9: u16 = 0x5400;
        pub const QSTRT: u16 = 0x4000;
    }
}
use crate::register_access::{Command, Register, ADDR};
mod max17043_44 {
    use crate::{Command, Error, Register, ADDR};
    use embedded_hal::i2c;
    /// Device driver
    pub struct Max17043<I2C> {
        i2c: I2C,
    }
    #[automatically_derived]
    impl<I2C: ::core::fmt::Debug> ::core::fmt::Debug for Max17043<I2C> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "Max17043",
                "i2c",
                &&self.i2c,
            )
        }
    }
    impl<I2C, E> Max17043<I2C>
    where
        I2C: i2c::I2c<Error = E>,
    {
        /// Create new instance of the device.
        pub fn new(i2c: I2C) -> Self {
            Max17043 { i2c }
        }
        /// Destroy driver instance, return I2C bus.
        pub fn destroy(self) -> I2C {
            self.i2c
        }
        /// Quick start
        ///
        /// Restarts fuel-gauge calculations in the same manner as initial power-up
        /// of the IC. This is useful if an application's power-up sequence
        /// is exceedingly noisy
        pub fn quickstart(&mut self) -> Result<(), Error<E>> {
            self.write_register(Register::MODE, Command::QSTRT)
        }
        /// Get IC version
        pub fn version(&mut self) -> Result<u16, Error<E>> {
            self.read_register(Register::VERSION)
        }
    }
    impl<I2C, E> Max17043<I2C>
    where
        I2C: i2c::I2c<Error = E>,
    {
        pub(crate) fn write_register(
            &mut self,
            register: u8,
            data: u16,
        ) -> Result<(), Error<E>> {
            let payload: [u8; 3] = [
                register,
                ((data & 0xFF00) >> 8) as u8,
                (data & 0xFF) as u8,
            ];
            self.i2c.write(ADDR, &payload).map_err(Error::I2C)
        }
        #[allow(unused)]
        pub(crate) fn write_u8_register(
            &mut self,
            register: u8,
            data: u8,
        ) -> Result<(), Error<E>> {
            let payload: [u8; 2] = [register, data];
            self.i2c.write(ADDR, &payload).map_err(Error::I2C)
        }
        pub(crate) fn read_register(&mut self, register: u8) -> Result<u16, Error<E>> {
            let mut data = [0; 2];
            self.i2c
                .write_read(ADDR, &[register], &mut data)
                .map_err(Error::I2C)
                .and(Ok((u16::from(data[0]) << 8) | u16::from(data[1])))
        }
    }
    /// Device driver
    pub struct Max17044<I2C> {
        i2c: I2C,
    }
    #[automatically_derived]
    impl<I2C: ::core::fmt::Debug> ::core::fmt::Debug for Max17044<I2C> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "Max17044",
                "i2c",
                &&self.i2c,
            )
        }
    }
    impl<I2C, E> Max17044<I2C>
    where
        I2C: i2c::I2c<Error = E>,
    {
        /// Create new instance of the device.
        pub fn new(i2c: I2C) -> Self {
            Max17044 { i2c }
        }
        /// Destroy driver instance, return I2C bus.
        pub fn destroy(self) -> I2C {
            self.i2c
        }
        /// Quick start
        ///
        /// Restarts fuel-gauge calculations in the same manner as initial power-up
        /// of the IC. This is useful if an application's power-up sequence
        /// is exceedingly noisy
        pub fn quickstart(&mut self) -> Result<(), Error<E>> {
            self.write_register(Register::MODE, Command::QSTRT)
        }
        /// Get IC version
        pub fn version(&mut self) -> Result<u16, Error<E>> {
            self.read_register(Register::VERSION)
        }
    }
    impl<I2C, E> Max17044<I2C>
    where
        I2C: i2c::I2c<Error = E>,
    {
        pub(crate) fn write_register(
            &mut self,
            register: u8,
            data: u16,
        ) -> Result<(), Error<E>> {
            let payload: [u8; 3] = [
                register,
                ((data & 0xFF00) >> 8) as u8,
                (data & 0xFF) as u8,
            ];
            self.i2c.write(ADDR, &payload).map_err(Error::I2C)
        }
        #[allow(unused)]
        pub(crate) fn write_u8_register(
            &mut self,
            register: u8,
            data: u8,
        ) -> Result<(), Error<E>> {
            let payload: [u8; 2] = [register, data];
            self.i2c.write(ADDR, &payload).map_err(Error::I2C)
        }
        pub(crate) fn read_register(&mut self, register: u8) -> Result<u16, Error<E>> {
            let mut data = [0; 2];
            self.i2c
                .write_read(ADDR, &[register], &mut data)
                .map_err(Error::I2C)
                .and(Ok((u16::from(data[0]) << 8) | u16::from(data[1])))
        }
    }
    impl<I2C, E> Max17043<I2C>
    where
        I2C: i2c::I2c<Error = E>,
    {
        /// Get state of charge of the cell as calculated by the ModelGauge
        /// algorithm as a percentage.
        pub fn soc(&mut self) -> Result<f32, Error<E>> {
            let soc = self.read_register(Register::SOC)?;
            Ok(f32::from((soc & 0xFF00) >> 8) + f32::from(soc & 0xFF) / 256.0)
        }
        /// Software reset
        pub fn reset(&mut self) -> Result<(), Error<E>> {
            self.write_register(Register::COMMAND, Command::POR_43_44)
        }
    }
    impl<I2C, E> Max17044<I2C>
    where
        I2C: i2c::I2c<Error = E>,
    {
        /// Get state of charge of the cell as calculated by the ModelGauge
        /// algorithm as a percentage.
        pub fn soc(&mut self) -> Result<f32, Error<E>> {
            let soc = self.read_register(Register::SOC)?;
            Ok(f32::from((soc & 0xFF00) >> 8) + f32::from(soc & 0xFF) / 256.0)
        }
        /// Software reset
        pub fn reset(&mut self) -> Result<(), Error<E>> {
            self.write_register(Register::COMMAND, Command::POR_43_44)
        }
    }
    impl<I2C, E> Max17043<I2C>
    where
        I2C: i2c::I2c<Error = E>,
    {
        /// Get battery voltage
        pub fn voltage(&mut self) -> Result<f32, Error<E>> {
            let vcell = self.read_register(Register::VCELL)?;
            Ok(f32::from(vcell >> 4) / 800.0)
        }
    }
    impl<I2C, E> Max17044<I2C>
    where
        I2C: i2c::I2c<Error = E>,
    {
        /// Get battery voltage
        pub fn voltage(&mut self) -> Result<f32, Error<E>> {
            let vcell = self.read_register(Register::VCELL)?;
            Ok(f32::from(vcell >> 4) / 400.0)
        }
    }
}
pub use crate::max17043_44::{Max17043, Max17044};
mod max170x8_x9 {
    use crate::{Command, Error, Register, ADDR};
    use embedded_hal::i2c;
    /// Device driver
    pub struct Max17048<I2C> {
        i2c: I2C,
    }
    #[automatically_derived]
    impl<I2C: ::core::fmt::Debug> ::core::fmt::Debug for Max17048<I2C> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "Max17048",
                "i2c",
                &&self.i2c,
            )
        }
    }
    impl<I2C, E> Max17048<I2C>
    where
        I2C: i2c::I2c<Error = E>,
    {
        /// Create new instance of the device.
        pub fn new(i2c: I2C) -> Self {
            Max17048 { i2c }
        }
        /// Destroy driver instance, return I2C bus.
        pub fn destroy(self) -> I2C {
            self.i2c
        }
        /// Quick start
        ///
        /// Restarts fuel-gauge calculations in the same manner as initial power-up
        /// of the IC. This is useful if an application's power-up sequence
        /// is exceedingly noisy
        pub fn quickstart(&mut self) -> Result<(), Error<E>> {
            self.write_register(Register::MODE, Command::QSTRT)
        }
        /// Get IC version
        pub fn version(&mut self) -> Result<u16, Error<E>> {
            self.read_register(Register::VERSION)
        }
    }
    impl<I2C, E> Max17048<I2C>
    where
        I2C: i2c::I2c<Error = E>,
    {
        pub(crate) fn write_register(
            &mut self,
            register: u8,
            data: u16,
        ) -> Result<(), Error<E>> {
            let payload: [u8; 3] = [
                register,
                ((data & 0xFF00) >> 8) as u8,
                (data & 0xFF) as u8,
            ];
            self.i2c.write(ADDR, &payload).map_err(Error::I2C)
        }
        #[allow(unused)]
        pub(crate) fn write_u8_register(
            &mut self,
            register: u8,
            data: u8,
        ) -> Result<(), Error<E>> {
            let payload: [u8; 2] = [register, data];
            self.i2c.write(ADDR, &payload).map_err(Error::I2C)
        }
        pub(crate) fn read_register(&mut self, register: u8) -> Result<u16, Error<E>> {
            let mut data = [0; 2];
            self.i2c
                .write_read(ADDR, &[register], &mut data)
                .map_err(Error::I2C)
                .and(Ok((u16::from(data[0]) << 8) | u16::from(data[1])))
        }
    }
    /// Device driver
    pub struct Max17049<I2C> {
        i2c: I2C,
    }
    #[automatically_derived]
    impl<I2C: ::core::fmt::Debug> ::core::fmt::Debug for Max17049<I2C> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "Max17049",
                "i2c",
                &&self.i2c,
            )
        }
    }
    impl<I2C, E> Max17049<I2C>
    where
        I2C: i2c::I2c<Error = E>,
    {
        /// Create new instance of the device.
        pub fn new(i2c: I2C) -> Self {
            Max17049 { i2c }
        }
        /// Destroy driver instance, return I2C bus.
        pub fn destroy(self) -> I2C {
            self.i2c
        }
        /// Quick start
        ///
        /// Restarts fuel-gauge calculations in the same manner as initial power-up
        /// of the IC. This is useful if an application's power-up sequence
        /// is exceedingly noisy
        pub fn quickstart(&mut self) -> Result<(), Error<E>> {
            self.write_register(Register::MODE, Command::QSTRT)
        }
        /// Get IC version
        pub fn version(&mut self) -> Result<u16, Error<E>> {
            self.read_register(Register::VERSION)
        }
    }
    impl<I2C, E> Max17049<I2C>
    where
        I2C: i2c::I2c<Error = E>,
    {
        pub(crate) fn write_register(
            &mut self,
            register: u8,
            data: u16,
        ) -> Result<(), Error<E>> {
            let payload: [u8; 3] = [
                register,
                ((data & 0xFF00) >> 8) as u8,
                (data & 0xFF) as u8,
            ];
            self.i2c.write(ADDR, &payload).map_err(Error::I2C)
        }
        #[allow(unused)]
        pub(crate) fn write_u8_register(
            &mut self,
            register: u8,
            data: u8,
        ) -> Result<(), Error<E>> {
            let payload: [u8; 2] = [register, data];
            self.i2c.write(ADDR, &payload).map_err(Error::I2C)
        }
        pub(crate) fn read_register(&mut self, register: u8) -> Result<u16, Error<E>> {
            let mut data = [0; 2];
            self.i2c
                .write_read(ADDR, &[register], &mut data)
                .map_err(Error::I2C)
                .and(Ok((u16::from(data[0]) << 8) | u16::from(data[1])))
        }
    }
    /// Device driver
    pub struct Max17058<I2C> {
        i2c: I2C,
    }
    #[automatically_derived]
    impl<I2C: ::core::fmt::Debug> ::core::fmt::Debug for Max17058<I2C> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "Max17058",
                "i2c",
                &&self.i2c,
            )
        }
    }
    impl<I2C, E> Max17058<I2C>
    where
        I2C: i2c::I2c<Error = E>,
    {
        /// Create new instance of the device.
        pub fn new(i2c: I2C) -> Self {
            Max17058 { i2c }
        }
        /// Destroy driver instance, return I2C bus.
        pub fn destroy(self) -> I2C {
            self.i2c
        }
        /// Quick start
        ///
        /// Restarts fuel-gauge calculations in the same manner as initial power-up
        /// of the IC. This is useful if an application's power-up sequence
        /// is exceedingly noisy
        pub fn quickstart(&mut self) -> Result<(), Error<E>> {
            self.write_register(Register::MODE, Command::QSTRT)
        }
        /// Get IC version
        pub fn version(&mut self) -> Result<u16, Error<E>> {
            self.read_register(Register::VERSION)
        }
    }
    impl<I2C, E> Max17058<I2C>
    where
        I2C: i2c::I2c<Error = E>,
    {
        pub(crate) fn write_register(
            &mut self,
            register: u8,
            data: u16,
        ) -> Result<(), Error<E>> {
            let payload: [u8; 3] = [
                register,
                ((data & 0xFF00) >> 8) as u8,
                (data & 0xFF) as u8,
            ];
            self.i2c.write(ADDR, &payload).map_err(Error::I2C)
        }
        #[allow(unused)]
        pub(crate) fn write_u8_register(
            &mut self,
            register: u8,
            data: u8,
        ) -> Result<(), Error<E>> {
            let payload: [u8; 2] = [register, data];
            self.i2c.write(ADDR, &payload).map_err(Error::I2C)
        }
        pub(crate) fn read_register(&mut self, register: u8) -> Result<u16, Error<E>> {
            let mut data = [0; 2];
            self.i2c
                .write_read(ADDR, &[register], &mut data)
                .map_err(Error::I2C)
                .and(Ok((u16::from(data[0]) << 8) | u16::from(data[1])))
        }
    }
    /// Device driver
    pub struct Max17059<I2C> {
        i2c: I2C,
    }
    #[automatically_derived]
    impl<I2C: ::core::fmt::Debug> ::core::fmt::Debug for Max17059<I2C> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "Max17059",
                "i2c",
                &&self.i2c,
            )
        }
    }
    impl<I2C, E> Max17059<I2C>
    where
        I2C: i2c::I2c<Error = E>,
    {
        /// Create new instance of the device.
        pub fn new(i2c: I2C) -> Self {
            Max17059 { i2c }
        }
        /// Destroy driver instance, return I2C bus.
        pub fn destroy(self) -> I2C {
            self.i2c
        }
        /// Quick start
        ///
        /// Restarts fuel-gauge calculations in the same manner as initial power-up
        /// of the IC. This is useful if an application's power-up sequence
        /// is exceedingly noisy
        pub fn quickstart(&mut self) -> Result<(), Error<E>> {
            self.write_register(Register::MODE, Command::QSTRT)
        }
        /// Get IC version
        pub fn version(&mut self) -> Result<u16, Error<E>> {
            self.read_register(Register::VERSION)
        }
    }
    impl<I2C, E> Max17059<I2C>
    where
        I2C: i2c::I2c<Error = E>,
    {
        pub(crate) fn write_register(
            &mut self,
            register: u8,
            data: u16,
        ) -> Result<(), Error<E>> {
            let payload: [u8; 3] = [
                register,
                ((data & 0xFF00) >> 8) as u8,
                (data & 0xFF) as u8,
            ];
            self.i2c.write(ADDR, &payload).map_err(Error::I2C)
        }
        #[allow(unused)]
        pub(crate) fn write_u8_register(
            &mut self,
            register: u8,
            data: u8,
        ) -> Result<(), Error<E>> {
            let payload: [u8; 2] = [register, data];
            self.i2c.write(ADDR, &payload).map_err(Error::I2C)
        }
        pub(crate) fn read_register(&mut self, register: u8) -> Result<u16, Error<E>> {
            let mut data = [0; 2];
            self.i2c
                .write_read(ADDR, &[register], &mut data)
                .map_err(Error::I2C)
                .and(Ok((u16::from(data[0]) << 8) | u16::from(data[1])))
        }
    }
    impl<I2C, E> Max17048<I2C>
    where
        I2C: i2c::I2c<Error = E>,
    {
        /// Get state of charge of the cell as calculated by the ModelGauge
        /// algorithm as a percentage.
        pub fn soc(&mut self) -> Result<f32, Error<E>> {
            let soc = self.read_register(Register::SOC)?;
            Ok(f32::from(soc) / 256.0)
        }
        /// Software reset
        pub fn reset(&mut self) -> Result<(), Error<E>> {
            self.write_register(Register::COMMAND, Command::POR_X8_X9)
        }
    }
    impl<I2C, E> Max17048<I2C>
    where
        I2C: i2c::I2c<Error = E>,
    {
        /// Set table values.
        ///
        /// This unlocks the table registers, writes the table values and locks the
        /// table registers again.
        pub fn set_table(&mut self, table: &[u16; 64]) -> Result<(), Error<E>> {
            self.write_u8_register(0x3F, 0x57)?;
            self.write_u8_register(0x3E, 0x4A)?;
            let mut data = [0; 129];
            data[0] = 0x40;
            for (i, v) in table.iter().enumerate() {
                data[i * 2 + 1] = ((v & 0xFF00) >> 8) as u8;
                data[i * 2 + 2] = (v & 0xFF) as u8;
            }
            self.i2c.write(ADDR, &data).map_err(Error::I2C)?;
            self.write_u8_register(0x3F, 0x00)?;
            self.write_u8_register(0x3E, 0x00)
        }
    }
    impl<I2C, E> Max17049<I2C>
    where
        I2C: i2c::I2c<Error = E>,
    {
        /// Get state of charge of the cell as calculated by the ModelGauge
        /// algorithm as a percentage.
        pub fn soc(&mut self) -> Result<f32, Error<E>> {
            let soc = self.read_register(Register::SOC)?;
            Ok(f32::from(soc) / 256.0)
        }
        /// Software reset
        pub fn reset(&mut self) -> Result<(), Error<E>> {
            self.write_register(Register::COMMAND, Command::POR_X8_X9)
        }
    }
    impl<I2C, E> Max17049<I2C>
    where
        I2C: i2c::I2c<Error = E>,
    {
        /// Set table values.
        ///
        /// This unlocks the table registers, writes the table values and locks the
        /// table registers again.
        pub fn set_table(&mut self, table: &[u16; 64]) -> Result<(), Error<E>> {
            self.write_u8_register(0x3F, 0x57)?;
            self.write_u8_register(0x3E, 0x4A)?;
            let mut data = [0; 129];
            data[0] = 0x40;
            for (i, v) in table.iter().enumerate() {
                data[i * 2 + 1] = ((v & 0xFF00) >> 8) as u8;
                data[i * 2 + 2] = (v & 0xFF) as u8;
            }
            self.i2c.write(ADDR, &data).map_err(Error::I2C)?;
            self.write_u8_register(0x3F, 0x00)?;
            self.write_u8_register(0x3E, 0x00)
        }
    }
    impl<I2C, E> Max17058<I2C>
    where
        I2C: i2c::I2c<Error = E>,
    {
        /// Get state of charge of the cell as calculated by the ModelGauge
        /// algorithm as a percentage.
        pub fn soc(&mut self) -> Result<f32, Error<E>> {
            let soc = self.read_register(Register::SOC)?;
            Ok(f32::from(soc) / 256.0)
        }
        /// Software reset
        pub fn reset(&mut self) -> Result<(), Error<E>> {
            self.write_register(Register::COMMAND, Command::POR_X8_X9)
        }
    }
    impl<I2C, E> Max17058<I2C>
    where
        I2C: i2c::I2c<Error = E>,
    {
        /// Set table values.
        ///
        /// This unlocks the table registers, writes the table values and locks the
        /// table registers again.
        pub fn set_table(&mut self, table: &[u16; 64]) -> Result<(), Error<E>> {
            self.write_u8_register(0x3F, 0x57)?;
            self.write_u8_register(0x3E, 0x4A)?;
            let mut data = [0; 129];
            data[0] = 0x40;
            for (i, v) in table.iter().enumerate() {
                data[i * 2 + 1] = ((v & 0xFF00) >> 8) as u8;
                data[i * 2 + 2] = (v & 0xFF) as u8;
            }
            self.i2c.write(ADDR, &data).map_err(Error::I2C)?;
            self.write_u8_register(0x3F, 0x00)?;
            self.write_u8_register(0x3E, 0x00)
        }
    }
    impl<I2C, E> Max17059<I2C>
    where
        I2C: i2c::I2c<Error = E>,
    {
        /// Get state of charge of the cell as calculated by the ModelGauge
        /// algorithm as a percentage.
        pub fn soc(&mut self) -> Result<f32, Error<E>> {
            let soc = self.read_register(Register::SOC)?;
            Ok(f32::from(soc) / 256.0)
        }
        /// Software reset
        pub fn reset(&mut self) -> Result<(), Error<E>> {
            self.write_register(Register::COMMAND, Command::POR_X8_X9)
        }
    }
    impl<I2C, E> Max17059<I2C>
    where
        I2C: i2c::I2c<Error = E>,
    {
        /// Set table values.
        ///
        /// This unlocks the table registers, writes the table values and locks the
        /// table registers again.
        pub fn set_table(&mut self, table: &[u16; 64]) -> Result<(), Error<E>> {
            self.write_u8_register(0x3F, 0x57)?;
            self.write_u8_register(0x3E, 0x4A)?;
            let mut data = [0; 129];
            data[0] = 0x40;
            for (i, v) in table.iter().enumerate() {
                data[i * 2 + 1] = ((v & 0xFF00) >> 8) as u8;
                data[i * 2 + 2] = (v & 0xFF) as u8;
            }
            self.i2c.write(ADDR, &data).map_err(Error::I2C)?;
            self.write_u8_register(0x3F, 0x00)?;
            self.write_u8_register(0x3E, 0x00)
        }
    }
    impl<I2C, E> Max17048<I2C>
    where
        I2C: i2c::I2c<Error = E>,
    {
        /// Get battery voltage in Volts
        pub fn voltage(&mut self) -> Result<f32, Error<E>> {
            let vcell = self.read_register(Register::VCELL)?;
            Ok(f32::from(vcell) * 5.0 / 64000.0)
        }
    }
    impl<I2C, E> Max17058<I2C>
    where
        I2C: i2c::I2c<Error = E>,
    {
        /// Get battery voltage in Volts
        pub fn voltage(&mut self) -> Result<f32, Error<E>> {
            let vcell = self.read_register(Register::VCELL)?;
            Ok(f32::from(vcell) * 5.0 / 64000.0)
        }
    }
    impl<I2C, E> Max17049<I2C>
    where
        I2C: i2c::I2c<Error = E>,
    {
        /// Get battery voltage in Volts
        pub fn voltage(&mut self) -> Result<f32, Error<E>> {
            let vcell = self.read_register(Register::VCELL)?;
            Ok(f32::from(vcell) * 5.0 / 32000.0)
        }
    }
    impl<I2C, E> Max17059<I2C>
    where
        I2C: i2c::I2c<Error = E>,
    {
        /// Get battery voltage in Volts
        pub fn voltage(&mut self) -> Result<f32, Error<E>> {
            let vcell = self.read_register(Register::VCELL)?;
            Ok(f32::from(vcell) * 5.0 / 32000.0)
        }
    }
    impl<I2C, E> Max17048<I2C>
    where
        I2C: i2c::I2c<Error = E>,
    {
        /// Get the approximate charge or discharge rate of the battery
        /// in percentage / hour
        pub fn charge_rate(&mut self) -> Result<f32, Error<E>> {
            let rate = self.read_register(Register::CRATE)? as i16;
            Ok(f32::from(rate) * 0.208)
        }
    }
    impl<I2C, E> Max17049<I2C>
    where
        I2C: i2c::I2c<Error = E>,
    {
        /// Get the approximate charge or discharge rate of the battery
        /// in percentage / hour
        pub fn charge_rate(&mut self) -> Result<f32, Error<E>> {
            let rate = self.read_register(Register::CRATE)? as i16;
            Ok(f32::from(rate) * 0.208)
        }
    }
}
pub use crate::max170x8_x9::{Max17048, Max17049, Max17058, Max17059};
