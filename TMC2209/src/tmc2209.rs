use embedded_hal::digital::OutputPin;
pub struct TMC2209 {}

impl TMC2209 {}

impl TMC2209 {
    pub fn new() -> Self {
        todo!()
    }

    // ===========================

    pub fn set_hardware_enable_pin(mut enable_pin: impl OutputPin) {
        let _ = enable_pin.set_high();
    }

    // ===========================

    pub fn enable() {
        todo!()
    }

    pub fn disable() {
        todo!()
    }

    // ===========================

    pub fn set_microsteps_per_step(microstep_per_step: u16) {
        todo!()
    }

    pub fn set_microsteps_per_step_power_of_two(exponent: u8) {
        todo!()
    }

    pub fn set_run_current(percent: u8) {
        todo!()
    }

    pub fn set_hold_current(percent: u8) {
        todo!()
    }

    pub fn set_all_current_values(percent: u8) {
        todo!()
    }
}

impl Default for TMC2209 {
    fn default() -> Self {
        Self {}
    }
}
