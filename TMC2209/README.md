# TMC2209

Allocation-free, `no_std`, blocking UART driver for the TMC2209. It uses
`embedded-hal` 1.0 `DelayNs`, `embedded-hal-nb` 1.0 serial `Read`/`Write`, bilge
register types, bounded polling, reply/CRC validation, echoed/non-echoed UART
topologies and IFCNT-verified writes. Optional `defmt` logging supplements error
returns. All 24 documented registers are typed; access traits distinguish
readable and writable words. GSTAT is write-one-clear; OTP uses a separate,
explicit programming operation.

This completes the pre-existing `TMC2209/` scaffold, retaining its module path
and adding a root `TMC2209` export. The former argument-free constructor,
empty methods and error-discarding enable-pin placeholder are replaced by real
resource-owning methods. Hardware ENN and STEP/DIR pins remain application-owned.

## Connect without starting the motor

```rust
use embedded_hal::delay::DelayNs;
use embedded_hal_nb::serial::{Read, Write};
use tmc2209::{
    builder::Builder, CommunicationSettings, DeviceError, MicrostepResolution,
    register::DriverStatus, u2, u4, u5,
};

fn configure<U: Read<u8> + Write<u8>, D: DelayNs>(
    uart: U,
    delay: D,
) -> Result<(), DeviceError<U::Error>> {
    // Application keeps hardware ENN high and STEP inactive during configuration.
    let settings = CommunicationSettings::builder()
        .address(u2::new(0))
        .build();
    let mut device = Builder::builder()
        .uart(uart)
        .delay(delay)
        .settings(settings)
        .build()
        .connect()?;
    device.configure_uart(u4::new(2))?;
    device.set_currents(u5::new(8), u5::new(16), u4::new(2))?;
    device.set_microsteps(MicrostepResolution::Microsteps16)?;
    let _status = device.read_register::<DriverStatus>()?;
    let (_uart, _delay) = device.release();
    Ok(())
}
```

The default echo mode matches the shared TX/RX resistor circuit. Use
`CommunicationSettings::builder().echo(EchoMode::Disabled)` only when the
application's configured UART/adapter suppresses TX echo. Configure UART hardware
to the same baud as settings; the driver cannot change peripheral registers.

## Typed register access

```rust
use embedded_hal::delay::DelayNs;
use embedded_hal_nb::serial::{Read, Write};
use tmc2209::{DeviceError, TMC2209, register::PwmConfiguration};

fn configure_pwm<U: Read<u8> + Write<u8>, D: DelayNs>(
    device: &mut TMC2209<U, D>,
) -> Result<(), DeviceError<U::Error>> {
    let mut configuration = device.read_register::<PwmConfiguration>()?;
    configuration.set_automatic_amplitude(true);
    device.write_register(configuration)?;
    Ok(())
}
```

Write-only registers such as `CurrentControl` cannot be read, and must be
provided as complete values. The device holds no fabricated register shadows.
GSTAT acknowledgement writes only requested bits; it is not read-modify-write.
Reserved bits, TEST_MODE, invalid microstep/off-time/hysteresis settings and
incompatible double-edge/filter configuration are rejected.

**Motion is explicit:** `enable` can energize the motor and `set_velocity` can
start the pulse generator. `disable` turns bridges off but does not clear
VACTUAL. VACTUAL=0 selects external STEP/DIR; it does not stop external STEP
pulses. Never rely on releasing/dropping the object to stop hardware.

## Errors and recovery

Every fallible method returns `DeviceError<U::Error>`. `Interface(error)` owns
the original UART error, including non-`Copy` payloads; it is never replaced by
`ErrorKind`, a boolean result, or logging-only handling.

Specific variants distinguish polling timeout (TX/RX/flush), echo mismatch,
bad sync/master/register/CRC, unsupported silicon version, missing write
acknowledgement, invalid settings/register fields, unavailable measurements,
unsupported current estimation, and OTP verification failures.

