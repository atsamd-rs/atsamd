use crate::gpio::{
    self, pin::*, AnyPin, FloatingInterrupt, PinId, PinMode, PullDownInterrupt, PullUpInterrupt,
};
use crate::pac;

/// The EicPin trait makes it more ergonomic to convert a gpio pin into an EIC
/// pin. You should not implement this trait for yourself; only the
/// implementations in the EIC module make sense.
// This is more complicated than it needs to be, due to the ExtInt structs being
// defined through macros below.
pub trait EicPin {
    type Floating;
    type PullUp;
    type PullDown;

    /// Configure a pin as a floating external interrupt
    fn into_floating_ei(self) -> Self::Floating;

    /// Configure a pin as pulled-up external interrupt
    fn into_pull_up_ei(self) -> Self::PullUp;

    /// Configure a pin as pulled-down external interrupt
    fn into_pull_down_ei(self) -> Self::PullDown;
}

pub type Sense = pac::eic::config::SENSE0_A;

pub type ExternalInterruptID = usize;

/// ExternalInterrupt describes something with an external interrupt ID.
pub trait ExternalInterrupt {
    fn id(&self) -> ExternalInterruptID;
}

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
    pub struct [<$PadType $num>]<GPIO>
    where
        GPIO: AnyPin,
    {
        _pin: Pin<GPIO::Id, GPIO::Mode>,
    }

    // impl !Send for [<$PadType $num>]<GPIO> {}
    // impl !Sync for [<$PadType $num>]<GPIO> {}

    impl<GPIO: AnyPin> [<$PadType $num>]<GPIO> {
        /// Construct pad from the appropriate pin in any mode.
        /// You may find it more convenient to use the `into_pad` trait
        /// and avoid referencing the pad type.
        pub fn new(pin: GPIO) -> Self {
            [<$PadType $num>]{
                _pin:pin.into()
            }
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
            unsafe { &(*pac::EIC::ptr()) }.intflag.read().[<extint $num>]().bit_is_set()
        }

        pub fn clear_interrupt(&mut self) {
            unsafe { &(*pac::EIC::ptr()) }.intflag.modify(|_, w| {
                w.[<extint $num>]().set_bit()
            });
        }

        pub fn sense(&mut self, _eic: &mut super::EIC, sense: Sense) {
            // Which of the two config blocks this eic config is in
            let offset = ($num >> 3) & 0b0001;
            let config = unsafe { &(*pac::EIC::ptr()).config[offset] };

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
            let config = unsafe { &(*pac::EIC::ptr()).config[offset] };

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
        impl<M: PinMode> EicPin for Pin<gpio::$PinType, M> {
            type Floating = [<$PadType $num>]<Pin<gpio::$PinType, FloatingInterrupt>>;
            type PullUp = [<$PadType $num>]<Pin<gpio::$PinType, PullUpInterrupt>>;
            type PullDown = [<$PadType $num>]<Pin<gpio::$PinType, PullDownInterrupt>>;

            fn into_floating_ei(self) -> Self::Floating {
                [<$PadType $num>]::new(self.into_floating_interrupt())
            }

            fn into_pull_up_ei(self) -> Self::PullUp {
                [<$PadType $num>]::new(self.into_pull_up_interrupt())
            }

            fn into_pull_down_ei(self) -> Self::PullDown {
                [<$PadType $num>]::new(self.into_pull_down_interrupt())
            }
        }

        $(#[$attr])*
        impl ExternalInterrupt for gpio::$PinType {
            fn id(&self) -> ExternalInterruptID {
                $num
            }
        }
    )+
}

    };
}

impl<I, M> ExternalInterrupt for Pin<I, M>
where
    I: PinId,
    M: PinMode,
    Pin<I, M>: ExternalInterrupt,
{
    fn id(&self) -> ExternalInterruptID {
        Pin::<I, M>::id(self)
    }
}

// The SAMD11 and SAMD21 devices have different ExtInt designations. Just for
// clarity's sake, the `ei!()` invocations below have been split into SAMD11-
// and SAMD21-specific declarations.

// SAMD11

#[cfg(feature = "samd11")]
ei!(ExtInt[1] {
    PA15,
});

