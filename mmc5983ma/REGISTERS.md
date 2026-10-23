# MMC5983MA register map

Source: [MEMSIC MMC5983MA Rev A, formal release 2019-04-03](https://www.memsic.com/Public/Uploads/uploadfile/files/20220119/MMC5983MADatasheetRevA.pdf),
pages 13-18. This is the revision linked from the manufacturer product page
when checked on 2026-10-23. No separate public errata was found.

All addresses and reset values below are hexadecimal. Each register is one byte;
wire bytes are MSB-first. Magnetic data is unsigned offset binary, not signed.
The six leading output bytes contain bits 17:2, high byte first; register 06
contains the remaining two bits per axis. There is no resolution selection bit:
16-bit readings omit the final two bits, while this driver always reads all 18.

| Address | Register | Bits | Access | Reset | Read requirements / side effects |
|---|---|---|---|---|---|
| 00 | Xout0 | X[17:10] | R | Unspecified | Check Status.M_Done; burst 00-06 |
| 01 | Xout1 | X[9:2] | R | Unspecified | Same magnetic measurement |
| 02 | Yout0 | Y[17:10] | R | Unspecified | Same magnetic measurement |
| 03 | Yout1 | Y[9:2] | R | Unspecified | Same magnetic measurement |
| 04 | Zout0 | Z[17:10] | R | Unspecified | Same magnetic measurement |
| 05 | Zout1 | Z[9:2] | R | Unspecified | Same magnetic measurement |
| 06 | XYZout2 | X[1:0] 7:6; Y 5:4; Z 3:2; reserved 1:0 | R | Unspecified | Read with 00-05 |
| 07 | Tout | Unsigned temperature | R | Unspecified | Check Status.T_Done |
| 08 | Status | M_Done 0; T_Done 1; OTP_Done 4 | R/W | 00 | Reads have no documented clearing effect; write 1 to bits 0/1 acknowledges interrupt |
| 09 | Internal control 0 | TM_M 0; TM_T 1; interrupt 2; SET 3; RESET 4; auto S/R 5; OTP read 6; reserved 7 | W | 00 | Not readable; use driver shadow, never hardware RMW |
| 0A | Internal control 1 | BW 1:0; X inhibit 2; YZ inhibit 4:3; reserved 6:5; software reset 7 | W | 00 | Not readable; use shadow |
| 0B | Internal control 2 | Frequency 2:0; continuous 3; SET interval 6:4; periodic SET 7 | W | 00 | Not readable; use shadow |
| 0C | Internal control 3 | Reserved 0; test positive 1; test negative 2; reserved 5:3; SPI 3-wire 6; reserved 7 | W | 00 | Not readable; use shadow |
| 2F | Product ID 1 | Identification | R | 30 | No measurement prerequisite or documented read side effects |

No other addresses are documented; they must not be accessed. Output reads do
not document an interrupt acknowledgement or a guaranteed snapshot latch.
Use single-shot acquisition for a stationary output bank; a continuous burst
can overlap a conversion, especially at 1000 Hz.

## Write requirements

Reserved bits are zero. Control 0 TM_M and TM_T must never be asserted together.
Measurement, SET, RESET and OTP commands self-clear; only interrupt enable and
auto S/R are retained in the driver's control-0 shadow. Status acknowledgement
writes only the requested W1C bits (never a read-modify-write of Status).
YZ inhibit accepts 00 or 11; the other encodings are undocumented.

Continuous mode requires a nonzero frequency. At 200 Hz BW must be 01; at
1000 Hz BW must be 11. The driver also requires enough nominal measurement time
for lower rates (8/4/2/0.5 ms at BW 00/01/10/11 respectively).
Periodic SET requires both continuous mode and auto S/R. Three-wire SPI requires
half-duplex bus support not provided here and is rejected. Opposing test currents
cannot be enabled simultaneously by this driver.

Controls are changed only after disabling continuous mode. Writes are sequential,
not atomic. Successful writes update the shadow; any failed write invalidates
initialization because the host cannot know whether the device accepted it.
Reinitialize after a failed configuration/command write.
