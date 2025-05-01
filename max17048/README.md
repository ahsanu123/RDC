# MAX17048

The MAX17048/MAX17049 ICs are tiny, micropower current fuel gauges for lithium-ion (Li+) batteries in handheld
and portable equipment. The MAX17048 operates with
a single lithium cell and the MAX17049 with two lithium
cells in series

### Note 

syntax `where I2C: I2c<Error = E>` is not define Error default to `E`, but `Error` is 
aliased as `E`.

```rust

impl<I2C, E> MAX17048<I2C>
where
    I2C: I2c<Error = E>,
{
    pub fn new(i2c: I2C) -> Self {
        MAX17048 { i2c }
    }

    pub fn destroy(self) -> I2C {
        self.i2c
    }
}
```

### 🥖 References 

- [max170xx-eldruin](https://github.com/eldruin/max170xx-rs)
- [embedded test](https://barretts.club/posts/embedded-tests/)
- [TI understanding I2C Bus](https://www.ti.com/lit/an/slva704/slva704.pdf)
- [MAX17048 Datasheet](https://www.analog.com/media/en/technical-documentation/data-sheets/max17048-max17049.pdf)