#[cfg(feature = "samd11")]
ei!(ExtInt[2] {
    PA02,
});

#[cfg(feature = "samd11")]
ei!(ExtInt[3] {
    PA31,
});

#[cfg(feature = "samd11")]
ei!(ExtInt[4] {
    PA04,
    PA24,
});

#[cfg(feature = "samd11")]
ei!(ExtInt[5] {
    PA05,
    PA25,
});

#[cfg(feature = "samd11")]
ei!(ExtInt[6] {
    PA08,
});

#[cfg(feature = "samd11")]
ei!(ExtInt[7] {
    PA09,
});

// SAMD21

#[cfg(feature = "samd21")]
ei!(ExtInt[0] {
    #[cfg(not(any(feature = "samd21el", feature = "samd21gl")))]
    PA00,
    PA16,
    #[cfg(feature = "min-samd21j")]
    PB00,
    #[cfg(feature = "min-samd21j")]
    PB16,
});

#[cfg(feature = "samd21")]
ei!(ExtInt[1] {
    #[cfg(not(any(feature = "samd21el", feature = "samd21gl")))]
    PA01,
    PA17,
    #[cfg(feature = "min-samd21j")]
    PB01,
    #[cfg(feature = "min-samd21j")]
    PB17,
});

#[cfg(feature = "samd21")]
ei!(ExtInt[2] {
    PA02,
    PA18,
    #[cfg(feature = "min-samd21g")]
    PB02,
});

#[cfg(feature = "samd21")]
ei!(ExtInt[3] {
    PA03,
    PA19,
    #[cfg(feature = "min-samd21g")]
    PB03,
});

#[cfg(feature = "samd21")]
ei!(ExtInt[4] {
    PA04,
    #[cfg(feature = "min-samd21g")]
    PA20,
    #[cfg(feature = "min-samd21j")]
    PB04,
});

#[cfg(feature = "samd21")]
ei!(ExtInt[5] {
    PA05,
    #[cfg(feature = "min-samd21g")]
    PA21,
    #[cfg(feature = "min-samd21j")]
    PB05,
});

#[cfg(feature = "samd21")]
ei!(ExtInt[6] {
    PA06,
    PA22,
    #[cfg(feature = "min-samd21j")]
    PB06,
    #[cfg(feature = "min-samd21g")]
    PB22,
});

#[cfg(feature = "samd21")]
ei!(ExtInt[7] {
    PA07,
    PA23,
    #[cfg(feature = "min-samd21j")]
    PB07,
    #[cfg(feature = "min-samd21g")]
    PB23,
});

#[cfg(feature = "samd21")]
ei!(ExtInt[8] {
    #[cfg(not(feature = "samd21el"))]
    PA28,
    #[cfg(feature = "min-samd21g")]
    PB08,
});

#[cfg(feature = "samd21")]
ei!(ExtInt[9] {
    PA09,
    #[cfg(feature = "min-samd21g")]
    PB09,
});

#[cfg(feature = "samd21")]
ei!(ExtInt[10] {
    PA10,
    PA30,
    #[cfg(feature = "min-samd21g")]
    PB10,
});

#[cfg(feature = "samd21")]
ei!(ExtInt[11] {
   PA11,
   PA31,
   #[cfg(feature = "min-samd21g")]
   PB11,
});

#[cfg(feature = "samd21")]
ei!(ExtInt[12] {
    #[cfg(feature = "min-samd21g")]
    PA12,
    PA24,
    #[cfg(feature = "min-samd21j")]
    PB12,
});

#[cfg(feature = "samd21")]
ei!(ExtInt[13] {
    #[cfg(feature = "min-samd21g")]
    PA13,
    PA25,
    #[cfg(feature = "min-samd21j")]
    PB13,
});

#[cfg(feature = "samd21")]
ei!(ExtInt[14] {
    PA14,
    #[cfg(feature = "min-samd21j")]
    PB14,
    #[cfg(feature = "min-samd21j")]
    PB30,
});

#[cfg(feature = "samd21")]
ei!(ExtInt[15] {
    PA15,
    PA27,
    #[cfg(feature = "min-samd21j")]
    PB15,
    #[cfg(feature = "min-samd21j")]
    PB31,
});