There are no automatic read/write retries or silent defaults. After an
incomplete exchange, call `recover_uart` with exclusive bus ownership, then
reconcile any uncertain write before retrying. A later error in a multi-step
operation does not undo earlier writes. A failed constructor consumes resources;
borrow/shared-own the UART if it must be recovered outside the constructor.

## Hardware and scope

- Follow the manufacturer's PDN_UART resistor/buffer circuit, voltage/current,
  clock, grounding and supply requirements. A UART adapter with automatic
  half-duplex direction must release TX after the final stop bit.
- Hold ENN disabled during configuration. The **chip's** default state may
  energize a connected motor if ENN is low, even though connection is read-only.
- Use address pins 0-3 and SENDDELAY>=2 on shared buses. Exclusively lock the
  physical UART for complete driver operations, including counter verification.
- Defaults assume 115200 baud and a 12 MHz device clock. Clock trim or an
  external clock changes frequency/current-control timing assumptions; reconnect
  with matching settings after calibration. Baud must be >=9000 and <=fCLK/16.
- Polling is bounded per byte/flush, not a hard real-time guarantee against a
  blocking HAL or scheduler stalls. See PROCEDURE.md for the timing budget.
- Current estimation supports external Rsense with analog scaling disabled;
  signed phase table values are not measured amperes. Respect sense/motor/board
  limits independently of a nominal calculated current.
- OTP programming is irreversible, explicit and not performed by ordinary
  writes or setup. Read the manufacturer programming procedure before calling it.
- This is the TMC2209 UART device driver, not a TMC2208/TMC2225 driver, motion
  planner, async UART adapter or automatic motor-tuning controller.

See [REGISTERS.md](REGISTERS.md), [FUNCTIONS.md](FUNCTIONS.md) and
[PROCEDURE.md](PROCEDURE.md) for every field, read requirement, equation and
communication/programming procedure.

## Struct and function connections

These diagrams cover every library struct and handwritten function, including
private helpers. Bilge field constructors/getters/setters and typed-builder
methods are grouped; compiler-derived formatting/equality/copying and test-only
helpers are omitted. Arrows identify ownership, calls or passed values as labeled.

### Ownership, construction and release

```mermaid
flowchart TD
    builder["Builder&lt;UART, DELAY&gt;"]
    settings["CommunicationSettings"]
    device["TMC2209&lt;UART, DELAY&gt;"]
    transport["UartTransport&lt;UART, DELAY&gt;<br/>private comproc.rs struct"]
    processing["RegisterProcessing<br/>private function.rs struct"]
    builderApi["Builder::builder / uart / delay / settings / build<br/>generated typed-builder methods"]
    settingsApi["CommunicationSettings::builder / setters / build<br/>generated typed-builder methods"]
    settingsDefault["CommunicationSettings::default"]
    settingsValidate["CommunicationSettings::validate"]
    connect["Builder::connect"]
    new["TMC2209::new"]
    transportNew["UartTransport::new"]

    builderApi -->|"constructs"| builder
    settingsApi -->|"constructs"| settings
    settingsDefault --> settingsApi
    builder -->|"owns UART, delay and settings until connect"| connect
    settings -->|"connection parameter"| new
    connect --> new
    new -->|"constructs transport"| transportNew
    transportNew -->|"before I/O"| settingsValidate
    transportNew -->|"owns resources and settings"| transport
    new -->|"reads IOIN, requires VERSION=0x21"| identity["TMC2209::read_register&lt;InputStatus&gt;"]
    new -->|"success"| device
    device -->|"owns"| transport
    device -->|"owns"| processing
    transport -->|"owns"| uart["embedded_hal_nb::serial::Read + Write"]
    transport -->|"owns"| delay["embedded_hal::delay::DelayNs"]
    device -->|"consumed by"| release["TMC2209::release"]
    release --> transportRelease["UartTransport::release"]
    transportRelease -->|"returns"| resources["UART and DELAY"]
```

### All 24 register structs and access permissions

