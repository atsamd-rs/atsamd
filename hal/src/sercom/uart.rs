use clock;
use hal::blocking::serial::write::Default;
use hal::serial::Write;
use nb;
use sercom::pads::*;
use atsamd21g18a::{PM, SERCOM0, SERCOM1, SERCOM2, SERCOM3, NVIC};
use atsamd21g18a::interrupt::Interrupt;
use time::Hertz;


pub struct Uart {
    rx: Sercom0Pad3,
    tx: Sercom0Pad2,
    sercom: SERCOM0,
}

impl Uart {
    pub fn new<F: Into<Hertz>>(
        clock: &clock::Sercom0CoreClock,
        freq: F,
        sercom: SERCOM0,
        nvic: &mut NVIC,
        pm: &mut PM,
        tx: Sercom0Pad2,
        rx: Sercom0Pad3,
    ) -> Uart {
        pm.apbcmask.modify(|_, w| w.sercom0_().set_bit());

        // Lots of union fields which require unsafe access
        unsafe {
            // Reset
            sercom.usart.ctrla.modify(|_, w| w.swrst().set_bit());
            while sercom.usart.syncbusy.read().swrst().bit_is_set()
                || sercom.usart.ctrla.read().swrst().bit_is_set() {
                // wait for sync of CTRLA.SWRST
            }

            // Unsafe b/c of direct call to bits on rxpo/txpo
            sercom.usart.ctrla.modify(|_, w| {
                w.dord().set_bit();

                w.rxpo().bits(0x03); // Uses pad 3 for rx
                w.txpo().bits(0x01); // Uses pad 2 for tx (and pad 3 for xck)

                w.sampr().bits(0); // 16x oversample
                w.runstdby().set_bit(); // Run in standby
                w.form().bits(0); // 0 is no parity bits

                w.mode().usart_int_clk() // Internal clock mode
            });

            // Calculate value for BAUD register
            let sample_rate: u32 = 16;
            let fref = clock.freq();

            // Asynchronous arithmetic mode (Table 24-2 in datasheet)
            let baud = 65536 * (1 - (sample_rate * (freq.into().0/ fref.0))) as u16;

            sercom.usart.baud.baud.modify(|_, w| {
                w.baud().bits(baud)
            });

            sercom.usart.ctrlb.modify(|_, w| {
                w.sbmode().clear_bit(); // 0 is one stop bit see sec 25.8.2
                w.chsize().bits(0x0);
                w.txen().set_bit();
                w.rxen().set_bit()
            });

            while sercom.usart.syncbusy.read().ctrlb().bit_is_set() {}

            nvic.enable(Interrupt::SERCOM0);

            sercom.usart.intenset.modify(|_, w| {
                w.rxc().set_bit()
                //w.txc().set_bit()
            });

            sercom.usart.ctrla.modify(|_, w| w.enable().set_bit());
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
            let ready = self.sercom.usart.intflag.read().dre().bit_is_set();

            if !ready {
                return Err(nb::Error::WouldBlock);
            }

            self.sercom.usart.data.write(|w| {
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