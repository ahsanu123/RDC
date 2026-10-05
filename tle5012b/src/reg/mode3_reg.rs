use crate::{
    communication::possible_address::PossibleAddress, device::DeviceTrait,
    reg::prelude::RegisterFromRaw,
};
use bilge::prelude::*;
use bitflags::bitflags;

// Field Bits Type Description
//
// ANG_BASE 15:4 w Angle Base
//     Sets the 0° angle position (12 bit value). Angle base is
//     factory-calibrated to make the 0° direction parallel to the
//     edge of the chip.
//     800H -180°
//     000H 0°
//     7FFH +179.912°
//     Reset: device-specific
// SPIKEF 3 w Analog Spike Filter of Input Pads
//     Filters voltage spikes on input pads (IFC, SCK and CSQ).
//     Additional delay of 10 µs for data input.
//     0B spike filter disabled
//     1B spike filter enabled
//     Reset: derivate-specific
// SSC_OD 2 w SSC-Interface Data Pin Output Mode
//     0B Push-Pull
//     1B Open Drain
//     Reset: 0B
// PAD_DRV 1:0 w Configuration of Pad-Driver
//     00B IFA/IFB/IFC: strong driver, DATA: strong driver,
//     fast edge
//     01B IFA/IFB/IFC: strong driver, DATA: strong driver,
//     slow edge
//     10B IFA/IFB/IFC: weak driver, DATA: medium driver,
//     fast edge
//     11B IFA/IFB/IFC: weak driver, DATA: weak driver, slow
//     edge
//     Reset: derivate-specific

bitflags! {
    // address = 09H
    //
    // ru = read or upate
    // r = readonly
    // w = writeable
    // wu = writeable and update
    #[derive(Default)]
    pub struct Mode3Register: u16{
                                                        //FEDC_BA98_7654_3210
        const ANGLE_BASE_W                            = 0b1111_1111_1111_0000;
        const ANALOG_SPIKE_FILTER_OF_INPUT_PADS_W     = 0b0000_0000_0000_1000;
        const SSC_INTERFACE_DATA_PIN_OUTPUT_MODE_W    = 0b0000_0000_0000_0100;
        const CONFIGURATION_OF_PAD_DRIVER_W           = 0b0000_0000_0000_0011;
    }
}

#[allow(unused)]
pub trait Mode3RegisterHandler<SPI, DEVICE>
where
    SPI: embedded_hal::spi::SpiDevice,
    DEVICE: DeviceTrait<SPI>,
{
    fn read_status(&mut self, dev: &mut DEVICE) -> Result<(), SPI::Error>;
    fn write_slave_number(&mut self, dev: &mut DEVICE, number: u2) -> Result<(), SPI::Error>;
}

impl<SPI, DEVICE> Mode3RegisterHandler<SPI, DEVICE> for Mode3Register
where
    SPI: embedded_hal::spi::SpiDevice,
    DEVICE: DeviceTrait<SPI>,
{
    fn read_status(&mut self, dev: &mut DEVICE) -> Result<(), SPI::Error> {
        todo!()
    }

    fn write_slave_number(&mut self, dev: &mut DEVICE, number: u2) -> Result<(), SPI::Error> {
        todo!()
    }
}

impl RegisterFromRaw for Mode3RegisterStructure {
    const ADDRESS: PossibleAddress = PossibleAddress::Mode3Register;

    fn into_u16(self) -> u16 {
        self.value
    }
}

#[bitsize(16)]
#[derive(FromBits)]
// LSB field write first
pub struct Mode3RegisterStructure {
    configuration_of_pad_driver: ConfigurationOfPadDriver,
    ssc_interface_data_pin_output_mode: SSCInterfaceDataPinOutputMode,
    analog_spike_filter_of_input_pads: AnalogSpikeFilterOfInputPads,
    angle_base: u12,
}

#[bitsize(2)]
#[derive(FromBits)]
pub enum ConfigurationOfPadDriver {
    StrongDataStrongFastEdge = 0,
    StrongDataStrongSlowEdge,
    WeakDataMediumFastEdge,
    WeakDataWeakSlowEdge,
}

#[bitsize(1)]
#[derive(FromBits)]
pub enum SSCInterfaceDataPinOutputMode {
    PushPull = 0,
    OpenDrain,
}

#[bitsize(1)]
#[derive(FromBits)]
pub enum AnalogSpikeFilterOfInputPads {
    Disabled = 0,
    Enabled,
}
