#![no_std]
#![doc = include_str!("../README.md")]

mod comproc;
pub mod error;
mod fault_cycle;
mod function;
mod numwires;
pub mod register;
mod traits;

use embedded_hal::{delay::DelayNs, spi::SpiDevice};
use error::{HardwareErr, Max31865Err};
use function::{FaultEvaluation, RtdConversion};
use register::{Address, Configuration, FaultStatus, FaultThreshold, MainsFilter, RtdResistance};

pub use fault_cycle::FaultCycle;
pub use numwires::Numwires;
pub use register::Register;
pub use traits::{ReadTrait, WriteTrait};

const STARTUP_BIAS_DELAY_MS: u32 = 20;
const ACQUISITION_BIAS_DELAY_MS: u32 = 10;
const CONVERSION_60_HZ_DELAY_MS: u32 = 65;
const CONVERSION_50_HZ_DELAY_MS: u32 = 66;
const FAULT_CYCLE_DELAY_MS: u32 = 1;

/// Temperature, ohms and normalized ratio from one RTD sample.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DebugValue {
    pub temperature: f32,
    pub resistance: f32,
    pub ratio: f32,
}

/// MAX31865 RTD converter, owning SPI, delay and private processing state.
/// Datasheet: 19-6478, Rev 3, pages 13-18.
pub struct Max31865<SPI, Delay>
where
    SPI: SpiDevice,
    Delay: DelayNs,
{
    spi: SPI,
    delay: Delay,
    rtd_conversion: RtdConversion,
    fault_evaluation: FaultEvaluation,
}

impl<SPI: SpiDevice, Delay: DelayNs> WriteTrait for Max31865<SPI, Delay> {
    type InterfaceError = SPI::Error;

    fn write_u8(&mut self, address: Register, data: u8) -> Result<(), Max31865Err<SPI::Error>> {
        comproc::write(&mut self.spi, address.bits(), data)
    }
}

impl<SPI: SpiDevice, Delay: DelayNs> ReadTrait for Max31865<SPI, Delay> {
    type InterfaceError = SPI::Error;

    fn read_u16(&mut self, address: Register) -> Result<u16, Max31865Err<SPI::Error>> {
        let mut bytes = [0; 2];
        self.read_n(address, &mut bytes)?;
        Ok(u16::from_be_bytes(bytes))
    }

    fn read_u8(&mut self, address: Register) -> Result<u8, Max31865Err<SPI::Error>> {
        let mut bytes = [0];
        self.read_n(address, &mut bytes)?;
        Ok(bytes[0])
    }

    fn read_n<const N: usize>(
        &mut self,
        address: Register,
        buffer: &mut [u8; N],
    ) -> Result<(), Max31865Err<SPI::Error>> {
        comproc::read(&mut self.spi, address.bits(), buffer)
    }
}

impl<SPI: SpiDevice, Delay: DelayNs> Max31865<SPI, Delay> {
    /// Legacy constructor, retaining its panic-on-initialization-failure contract.
    ///
    /// # Panics
    /// Panics on invalid resistor values or initialization I/O failure.
    #[deprecated(note = "use try_new to receive initialization errors instead of panicking")]
    pub fn new(
        spi: SPI,
        delay: Delay,
        wire: Numwires,
        rtd_nominal: f32,
        ref_resistor: f32,
    ) -> Self {
        Self::try_new(spi, delay, wire, rtd_nominal, ref_resistor)
            .expect("MAX31865 initialization failed")
    }

    /// Fallible initialization preserving the existing successful startup sequence.
    ///
    /// Sets wires, enables bias, waits 20 ms, enables automatic conversions,
    /// sets raw thresholds to 0000/FFFF and clears latched faults.
    pub fn try_new(
        spi: SPI,
        delay: Delay,
        wire: Numwires,
        rtd_nominal: f32,
        ref_resistor: f32,
    ) -> Result<Self, Max31865Err<SPI::Error>> {
        if !rtd_nominal.is_finite() || rtd_nominal <= 0.0 {
            return Err(Max31865Err::InvalidNominalResistance);
        }
        if !ref_resistor.is_finite() || ref_resistor <= 0.0 {
            return Err(Max31865Err::InvalidReferenceResistance);
        }
        let mut instance = Self {
            spi,
            delay,
            rtd_conversion: RtdConversion {
                reference_resistor: ref_resistor,
                nominal_resistance: rtd_nominal,
            },
            fault_evaluation: FaultEvaluation,
        };
        instance.set_wires(wire)?;
        instance.enable_bias(true)?;
        instance.delay.delay_ms(STARTUP_BIAS_DELAY_MS);
        instance.auto_convert(true)?;
        instance.set_threshold(0, 0xffff)?;
        instance.clear_fault()?;
        #[cfg(feature = "defmt")]
        defmt::debug!("MAX31865 initialized");
        Ok(instance)
    }

