use crate::models::degree::Degree;
use crate::models::radiant::Radiant;
use crate::reg::angle_value_reg::ReadStatusAngleValue;
use crate::signed_conversion::to_signed;
use crate::{TLE5012B, device::DeviceError, reg::angle_value_reg::AngleValueRegisterHandler};
use bilge::prelude::*;

pub trait AngleValueReader<SPI: embedded_hal::spi::SpiDevice> {
    fn read_angle_value(&mut self) -> Result<Degree, DeviceError<SPI::Error>>;
    fn check_for_new_angle_value(&mut self) -> Result<Option<Degree>, DeviceError<<SPI>::Error>>;
    fn convert_to_angle(&mut self, raw_val: u15) -> f32 {
        let numerator = 360.0;
        let denominator = 2_u32.pow(15) as f32;

        let signed_val = to_signed::<15>(raw_val);

        (numerator / denominator) * signed_val as f32
    }
}

impl<SPI> AngleValueReader<SPI> for TLE5012B<SPI>
where
    SPI: embedded_hal::spi::SpiDevice,
{
    fn read_angle_value(&mut self) -> Result<Degree, DeviceError<SPI::Error>> {
        let raw_val: u15 = self.angle_value_reg.read_raw_angval(&mut self.inner)?;
        let result = self.convert_to_angle(raw_val);
        Ok(Degree { value: result })
    }

    fn check_for_new_angle_value(&mut self) -> Result<Option<Degree>, DeviceError<<SPI>::Error>> {
        let reg_struct = self.angle_value_reg.get_reg_value(&mut self.inner)?;

        match reg_struct.read_status_angle_value() {
            ReadStatusAngleValue::NoNewAngleValue => Ok(None),
            ReadStatusAngleValue::NewAngleValuePresent => {
                let raw_val: u15 = reg_struct.calculated_angle_value();
                let result = self.convert_to_angle(raw_val);

                let some_degree = Some(Degree { value: result });

                Ok(some_degree)
            }
        }
    }
}

impl From<Degree> for Radiant {
    fn from(value: Degree) -> Self {
        Radiant {
            value: value.value.to_radians(),
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;

    #[test]
    fn test_angle_value_reading() {
        // value choose based on tle5012b manual on page 70
        let raw_val = u15::new(0b100_1101_1001_0011);

        let numerator = 360.0;
        let denominator = 2_u32.pow(15) as f32;

        let signed_val = to_signed::<15>(raw_val);

        let result = (numerator / denominator) * signed_val as f32;

        std::println!("calculated angle: {}", result);
        assert!(result <= -141.82);
    }
}
