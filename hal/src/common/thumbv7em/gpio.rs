//! Working with GPIO pins.
//! The pins are associated with the PORT hardware.  This module
//! defines a `split` method on the `PORT` type that is used to safely
//! reference the individual pin configuration.
//! The IO pins can be switched into alternate function modes, which
//! routes the pins to different peripherals depending on the mode
//! for the pin.  The pin configuration is reflected through the
//! use of type states to make the interface (ideally, or at least practically)
//! impossible to misuse.
use crate::target_device::port::group::{DIRCLR, DIRSET, OUTCLR, OUTSET, PINCFG, PMUX};

use crate::target_device::PORT;
use core::marker::PhantomData;
use hal::digital::v2::OutputPin;

#[cfg(feature = "unproven")]
use hal::digital::v2::{InputPin, StatefulOutputPin, ToggleableOutputPin};

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
/// Open drain output, which can be read when not driven
pub struct ReadableOpenDrain;

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
/// Peripheral Function J
pub struct PfJ;
/// Peripheral Function K
pub struct PfK;
/// Peripheral Function L
pub struct PfL;
/// Peripheral Function M
pub struct PfM;
/// Peripheral Function N
pub struct PfN;

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
#[rustfmt::skip]
macro_rules! pin {
    (
        $PinType:ident,
        $pin_ident:ident,
        $pin_no:expr,
        $group:ident,
        $dirset:ident,
        $dirclr:ident,
        $pincfg:ident,
        $outset:ident,
        $outclr:ident,
        $pinmux:ident,
        $out:ident
    ) => {
        // Helper for pmux peripheral function configuration
        macro_rules! function {
            ($FuncType:ty, $func_ident:ident, $variant:expr) => {

        impl<MODE> $PinType<MODE> {
            /// Configures the pin to operate with a peripheral
            pub fn $func_ident(
                self,
                port: &mut Port
            ) -> $PinType<$FuncType> {
                port.$pinmux()[$pin_no >> 1].modify(|_, w| {
                    if $pin_no & 1 == 1 {
                        // Odd-numbered pin
                        unsafe { w.pmuxo().bits($variant) }
                    } else {
                        // Even-numbered pin
                        unsafe { w.pmuxe().bits($variant) }
                    }
                });

                port.$pincfg()[$pin_no].modify(|_, bits| {
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

        function!(PfA, into_function_a, 0);
        function!(PfB, into_function_b, 1);
        function!(PfC, into_function_c, 2);
        function!(PfD, into_function_d, 3);
        function!(PfE, into_function_e, 4);
        function!(PfF, into_function_f, 5);
        function!(PfG, into_function_g, 6);
        function!(PfH, into_function_h, 7);
        function!(PfI, into_function_i, 8);
        function!(PfJ, into_function_j, 9);
        function!(PfK, into_function_k, 10);
        function!(PfL, into_function_l, 11);
        function!(PfM, into_function_m, 12);
        function!(PfN, into_function_n, 13);

        impl<MODE> $PinType<MODE> {

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

            /// Configures the pin to operate as an open drain output which can be read
            pub fn into_readable_open_drain_output(self, port: &mut Port) -> $PinType<Output<ReadableOpenDrain>> {
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
                    (*PORT::ptr()).$group.outtgl.write(|bits| {
                        bits.bits(1 << $pin_no);
                        bits
                    });
                }
            }
        }

        #[cfg(feature = "unproven")]
        impl<MODE> ToggleableOutputPin for $PinType<Output<MODE>> {
            // TODO: switch to ! when it’s stable
            type Error = ();

            fn toggle(&mut self) -> Result<(), Self::Error> {
                self.toggle_impl();

                Ok(())
            }
        }

        #[cfg(feature = "unproven")]
        impl InputPin for $PinType<Output<ReadableOpenDrain>> {
            // TODO: switch to ! when it’s stable
            type Error = ();

            fn is_high(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { (((*PORT::ptr()).$group.in_.read().bits()) & (1 << $pin_no)) != 0 })
            }

            fn is_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { (((*PORT::ptr()).$group.in_.read().bits()) & (1 << $pin_no)) == 0 })
            }
        }

        #[cfg(feature = "unproven")]
        impl<MODE> InputPin for $PinType<Input<MODE>> {
            // TODO: switch to ! when it’s stable
            type Error = ();

            fn is_high(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { (((*PORT::ptr()).$group.in_.read().bits()) & (1 << $pin_no)) != 0 })
            }

            fn is_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { (((*PORT::ptr()).$group.in_.read().bits()) & (1 << $pin_no)) == 0 })
            }
        }

        #[cfg(feature = "unproven")]
        impl<MODE> StatefulOutputPin for $PinType<Output<MODE>> {
            fn is_set_high(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { (((*PORT::ptr()).$group.out.read().bits()) & (1 << $pin_no)) != 0 })
            }

            fn is_set_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { (((*PORT::ptr()).$group.out.read().bits()) & (1 << $pin_no)) == 0 })
            }
        }

        impl<MODE> OutputPin for $PinType<Output<MODE>> {
            // TODO: switch to ! when it’s stable
            type Error = ();

            fn set_high(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*PORT::ptr()).$group.outset.write(|bits| {
                        bits.bits(1 << $pin_no);
                        bits
                    });
                }

                Ok(())
            }

            fn set_low(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*PORT::ptr()).$group.outclr.write(|bits| {
                        bits.bits(1 << $pin_no);
                        bits
                    });
                }

                Ok(())
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
        unsafe { &(*PORT::ptr()).group0.dirset }
    }
    fn dirclr0(&mut self) -> &DIRCLR {
        unsafe { &(*PORT::ptr()).group0.dirclr }
    }
    fn pincfg0(&mut self) -> &[PINCFG; 32] {
        unsafe { &(*PORT::ptr()).group0.pincfg }
    }
    fn outset0(&mut self) -> &OUTSET {
        unsafe { &(*PORT::ptr()).group0.outset }
    }
    fn outclr0(&mut self) -> &OUTCLR {
        unsafe { &(*PORT::ptr()).group0.outclr }
    }
    fn pmux0(&mut self) -> &[PMUX; 16] {
        unsafe { &(*PORT::ptr()).group0.pmux }
    }

    fn dirset1(&mut self) -> &DIRSET {
        unsafe { &(*PORT::ptr()).group1.dirset }
    }
    fn dirclr1(&mut self) -> &DIRCLR {
        unsafe { &(*PORT::ptr()).group1.dirclr }
    }
    fn pincfg1(&mut self) -> &[PINCFG; 32] {
        unsafe { &(*PORT::ptr()).group1.pincfg }
    }
    fn outset1(&mut self) -> &OUTSET {
        unsafe { &(*PORT::ptr()).group1.outset }
    }
    fn outclr1(&mut self) -> &OUTCLR {
        unsafe { &(*PORT::ptr()).group1.outclr }
    }
    fn pmux1(&mut self) -> &[PMUX; 16] {
        unsafe { &(*PORT::ptr()).group1.pmux }
    }

    #[cfg(any(feature = "samd51p19a", feature = "samd51n20a", feature = "same54"))]
    fn dirset2(&mut self) -> &DIRSET {
        unsafe { &(*PORT::ptr()).group2.dirset }
    }
    #[cfg(any(feature = "samd51p19a", feature = "samd51n20a", feature = "same54"))]
    fn dirclr2(&mut self) -> &DIRCLR {
        unsafe { &(*PORT::ptr()).group2.dirclr }
    }
    #[cfg(any(feature = "samd51p19a", feature = "samd51n20a", feature = "same54"))]
    fn pincfg2(&mut self) -> &[PINCFG; 32] {
        unsafe { &(*PORT::ptr()).group2.pincfg }
    }
    #[cfg(any(feature = "samd51p19a", feature = "samd51n20a", feature = "same54"))]
    fn outset2(&mut self) -> &OUTSET {
        unsafe { &(*PORT::ptr()).group2.outset }
    }
    #[cfg(any(feature = "samd51p19a", feature = "samd51n20a", feature = "same54"))]
    fn outclr2(&mut self) -> &OUTCLR {
        unsafe { &(*PORT::ptr()).group2.outclr }
    }
    #[cfg(any(feature = "samd51p19a", feature = "samd51n20a", feature = "same54"))]
    fn pmux2(&mut self) -> &[PMUX; 16] {
        unsafe { &(*PORT::ptr()).group2.pmux }
    }

    #[cfg(any(feature = "samd51p19a", feature = "same54"))]
    fn dirset3(&mut self) -> &DIRSET {
        unsafe { &(*PORT::ptr()).group3.dirset }
    }
    #[cfg(any(feature = "samd51p19a", feature = "same54"))]
    fn dirclr3(&mut self) -> &DIRCLR {
        unsafe { &(*PORT::ptr()).group3.dirclr }
    }
    #[cfg(any(feature = "samd51p19a", feature = "same54"))]
    fn pincfg3(&mut self) -> &[PINCFG; 32] {
        unsafe { &(*PORT::ptr()).group3.pincfg }
    }
    #[cfg(any(feature = "samd51p19a", feature = "same54"))]
    fn outset3(&mut self) -> &OUTSET {
        unsafe { &(*PORT::ptr()).group3.outset }
    }
    #[cfg(any(feature = "samd51p19a", feature = "same54"))]
    fn outclr3(&mut self) -> &OUTCLR {
        unsafe { &(*PORT::ptr()).group3.outclr }
    }
    #[cfg(any(feature = "samd51p19a", feature = "same54"))]
    fn pmux3(&mut self) -> &[PMUX; 16] {
        unsafe { &(*PORT::ptr()).group3.pmux }
    }
}

