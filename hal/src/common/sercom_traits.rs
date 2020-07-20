use crate::hal::spi::Mode;
use crate::time::Hertz;

/// A trait for an SPI device that can change its CPOL and CPHA mode
pub trait ReconfigurableSpiMode {
    fn change_spi_mode(&mut self, mode: Mode);
}

/// A trait for a device that can change its baudrate
pub trait ReconfigurableBaudrate {
    fn change_baudrate<CLOCK, FREQ>(&mut self, clock: CLOCK, freq: FREQ)
        where CLOCK: Into<Hertz>,
            FREQ: Into<Hertz>;
}