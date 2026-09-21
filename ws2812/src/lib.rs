#![no_std]

pub struct Nanos {
    pub value: u32,
}

pub struct Constant {}

// based on WS2812B-2020
// non 3.3V compitable
impl Constant {
    pub const T0H: Nanos = Nanos { value: 350 };
    pub const T1H: Nanos = Nanos { value: 2 * 350 };
    pub const T0L: Nanos = Nanos { value: 2 * 350 };
    pub const T1L: Nanos = Nanos { value: 2 * 350 };
    pub const RES: Nanos = Nanos { value: 850 * 350 }; // 297.5us

    pub const HIGH: u8 = 0x0C;
    pub const LOW: u8 = 0x08;
}

pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGB {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

pub struct Ws2812<SPI>
where
    SPI: embedded_hal::spi::SpiDevice,
{
    spi: SPI,
}

impl<SPI> Ws2812<SPI>
where
    SPI: embedded_hal::spi::SpiDevice,
{
    // TODO: make sure timing is correct,
    // and follow constant config
    // 3.2Mhz
    pub fn new(spi: SPI) -> Self {
        Self { spi }
    }

    pub fn write_colors<const COLOR_COUNTS: usize>(
        &mut self,
        colors: &[RGB; COLOR_COUNTS],
    ) -> Result<(), SPI::Error> {
        for color in colors.iter() {
            self.write_color(color)?;
        }
        Ok(())
    }

    pub fn write_color(&mut self, color: &RGB) -> Result<(), SPI::Error> {
        let grb = self.build_rgb(color);

        self.spi.write(&grb)?;

        Ok(())
    }

    fn build_rgb(&self, rgb: &RGB) -> [u8; 12] {
        // ws2812 expects GRB order, not RGB
        // so convert it into GRB

        let mut grb = [0u8; 12];

        let r = self.build_byte(rgb.r);
        let g = self.build_byte(rgb.g);
        let b = self.build_byte(rgb.b);

        grb[0..4].copy_from_slice(&g);
        grb[5..8].copy_from_slice(&r);
        grb[9..12].copy_from_slice(&b);

        grb
    }

    fn build_byte(&self, byte: u8) -> [u8; 4] {
        let mut data: [u8; 4] = [0; 4];

        (0..8).for_each(|i| {
            // NOTE:
            // use != 0 , because if == 1 it only work on first bit
            // remember shifting one to left if bit is 1 its not == 1 but its may be 2
            // or more!, based on bit position,
            // so == 1 is not suitable here.
            let nibble = if byte & (1 << i) != 0 {
                Constant::HIGH
            } else {
                Constant::LOW
            };

            // high nibble leaves the SPI peripheral first.
            if i % 2 == 0 {
                data[i / 2] |= nibble << 4;
            } else {
                data[i / 2] |= nibble;
            }
        });
        data
    }
}