Every register is a bilge 32-bit struct with `R::new`, field getters/setters,
`R::from(u32)` and `u32::from(R)` / `R::into`. Generated setters only change a
local value, not hardware access rights. The sealed `Register` trait supplies
address/reset metadata; read/write traits gate the device methods.

```mermaid
flowchart TD
    subgraph rw["Readable and writable structs"]
        general["GeneralConfiguration 0x00"]
        status["GlobalStatus 0x01<br/>write-one-clear"]
        factory["FactoryConfiguration 0x07"]
        chopper["ChopperConfiguration 0x6C"]
        pwm["PwmConfiguration 0x70"]
    end
    subgraph ro["Read-only structs"]
        counter["InterfaceCounter 0x02"]
        otp["OtpConfiguration 0x05"]
        input["InputStatus 0x06"]
        interval["StepInterval 0x12"]
        stallResult["StallDetectionResult 0x41"]
        microCounter["MicrostepCounter 0x6A"]
        microCurrent["MicrostepCurrent 0x6B"]
        driver["DriverStatus 0x6F"]
        scaling["PwmScaling 0x71"]
        automatic["AutomaticPwmValues 0x72"]
    end
    subgraph wo["Write-only structs"]
        node["NodeConfiguration 0x03"]
        currents["CurrentControl 0x10"]
        powerDown["PowerDownDelay 0x11"]
        pwmThreshold["PwmVelocityThreshold 0x13"]
        currentThreshold["SmartCurrentVelocityThreshold 0x14"]
        velocity["TargetVelocity 0x22"]
        stallThreshold["StallDetectionThreshold 0x40"]
        smartCurrent["SmartCurrentConfiguration 0x42"]
    end
    command["OtpProgramming 0x04<br/>irreversible command"]

    rw -.-> readable["ReadableRegister"]
    ro -.-> readable
    rw -.-> writable["WritableRegister<br/>WRITE_MASK"]
    wo -.-> writable
    readable -->|"extends"| register["Register<br/>ADDRESS and RESET"]
    writable -->|"extends"| register
    command -.-> register
    readable --> read["TMC2209::read_register"]
    writable --> write["TMC2209::write_register"]
    command -->|"constructed only by explicit operation"| program["TMC2209::program_otp_bit"]
    chopper -->|"typed setter argument; invalid raw read encodings rejected"| resolution["MicrostepResolution enum<br/>TryFromBits and Into u4"]
    pwm -->|"typed field"| standstill["StandstillMode enum<br/>From and Into u2"]
    register -.->|"bilge implementations on each struct"| generated["R::new / field / set_field<br/>From and Into u32"]
```

### Configuration methods and verified writes

Setter edge labels name the affected register. `validate_write` may read the
other configuration register to reject double-edge/filter conflicts. Every
actual write is checked against IFCNT; returning an error does not roll back
an already-transmitted write.

