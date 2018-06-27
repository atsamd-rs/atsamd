//! Working with GPIO pins.
//! The pins are associated with the PORT hardware.  This module
//! defines a `split` method on the `PORT` type that is used to safely
//! reference the individual pin configuration.
//! The IO pins can be switched into alternate function modes, which
//! routes the pins to different peripherals depending on the mode
//! for the pin.   The pin configuration is reflected through the
//! use of type states to make the interface (ideally, or at least practically)
//! impossible to misuse.
use target_device::port::{PINCFG0_, PMUX0_, DIRCLR, DIRSET, OUTCLR, OUTSET};

#[cfg(feature = "samd21g18a")]
use target_device::port::{PINCFG1_, PMUX1_};

use core::marker::PhantomData;
use hal::digital::OutputPin;
use target_device::PORT;

#[cfg(feature = "unproven")]
use hal::digital::{InputPin, ToggleableOutputPin, StatefulOutputPin};

/// The GpioExt trait allows splitting the PORT hardware into
/// its constituent pin parts.
pub trait GpioExt {
    type Parts;

    /// Consume and split the device into its constitent parts
    fn split(self) -> Self::Parts;
}

/// Represents a pin configured for input.
/// The MODE type is typically one of `Floating`, `PullDown` or
/// `PullUp`.
pub struct Input<MODE> {
    _mode: PhantomData<MODE>,
}

/// Represents a pin configured for output.
/// The MODE type is typically one of `PushPull`, or
/// `OpenDrain`.
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

/// A trait that makes it easier to generically manage
/// converting a pin from its current state into some
/// other functional mode.  The configuration change
/// requires exclusive access to the Port hardware,
/// which is why this isn't simply the standard `Into`
/// trait.
pub trait IntoFunction<T> {
    /// Consume the pin and configure it to operate in
    /// the mode T.
    fn into_function(self, port: &mut Port) -> T;
}

