//! Register-level access to UART configuration

use crate::sercom::v2::*;

use super::{BaudMode, Parity, StopBits};
use crate::time::Hertz;

pub(super) trait RegisterInterface {
    type Sercom: Sercom;
    fn sercom(&self) -> &Self::Sercom;
    fn freq(&self) -> Hertz;

    /// Change the bit order of transmission (MSB/LSB first)
    #[inline]
    fn _msb_first(&mut self, msb_first: bool) {
        self.sercom()
            .usart_int()
            .ctrla
            .modify(|_, w| w.dord().bit(!msb_first));
    }

    /// Change the parity setting
    #[inline]
    fn _parity(&mut self, parity: Option<Parity>) {
        // Use only the first two available settings in the FORM field.
        // Ignore auto-baud options.
        let enabled = if let Some(p) = parity {
            let odd = match p {
                Parity::Even => false,
                Parity::Odd => true,
            };
            self.sercom()
                .usart_int()
                .ctrlb
                .modify(|_, w| w.pmode().bit(odd));
            true
        } else {
            false
        };

        self.sercom()
            .usart_int()
            .ctrla
            .modify(|_, w| unsafe { w.form().bits(!enabled as u8) });
    }

    /// Change the stop bit setting
    #[inline]
    fn _stop_bit(&mut self, stop_bit: StopBits) {
        let two_bits = match stop_bit {
            StopBits::OneBit => false,
            StopBits::TwoBits => true,
        };

        self.sercom()
            .usart_int()
            .ctrlb
            .modify(|_, w| w.sbmode().bit(two_bits));
    }

    /// Enable or disable the start of frame detector.
    ///
    /// When set, the UART will generate interrupts for
    /// RXC and/or RXS if these interrupt flags have been enabled.
    #[inline]
    fn _start_of_frame_detection(&mut self, enabled: bool) {
        self.sercom()
            .usart_int()
            .ctrlb
            .modify(|_, w| w.sfde().bit(enabled));
    }

    /// Enable or disable the collision detector.
    ///
    /// When set, the UART will detect collisions and update the
    /// corresponding flag in the STATUS register.
    #[inline]
    fn _collision_detection(&mut self, enabled: bool) {
        self.sercom()
            .usart_int()
            .ctrlb
            .modify(|_, w| w.colden().bit(enabled));
    }

    /// Set the baud rate
    ///
    /// This function will calculate the best BAUD register setting based on the
    /// stored GCLK frequency and desired baud rate. The maximum baud rate is
    /// GCLK frequency/oversampling. Values outside this range will saturate at
    /// the maximum supported baud rate.
    ///
    /// Note that 3x oversampling is not supported.
    #[inline]
    fn _baud<B: Into<Hertz>>(&mut self, baud: B, mode: BaudMode) {
        let baud: Hertz = baud.into();
        let usart_int = self.sercom().usart_int();

        usart_int
            .ctrla
            .modify(|_, w| unsafe { w.sampr().bits(mode.sampr()) });

        match mode {
            BaudMode::Arithmetic(n) => {
                let baud = calculate_baud_asynchronous_arithm(baud.0, self.freq().0, n as u8);
                unsafe { usart_int.baud_usartfp_mode().write(|w| w.baud().bits(baud)) };
            }

            BaudMode::Fractional(n) => {
                let (baud, frac) =
                    calculate_baud_asynchronous_fractional(baud.0, self.freq().0, n as u8);
                unsafe {
                    usart_int.baud_frac_mode().write(|w| {
                        w.fp().bits(frac);
                        w.baud().bits(baud)
                    });
                }
            }
        };
    }

    /// Control the buffer overflow notification
    ///
    /// If set to true, an [`RxError::Overflow`] will be issued as soon as an
    /// overflow occurs. Otherwise, it will not be issued until its place within
    /// the data stream.
    #[inline]
    fn _immediate_overflow_notification(&mut self, set: bool) {
        self.sercom()
            .usart_int()
            .ctrla
            .modify(|_, w| w.ibon().bit(set));
    }

