use crate::gpio::{self, IntoFunction, Port};
use crate::target_device;

/// The EicPin trait makes it more ergonomic to convert a gpio pin into an EIC
/// pin. You should not implement this trait for yourself; only the
/// implementations in the EIC module make sense.
pub trait EicPin<T> {
    fn into_ei(self, port: &mut Port) -> T;
}

pub type Sense = target_device::eic::config::SENSE0_A;

pub type ExternalInterruptID = usize;

/// ExternalInterrupt describes something with an external interrupt ID.
pub trait ExternalInterrupt {
    fn id(self) -> ExternalInterruptID;
}

/// The pad macro defines the given EIC pin and implements EicPin for the
/// given pins. The EicPin implementation will configure the pin for the
/// appropriate function and return the pin wrapped in the EIC type.
#[allow(unused_macros)]
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

    // impl !Send for [<$PadType $num>]<GPIO> {};
    // impl !Sync for [<$PadType $num>]<GPIO> {}}

    impl<GPIO> [<$PadType $num>]<GPIO> {
        /// Construct pad from the appropriate pin in any mode.
        /// You may find it more convenient to use the `into_pad` trait
        /// and avoid referencing the pad type.
        pub fn new(pin: GPIO) -> Self {
            [<$PadType $num>](pin)
        }

        pub fn enable_event(&mut self, eic: &mut super::ConfigurableEIC) {
            eic.eic.evctrl.modify(|_, w| unsafe {
                w.bits(1 << $num)
            });
        }

        pub fn enable_interrupt(&mut self, eic: &mut super::ConfigurableEIC) {
            eic.eic.intenset.write(|w| unsafe {
                w.bits(1 << $num)
            })
        }

        pub fn disable_interrupt(&mut self, eic: &mut super::ConfigurableEIC) {
            eic.eic.intenclr.write(|w| unsafe {
                w.bits(1 << $num)
            })
        }

        pub fn is_interrupt(&mut self) -> bool {
            let intflag = unsafe { &(*target_device::EIC::ptr()) }.intflag.read().bits();
            intflag & (1 << $num) != 0
        }

        pub fn state(&mut self) -> bool {
            let state = unsafe { &(*target_device::EIC::ptr()) }.pinstate.read().bits();
            state & (1 << $num) != 0
        }

        pub fn clear_interrupt(&mut self) {
            unsafe {
                &(*target_device::EIC::ptr()).intflag.write(|w| {
                    w.bits(1 << $num)
                });
            }
        }

        pub fn sense(&mut self, _eic: &mut super::ConfigurableEIC, sense: Sense) {
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

        pub fn filter(&mut self, _eic: &mut super::ConfigurableEIC, filter: bool) {
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
        $(#[$attr])*
        impl<MODE> ExternalInterrupt for &gpio::$PinType<MODE> {
            fn id(self) -> ExternalInterruptID {
                $num
            }
        }
    )+
}

    };
}

ei!(ExtInt[0] {
    Pa0,
    Pa16,
    Pb0,
    Pb16,
    #[cfg(feature = "min-samd51n")]
    Pc0,
    #[cfg(feature = "min-samd51n")]
    Pc16,
    #[cfg(feature = "min-samd51p")]
    Pd0,
});

ei!(ExtInt[1] {
    Pa1,
    Pa17,
    Pb1,
    Pb17,
    #[cfg(feature = "min-samd51n")]
    Pc1,
    #[cfg(feature = "min-samd51n")]
    Pc17,
    #[cfg(feature = "min-samd51p")]
    Pd1,
});

ei!(ExtInt[2] {
    Pa2,
    Pa18,
    Pb2,
    Pb18,
    #[cfg(feature = "min-samd51n")]
    Pc2,
    #[cfg(feature = "min-samd51n")]
    Pc18,
});

ei!(ExtInt[3] {
    Pa3,
    Pa19,
    Pb3,
    Pb19,
    #[cfg(feature = "min-samd51n")]
    Pc3,
    #[cfg(feature = "min-samd51n")]
    Pc19,
    #[cfg(feature = "min-samd51p")]
    Pd8,
});

ei!(ExtInt[4] {
    Pa4,
    Pa20,
    Pb4,
    Pb20,
    #[cfg(feature = "min-samd51n")]
    Pc4,
    #[cfg(feature = "min-samd51n")]
    Pc20,
    #[cfg(feature = "min-samd51p")]
    Pd9,
});

ei!(ExtInt[5] {
    Pa5,
    Pa21,
    Pb5,
    Pb21,
    #[cfg(feature = "min-samd51n")]
    Pc5,
    #[cfg(feature = "min-samd51n")]
    Pc21,
    #[cfg(feature = "min-samd51p")]
    Pd10,
});

ei!(ExtInt[6] {
    Pa6,
    Pa22,
    Pb6,
    Pb22,
    #[cfg(feature = "min-samd51n")]
    Pc6,
    #[cfg(feature = "min-samd51n")]
    Pc22,
    #[cfg(feature = "min-samd51p")]
    Pd11,
});

ei!(ExtInt[7] {
    Pa7,
    Pa23,
    Pb7,
    Pb23,
    #[cfg(feature = "min-samd51n")]
    Pc23,
    #[cfg(feature = "min-samd51p")]
    Pd12,
});

ei!(ExtInt[8] {
    Pa24,
    Pb8,
    Pb24,
    #[cfg(feature = "min-samd51n")]
    Pc24,
});

ei!(ExtInt[9] {
    Pa9,
    Pa25,
    Pb9,
    Pb25,
    #[cfg(feature = "min-samd51n")]
    Pc7,
    #[cfg(feature = "min-samd51n")]
    Pc25,
});

ei!(ExtInt[10] {
    Pa10,
    Pb10,
    #[cfg(feature = "min-samd51n")]
    Pc10,
    #[cfg(feature = "min-samd51n")]
    Pc26,
    #[cfg(feature = "min-samd51p")]
    Pd20,
});

ei!(ExtInt[11] {
    Pa11,
    Pa27,
    Pb11,
    #[cfg(feature = "min-samd51n")]
    Pc11,
    #[cfg(feature = "min-samd51n")]
    Pc27,
    #[cfg(feature = "min-samd51p")]
    Pd21,
});

ei!(ExtInt[12] {
    Pa12,
    Pb12,
    Pb26,
    #[cfg(feature = "min-samd51n")]
    Pc12,
    #[cfg(feature = "min-samd51n")]
    Pc28,
});

ei!(ExtInt[13] {
    Pa13,
    Pb13,
    Pb27,
    #[cfg(feature = "min-samd51n")]
    Pc13,
});

ei!(ExtInt[14] {
    Pa14,
    Pa30,
    Pb14,
    Pb28,
    Pb30,
    #[cfg(feature = "min-samd51n")]
    Pc14,
    #[cfg(feature = "min-samd51n")]
    Pc30,
});

ei!(ExtInt[15] {
    Pa15,
    Pa31,
    Pb15,
    Pb29,
    Pb31,
    #[cfg(feature = "min-samd51n")]
    Pc15,
    #[cfg(feature = "min-samd51n")]
    Pc31,
});
