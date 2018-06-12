use clock;
use hal::blocking::serial::write::Default;
use hal::serial::{Write, Read};
use nb;
use sercom::pads::*;
use atsamd21g18a::{PM, SERCOM0, SERCOM1, SERCOM2, SERCOM3, NVIC};
use atsamd21g18a::sercom0::USART;
use atsamd21g18a::interrupt::Interrupt;
use time::Hertz;


pub struct Uart {
    rx: Sercom0Pad3,
    tx: Sercom0Pad2,
    sercom: SERCOM0,
}

const SHIFT: u8 = 32;

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

                w.form().bits(0x00);
                w.sampr().bits(0x00); // 16x oversample fractional
                w.runstdby().set_bit(); // Run in standby
                w.form().bits(0); // 0 is no parity bits

                w.mode().usart_int_clk() // Internal clock mode
            });

            // Calculate value for BAUD register
            let sample_rate: u8 = 16;
            let fref = clock.freq().0;

//            let mul_ratio = (fref.0 * 1000) / (freq.into().0 * 16);
//
//            let baud = mul_ratio / 1000;
//            let fp = ((mul_ratio - (baud*1000))*8)/1000;
//
//            sercom.usart.baud.baud_frac_mode.modify(|_, w| {
//                w.baud().bits(baud as u16);
//                w.fp().bits(fp as u8)
//            });

            // Asynchronous arithmetic mode (Table 24-2 in datasheet)
            let baud = Self::calulate_baud_value(freq.into().0, fref, sample_rate);

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
                //w.dre().set_bit()
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

    pub fn calulate_baud_value(baudrate: u32, clk_freq: u32, n_samples: u8) -> u16 {
        let mut ratio: u64 = 0;
        let mut scale: u64 = 0;
        let mut baud_calculated: u64 = 0;
        let mut temp1: u64 = 0;

        temp1 = ((n_samples as u64 * baudrate as u64) << 32);
        ratio = temp1 / clk_freq as u64;
        //ratio = long_division(temp1, perip);
        scale = (1u64 << SHIFT) - ratio;
        baud_calculated = (65536 * scale) >> SHIFT;

        return baud_calculated as u16;
    }

    pub fn enable_tx_interrupt(&mut self) {
        unsafe {
            self.sercom.usart.intenset.write(|w| {
                w.txc().set_bit()
            });
        }
    }

    fn usart(&self) -> &USART {
        unsafe {
            return &self.sercom.usart
        }
    }

    fn dre(&self) -> bool {
        self.usart().intflag.read().dre().bit_is_set()
    }
}

impl Write<u8> for Uart {
    type Error = ();

    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        unsafe {
            if !self.dre() {
                return Err(nb::Error::WouldBlock);
            }

            self.sercom.usart.data.write(|w| {
                w.bits(word as u16)
            });
        }

        Ok(())
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        // simply await DRE empty
        let ready = self.usart().intflag.read().dre().bit_is_set();

        if !self.dre() {
            return Err(nb::Error::WouldBlock);
        }

        Ok(())
    }
}

impl Read<u8> for Uart {
    type Error = ();

    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        let has_data = self.usart().intflag.read().rxc().bit_is_set();

        if !has_data {
            return Err(nb::Error::WouldBlock);
        }

        let data = self.usart().data.read().bits();

        Ok(data as u8)
    }
}

impl Default<u8> for Uart {}