```mermaid
flowchart TD
    configure["TMC2209::configure_uart"]
    currents["TMC2209::set_currents"]
    microsteps["TMC2209::set_microsteps"]
    enable["TMC2209::enable"]
    disable["TMC2209::disable"]
    velocity["TMC2209::set_velocity"]
    direction["TMC2209::set_reversed_direction"]
    spread["TMC2209::set_spread_cycle"]
    standstill["TMC2209::set_standstill_mode"]
    clear["TMC2209::clear_status_flags"]
    read["TMC2209::read_register&lt;R&gt;"]
    write["TMC2209::write_register&lt;R&gt;"]
    validate["TMC2209::validate_write"]
    checked["TMC2209::write_checked"]
    verify["TMC2209::verify_write_counter"]

    configure -->|"read GCONF"| read
    configure -->|"write GCONF then NODECONF"| write
    currents -->|"complete CurrentControl word"| write
    microsteps -->|"read CHOPCONF and GCONF"| read
    microsteps -->|"write CHOPCONF then GCONF"| write
    enable -->|"read CHOPCONF"| read
    disable -->|"read CHOPCONF"| read
    enable -->|"nonzero TOFF"| write
    disable -->|"TOFF=0"| write
    velocity -->|"signed TargetVelocity word"| write
    direction -->|"read GCONF"| read
    spread -->|"read GCONF"| read
    direction -->|"modify direction"| write
    spread -->|"modify mode"| write
    standstill -->|"read PWMCONF"| read
    standstill -->|"modify FREEWHEEL"| write
    clear -->|"selected GlobalStatus bits, no status read"| write
    read -->|"ADDRESS, then convert returned u32 to R"| transportRead["UartTransport::read"]
    write -->|"reject bits outside WRITE_MASK"| validate
    validate -->|"conditional GCONF / CHOPCONF conflict check"| read
    validate -->|"valid value returns to write_register"| write
    write -->|"after validation"| checked
    checked -->|"1: IFCNT before"| read
    checked -->|"2: register address and serialized value"| transportWrite["UartTransport::write"]
    checked -->|"3: expected counter modulo 256"| verify
    verify -->|"IFCNT after"| read
    verify -->|"mismatch"| error["DeviceError::WriteNotAcknowledged"]
```

### Register-result processing and physical quantities

Device methods perform reads before invoking the pure `RegisterProcessing`
methods. User-supplied resistance and the configured clock never enter those
pure functions.

```mermaid
flowchart LR
    microsteps["TMC2209::read_microsteps"]
    phases["TMC2209::read_phase_currents"]
    correction["TMC2209::read_pwm_correction"]
    frequency["TMC2209::measured_256_microstep_frequency_hz"]
    current["TMC2209::estimated_rms_current_amperes"]
    read["TMC2209::read_register"]
    microCount["RegisterProcessing::microstep_count"]
    phaseValues["RegisterProcessing::phase_currents"]
    pwmValue["RegisterProcessing::pwm_correction"]
    period["RegisterProcessing::step_clock_period"]
    voltage["RegisterProcessing::rms_sense_voltage"]
    sign["signed_nine_bits<br/>private helper"]

    microsteps -->|"GCONF selects registers, then CHOPCONF"| read
    phases -->|"MicrostepCurrent"| read
    correction -->|"PwmScaling"| read
    frequency -->|"StepInterval"| read
    current -->|"GCONF, CHOPCONF, DriverStatus"| read
    microsteps -->|"CHOPCONF read result"| microCount
    phases -->|"signed phase table read result"| phaseValues
    correction -->|"PWM_SCALE read result"| pwmValue
    frequency -->|"TSTEP read result"| period
    current -->|"CHOPCONF and DRV_STATUS read results"| voltage
    phaseValues --> sign
    pwmValue --> sign
    frequency -->|"clock for division by returned period"| clock["UartTransport::clock_hz"]
    voltage -->|"device divides by Rsense + 0.020 ohms"| current
    processing["RegisterProcessing"] -.->|"derived; not used during connection"| defaultProcessing["RegisterProcessing::default"]
```

### UART framing, validation and error forwarding

```mermaid
flowchart TD
    read["UartTransport::read"]
    write["UartTransport::write"]
    wait["UartTransport::wait_bit_times"]
    send["UartTransport::send_frame"]
    receive["UartTransport::receive_byte"]
    poll["UartTransport::poll"]
    crc["crc8<br/>validation.rs, LSB-first CRC8-ATM"]
    validate["validate_reply<br/>sync, master, register, CRC and big-endian data"]
    hal["UART Read::read / Write::write / Write::flush"]
    pause["DelayNs::delay_us"]
    errorFrom["DeviceError::from"]
    report["DeviceError::reported"]

    read -->|"16 idle bit times"| wait
    write -->|"16 idle bit times"| wait
    wait --> pause
    read -->|"four-byte request CRC"| crc
    write -->|"eight-byte write CRC"| crc
    read --> send
    write --> send
    send -->|"each TX byte and final flush"| poll
    send -->|"if EchoMode::Enabled, check each byte"| receive
    read -->|"eight reply bytes"| receive
    receive -->|"bounded RX polling"| poll
    poll --> hal
    poll -->|"WouldBlock, attempts remain"| pause
    poll -->|"original HAL error"| errorFrom
    poll -->|"budget exhausted"| timeout["DeviceError::Timeout with UartOperation"]
    read -->|"complete reply"| validate
    validate -->|"recompute reply CRC"| crc
    validate -->|"invalid reply"| protocolError["Specific DeviceError variant"]
    errorFrom --> report
    timeout --> report
    protocolError --> report
    report -->|"unchanged error returned, never log-only"| caller["Calling device method / application"]
```

