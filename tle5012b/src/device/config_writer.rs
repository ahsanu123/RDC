use crate::{
    TLE5012B,
    communication::possible_address::PossibleAddress,
    device::{DeviceError, DeviceTrait},
    reg::{mode1_reg::Mode1RegisterStructure, mode2_reg::Mode2RegisterStructure},
};

pub trait ConfigWriter<SPI: embedded_hal::spi::SpiDevice> {
    fn mutate_mod1_reg(
        &mut self,
        mutator_fn: impl FnOnce(&mut Mode1RegisterStructure),
    ) -> Result<Mode1RegisterStructure, DeviceError<SPI::Error>>;

    fn mutate_mod2_reg(
        &mut self,
        mutator_fn: impl FnOnce(&mut Mode2RegisterStructure),
    ) -> Result<Mode1RegisterStructure, DeviceError<SPI::Error>>;
}

impl<SPI> ConfigWriter<SPI> for TLE5012B<SPI>
where
    SPI: embedded_hal::spi::SpiDevice,
{
    fn mutate_mod1_reg(
        &mut self,
        mutator_fn: impl FnOnce(&mut Mode1RegisterStructure),
    ) -> Result<Mode1RegisterStructure, DeviceError<<SPI>::Error>> {
        let _safety_word = self.inner.read_then_mutate(mutator_fn)?;

        // TODO: Check safety word

        let updated_mod1 = self.inner.read(PossibleAddress::Mode1Register)?;
        let updated_mod1 = (updated_mod1 >> 16) as u16;

        let parsed_mod1 = Mode1RegisterStructure::from(updated_mod1);

        Ok(parsed_mod1)
    }

    fn mutate_mod2_reg(
        &mut self,
        mutator_fn: impl FnOnce(&mut Mode2RegisterStructure),
    ) -> Result<Mode1RegisterStructure, DeviceError<<SPI>::Error>> {
        let _safety_word = self.inner.read_then_mutate(mutator_fn)?;

        // TODO: Check safety word

        let updated_mod1 = self.inner.read(PossibleAddress::Mode2Register)?;
        let updated_mod2 = (updated_mod1 >> 16) as u16;

        let parsed_mod2 = Mode1RegisterStructure::from(updated_mod2);

        Ok(parsed_mod2)
    }
}
