use crate::{
    TLE5012B,
    device::DeviceError,
    models::degree_per_second::DegreePerSecond,
    reg::{
        angle_speed_reg::AngleSpeedRegisterHandler as _, mode1_reg::Mode1RegisterHandler,
        mode2_reg::Mode2RegisterHandler,
    },
    signed_conversion::to_signed,
};

pub trait AngleRangeWriter<SPI: embedded_hal::spi::SpiDevice> {
    fn read_angular_speed(&mut self) -> Result<DegreePerSecond, DeviceError<SPI::Error>>;
}

impl<SPI> AngleRangeWriter<SPI> for TLE5012B<SPI>
where
    SPI: embedded_hal::spi::SpiDevice,
{
    fn read_angular_speed(&mut self) -> Result<DegreePerSecond, DeviceError<<SPI>::Error>> {
        let angular_speed = self.angle_speed_reg.read_angular_speed(&mut self.inner)?;
        let mod2reg = self.mode2_reg.get_mod2_reg(&mut self.inner)?;
        let mod1reg = self.mode1_reg.get_mod1_reg(&mut self.inner)?;
        let angle_range = self.mode2_reg.get_angle_range_as_degree(&mut self.inner)?;
        let update_rate_in_s = mod1reg.update_rate_setting().get_rate_in_us() / 1_000_000.0;

        let signed_angular_speed = to_signed::<15>(angular_speed);

        let time_update_multiplier = match mod2reg.prediction() {
            crate::reg::mode2_reg::Prediction::Disabled => 2,
            crate::reg::mode2_reg::Prediction::Enabled => 3,
        };

        let calculated_speed = (angle_range as f32) / 2u32.pow(15) as f32;
        let calculated_speed = calculated_speed * signed_angular_speed as f32;
        let calculated_speed =
            calculated_speed / (time_update_multiplier as f32 * update_rate_in_s);

        Ok(DegreePerSecond {
            value: calculated_speed,
        })
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;

    #[test]
    fn todo_at_test_for_angular_speed() {}
}
