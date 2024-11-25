use atsamd_hal_macros::hal_cfg;

use crate::ehal_02::digital::v2::InputPin;
use crate::eic::*;
use crate::gpio::{
    self, pin::*, AnyPin, FloatingInterrupt, PinMode, PullDownInterrupt, PullUpInterrupt,
};

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
            $(
                $(#[$attr])*
                impl<M: PinMode> EicPin for Pin<gpio::$PinType, M> {
                    type Floating = ExtInt<Pin<gpio::$PinType, FloatingInterrupt>, Self::ChId>;
                    type PullUp = ExtInt<Pin<gpio::$PinType, PullUpInterrupt>, Self::ChId>;
                    type PullDown = ExtInt<Pin<gpio::$PinType, PullDownInterrupt>, Self::ChId>;

                    type ChId = [<Ch $num>];

                    #[cfg(feature = "async")]
                    type InterruptSource = crate::async_hal::interrupts::[<EIC_EXTINT_ $num>];

                    fn into_floating_ei(self, channel: Channel<Self::ChId>) -> Self::Floating {
                        ExtInt::new(self.into_floating_interrupt(), channel)
                    }

                    fn into_pull_up_ei(self,channel: Channel<Self::ChId>) -> Self::PullUp {
                        ExtInt::new(self.into_pull_up_interrupt(), channel)
                    }

                    fn into_pull_down_ei(self, channel: Channel<Self::ChId>) -> Self::PullDown {
                        ExtInt::new(self.into_pull_down_interrupt(), channel)
                    }
                }
            )+

        }
    };
}