    /// Return owned resources without changing hardware configuration.
    pub fn release(self) -> (SPI, Delay) {
        (self.spi, self.delay)
    }

    pub fn read_configuration(&mut self) -> Result<Configuration, Max31865Err<SPI::Error>> {
        self.read_u8(Address::Configuration.into()).map(Into::into)
    }

    /// Raw completed output; reading the pair acknowledges DRDY.
    pub fn read_resistance_register(&mut self) -> Result<RtdResistance, Max31865Err<SPI::Error>> {
        self.read_u16(Address::RtdResistanceMostSignificant.into())
            .map(Into::into)
    }

    /// Read latched faults without clearing them or initiating a detection cycle.
    pub fn read_fault_status(&mut self) -> Result<FaultStatus, Max31865Err<SPI::Error>> {
        self.read_u8(Address::FaultStatus.into()).map(Into::into)
    }

    pub fn read_lower_threshold(&mut self) -> Result<FaultThreshold, Max31865Err<SPI::Error>> {
        self.read_u16(Address::LowFaultThresholdMostSignificant.into())
            .map(Into::into)
    }

    pub fn read_upper_threshold(&mut self) -> Result<FaultThreshold, Max31865Err<SPI::Error>> {
        self.read_u16(Address::HighFaultThresholdMostSignificant.into())
            .map(Into::into)
    }

    pub fn clear_fault(&mut self) -> Result<(), Max31865Err<SPI::Error>> {
        let mut configuration = self.read_configuration()?.stable();
        configuration.set_clear_fault_status(true);
        self.write_configuration(configuration)
    }

    pub fn enable_bias(&mut self, enable: bool) -> Result<(), Max31865Err<SPI::Error>> {
        let mut configuration = self.read_configuration()?.stable();
        configuration.set_bias_enabled(enable);
        self.write_configuration(configuration)
    }

    /// Legacy fault-cycle control; manual stages require application RC delays.
    /// Non-None cycles leave automatic conversion disabled and bias enabled.
    pub fn read_fault(&mut self, cycle: FaultCycle) -> Result<(), Max31865Err<SPI::Error>> {
        if cycle != FaultCycle::None {
            let mut configuration = self.read_configuration()?.stable();
            configuration.set_automatic_conversion(false);
            configuration.set_bias_enabled(true);
            configuration.set_fault_detection_cycle(cycle);
            self.write_configuration(configuration)?;
            if cycle != FaultCycle::ManualRun {
                self.delay.delay_ms(FAULT_CYCLE_DELAY_MS);
            }
        }
        let status = self.read_fault_status()?;
        self.fault_evaluation
            .evaluate(status)
            .map_err(Max31865Err::Hardware)
    }

    pub fn auto_convert(&mut self, enable: bool) -> Result<(), Max31865Err<SPI::Error>> {
        let mut configuration = self.read_configuration()?.stable();
        configuration.set_automatic_conversion(enable);
        self.write_configuration(configuration)
    }

    /// Select notch filter; stop automatic conversion while changing it.
    pub fn enable_50hz(&mut self, enable: bool) -> Result<(), Max31865Err<SPI::Error>> {
        let mut configuration = self.read_configuration()?.stable();
        let filter = if enable {
            MainsFilter::Hertz50
        } else {
            MainsFilter::Hertz60
        };
        let restore_automatic =
            configuration.automatic_conversion() && configuration.mains_filter() != filter;
        if restore_automatic {
            configuration.set_automatic_conversion(false);
            self.write_configuration(configuration)?;
        }
        configuration.set_mains_filter(filter);
        self.write_configuration(configuration)?;
        if restore_automatic {
            configuration.set_automatic_conversion(true);
            self.write_configuration(configuration)?;
        }
        Ok(())
    }

