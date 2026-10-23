# MAX17048 functions

Equations follow datasheet Rev. 7, pages 10-14; see [REGISTERS.md](REGISTERS.md).
Private `Functions` is a default-derived stateless struct. Each conversion
accepts only a typed register read result; user configuration belongs to the
device, not the conversion layer.

| Device method / private conversion | Read | Result or operation |
|---|---|---|
| `voltage` / `voltage` | VCELL | $V = n \times 0.000078125$ volts |
| `soc` / `state_of_charge` | SOC | $SOC = n / 256$ percent; no clamping |
| `charge_rate` / `charge_rate` | CRATE | $r = \mathrm{int16}(n) \times 0.208$ %/hour |
| `hibernate_threshold` / `hibernate_threshold` | HIBRT | $r = HibThr \times 0.208$ %/hour |
| `activity_threshold` / `activity_threshold` | HIBRT | $V = ActThr \times 0.00125$ volts |
| `empty_alert_threshold` / `empty_alert_threshold` | CONFIG | $p = 32 - ATHD$ percent |
| `voltage_alert` / `voltage_alert` | VALRT | $(MIN, MAX) \times 0.020$ volts |
| `reset_voltage` / `reset_voltage` | VRESET/ID | $V = threshold \times 0.040$ volts |
| `version`, `chip_id`, `is_hibernating`, `is_active_alert` | VERSION, VRESET/ID, MODE, CONFIG respectively | Extract typed field |
| `read_register<R>`, `get_config`, `get_status`, `get_hibernate_threshold` | selected typed register | Complete unmodified typed word |
| `set_hibernate_threshold` | HIBRT | Replace both fields; 0 disables, 0xFFFF forces hibernate |
| `set_hibernate_entry_threshold`, `set_activity_threshold` | HIBRT | Replace one raw count, preserve the other |
| `set_config`, `set_compensation` | CONFIG | Replace policy / RCOMP; preserve ALRT with write-one |
| `set_empty_alert_threshold` | CONFIG | Require 1..32%, write ATHD = 32 - percent |
| `enable_state_of_charge_change_alert` | CONFIG | Set ALSC; preserve other policy and ALRT |
| `clear_alert` | CONFIG | Write ALRT = 0; preserve all policy |
| `set_status` | STATUS | Replace EnVR only, write ones to all event flags |
| `clear_status_flags` | STATUS | Write zero to selected true flags, ones to other flags; preserve EnVR |
| `set_voltage_alert` | VALRT | Require ordered 0..5100 mV multiples of 20; counts = mV/20 |
| `set_reset_voltage`, `enable_reset_comparator` | VRESET/ID | Require 0..5080 mV multiples of 40; replace threshold / Dis, preserve factory ID |
| `enable_sleep` | MODE | Replace EnSleep, force QuickStart = 0 |
| `sleep(true)` | MODE then CONFIG | Enable EnSleep then set SLEEP; preserve other settings |
| `sleep(false)` | CONFIG | Clear SLEEP; leave sleep permission unchanged |
| `quick_start` | MODE | Set QuickStart, preserve EnSleep |
| `reset` | none | Write CMD = 0x5400; forward final-byte NACK |
| `write_model_table` | none | Unlock, transfer 32 words, attempt both relock bytes even after failure |
| `new`, `destroy` | none | Own / return bus, no I/O or device state changes |

All configuration operations read before writing and stop on read failure.
Invalid arguments return `Error::InvalidInputData` **before bus traffic**.
No current in amperes is derivable without a battery capacity/model; CRATE is
not a current sensor. No CRC is defined by this protocol, so no separate
validation module or math library is needed.
