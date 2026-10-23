# WS2812B-2020 functions

Source: Worldsemi WS2812B-2020, unversioned 10-page datasheet, pages 4-5;
see [PROCEDURE.md](PROCEDURE.md) for the exact source and timing limits.

| Component / method | Inputs | Result |
|---|---|---|
| Private `ColorEncoding::encode` | One RGB color | 12 bytes in GRB, MSB-first order |
| Private procedure `encode_frame` | RGB slice and mutable scratch slice | Validated, populated payload slice; preserves unused scratch bytes |
| `Ws2812::new` / `try_new` | SPI device / device and reported clock | Driver ownership, no I/O; `try_new` validates clock |
| `write_color` | One RGB color | One complete single-pixel frame |
| `write_colors` | Fixed-size RGB array | One complete chain frame, 12 bytes per pixel on stack |
| `write_colors_buffered` | RGB slice and caller scratch buffer | Complete frame without a frame-sized local stack allocation |
| `latch` | None | Low reset/latch sequence only |
| `release` | None | Owned SPI device, no I/O |
| `*_async` counterparts | Same as blocking methods | Same framing using `embedded-hal-async` when the `async` feature is enabled |

`function.rs` is private and the processing struct derives `Default`. Unlike a
register-based sensor, this device has no read results: the private encoder
necessarily processes color values supplied through the public device methods.
It performs no gamma correction, brightness scaling or color-space conversion.

For one channel byte $C$, encoded output byte $E_j$ ($j = 0,1,2,3$) is:

$$s(b) = \begin{cases}12 & b=1\\8 & b=0\end{cases}$$

$$E_j = 16s((C \gg (7-2j)) \mathbin{\&} 1) +
             s((C \gg (6-2j)) \mathbin{\&} 1)$$

For $N$ pixels, payload storage is $12N$ bytes; wire length, including both
reset intervals, is $12N + 256$ SPI bytes. At nominal 3.2 MHz:

$$t_{\mathrm{frame}} = 30N + 640 \quad \mathrm{microseconds}$$

These are wire times only; encoding and HAL scheduling add overhead.
