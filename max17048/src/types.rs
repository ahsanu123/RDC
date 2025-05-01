pub enum Error<E> {
    I2C(E),
    InvalidInputdata,
}