// rustfmt wants to keep indenting the nested macro on each run,
// so disable it for this whole block :-/
#[cfg_attr(rustfmt, rustfmt_skip)]
macro_rules! pin {
    (
        $PinType:ident,
        $pin_ident:ident,
        $pin_no:expr,
        $pin_mode:ty,
        $dirset:ident,
        $dirclr:ident,
        $pincfg:ident,
        $outset:ident,
        $outclr:ident,
        $pinmux:ident,
        $out:ident,
        $outtgl:ident,
        $in:ident
    ) => {
        // Helper for pmux peripheral function configuration
        macro_rules! function {
            ($FuncType:ty, $func_ident:ident, $variant:ident) => {

        impl<MODE> $PinType<MODE> {
            /// Configures the pin to operate with a peripheral
            pub fn $func_ident(
                self,
                port: &mut Port
            ) -> $PinType<$FuncType> {
                port.$pinmux()[$pin_no >> 1].modify(|_, w| {
                    if $pin_no & 1 == 1 {
                        // Odd-numbered pin
                        w.pmuxo().$variant()
                    } else {
                        // Even-numbered pin
                        w.pmuxe().$variant()
                    }
                });
                port.$pincfg()[$pin_no].write(|bits| {
                    bits.pmuxen().set_bit()
                });

                $PinType { _mode: PhantomData }
            }
        }
        impl<MODE> IntoFunction<$PinType<$FuncType>> for $PinType<MODE> {
            fn into_function(self, port: &mut Port) -> $PinType<$FuncType> {
                self.$func_ident(port)
            }
        }

            };
        }

        /// Represents the IO pin with the matching name.
        pub struct $PinType<MODE> {
            _mode: PhantomData<MODE>,
        }

        function!(PfA, into_function_a, a);
        function!(PfB, into_function_b, b);
        function!(PfC, into_function_c, c);
        function!(PfD, into_function_d, d);
        function!(PfE, into_function_e, e);
        function!(PfF, into_function_f, f);
        function!(PfG, into_function_g, g);
        function!(PfH, into_function_h, h);

        impl<MODE> $PinType<MODE> {

            // TODO: datasheet mentions this, but is likely for
            // a slightly different variant
            // function!(PfI, into_function_i, i);

            /// Configures the pin to operate as a floating input
            pub fn into_floating_input(self, port: &mut Port) -> $PinType<Input<Floating>> {
                port.$dirclr().write(|bits| unsafe {
                    bits.bits(1 << $pin_no);
                    bits
                });

                port.$pincfg()[$pin_no].write(|bits| {
                    bits.pmuxen().clear_bit();
                    bits.inen().set_bit();
                    bits.pullen().clear_bit();
                    bits.drvstr().clear_bit();
                    bits
                });

                $PinType { _mode: PhantomData }
            }

            /// Configures the pin to operate as a pulled down input pin
            pub fn into_pull_down_input(self, port: &mut Port) -> $PinType<Input<PullDown>> {
                port.$dirclr().write(|bits| unsafe {
                    bits.bits(1 << $pin_no);
                    bits
                });

                port.$pincfg()[$pin_no].write(|bits| {
                    bits.pmuxen().clear_bit();
                    bits.inen().set_bit();
                    bits.pullen().set_bit();
                    bits.drvstr().clear_bit();
                    bits
                });

                // Pull down
                port.$outclr().write(|bits| unsafe {
                    bits.bits(1 << $pin_no);
                    bits
                });

                $PinType { _mode: PhantomData }
            }

            /// Configures the pin to operate as a pulled up input pin
            pub fn into_pull_up_input(self, port: &mut Port) -> $PinType<Input<PullUp>> {
                port.$dirclr().write(|bits| unsafe {
                    bits.bits(1 << $pin_no);
                    bits
                });

                port.$pincfg()[$pin_no].write(|bits| {
                    bits.pmuxen().clear_bit();
                    bits.inen().set_bit();
                    bits.pullen().set_bit();
                    bits.drvstr().clear_bit();
                    bits
                });

                // Pull up
                port.$outset().write(|bits| unsafe {
                    bits.bits(1 << $pin_no);
                    bits
                });

                $PinType { _mode: PhantomData }
            }

            /// Configures the pin to operate as an open drain output
            pub fn into_open_drain_output(self, port: &mut Port) -> $PinType<Output<OpenDrain>> {
                port.$dirset().write(|bits| unsafe {
                    bits.bits(1 << $pin_no);
                    bits
                });

                port.$pincfg()[$pin_no].write(|bits| {
                    bits.pmuxen().clear_bit();
                    bits.inen().clear_bit();
                    bits.pullen().clear_bit();
                    bits.drvstr().clear_bit();
                    bits
                });

                $PinType { _mode: PhantomData }
            }

            /// Configures the pin to operate as a push-pull output
            pub fn into_push_pull_output(self, port: &mut Port) -> $PinType<Output<PushPull>> {
                port.$dirset().write(|bits| unsafe {
                    bits.bits(1 << $pin_no);
                    bits
                });

                port.$pincfg()[$pin_no].write(|bits| {
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
                port.$pincfg()[$pin_no].write(|bits| {
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
            /// Toggle the logic level of the pin; if it is currently
            /// high, set it low and vice versa.
            pub fn toggle(&mut self) {
                self.toggle_impl();
            }

            fn toggle_impl(&mut self) {
                unsafe {
                    (*PORT::ptr()).$outtgl.write(|bits| {
                        bits.bits(1 << $pin_no);
                        bits
                    });
                }
            }
        }

        #[cfg(feature = "unproven")]
        impl<MODE> ToggleableOutputPin for $PinType<Output<MODE>> {
            fn toggle(&mut self) {
                self.toggle_impl();
            }
        }

        #[cfg(feature = "unproven")]
        impl<MODE> InputPin for $PinType<Input<MODE>> {
            fn is_high(&self) -> bool {
                unsafe { (((*PORT::ptr()).$in.read().bits()) & (1 << $pin_no)) != 0 }
            }

            fn is_low(&self) -> bool {
                unsafe { (((*PORT::ptr()).$in.read().bits()) & (1 << $pin_no)) == 0 }
            }
        }

        #[cfg(feature = "unproven")]
        impl<MODE> StatefulOutputPin for $PinType<Output<MODE>> {
            fn is_set_high(&self) -> bool {
                unsafe { (((*PORT::ptr()).$out.read().bits()) & (1 << $pin_no)) != 0 }
            }

            fn is_set_low(&self) -> bool {
                unsafe { (((*PORT::ptr()).$out.read().bits()) & (1 << $pin_no)) == 0 }
            }
        }


        impl<MODE> OutputPin for $PinType<Output<MODE>> {
            fn set_high(&mut self) {
                unsafe {
                    (*PORT::ptr()).$outset.write(|bits| {
                        bits.bits(1 << $pin_no);
                        bits
                    });
                }
            }

            fn set_low(&mut self) {
                unsafe {
                    (*PORT::ptr()).$outclr.write(|bits| {
                        bits.bits(1 << $pin_no);
                        bits
                    });
                }
            }
        }
    };
}

/// Opaque port reference
pub struct Port {
    _0: (),
}

impl Port {
    fn dirset0(&mut self) -> &DIRSET {
        unsafe { &(*PORT::ptr()).dirset0 }
    }
    fn dirclr0(&mut self) -> &DIRCLR {
        unsafe { &(*PORT::ptr()).dirclr0 }
    }
    fn pincfg0(&mut self) -> &[PINCFG0_; 32] {
        unsafe { &(*PORT::ptr()).pincfg0_ }
    }
    fn outset0(&mut self) -> &OUTSET {
        unsafe { &(*PORT::ptr()).outset0 }
    }
    fn outclr0(&mut self) -> &OUTCLR {
        unsafe { &(*PORT::ptr()).outclr0 }
    }
    fn pmux0(&mut self) -> &[PMUX0_; 16] {
        unsafe { &(*PORT::ptr()).pmux0_ }
    }

    #[cfg(feature = "samd21g18a")]
    fn dirset1(&mut self) -> &DIRSET {
        unsafe { &(*PORT::ptr()).dirset1 }
    }
    #[cfg(feature = "samd21g18a")]
    fn dirclr1(&mut self) -> &DIRCLR {
        unsafe { &(*PORT::ptr()).dirclr1 }
    }
    #[cfg(feature = "samd21g18a")]
    fn pincfg1(&mut self) -> &[PINCFG1_; 32] {
        unsafe { &(*PORT::ptr()).pincfg1_ }
    }
    #[cfg(feature = "samd21g18a")]
    fn outset1(&mut self) -> &OUTSET {
        unsafe { &(*PORT::ptr()).outset1 }
    }
    #[cfg(feature = "samd21g18a")]
    fn outclr1(&mut self) -> &OUTCLR {
        unsafe { &(*PORT::ptr()).outclr1 }
    }
    #[cfg(feature = "samd21g18a")]
    fn pmux1(&mut self) -> &[PMUX1_; 16] {
        unsafe { &(*PORT::ptr()).pmux1_ }
    }
}

macro_rules! port {
    ([
       $($PinTypeA:ident: ($pin_identA:ident, $pin_noA:expr, $pin_modeA:ty),)+
    ],[
       $($PinTypeB:ident: ($pin_identB:ident, $pin_noB:expr, $pin_modeB:ty),)+
    ]) => {

/// Holds the GPIO Port peripheral and broken out pin instances
pub struct Parts {
    /// Opaque port reference
    pub port: Port,

    $(
        /// Pin $pin_identA
        pub $pin_identA: $PinTypeA<$pin_modeA>,
    )+
    $(
        /// Pin $pin_identB
        #[cfg(feature = "samd21g18a")]
        pub $pin_identB: $PinTypeB<$pin_modeB>,
    )+
}

impl GpioExt for PORT {
    type Parts = Parts;

    /// Split the PORT peripheral into discrete pins
    fn split(self) -> Parts {
        Parts {
            port: Port {_0: ()},
            $(
                $pin_identA: $PinTypeA { _mode: PhantomData },
            )+
            $(
                #[cfg(feature = "samd21g18a")]
                $pin_identB: $PinTypeB { _mode: PhantomData },
            )+
        }
    }
}

$(
    pin!($PinTypeA, $pin_identA, $pin_noA, $pin_modeA, dirset0, dirclr0,
        pincfg0, outset0, outclr0, pmux0, out0, outtgl0, in0);
)+
$(
    #[cfg(feature = "samd21g18a")]
    pin!($PinTypeB, $pin_identB, $pin_noB, $pin_modeB, dirset1, dirclr1,
        pincfg1, outset1, outclr1, pmux1, out1, outtgl1, in1);
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
],[
    Pb0: (pb0, 0, Input<Floating>),
    Pb1: (pb1, 1, Input<Floating>),
    Pb2: (pb2, 2, Input<Floating>),
    Pb3: (pb3, 3, Input<Floating>),
    Pb4: (pb4, 4, Input<Floating>),
    Pb5: (pb5, 5, Input<Floating>),
    Pb6: (pb6, 6, Input<Floating>),
    Pb7: (pb7, 7, Input<Floating>),
    Pb8: (pb8, 8, Input<Floating>),
    Pb9: (pb9, 9, Input<Floating>),
    Pb10: (pb10, 10, Input<Floating>),
    Pb11: (pb11, 11, Input<Floating>),
    Pb12: (pb12, 12, Input<Floating>),
    Pb13: (pb13, 13, Input<Floating>),
    Pb14: (pb14, 14, Input<Floating>),
    Pb15: (pb15, 15, Input<Floating>),
    Pb16: (pb16, 16, Input<Floating>),
    Pb17: (pb17, 17, Input<Floating>),
    Pb18: (pb18, 18, Input<Floating>),
    Pb19: (pb19, 19, Input<Floating>),
    Pb20: (pb20, 20, Input<Floating>),
    Pb21: (pb21, 21, Input<Floating>),
    Pb22: (pb22, 22, Input<Floating>),
    Pb23: (pb23, 23, Input<Floating>),
    Pb24: (pb24, 24, Input<Floating>),
    Pb25: (pb25, 25, Input<Floating>),
    Pb26: (pb26, 26, Input<Floating>),
    Pb27: (pb27, 27, Input<Floating>),
    Pb28: (pb28, 28, Input<Floating>),
    Pb29: (pb29, 29, Input<Floating>),
    Pb30: (pb30, 30, Input<Floating>),
    Pb31: (pb31, 31, Input<Floating>),
]);
