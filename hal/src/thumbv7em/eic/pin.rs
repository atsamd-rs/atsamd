#[cfg(feature = "unproven")]
use crate::ehal::digital::v2::InputPin;
use crate::gpio::{
    self, pin::*, AnyPin, FloatingInterrupt, PinMode, PullDownInterrupt, PullUpInterrupt,
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
    pub struct [<$PadType $num>]<GPIO>
    where
        GPIO: AnyPin,
    {
        _pin: Pin<GPIO::Id, GPIO::Mode>,
    }

    // impl !Send for [<$PadType $num>]<GPIO> {};
    // impl !Sync for [<$PadType $num>]<GPIO> {}}

    impl<GPIO: AnyPin> [<$PadType $num>]<GPIO> {
        /// Construct pad from the appropriate pin in any mode.
        /// You may find it more convenient to use the `into_pad` trait
        /// and avoid referencing the pad type.
        pub fn new(pin: GPIO) -> Self {
            [<$PadType $num>]{
                _pin: pin.into()
            }
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
            let intflag = unsafe { &(*pac::EIC::ptr()) }.intflag.read().bits();
            intflag & (1 << $num) != 0
        }

        pub fn state(&mut self) -> bool {
            let state = unsafe { &(*pac::EIC::ptr()) }.pinstate.read().bits();
            state & (1 << $num) != 0
        }

        pub fn clear_interrupt(&mut self) {
            unsafe {
                {&(*pac::EIC::ptr())}.intflag.write(|w| { w.bits(1 << $num) });
            }
        }

        pub fn sense(&mut self, _eic: &mut super::ConfigurableEIC, sense: Sense) {
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

        pub fn filter(&mut self, _eic: &mut super::ConfigurableEIC, filter: bool) {
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

    impl<GPIO: AnyPin> ExternalInterrupt for [<$PadType $num>]<GPIO> {
        fn id(&self) -> ExternalInterruptID {
            $num
        }
    }

    #[cfg(feature = "unproven")]
    impl<GPIO, C> InputPin for [<$PadType $num>]<GPIO>
    where
        GPIO: AnyPin<Mode = Interrupt<C>>,
        C: InterruptConfig,
    {
        type Error = core::convert::Infallible;
        #[inline]
        fn is_high(&self) -> Result<bool, Self::Error> {
            self._pin.is_high()
        }
        #[inline]
        fn is_low(&self) -> Result<bool, Self::Error> {
            self._pin.is_low()
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
        impl<M: PinMode> ExternalInterrupt for Pin<gpio::$PinType, M>
        {
            fn id(&self) -> ExternalInterruptID {
                $num
            }
        }
    )+
}

    };
}

ei!(ExtInt[0] {
    PA00,
    PA16,
    #[cfg(feature = "has-pb00")]
    PB00,
    #[cfg(feature = "pins-64")]
    PB16,
    #[cfg(feature = "pins-100")]
    PC00,
    #[cfg(feature = "pins-100")]
    PC16,
    #[cfg(feature = "pins-128")]
    PD00,
});

ei!(ExtInt[1] {
    PA01,
    PA17,
    #[cfg(feature = "has-pb01")]
    PB01,
    #[cfg(feature = "pins-64")]
    PB17,
    #[cfg(feature = "pins-100")]
    PC01,
    #[cfg(feature = "pins-100")]
    PC17,
    #[cfg(feature = "pins-128")]
    PD01,
});

ei!(ExtInt[2] {
    PA02,
    PA18,
    PB02,
    #[cfg(feature = "pins-100")]
    PB18,
    #[cfg(feature = "pins-100")]
    PC02,
    #[cfg(feature = "pins-128")]
    PC18,
});

ei!(ExtInt[3] {
    PA03,
    PA19,
    PB03,
    #[cfg(feature = "pins-100")]
    PB19,
    #[cfg(feature = "pins-100")]
    PC03,
    #[cfg(feature = "pins-100")]
    PC19,
    #[cfg(feature = "pins-128")]
    PD08,
});

ei!(ExtInt[4] {
    PA04,
    PA20,
    #[cfg(feature = "has-pb04")]
    PB04,
    #[cfg(feature = "pins-100")]
    PB20,
    #[cfg(feature = "pins-128")]
    PC04,
    #[cfg(feature = "pins-100")]
    PC20,
    #[cfg(feature = "pins-128")]
    PD09,
});

ei!(ExtInt[5] {
    PA05,
    PA21,
    #[cfg(feature = "has-pb05")]
    PB05,
    #[cfg(feature = "pins-100")]
    PB21,
    #[cfg(feature = "pins-100")]
    PC05,
    #[cfg(feature = "pins-100")]
    PC21,
    #[cfg(feature = "pins-128")]
    PD10,
});

ei!(ExtInt[6] {
    PA06,
    PA22,
    #[cfg(feature = "pins-64")]
    PB06,
    PB22,
    #[cfg(feature = "pins-100")]
    PC06,
    #[cfg(feature = "pins-128")]
    PC22,
    #[cfg(feature = "pins-128")]
    PD11,
});

ei!(ExtInt[7] {
    PA07,
    PA23,
    #[cfg(feature = "pins-64")]
    PB07,
    PB23,
    #[cfg(feature = "pins-128")]
    PC23,
    #[cfg(feature = "pins-128")]
    PD12,
});

ei!(ExtInt[8] {
    PA24,
    PB08,
    #[cfg(feature = "pins-100")]
    PB24,
    #[cfg(feature = "pins-100")]
    PC24,
});

ei!(ExtInt[9] {
    PA09,
    PA25,
    PB09,
    #[cfg(feature = "pins-100")]
    PB25,
    #[cfg(feature = "pins-100")]
    PC07,
    #[cfg(feature = "pins-100")]
    PC25,
});

ei!(ExtInt[10] {
    PA10,
    PB10,
    #[cfg(feature = "pins-100")]
    PC10,
    #[cfg(feature = "pins-100")]
    PC26,
    #[cfg(feature = "pins-128")]
    PD20,
});

ei!(ExtInt[11] {
    PA11,
    PA27,
    PB11,
    #[cfg(feature = "pins-100")]
    PC11,
    #[cfg(feature = "pins-100")]
    PC27,
    #[cfg(feature = "pins-128")]
    PD21,
});

ei!(ExtInt[12] {
    PA12,
    #[cfg(feature = "pins-64")]
    PB12,
    #[cfg(feature = "pins-128")]
    PB26,
    #[cfg(feature = "pins-100")]
    PC12,
    #[cfg(feature = "pins-100")]
    PC28,
});

ei!(ExtInt[13] {
    PA13,
    #[cfg(feature = "pins-64")]
    PB13,
    #[cfg(feature = "pins-128")]
    PB27,
    #[cfg(feature = "pins-100")]
    PC13,
});

ei!(ExtInt[14] {
    PA14,
    PA30,
    #[cfg(feature = "pins-64")]
    PB14,
    #[cfg(feature = "pins-128")]
    PB28,
    #[cfg(feature = "pins-64")]
    PB30,
    #[cfg(feature = "pins-100")]
    PC14,
    #[cfg(feature = "pins-128")]
    PC30,
});

ei!(ExtInt[15] {
    PA15,
    PA31,
    #[cfg(feature = "pins-64")]
    PB15,
    #[cfg(feature = "pins-128")]
    PB29,
    #[cfg(feature = "pins-64")]
    PB31,
    #[cfg(feature = "pins-100")]
    PC15,
    #[cfg(feature = "pins-128")]
    PC31,
});
