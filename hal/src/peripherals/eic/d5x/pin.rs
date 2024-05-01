use crate::ehal_02::digital::v2::InputPin;
use crate::gpio::{
    self, pin::*, AnyPin, FloatingInterrupt, PinMode, PullDownInterrupt, PullUpInterrupt,
};
use crate::pac;
use atsamd_hal_macros::hal_cfg;

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

pub type Sense = pac::eic::config::SENSE0SELECT_A;

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
    #[hal_cfg("pa00")]
    PA00,
    #[hal_cfg("pa16")]
    PA16,
    #[hal_cfg("pb00")]
    PB00,
    #[hal_cfg("pb16")]
    PB16,
    #[hal_cfg("pc00")]
    PC00,
    #[hal_cfg("pc16")]
    PC16,
    #[hal_cfg("pd00")]
    PD00,
});

ei!(ExtInt[1] {
    #[hal_cfg("pa01")]
    PA01,
    #[hal_cfg("pa17")]
    PA17,
    #[hal_cfg("pb01")]
    PB01,
    #[hal_cfg("pb17")]
    PB17,
    #[hal_cfg("pc01")]
    PC01,
    #[hal_cfg("pc17")]
    PC17,
    #[hal_cfg("pd01")]
    PD01,
});

ei!(ExtInt[2] {
    #[hal_cfg("pa02")]
    PA02,
    #[hal_cfg("pa18")]
    PA18,
    #[hal_cfg("pb02")]
    PB02,
    #[hal_cfg("pb18")]
    PB18,
    #[hal_cfg("pc02")]
    PC02,
    #[hal_cfg("pc18")]
    PC18,
});

ei!(ExtInt[3] {
    #[hal_cfg("pa03")]
    PA03,
    #[hal_cfg("pa19")]
    PA19,
    #[hal_cfg("pb03")]
    PB03,
    #[hal_cfg("pb19")]
    PB19,
    #[hal_cfg("pc03")]
    PC03,
    #[hal_cfg("pc19")]
    PC19,
    #[hal_cfg("pd08")]
    PD08,
});

ei!(ExtInt[4] {
    #[hal_cfg("pa04")]
    PA04,
    #[hal_cfg("pa20")]
    PA20,
    #[hal_cfg("pb04")]
    PB04,
    #[hal_cfg("pb20")]
    PB20,
    #[hal_cfg("pc04")]
    PC04,
    #[hal_cfg("pc20")]
    PC20,
    #[hal_cfg("pd09")]
    PD09,
});

ei!(ExtInt[5] {
    #[hal_cfg("pa05")]
    PA05,
    #[hal_cfg("pa21")]
    PA21,
    #[hal_cfg("pb05")]
    PB05,
    #[hal_cfg("pb21")]
    PB21,
    #[hal_cfg("pc05")]
    PC05,
    #[hal_cfg("pc21")]
    PC21,
    #[hal_cfg("pd10")]
    PD10,
});

ei!(ExtInt[6] {
    #[hal_cfg("pa06")]
    PA06,
    #[hal_cfg("pa22")]
    PA22,
    #[hal_cfg("pb06")]
    PB06,
    #[hal_cfg("pb22")]
    PB22,
    #[hal_cfg("pc06")]
    PC06,
    #[hal_cfg("pc22")]
    PC22,
    #[hal_cfg("pd11")]
    PD11,
});

ei!(ExtInt[7] {
    #[hal_cfg("pa07")]
    PA07,
    #[hal_cfg("pa23")]
    PA23,
    #[hal_cfg("pb07")]
    PB07,
    #[hal_cfg("pb23")]
    PB23,
    #[hal_cfg("pc23")]
    PC23,
    #[hal_cfg("pd12")]
    PD12,
});

ei!(ExtInt[8] {
    #[hal_cfg("pa24")]
    PA24,
    #[hal_cfg("pb08")]
    PB08,
    #[hal_cfg("pb24")]
    PB24,
    #[hal_cfg("pc24")]
    PC24,
});

ei!(ExtInt[9] {
    #[hal_cfg("pa09")]
    PA09,
    #[hal_cfg("pa25")]
    PA25,
    #[hal_cfg("pb09")]
    PB09,
    #[hal_cfg("pb25")]
    PB25,
    #[hal_cfg("pc07")]
    PC07,
    #[hal_cfg("pc25")]
    PC25,
});

ei!(ExtInt[10] {
    #[hal_cfg("pa10")]
    PA10,
    #[hal_cfg("pb10")]
    PB10,
    #[hal_cfg("pc10")]
    PC10,
    #[hal_cfg("pc26")]
    PC26,
    #[hal_cfg("pd20")]
    PD20,
});

ei!(ExtInt[11] {
    #[hal_cfg("pa11")]
    PA11,
    #[hal_cfg("pa27")]
    PA27,
    #[hal_cfg("pb11")]
    PB11,
    #[hal_cfg("pc11")]
    PC11,
    #[hal_cfg("pc27")]
    PC27,
    #[hal_cfg("pd21")]
    PD21,
});

ei!(ExtInt[12] {
    #[hal_cfg("pa12")]
    PA12,
    #[hal_cfg("pb12")]
    PB12,
    #[hal_cfg("pb26")]
    PB26,
    #[hal_cfg("pc12")]
    PC12,
    #[hal_cfg("pc28")]
    PC28,
});

ei!(ExtInt[13] {
    #[hal_cfg("pa13")]
    PA13,
    #[hal_cfg("pb13")]
    PB13,
    #[hal_cfg("pb27")]
    PB27,
    #[hal_cfg("pc13")]
    PC13,
});

ei!(ExtInt[14] {
    #[hal_cfg("pa14")]
    PA14,
    #[hal_cfg("pa30")]
    PA30,
    #[hal_cfg("pb14")]
    PB14,
    #[hal_cfg("pb28")]
    PB28,
    #[hal_cfg("pb30")]
    PB30,
    #[hal_cfg("pc14")]
    PC14,
    #[hal_cfg("pc30")]
    PC30,
});

ei!(ExtInt[15] {
    #[hal_cfg("pa15")]
    PA15,
    #[hal_cfg("pa31")]
    PA31,
    #[hal_cfg("pb15")]
    PB15,
    #[hal_cfg("pb29")]
    PB29,
    #[hal_cfg("pb31")]
    PB31,
    #[hal_cfg("pc15")]
    PC15,
    #[hal_cfg("pc31")]
    PC31,
});
