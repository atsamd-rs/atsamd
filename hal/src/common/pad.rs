use crate::gpio::Port;

/// The PadPin trait makes it more ergonomic to convert a pin into a Sercom pad.
/// You should not implement this trait for yourself; only the implementations
/// in the sercom module make sense.
pub trait PadPin<T> {
    fn into_pad(self, port: &mut Port) -> T;
}

/// The pad macro defines the given sercom pad and implements PadPin for the
/// given pins. The PadPin implementation will configure the pin for the
/// appropriate function and return the pin wrapped in the pad type.
macro_rules! pad {
    ($PadType:ident {
        $(
            $( #[$attr:meta] )*
            $PinType:ident ($Pf:ident),
        )+
    }
    ) => {
/// Represents a numbered pad for the associated sercom instance. The pad is
/// generic over any pin, only the PadPin implementations in this the sercom
/// module make sense.
pub struct $PadType<PIN>(PIN);

impl<PIN> $PadType<PIN> {
    /// Construct pad from the appropriate pin in any mode.
    /// You may find it more convenient to use the `into_pad` trait
    /// and avoid referencing the pad type.
    pub fn new(pin: PIN) -> Self {
        $PadType(pin)
    }
}

$(
    $( #[$attr] )*
    impl<MODE> PadPin<$PadType<gpio::$PinType<gpio::$Pf>>> for gpio::$PinType<MODE> {
        fn into_pad(self, port: &mut Port) -> $PadType<gpio::$PinType<gpio::$Pf>> {
            $PadType::new(self.into_function(port))
        }
    }
)+

    };
}