    /// Acquire and return the unshifted 15-bit ratio count.
    pub fn read_rtd(&mut self) -> Result<u16, Max31865Err<SPI::Error>> {
        self.acquire_rtd().map(|register| register.count().value())
    }

    /// Write raw shifted threshold words, preserving the existing LSB-first order.
    /// This is four separate transactions, not an atomic update.
    pub fn set_threshold(&mut self, lower: u16, upper: u16) -> Result<(), Max31865Err<SPI::Error>> {
        let [lower_high, lower_low] = lower.to_be_bytes();
        let [upper_high, upper_low] = upper.to_be_bytes();
        self.write_u8(Address::LowFaultThresholdLeastSignificant.into(), lower_low)?;
        self.write_u8(Address::LowFaultThresholdMostSignificant.into(), lower_high)?;
        self.write_u8(
            Address::HighFaultThresholdLeastSignificant.into(),
            upper_low,
        )?;
        self.write_u8(
            Address::HighFaultThresholdMostSignificant.into(),
            upper_high,
        )
    }

    pub fn get_lower_threshold(&mut self) -> Result<u16, Max31865Err<SPI::Error>> {
        self.read_lower_threshold().map(Into::into)
    }

    pub fn get_upper_threshold(&mut self) -> Result<u16, Max31865Err<SPI::Error>> {
        self.read_upper_threshold().map(Into::into)
    }

    pub fn set_wires(&mut self, num_wire: Numwires) -> Result<(), Max31865Err<SPI::Error>> {
        let mut configuration = self.read_configuration()?.stable();
        configuration.set_three_wire_compensation(num_wire == Numwires::MAX31865_3_WIRE);
        self.write_configuration(configuration)
    }

    pub fn temperature(&mut self) -> Result<f32, Max31865Err<SPI::Error>> {
        let register = self.acquire_rtd()?;
        self.rtd_conversion
            .temperature(register)
            .ok_or(Max31865Err::InvalidConversion)
    }

    pub fn get_resistance(&mut self) -> Result<f32, Max31865Err<SPI::Error>> {
        let register = self.acquire_rtd()?;
        Ok(self.rtd_conversion.resistance(register))
    }

    pub fn get_debug_value(&mut self) -> Result<DebugValue, Max31865Err<SPI::Error>> {
        let register = self.acquire_rtd()?;
        self.rtd_conversion
            .debug_value(register)
            .ok_or(Max31865Err::InvalidConversion)
    }

    pub fn get_ratio(&mut self) -> Result<f32, Max31865Err<SPI::Error>> {
        let register = self.acquire_rtd()?;
        Ok(self.rtd_conversion.ratio(register))
    }

    fn write_configuration(
        &mut self,
        configuration: Configuration,
    ) -> Result<(), Max31865Err<SPI::Error>> {
        self.write_u8(Address::Configuration.into(), configuration.into())
    }

    fn acquire_rtd(&mut self) -> Result<RtdResistance, Max31865Err<SPI::Error>> {
        self.clear_fault()?;
        self.enable_bias(true)?;
        self.delay.delay_ms(ACQUISITION_BIAS_DELAY_MS);
        let mut configuration = self.read_configuration()?.stable();
        configuration.set_one_shot(true);
        self.write_configuration(configuration)?;
        let conversion_delay = match configuration.mains_filter() {
            MainsFilter::Hertz60 => CONVERSION_60_HZ_DELAY_MS,
            MainsFilter::Hertz50 => CONVERSION_50_HZ_DELAY_MS,
        };
        self.delay.delay_ms(conversion_delay);
        let register = self.read_resistance_register()?;
        self.enable_bias(false)?;
        if register.fault() {
            // Inspect the latched cause, rather than opening FORCE- mid-read.
            self.read_fault(FaultCycle::None)?;
            return Err(Max31865Err::Hardware(HardwareErr::UnspecifiedRtdFault));
        }
        Ok(register)
    }
}
