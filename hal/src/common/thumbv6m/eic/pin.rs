use crate::gpio::{self, IntoFunction, Port};
use crate::target_device;

/// The EicPin trait makes it more ergonomic to convert a gpio pin into an EIC
/// pin. You should not implement this trait for yourself; only the
/// implementations in the EIC module make sense.
pub trait EicPin<T> {
    fn into_ei(self, port: &mut Port) -> T;
}

pub type Sense = target_device::eic::config::SENSE0_A;

// TODO: implement support for NMI

/// The pad macro defines the given EIC pin and implements EicPin for the
/// given pins. The EicPin implementation will configure the pin for the
/// appropriate function and return the pin wrapped in the EIC type.
macro_rules! ei {
    (
        $PadType:ident [ $num:expr ] {
            $(
                $(#[$attr:meta])*
                $PinType:ident,
            )+
        }
    ) => {

crate::paste::item! {
    /// Represents a numbered external interrupt. The external interrupt is generic
    /// over any pin, only the EicPin implementations in this module make sense.
    pub struct [<$PadType $num>]<GPIO>(GPIO);

    // impl !Send for [<$PadType $num>]<GPIO> {}
    // impl !Sync for [<$PadType $num>]<GPIO> {}

    impl<GPIO> [<$PadType $num>]<GPIO> {
        /// Construct pad from the appropriate pin in any mode.
        /// You may find it more convenient to use the `into_pad` trait
        /// and avoid referencing the pad type.
        pub fn new(pin: GPIO) -> Self {
            [<$PadType $num>](pin)
        }

        /// Configure the eic with options for this external interrupt
        pub fn enable_event(&mut self, eic: &mut super::EIC) {
            eic.eic.evctrl.modify(|_, w| {
                w.[<extinteo $num>]().set_bit()
            });
        }

        pub fn enable_interrupt(&mut self, eic: &mut super::EIC) {
            eic.eic.intenset.modify(|_, w| {
                w.[<extint $num>]().set_bit()
            });
        }

        pub fn enable_interrupt_wake(&mut self, eic: &mut super::EIC) {
            eic.eic.wakeup.modify(|_, w| {
                w.[<wakeupen $num>]().set_bit()
            })
        }

        pub fn disable_interrupt(&mut self, eic: &mut super::EIC) {
            eic.eic.intenclr.modify(|_, w| {
                w.[<extint $num>]().set_bit()
            });
        }

        pub fn is_interrupt(&mut self) -> bool {
            unsafe { &(*target_device::EIC::ptr()) }.intflag.read().[<extint $num>]().bit_is_set()
        }

        pub fn clear_interrupt(&mut self) {
            unsafe { &(*target_device::EIC::ptr()) }.intflag.modify(|_, w| {
                w.[<extint $num>]().set_bit()
            });
        }

        pub fn sense(&mut self, _eic: &mut super::EIC, sense: Sense) {
            // Which of the two config blocks this eic config is in
            let offset = ($num >> 3) & 0b0001;
            let config = unsafe { &(*target_device::EIC::ptr()).config[offset] };

            config.modify(|_, w| unsafe {
                // Which of the eight eic configs in this config block
                match $num & 0b111 {
                    0b000 => w.sense0().bits(sense as u8),
                    0b001 => w.sense1().bits(sense as u8),
                    0b010 => w.sense2().bits(sense as u8),
                    0b011 => w.sense3().bits(sense as u8),
                    0b100 => w.sense4().bits(sense as u8),
                    0b101 => w.sense5().bits(sense as u8),
                    0b110 => w.sense6().bits(sense as u8),
                    0b111 => w.sense7().bits(sense as u8),
                    _ => unreachable!(),
                }
            });
        }

        pub fn filter(&mut self, _eic: &mut super::EIC, filter: bool) {
            // Which of the two config blocks this eic config is in
            let offset = ($num >> 3) & 0b0001;
            let config = unsafe { &(*target_device::EIC::ptr()).config[offset] };

            config.modify(|_, w| {
                // Which of the eight eic configs in this config block
                match $num & 0b111 {
                    0b000 => w.filten0().bit(filter),
                    0b001 => w.filten1().bit(filter),
                    0b010 => w.filten2().bit(filter),
                    0b011 => w.filten3().bit(filter),
                    0b100 => w.filten4().bit(filter),
                    0b101 => w.filten5().bit(filter),
                    0b110 => w.filten6().bit(filter),
                    0b111 => w.filten7().bit(filter),
                    _ => unreachable!(),
                }
            });
        }
    }

    $(
        $(#[$attr])*
        impl<MODE> EicPin<[<$PadType $num>]<gpio::$PinType<gpio::PfA>>> for gpio::$PinType<MODE> {
            fn into_ei(self, port: &mut Port) -> [<$PadType $num>]<gpio::$PinType<gpio::PfA>> {
                [<$PadType $num>]::new(self.into_function(port))
            }
        }
    )+
}

    };
}

#[cfg(feature = "samd21")]
ei!(ExtInt[0] {
    Pa0,
    Pa16,
    #[cfg(any(feature = "samd21g18a", feature="samd21j18a"))]
    Pb0,
    #[cfg(any(feature = "samd21g18a", feature="samd21j18a"))]
    Pb16,
});

#[cfg(feature = "samd11")]
ei!(ExtInt[1] {
    Pa15,
});

#[cfg(feature = "samd21")]
ei!(ExtInt[1] {
    Pa1,
    Pa17,
    #[cfg(any(feature = "samd21g18a", feature="samd21j18a"))]
    Pb1,
    #[cfg(any(feature = "samd21g18a", feature="samd21j18a"))]
    Pb17,
});

ei!(ExtInt[2] {
    Pa2,
    #[cfg(feature = "samd21")]
    Pa18,
    #[cfg(any(feature = "samd21g18a", feature="samd21j18a"))]
    Pb2,
});

#[cfg(feature = "samd11")]
ei!(ExtInt[3] {
    Pa31,
});

#[cfg(feature = "samd21")]
ei!(ExtInt[3] {
    Pa3,
    Pa19,
    #[cfg(any(feature = "samd21g18a", feature="samd21j18a"))]
    Pb3,
});

#[cfg(feature = "samd11")]
ei!(ExtInt[4] {
    Pa4,
    Pa24,
});

#[cfg(feature = "samd21")]
ei!(ExtInt[4] {
    Pa4,
    Pa20,
    #[cfg(any(feature = "samd21g18a", feature="samd21j18a"))]
    Pb4,
});

#[cfg(feature = "samd11")]
ei!(ExtInt[5] {
    Pa5,
    Pa25,
});

#[cfg(feature = "samd21")]
ei!(ExtInt[5] {
    Pa5,
    Pa21,
    #[cfg(any(feature = "samd21g18a", feature="samd21j18a"))]
    Pb5,
});

#[cfg(feature = "samd11")]
ei!(ExtInt[6] {
    Pa8,
});

#[cfg(feature = "samd21")]
ei!(ExtInt[6] {
    Pa6,
    Pa22,
    #[cfg(any(feature = "samd21g18a", feature="samd21j18a"))]
    Pb6,
    #[cfg(any(feature = "samd21g18a", feature="samd21j18a"))]
    Pb22,
});

#[cfg(feature = "samd11")]
ei!(ExtInt[7] {
    Pa9,
});

#[cfg(feature = "samd21")]
ei!(ExtInt[7] {
    Pa7,
    Pa23,
    #[cfg(any(feature = "samd21g18a", feature="samd21j18a"))]
    Pb7,
    #[cfg(any(feature = "samd21g18a", feature="samd21j18a"))]
    Pb23,
});

#[cfg(feature = "samd21")]
ei!(ExtInt[8] {
    Pa28,
    #[cfg(any(feature = "samd21g18a", feature="samd21j18a"))]
    Pb8,
});

#[cfg(feature = "samd21")]
ei!(ExtInt[9] {
    Pa9,
    #[cfg(any(feature = "samd21g18a", feature="samd21j18a"))]
    Pb9,
});

#[cfg(feature = "samd21")]
ei!(ExtInt[10] {
    Pa10,
    Pa30,
    #[cfg(any(feature = "samd21g18a", feature="samd21j18a"))]
    Pb10,
});

#[cfg(feature = "samd21")]
ei!(ExtInt[11] {
   Pa11,
   Pa31,
   #[cfg(any(feature = "samd21g18a", feature="samd21j18a"))]
   Pb11,
});

#[cfg(feature = "samd21")]
ei!(ExtInt[12] {
    Pa12,
    Pa24,
    #[cfg(any(feature = "samd21g18a", feature="samd21j18a"))]
    Pb12,
});

#[cfg(feature = "samd21")]
ei!(ExtInt[13] {
    Pa13,
    Pa25,
    #[cfg(any(feature = "samd21g18a", feature="samd21j18a"))]
    Pb13,
});

#[cfg(feature = "samd21")]
ei!(ExtInt[14] {
    Pa14,
    #[cfg(any(feature = "samd21g18a", feature="samd21j18a"))]
    Pb14,
    #[cfg(any(feature = "samd21g18a", feature="samd21j18a"))]
    Pb30,
});

#[cfg(feature = "samd21")]
ei!(ExtInt[15] {
    Pa15,
    Pa27,
    #[cfg(any(feature = "samd21g18a", feature="samd21j18a"))]
    Pb15,
    #[cfg(any(feature = "samd21g18a", feature="samd21j18a"))]
    Pb31,
});