impl<P, Id, F> ExtInt<P, Id, F>
where
    P: EicPin,
    Id: ChId,
{
    pub fn enable_event(&mut self) {
        self.chan
            .eic
            .evctrl()
            .modify(|_, w| unsafe { w.bits(1 << P::ChId::ID) });
    }

    pub fn enable_interrupt(&mut self) {
        self.chan
            .eic
            .intenset()
            .write(|w| unsafe { w.bits(1 << P::ChId::ID) })
    }

    pub fn disable_interrupt(&mut self) {
        self.chan
            .eic
            .intenclr()
            .write(|w| unsafe { w.bits(1 << P::ChId::ID) })
    }

    pub fn is_interrupt(&mut self) -> bool {
        let intflag = self.chan.eic.intflag().read().bits();
        intflag & (1 << P::ChId::ID) != 0
    }

    pub fn state(&mut self) -> bool {
        let state = self.chan.eic.pinstate().read().bits();
        state & (1 << P::ChId::ID) != 0
    }

    pub fn clear_interrupt(&mut self) {
        unsafe {
            self.chan.eic.intflag().write(|w| w.bits(1 << P::ChId::ID));
        }
    }

    pub fn sense(&mut self, sense: Sense) {
        self.chan.with_disable(|e| {
            // Which of the two config blocks this eic config is in
            let offset = (P::ChId::ID >> 3) & 0b0001;
            let config = &e.config(offset);

            config.modify(|_, w| unsafe {
                // Which of the eight eic configs in this config block
                match P::ChId::ID & 0b111 {
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
        });
    }

    pub fn filter(&mut self, filter: bool) {
        self.chan.with_disable(|e| {
            // Which of the two config blocks this eic config is in
            let offset = (P::ChId::ID >> 3) & 0b0001;
            let config = &e.config(offset);

            config.modify(|_, w| {
                // Which of the eight eic configs in this config block
                match P::ChId::ID & 0b111 {
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
        });
    }

    /// Enable debouncing for this pin, with a configuration appropriate for debouncing physical buttons.
    pub fn debounce(&mut self) {
        self.chan.with_disable(|e| {
            e.dprescaler().modify(|_, w| {
                w.tickon().set_bit()    // Use the 32k clock for debouncing.
                .states0().set_bit()    // Require 7 0 samples to see a falling edge.
                .states1().set_bit()    // Require 7 1 samples to see a rising edge.
                .prescaler0().div16()
                .prescaler1().div16()
            });

            e.debouncen()
                .modify(|_, w| unsafe { w.bits(P::ChId::ID as u32) });
        });
    }
}

impl<P, C, Id, F> InputPin for ExtInt<P, Id, F>
where
    P: EicPin + AnyPin<Mode = Interrupt<C>>,
    C: InterruptConfig,
    Id: ChId,
{
    type Error = core::convert::Infallible;
    #[inline]
    fn is_high(&self) -> Result<bool, Self::Error> {
        self.pin.is_high()
    }
    #[inline]
    fn is_low(&self) -> Result<bool, Self::Error> {
        self.pin.is_low()
    }
}

#[cfg(feature = "async")]
mod async_impls {
    use core::convert::Infallible;

    use embedded_hal_1::digital::ErrorType;
    use embedded_hal_async::digital::Wait;

    use crate::{
        async_hal::interrupts::{Binding, Handler, InterruptSource},
        eic::EicFuture,
        typelevel::NoneT,
    };

    use super::{
        super::async_api::{InterruptHandler, WAKERS},
        *,
    };

    impl<P, Id> ExtInt<P, Id, NoneT>
    where
        P: EicPin,
        Id: ChId,
    {
        /// Turn an EIC pin into a pin usable as a [`Future`](core::future::Future).
        /// The correct interrupt source is needed.
        pub fn into_future<I>(self, _irq: I) -> ExtInt<P, Id, EicFuture>
        where
            I: Binding<P::InterruptSource, InterruptHandler>,
            InterruptHandler: Handler<P::InterruptSource>,
        {
            P::InterruptSource::unpend();
            unsafe { P::InterruptSource::enable() };

            ExtInt {
                pin: self.pin,
                chan: self.chan.change_mode(),
            }
        }
    }

    impl<P, Id> ExtInt<P, Id, EicFuture>
    where
        P: EicPin,
        Self: InputPin<Error = Infallible>,
        Id: ChId,
    {
        pub async fn wait(&mut self, sense: Sense) {
            use core::{future::poll_fn, task::Poll};
            self.disable_interrupt();

            self.sense(sense);
            poll_fn(|cx| {
                if self.is_interrupt() {
                    self.clear_interrupt();
                    self.disable_interrupt();
                    self.sense(Sense::None);
                    return Poll::Ready(());
                }

                WAKERS[P::ChId::ID].register(cx.waker());
                self.enable_interrupt();

                if self.is_interrupt() {
                    self.clear_interrupt();
                    self.disable_interrupt();
                    self.sense(Sense::None);
                    return Poll::Ready(());
                }

                Poll::Pending
            })
            .await;
        }
    }

    impl<P, Id> ErrorType for ExtInt<P, Id, EicFuture>
    where
        P: EicPin,
        Self: InputPin<Error = Infallible>,
        Id: ChId,
    {
        type Error = Infallible;
    }

    impl<P, Id> Wait for ExtInt<P, Id, EicFuture>
    where
        P: EicPin,
        Self: InputPin<Error = Infallible>,
        Id: ChId,
    {
        async fn wait_for_high(&mut self) -> Result<(), Self::Error> {
            self.wait(Sense::High).await;
            Ok(())
        }

        async fn wait_for_low(&mut self) -> Result<(), Self::Error> {
            self.wait(Sense::Low).await;
            Ok(())
        }

        async fn wait_for_rising_edge(&mut self) -> Result<(), Self::Error> {
            self.wait(Sense::Rise).await;
            Ok(())
        }

        async fn wait_for_falling_edge(&mut self) -> Result<(), Self::Error> {
            self.wait(Sense::Fall).await;
            Ok(())
        }

        async fn wait_for_any_edge(&mut self) -> Result<(), Self::Error> {
            self.wait(Sense::Both).await;
            Ok(())
        }
    }
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
