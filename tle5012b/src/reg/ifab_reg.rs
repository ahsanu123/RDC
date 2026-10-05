use crate::{
    communication::possible_address::PossibleAddress, device::DeviceTrait,
    reg::prelude::RegisterFromRaw,
};
use bilge::prelude::*;
use bitflags::bitflags;

// Field Bits Type Description
//
// ORTHO 15:4 w Orthogonality Correction of X and Y Components
//     12-bit signed integer value of orthogonality correction.
//     GMR element orthogonality correction.
//     +2047D 11.2445°
//     0D 0°
//     -2048D -11.2500°
//     Reset: device-specific
// FIR_UDR 3 w FIR Update Rate
//     Initial filter update rate (FIR) setting to be loaded into
//     FIR_MD on startup. Changing of the FIR setting can only
//     be done by writing to the FIR_MD bits via SPI after
//     power-on.
//     0B FIR_MD = ‘10’ (85.3 µs)
//     1B FIR_MD = ‘01’ (42.7 µs)
//     Reset: derivate-specific
// IFAB_OD 2 w IFA,IFB,IFC Output Mode
//     0B Push-Pull
//     1B Open Drain
//     Reset: derivate-specific
// IFAB_HYST (multi-purpose) 1:0 w HSM and IIF Mode: Hysteresis
//     Switching hysteresis on direction change for HSM and IIF
//     interface.
//     00B 0°
//     01B 0.175°
//     10B 0.35°
//     11B 0.70°
//     SPC Mode: Unit Time
//     00B 3.0 µs
//     01B 2.5 µs
//     10B 2.0 µs
//     11B 1.5 µs
//     Reset: derivate-specific

bitflags! {
    // address = 0DH
    //
    // ru = read or upate
    // r = readonly
    // w = writeable
    // wu = writeable and update
    #[derive(Default)]
    pub struct IFABRegister: u16{
                                                                 //FEDC_BA98_7654_3210
        const ORTHOGONALITY_CORRECTION_OF_X_AND_Y_COMPONENTS_W = 0b1111_1111_1111_0000;
        const FIR_UPDATE_RATE                                  = 0b0000_0000_0000_1000;
        const IFA_IFB_IFC_OUTPUT_MODE                          = 0b0000_0000_0000_0100;
        const HSM_AND_IIF_MODE                                 = 0b0000_0000_0000_0011;
    }
}

#[allow(unused)]
pub trait IFABRegisterHandler<SPI, DEVICE>
where
    SPI: embedded_hal::spi::SpiDevice,
    DEVICE: DeviceTrait<SPI>,
{
    fn read_status(&mut self, dev: &mut DEVICE) -> Result<(), SPI::Error>;
    fn write_slave_number(&mut self, dev: &mut DEVICE, number: u2) -> Result<(), SPI::Error>;
}

impl<SPI, DEVICE> IFABRegisterHandler<SPI, DEVICE> for IFABRegister
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

impl RegisterFromRaw for IFABRegisterStructure {
    const ADDRESS: PossibleAddress = PossibleAddress::IfabRegister;

    fn into_u16(self) -> u16 {
        self.value
    }
}

#[bitsize(16)]
#[derive(FromBits)]
// LSB field write first
pub struct IFABRegisterStructure {
    hsm_and_iif_unit_time: HSMAndIIFModeHysteresisOrSPCUnitTime,
    ifa_ifb_ifc_output_mode: IFAIFBIFCOutputMode,
    fir_update_rate: FIRUpdateRate,
    orthogonality_correction_of_x_and_y_components: u12,
}

#[bitsize(2)]
#[derive(FromBits)]
pub enum HSMAndIIFModeHysteresisOrSPCUnitTime {
    Deg0OrUs3_0 = 0,
    Deg0_175OrUs2_5,
    Deg0_35OrUs2_0,
    Deg0_70OrUs1_5,
}

#[bitsize(1)]
#[derive(FromBits)]
pub enum IFAIFBIFCOutputMode {
    PushPull = 0,
    OpenDrain,
}

#[bitsize(1)]
#[derive(FromBits)]
pub enum FIRUpdateRate {
    FirMd10Us85_3 = 0,
    FirMd01Us42_7,
}
