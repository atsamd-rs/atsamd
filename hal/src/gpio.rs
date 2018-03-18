use atsamd21g18a::PORT;
use atsamd21g18a::port::{PINCFG0_, DIRCLR, DIRSET, OUTCLR, OUTSET};
use core::marker::PhantomData;
use hal::digital::OutputPin;

pub trait GpioExt {
    type Parts;
    fn split(self) -> Self::Parts;
}

pub struct Input<MODE> {
    _mode: PhantomData<MODE>,
}

pub struct Output<MODE> {
    _mode: PhantomData<MODE>,
}

// The following collection of types is used to encode the
// state of the pin at compile time and helps to avoid misuse.

/// Floating Input
pub struct Floating;
/// Pulled down Input
pub struct PullDown;
/// Pulled up Input
pub struct PullUp;

/// Totem Pole aka Push-Pull
pub struct PushPull;
/// Open drain output
pub struct OpenDrain;

/// Peripheral Function A
pub struct PfA;
/// Peripheral Function B
pub struct PfB;
/// Peripheral Function C
pub struct PfC;
/// Peripheral Function D
pub struct PfD;
/// Peripheral Function E
pub struct PfE;
/// Peripheral Function F
pub struct PfF;
/// Peripheral Function G
pub struct PfG;
/// Peripheral Function H
pub struct PfH;
/// Peripheral Function I
pub struct PfI;

macro_rules! port {
    ([
       $($PinType:ident: ($pin_ident:ident, $pin_no:expr, $pin_mode:ty),)+
    ]) => {

pub struct Parts {
    /// Opaque port reference
    pub port: Port,

    $(
        /// Pin $pin_ident
        pub $pin_ident: $PinType<$pin_mode>,
    )+
}

/// Opaque port reference
pub struct Port {
    _0: ()
}

impl Port {
    fn dirset0(&mut self) -> &DIRSET {
        unsafe {
            &(*PORT::ptr()).dirset0
        }
    }
    fn dirclr0(&mut self) -> &DIRCLR {
        unsafe {
            &(*PORT::ptr()).dirclr0
        }
    }
    fn pincfg0(&mut self) -> &[PINCFG0_; 32] {
        unsafe {
            &(*PORT::ptr()).pincfg0_
        }
    }
    fn outset0(&mut self) -> &OUTSET {
        unsafe {
            &(*PORT::ptr()).outset0
        }
    }
    fn outclr0(&mut self) -> &OUTCLR {
        unsafe {
            &(*PORT::ptr()).outclr0
        }
    }
}

impl GpioExt for PORT {
    type Parts = Parts;

    fn split(self) -> Parts {
        Parts {
            port: Port {_0: ()},
            $(
                $pin_ident: $PinType { _mode: PhantomData },
            )+
        }
    }
}

$(
    pub struct $PinType<MODE> {
        _mode: PhantomData<MODE>,
    }

    impl<MODE> $PinType<MODE> {
        /// Configures the pin to operate as a floating input
        pub fn into_floating_input(
            self,
            port: &mut Port
        ) -> $PinType<Input<Floating>> {
            port.dirclr0().write(|bits| unsafe {
                bits.bits(1 << $pin_no);
                bits
            });

            port.pincfg0()[$pin_no].write(|bits| {
                bits.pmuxen().clear_bit();
                bits.inen().set_bit();
                bits.pullen().clear_bit();
                bits.drvstr().clear_bit();
                bits
            });

            $PinType { _mode: PhantomData }
        }

        /// Configures the pin to operate as a pulled down input pin
        pub fn into_pull_down_input(
            self,
            port: &mut Port
        ) -> $PinType<Input<PullDown>> {
            port.dirclr0().write(|bits| unsafe {
                bits.bits(1 << $pin_no);
                bits
            });

            port.pincfg0()[$pin_no].write(|bits| {
                bits.pmuxen().clear_bit();
                bits.inen().set_bit();
                bits.pullen().set_bit();
                bits.drvstr().clear_bit();
                bits
            });

            // Pull down
            port.outclr0().write(|bits| unsafe {
                bits.bits(1 << $pin_no);
                bits
            });

            $PinType { _mode: PhantomData }
        }

        /// Configures the pin to operate as a pulled up input pin
        pub fn into_pull_up_input(
            self,
            port: &mut Port
        ) -> $PinType<Input<PullUp>> {
            port.dirclr0().write(|bits| unsafe {
                bits.bits(1 << $pin_no);
                bits
            });

            port.pincfg0()[$pin_no].write(|bits| {
                bits.pmuxen().clear_bit();
                bits.inen().set_bit();
                bits.pullen().set_bit();
                bits.drvstr().clear_bit();
                bits
            });

            // Pull up
            port.outset0().write(|bits| unsafe {
                bits.bits(1 << $pin_no);
                bits
            });

            $PinType { _mode: PhantomData }
        }

        /// Configures the pin to operate as an open drain output
        pub fn into_open_drain_output(
            self,
            port: &mut Port
        ) -> $PinType<Output<OpenDrain>> {
            port.dirset0().write(|bits| unsafe {
                bits.bits(1 << $pin_no);
                bits
            });

            port.pincfg0()[$pin_no].write(|bits| {
                bits.pmuxen().clear_bit();
                bits.inen().clear_bit();
                bits.pullen().clear_bit();
                bits.drvstr().clear_bit();
                bits
            });

            $PinType { _mode: PhantomData }
        }

        /// Configures the pin to operate as a push-pull output
        pub fn into_push_pull_output(
            self,
            port: &mut Port
        ) -> $PinType<Output<PushPull>> {
            port.dirset0().write(|bits| unsafe {
                bits.bits(1 << $pin_no);
                bits
            });

            port.pincfg0()[$pin_no].write(|bits| {
                bits.pmuxen().clear_bit();
                bits.inen().set_bit();
                bits.pullen().clear_bit();
                bits.drvstr().clear_bit();
                bits
            });

            $PinType { _mode: PhantomData }
        }

    }

    impl $PinType<Output<OpenDrain>> {
        /// Control state of the internal pull up
        pub fn internal_pull_up(&mut self, port: &mut Port, on: bool) {
            port.pincfg0()[$pin_no].write(|bits| {
                if on {
                    bits.pullen().set_bit();
                } else {
                    bits.pullen().clear_bit();
                }
                bits
            });
        }
    }

    impl<MODE> $PinType<Output<MODE>> {
        // This should eventually make it into the OutputPin
        // trait: https://github.com/japaric/embedded-hal/pull/44
        pub fn toggle(&mut self) {
            unsafe {
                (*PORT::ptr()).outtgl0.write(|bits| {
                    bits.bits(1 << $pin_no);
                    bits
                });
            }
        }
    }

    impl<MODE> OutputPin for $PinType<Output<MODE>> {
        fn is_high(&self) -> bool {
            unsafe {
                (((*PORT::ptr()).out0.read().bits()) & (1<<$pin_no)) != 0
            }
        }

        fn is_low(&self) -> bool {
            !self.is_high()
        }

        fn set_high(&mut self) {
            unsafe {
                (*PORT::ptr()).outset0.write(|bits| {
                    bits.bits(1 << $pin_no);
                    bits
                });
            }
        }

        fn set_low(&mut self) {
            unsafe {
                (*PORT::ptr()).outclr0.write(|bits| {
                    bits.bits(1 << $pin_no);
                    bits
                });
            }
        }
    }
)+
    };
}

