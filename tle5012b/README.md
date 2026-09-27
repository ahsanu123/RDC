
## Note 

Concept learned from this project 

**Associated Constant**

```rust

pub trait RegisterFromRaw: From<u16> {
    const ADDRESS: PossibleAddress;
    fn into_u16(self) -> u16;
}

// differentiate with ADDRESS
impl RegisterFromRaw for StatusRegisterStructure {
    const ADDRESS: PossibleAddress = PossibleAddress::StatusRegister;

    fn into_u16(self) -> u16 {
        self.value
    }
}

// still generic here
fn read_then_mutate<T>(
    &mut self,
    address: PossibleAddress,
    mutator_fn: impl FnOnce(&mut T),
) -> Result<SafetyWord, <SPI>::Error>
where
    T: RegisterFromRaw,
{
    let raw_val = self.read(address.clone())?;
    let (mut reg_val, safety_word) = resulted_register::<T>(raw_val);

    (...)

    Ok(SafetyWord::from(write_result))
}

// Concrete Type in function user 
pub fn change_config(&mut self) {
    self.read_then_mutate::<StatusRegisterStructure>(
        PossibleAddress::StatusRegister,
        |status_reg| {
            status_reg.set_status_reset(StatusReset::ResetHappen);
        },
    );
}
```

## Good Reference to look 

- https://github.com/nisembedded/tle5012/tree/master
- official cpp driver https://github.com/Infineon/arduino-xensiv-angle-sensor-tlx5012/tree/main
- bible: https://www.farnell.com/datasheets/2700287.pdf
