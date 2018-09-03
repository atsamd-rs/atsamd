use clock;
use core::fmt;
use hal::blocking::serial::{write::Default, Write};
use hal::serial;
use nb;
use sercom::pads::*;
use target_device::sercom0::USART;
use target_device::Interrupt;
use target_device::{NVIC, PM, SERCOM0, SERCOM1, SERCOM2, SERCOM3};
#[cfg(feature = "samd21g18a")]
use target_device::{SERCOM4, SERCOM5};
use time::Hertz;

macro_rules! uart_pinout {
    ([$($Type:ident:
        ($pad0:ident, $pad1:ident, $pad2:ident, $pad3:ident),)+
    ]) => {
$(
/// Similar to SPI Pinout the UART allows selecting any pad for RX and either pad 0 or 2 for TX
pub enum $Type {
    /// Construct pinout with rx assigned to pad0,
    /// TX here must be pad 2
    /// One entry, can't put both on pad 0
    Rx0Tx2{rx: $pad0, tx: $pad2},

    Rx1Tx0{rx: $pad1, tx: $pad0},
    Rx1Tx2{rx: $pad1, tx: $pad2},

    /// One entry, can't put both on pad 2
    Rx2Tx0{rx: $pad2, tx: $pad0},

    Rx3Tx0{rx: $pad3, tx: $pad0},
    Rx3Tx2{rx: $pad3, tx: $pad2},
}

impl $Type {
    /// Return the txpo and rxpo values for
    /// this pinout configuration
    fn rxpo_txpo(&self) -> (u8, u8) {
        match self {
            &$Type::Rx0Tx2{..} => (0, 1),

            &$Type::Rx1Tx0{..} => (1, 0),
            &$Type::Rx1Tx2{..} => (1, 1),

            &$Type::Rx2Tx0{..} => (2, 0),

            &$Type::Rx3Tx0{..} => (3, 0),
            &$Type::Rx3Tx2{..} => (3, 1),
        }
    }
}

)+

};
}

uart_pinout!([
    UART0Pinout: (Sercom0Pad0, Sercom0Pad1, Sercom0Pad2, Sercom0Pad3),
    UART1Pinout: (Sercom1Pad0, Sercom1Pad1, Sercom1Pad2, Sercom1Pad3),
    UART2Pinout: (Sercom2Pad0, Sercom2Pad1, Sercom2Pad2, Sercom2Pad3),
    UART3Pinout: (Sercom3Pad0, Sercom3Pad1, Sercom3Pad2, Sercom3Pad3),
]);
#[cfg(feature = "samd21g18a")]
uart_pinout!([
    UART4Pinout: (Sercom4Pad0, Sercom4Pad1, Sercom4Pad2, Sercom4Pad3),
    UART5Pinout: (Sercom5Pad0, Sercom5Pad1, Sercom5Pad2, Sercom5Pad3),
]);

macro_rules! uart {
    ([
        $($Type:ident: (
                        $pinout:ident,
                        $SERCOM:ident,
                        $powermask:ident,
                        $clock:ident),)+
    ]) => {
$(

pub struct $Type {
    _pinout: $pinout,
    sercom: $SERCOM,
}

impl $Type {
    pub fn new<F: Into<Hertz>>(
        clock: &clock::$clock,
        freq: F,
        sercom: $SERCOM,
        nvic: &mut NVIC,
        pm: &mut PM,
        pinout: $pinout
    ) -> $Type {
        pm.apbcmask.modify(|_, w| w.$powermask().set_bit());

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

                let (rxpo, txpo) = pinout.rxpo_txpo();
                w.rxpo().bits(rxpo); // Uses pad 3 for rx
                w.txpo().bits(txpo); // Uses pad 2 for tx (and pad 3 for xck)

                w.form().bits(0x00);
                w.sampr().bits(0x00); // 16x oversample fractional
                w.runstdby().set_bit(); // Run in standby
                w.form().bits(0); // 0 is no parity bits

                w.mode().usart_int_clk() // Internal clock mode
            });

            // Calculate value for BAUD register
            let sample_rate: u8 = 16;
            let fref = clock.freq().0;

//          TODO: Support fractional BAUD mode
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
            let baud = calculate_baud_value(freq.into().0, fref, sample_rate);

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

            nvic.enable(Interrupt::$SERCOM);

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
            _pinout: pinout,
            sercom,
        }
    }

    fn usart(&self) -> &USART {
        unsafe {
            return &self.sercom.usart;
        }
    }

    fn dre(&self) -> bool {
        self.usart().intflag.read().dre().bit_is_set()
    }
}


impl serial::Write<u8> for $Type {
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
        if !self.dre() {
            return Err(nb::Error::WouldBlock);
        }

        Ok(())
    }
}

impl serial::Read<u8> for $Type {
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

impl Default<u8> for $Type {}

impl fmt::Write for $Type {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.bwrite_all(s.as_bytes()).map_err(|_| fmt::Error)
    }
}

)+

};
}

uart!([
    UART0: (UART0Pinout, SERCOM0, sercom0_, Sercom0CoreClock),
    UART1: (UART1Pinout, SERCOM1, sercom1_, Sercom1CoreClock),
    UART2: (UART2Pinout, SERCOM2, sercom2_, Sercom2CoreClock),
    UART3: (UART3Pinout, SERCOM3, sercom3_, Sercom3CoreClock),
]);

#[cfg(feature = "samd21g18a")]
uart!([
    UART4: (UART4Pinout, SERCOM4, sercom4_, Sercom4CoreClock),
    UART5: (UART5Pinout, SERCOM5, sercom5_, Sercom5CoreClock),
]);

const SHIFT: u8 = 32;

fn calculate_baud_value(baudrate: u32, clk_freq: u32, n_samples: u8) -> u16 {
    let sample_rate = (n_samples as u64 * baudrate as u64) << 32;
    let ratio = sample_rate / clk_freq as u64;
    let scale = (1u64 << SHIFT) - ratio;
    let baud_calculated = (65536u64 * scale) >> SHIFT;

    return baud_calculated as u16;
}