    /// Run in standby mode
    ///
    /// When set, the UART peripheral will run in standby mode. See the
    /// datasheet for more details.
    #[inline]
    fn _run_in_standby(&mut self, set: bool) {
        self.sercom()
            .usart_int()
            .ctrla
            .modify(|_, w| w.runstdby().bit(set));
    }

    /// Enable or disable IrDA encoding. The pulse length controls the minimum
    /// pulse length that is required for a pulse to be accepted by the IrDA
    /// receiver with regards to the serial engine clock period.
    /// See datasheet for more information.
    fn _irda_encoding(&mut self, pulse_length: Option<u8>) {
        match pulse_length {
            Some(l) => {
                self.sercom()
                    .usart_int()
                    .rxpl
                    .write(|w| unsafe { w.rxpl().bits(l) });
                self.sercom()
                    .usart_int()
                    .ctrlb
                    .modify(|_, w| w.enc().bit(true));
            }
            None => {
                self.sercom()
                    .usart_int()
                    .ctrlb
                    .modify(|_, w| w.enc().bit(false));
            }
        }
    }

    /// Enable the UART peripheral
    ///
    /// UART transactions are not possible until the peripheral is enabled.
    #[inline]
    fn _enable(&mut self) {
        let usart_int = self.sercom().usart_int();

        // Enable RX
        usart_int.ctrlb.modify(|_, w| w.rxen().set_bit());
        while self
            .sercom()
            .usart_int()
            .syncbusy
            .read()
            .ctrlb()
            .bit_is_set()
        {}

        // Enable TX
        usart_int.ctrlb.modify(|_, w| w.txen().set_bit());
        while self
            .sercom()
            .usart_int()
            .syncbusy
            .read()
            .ctrlb()
            .bit_is_set()
        {}

        // Globally enable peripheral
        self._enable_peripheral(true);
    }

    #[inline]
    fn _disable(&mut self) {
        let usart_int = self.sercom().usart_int();

        // Disable RX
        usart_int.ctrlb.modify(|_, w| w.rxen().clear_bit());
        while usart_int.syncbusy.read().ctrlb().bit_is_set() {}

        // Disable TX
        usart_int.ctrlb.modify(|_, w| w.txen().clear_bit());
        while usart_int.syncbusy.read().ctrlb().bit_is_set() {}

        self._enable_peripheral(false);
    }

    /// Enable or disable the SERCOM peripheral, and wait for the ENABLE bit to
    /// synchronize.
    fn _enable_peripheral(&mut self, enable: bool) {
        self.sercom()
            .usart_int()
            .ctrla
            .modify(|_, w| w.enable().bit(enable));
        while self
            .sercom()
            .usart_int()
            .syncbusy
            .read()
            .enable()
            .bit_is_set()
        {}
    }
}

/// Calculate baudrate value using the asynchronous arithmetic method (Table
/// 24-2)
#[inline]
fn calculate_baud_asynchronous_arithm(baudrate: u32, clk_freq: u32, n_samples: u8) -> u16 {
    const SHIFT: u8 = 32;
    let sample_rate = (n_samples as u64 * baudrate as u64) << SHIFT;
    let ratio = sample_rate / clk_freq as u64;
    let scale = (1u64 << SHIFT) - ratio;
    let baud_calculated = (65536u64 * scale) >> SHIFT;

    baud_calculated as u16
}

/// Calculate baudrate value using the asynchronous frational method (Table
/// 24-2)
#[inline]
fn calculate_baud_asynchronous_fractional(
    baudrate: u32,
    clk_freq: u32,
    n_samples: u8,
) -> (u16, u8) {
    let baud_mult = (clk_freq * 8) / (n_samples as u32 * baudrate);
    ((baud_mult / 8) as u16, (baud_mult % 8) as u8)
}
