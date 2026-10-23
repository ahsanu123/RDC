use crate::{
    ENCODED_BYTES_PER_PIXEL, RgbColor,
    procedure::{ONE_SYMBOL, ZERO_SYMBOL},
};

/// GRB, MSB-first symbol encoding (WS2812B-2020, pages 4-5).
#[derive(Default)]
pub(crate) struct ColorEncoding;

impl ColorEncoding {
    pub(crate) fn encode(&self, color: RgbColor) -> [u8; ENCODED_BYTES_PER_PIXEL] {
        let mut encoded = [0; ENCODED_BYTES_PER_PIXEL];
        for (channel, bytes) in [color.g, color.r, color.b]
            .into_iter()
            .zip(encoded.as_chunks_mut::<4>().0)
        {
            for (index, byte) in bytes.iter_mut().enumerate() {
                let symbol = |bit| {
                    if channel & (1u8 << bit) != 0 {
                        ONE_SYMBOL
                    } else {
                        ZERO_SYMBOL
                    }
                };
                *byte = (symbol(7 - 2 * index) << 4) | symbol(6 - 2 * index);
            }
        }
        encoded
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_encoding_places_green_red_blue_in_contiguous_ranges() {
        assert_eq!(
            ColorEncoding.encode(RgbColor::new(0x01, 0x80, 0x55)),
            [
                0xc8, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x8c, 0x8c, 0x8c, 0x8c, 0x8c
            ]
        );
    }

    #[test]
    fn color_encoding_round_trips_every_channel_value_msb_first() {
        for value in 0..=u8::MAX {
            for channel in 0..3 {
                let mut channels = [0; 3];
                channels[channel] = value;
                let encoded =
                    ColorEncoding.encode(RgbColor::new(channels[1], channels[0], channels[2]));
                for (index, bytes) in encoded.as_chunks::<4>().0.iter().enumerate() {
                    let mut decoded = 0u8;
                    for byte in bytes {
                        for symbol in [byte >> 4, byte & 0x0f] {
                            assert!(matches!(symbol, 0b1000 | 0b1100));
                            decoded = (decoded << 1) | u8::from(symbol == 0b1100);
                        }
                    }
                    assert_eq!(decoded, channels[index]);
                }
            }
        }
    }

    #[test]
    fn color_encoding_symbols_meet_datasheet_timing_at_both_clock_bounds() {
        for frequency in [3_000_000, 3_200_000] {
            for (value, minimum_high, maximum_high, minimum_low, maximum_low) in
                [(0, 220, 380, 580, 1000), (255, 580, 1000, 580, 1000)]
            {
                let encoded = ColorEncoding.encode(RgbColor::new(value, value, value));
                for byte in encoded {
                    for symbol in [byte >> 4, byte & 0x0f] {
                        let high = (symbol << 4).leading_ones();
                        let low = 4 - high;
                        let within = |ticks: u32, minimum: u32, maximum: u32| {
                            let scaled = u64::from(ticks) * 1_000_000_000;
                            assert!(scaled >= u64::from(minimum) * frequency);
                            assert!(scaled <= u64::from(maximum) * frequency);
                        };
                        within(high, minimum_high, maximum_high);
                        within(low, minimum_low, maximum_low);
                        assert!(4_000_000_000u64 >= 1250 * frequency);
                    }
                }
                assert!(crate::RESET_BYTES as u64 * 8 * 1_000_000 > 280 * frequency);
            }
        }
    }
}