macro_rules! port {
    ([
       $($PinTypeA:ident: ($pin_identA:ident, $pin_noA:expr),)+
    ],[
       $($PinTypeB:ident: ($pin_identB:ident, $pin_noB:expr),)+
    ],[
       $($PinTypeC:ident: ($pin_identC:ident, $pin_noC:expr),)+
    ],[
       $($PinTypeD:ident: ($pin_identD:ident, $pin_noD:expr),)+
    ]) => {

/// Holds the GPIO Port peripheral and broken out pin instances
pub struct Parts {
    /// Opaque port reference
    pub port: Port,

    $(
        /// Pin $pin_identA
        pub $pin_identA: $PinTypeA<Input<Floating>>,
    )+
    $(
        /// Pin $pin_identB
        pub $pin_identB: $PinTypeB<Input<Floating>>,
    )+
    $(
        /// Pin $pin_identC
        #[cfg(any(feature = "samd51p19a", feature = "samd51n20a", feature = "same54"))]
        pub $pin_identC: $PinTypeC<Input<Floating>>,
    )+
    $(
        /// Pin $pin_identD
        #[cfg(any(feature = "samd51p19a", feature = "same54"))]
        pub $pin_identD: $PinTypeD<Input<Floating>>,
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
                $pin_identB: $PinTypeB { _mode: PhantomData },
            )+
            $(
                #[cfg(any(feature = "samd51p19a", feature = "samd51n20a", feature = "same54"))]
                $pin_identC: $PinTypeC { _mode: PhantomData },
            )+
            $(
                #[cfg(any(feature = "samd51p19a", feature = "same54"))]
                $pin_identD: $PinTypeD { _mode: PhantomData },
            )+
        }
    }
}

