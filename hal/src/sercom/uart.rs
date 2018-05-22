use clock;
use hal::blocking::serial::write::Default;
use hal::serial::Write;
use nb;
use sercom::pads::*;
use target_device::{PM, SERCOM0, SERCOM1, SERCOM2, SERCOM3};
use time::Hertz;


pub struct Uart {
    rx: Sercom0Pad2,
    tx: Sercom0Pad3,
    sercom: SERCOM0,
}

impl Uart {
    pub fn new<F: Into<Hertz>>(
        clock: &clock::Sercom0CoreClock,
        freq: F,
        sercom: SERCOM0,
        pm: &mut PM,
        tx: Sercom0Pad3,
        rx: Sercom0Pad2,
    ) -> Uart {
        pm.apbcmask.modify(|_, w| w.sercom0_().set_bit());

        // Lots of union fields which require unsafe access
        unsafe {
            sercom.usart.ctrla.write(|w| w.swrst().set_bit());
            while sercom.usart.syncbusy.read().swrst().bit_is_set()
                || sercom.usart.ctrla.read().swrst().bit_is_set() {
                // wait for sync of CTRLA.SWRST
            }

            // Set mode to USART with internal clock
            sercom.usart.ctrla.write(|w| w.mode().usart_int_clk());

            // Unsafe b/c of direct call to bits on rxpo/txpo
            sercom.usart.ctrla.write(|w| {
                w.cmode().clear_bit(); // 0 means async
                w.rxpo().bits(2);
                w.txpo().bits(3);

                w.dord().set_bit();
                w.form().bits(0) // 0 is standard USART
            });

            sercom.usart.ctrlb.write(|w| {
                w.chsize().bits(0x0); // Character size 0 is 8 bits see Table 25-12 in datasheet
                w.sbmode().clear_bit() // 0 is one stop bit see sec 25.8.2
            });

            // Calculate value for BAUD register
            let sample_rate: u32 = 16;
            let fref = clock.freq();

            // Asynchronous fractional mode (Table 24-2 in datasheet)
            //   BAUD = fref / (sampleRateValue * fbaud)
            // (multiply by 8, to calculate fractional piece)
            let baud_times8 = (fref.0 * 8) / (sample_rate * freq.into().0);

            sercom.usart.baud.baud_fracfp_mode.write(|w| {
                w.fp().bits((baud_times8 % 8) as u8);
                w.baud().bits((baud_times8 / 8) as u16)
            });

            sercom.usart.ctrlb.write(|w| {
                w.txen().set_bit()
            });
            // wait for sync of CTRLB
            while sercom.usart.syncbusy.read().ctrlb().bit_is_set() {}

            sercom.usart.ctrlb.write(|w| {
                w.rxen().set_bit()
            });

            // wait for sync of CTRLB
            while sercom.usart.syncbusy.read().ctrlb().bit_is_set() {}

            sercom.usart.ctrla.write(|w| w.enable().set_bit());
            // wait for sync of ENABLE
            while sercom.usart.syncbusy.read().enable().bit_is_set() {}
        }

        Self {
            rx,
            tx,
            sercom,
        }
    }

    pub fn enable_tx_interrupt(&mut self) {
        unsafe {
            self.sercom.usart.intenset.write(|w| {
                w.txc().set_bit()
            });
        }
    }
}

impl Write<u8> for Uart {
    type Error = ();

    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        unsafe {
            while self.sercom.usart.intflag.read().dre().bit_is_clear() {
                // Wait fo DATA register to become empty
            }

            self.sercom.usart.data.write(|w| unsafe {
                w.bits(word as u16)
            });
        }

        Ok(())
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        unimplemented!()
    }
}

impl Default<u8> for Uart {}