port!([
    Pa0: (pa0, 0, Input<Floating>),
    Pa1: (pa1, 1, Input<Floating>),
    Pa2: (pa2, 2, Input<Floating>),
    Pa3: (pa3, 3, Input<Floating>),
    Pa4: (pa4, 4, Input<Floating>),
    Pa5: (pa5, 5, Input<Floating>),
    Pa6: (pa6, 6, Input<Floating>),
    Pa7: (pa7, 7, Input<Floating>),
    Pa8: (pa8, 8, Input<Floating>),
    Pa9: (pa9, 9, Input<Floating>),
    Pa10: (pa10, 10, Input<Floating>),
    Pa11: (pa11, 11, Input<Floating>),
    Pa12: (pa12, 12, Input<Floating>),
    Pa13: (pa13, 13, Input<Floating>),
    Pa14: (pa14, 14, Input<Floating>),
    Pa15: (pa15, 15, Input<Floating>),
    Pa16: (pa16, 16, Input<Floating>),
    Pa17: (pa17, 17, Input<Floating>),
    Pa18: (pa18, 18, Input<Floating>),
    Pa19: (pa19, 19, Input<Floating>),
    Pa20: (pa20, 20, Input<Floating>),
    Pa21: (pa21, 21, Input<Floating>),
    Pa22: (pa22, 22, Input<Floating>),
    Pa23: (pa23, 23, Input<Floating>),
    Pa24: (pa24, 24, Input<Floating>),
    Pa25: (pa25, 25, Input<Floating>),
    Pa26: (pa26, 26, Input<Floating>),
    Pa27: (pa27, 27, Input<Floating>),
    Pa28: (pa28, 28, Input<Floating>),
    Pa29: (pa29, 29, Input<Floating>),
    Pa30: (pa30, 30, Input<Floating>),
    Pa31: (pa31, 31, Input<Floating>),
]);