$(
    pin!($PinTypeA, $pin_identA, $pin_noA, group0, dirset0, dirclr0,
        pincfg0, outset0, outclr0, pmux0, out0);
)+
$(
    pin!($PinTypeB, $pin_identB, $pin_noB, group1, dirset1, dirclr1,
        pincfg1, outset1, outclr1, pmux1, out1);
)+
$(
    #[cfg(any(feature = "samd51p19a", feature = "samd51n20a", feature = "same54"))]
    pin!($PinTypeC, $pin_identC, $pin_noC, group2, dirset2, dirclr2,
        pincfg2, outset2, outclr2, pmux2, out2);
)+
$(
    #[cfg(any(feature = "samd51p19a", feature = "same54"))]
    pin!($PinTypeD, $pin_identD, $pin_noD, group3, dirset3, dirclr3,
        pincfg3, outset3, outclr3, pmux3, out3);
)+

    };
}

port!([
    Pa0: (pa0, 0),
    Pa1: (pa1, 1),
    Pa2: (pa2, 2),
    Pa3: (pa3, 3),
    Pa4: (pa4, 4),
    Pa5: (pa5, 5),
    Pa6: (pa6, 6),
    Pa7: (pa7, 7),
    Pa8: (pa8, 8),
    Pa9: (pa9, 9),
    Pa10: (pa10, 10),
    Pa11: (pa11, 11),
    Pa12: (pa12, 12),
    Pa13: (pa13, 13),
    Pa14: (pa14, 14),
    Pa15: (pa15, 15),
    Pa16: (pa16, 16),
    Pa17: (pa17, 17),
    Pa18: (pa18, 18),
    Pa19: (pa19, 19),
    Pa20: (pa20, 20),
    Pa21: (pa21, 21),
    Pa22: (pa22, 22),
    Pa23: (pa23, 23),
    Pa24: (pa24, 24),
    Pa25: (pa25, 25),
    Pa26: (pa26, 26),
    Pa27: (pa27, 27),
    Pa28: (pa28, 28),
    Pa29: (pa29, 29),
    Pa30: (pa30, 30),
    Pa31: (pa31, 31),
],[
    Pb0: (pb0, 0),
    Pb1: (pb1, 1),
    Pb2: (pb2, 2),
    Pb3: (pb3, 3),
    Pb4: (pb4, 4),
    Pb5: (pb5, 5),
    Pb6: (pb6, 6),
    Pb7: (pb7, 7),
    Pb8: (pb8, 8),
    Pb9: (pb9, 9),
    Pb10: (pb10, 10),
    Pb11: (pb11, 11),
    Pb12: (pb12, 12),
    Pb13: (pb13, 13),
    Pb14: (pb14, 14),
    Pb15: (pb15, 15),
    Pb16: (pb16, 16),
    Pb17: (pb17, 17),
    Pb18: (pb18, 18),
    Pb19: (pb19, 19),
    Pb20: (pb20, 20),
    Pb21: (pb21, 21),
    Pb22: (pb22, 22),
    Pb23: (pb23, 23),
    Pb24: (pb24, 24),
    Pb25: (pb25, 25),
    Pb26: (pb26, 26),
    Pb27: (pb27, 27),
    Pb28: (pb28, 28),
    Pb29: (pb29, 29),
    Pb30: (pb30, 30),
    Pb31: (pb31, 31),
],
[
    Pc0: (pc0, 0),
    Pc1: (pc1, 1),
    Pc2: (pc2, 2),
    Pc3: (pc3, 3),
    Pc4: (pc4, 4),
    Pc5: (pc5, 5),
    Pc6: (pc6, 6),
    Pc7: (pc7, 7),
    Pc10: (pc10, 10),
    Pc11: (pc11, 11),
    Pc12: (pc12, 12),
    Pc13: (pc13, 13),
    Pc14: (pc14, 14),
    Pc15: (pc15, 15),
    Pc16: (pc16, 16),
    Pc17: (pc17, 17),
    Pc18: (pc18, 18),
    Pc19: (pc19, 19),
    Pc20: (pc20, 20),
    Pc21: (pc21, 21),
    Pc22: (pc22, 22),
    Pc23: (pc23, 23),
    Pc24: (pc24, 24),
    Pc25: (pc25, 25),
    Pc26: (pc26, 26),
    Pc27: (pc27, 27),
    Pc28: (pc28, 28),
    Pc30: (pc30, 30),
    Pc31: (pc31, 31),
],
[
    Pd0: (pd0, 0),
    Pd1: (pd1, 1),
    Pd8: (pd8, 8),
    Pd9: (pd9, 9),
    Pd10: (pd10, 10),
    Pd11: (pd11, 11),
    Pd12: (pd12, 12),
    Pd20: (pd20, 20),
    Pd21: (pd21, 21),
]);

/// This macro is a helper for defining a `Pins` type in a board support
/// crate.  This type is used to provide more meaningful aliases for the
/// various GPIO pins for a given board.
#[macro_export]
macro_rules! define_pins {
    ($(#[$topattr:meta])* struct $Type:ident,
     target_device: $target_device:ident,
     $( $(#[$attr:meta])* pin $name:ident = $pin_ident:ident),+ , ) => {

$crate::paste::item! {
    $(#[$topattr])*
    pub struct $Type {
        /// Opaque port reference
        pub port: Port,

        $(
            $(#[$attr])*
            pub $name: gpio::[<P $pin_ident>]<Input<Floating>>
        ),+
    }
}

impl $Type {
    /// Returns the pins for the device
    $crate::paste::item! {
        pub fn new(port: $target_device::PORT) -> Self {
            let pins = port.split();
            $Type {
                port: pins.port,
                $(
                $name: pins.[<p $pin_ident>]
                ),+
            }
        }
    }
}
}}