### Explicit OTP programming and UART recovery

Numbered OTP edges show order; every fallible step propagates its error without
retrying. Recovery deliberately discards RX data, not errors, and reports how
many bytes were discarded.

```mermaid
flowchart TD
    program["TMC2209::program_otp_bit"]
    readOtp["TMC2209::read_register&lt;OtpConfiguration&gt;"]
    readCounter["TMC2209::read_register&lt;InterfaceCounter&gt;"]
    write["UartTransport::write<br/>OtpProgramming with key 0xBD"]
    wait["UartTransport::delay_programming"]
    verify["TMC2209::verify_write_counter"]
    refresh["UartTransport::read at 0x04<br/>refresh OTP_READ"]

    program -->|"1: validate location and confirm bit is clear"| readOtp
    program -->|"2: record counter"| readCounter
    program -->|"3: irreversible write"| write
    program -->|"4: wait at least 10 ms"| wait
    wait --> delay["DelayNs::delay_ms"]
    program -->|"5: check accepted write"| verify
    program -->|"6: refresh"| refresh
    program -->|"7: verify selected bit set"| readOtp

    recover["TMC2209::recover_uart"] --> transportRecover["UartTransport::recover"]
    transportRecover -->|"1: flush TX"| poll["UartTransport::poll"]
    transportRecover -->|"2: wait 80 bit times"| idle["UartTransport::wait_bit_times"]
    transportRecover -->|"3: bounded RX drain"| receive["UART Read::read"]
    receive -->|"WouldBlock"| count["Return discarded count"]
    receive -->|"real interface error"| error["DeviceError::from"]
    transportRecover -->|"drain budget exhausted"| limit["DeviceError::RecoveryLimitExceeded"]
```

## Validation

```text
cargo test --manifest-path TMC2209/Cargo.toml --no-default-features
cargo test --manifest-path TMC2209/Cargo.toml --all-features
cargo clippy --manifest-path TMC2209/Cargo.toml --all-targets --all-features -- -D warnings
cargo fmt --manifest-path TMC2209/Cargo.toml -- --check
cargo build --manifest-path TMC2209/Cargo.toml --lib --target thumbv7em-none-eabi --all-features
```

Install the desired bare-metal Rust target if missing. Host mocks check UART
framing, echo, bounded polling, error forwarding, counter rollover and typed
device operations. CRC and all pure processing functions have focused tests.
**No physical hardware or OTP programming validation has been performed.**

## References

- Primary source: [bundled manufacturer datasheet, rev. 1.09 (2023-02-16)](documentation/TMC2209_datasheet_rev1.09.pdf).
  The manufacturer download endpoint was unreachable during implementation;
  the availability of newer revisions/errata could not be verified.
- [Janelia Arduino TMC2209](https://github.com/janelia-arduino/TMC2209) and
  [its implementation](https://github.com/janelia-arduino/TMC2209/blob/main/src/TMC2209/TMC2209.cpp)
  consulted as reference only. This Rust implementation is independent and does
  not reproduce its cached-register/unidirectional-write behavior.
- [Bundled SilentStepStick board datasheet](documentation/tmc2209_silentstepstick_datasheet_rev1.20.pdf)
  is board-specific; do not assume its pinout/layout applies to every carrier.
