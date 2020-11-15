//! Implementations of the [`pads`] [`Map`] trait
//!
//! This module provides implementations of [`Map`] for the SAMD11 and SAMD21
//! series chips. Unlike the SAMD5x & SAME5x chips, the SAMD11 and SAMD21 chips
//! do not have any concept of IOSET. A given SERCOM can use any combination of
//! GPIO pins, so long as those pins can be configured as pads for that SERCOM.
//!
//! This module implements [`Map`] directly on [`PinId`]s. For a given
//! [`Sercom`] and [`PadNum`], the [`Map`] trait identifies the correct
//! [`PinMode`] for the [`PinId`].
//!
//! [`pads`]: crate::sercom::v2::pads
//! [`Sercom`]: crate::sercom::v2::pads::Sercom
//! [`PadNum`]: crate::sercom::v2::pads::PadNum
//! [`Map`]: crate::sercom::v2::pads::Map
//! [`PinId`]: crate::gpio::v2::PinId
//! [`PinMode`]: crate::gpio::v2::PinMode
use crate::gpio::v2::*;
use crate::sercom::v2::pads::*;

//==============================================================================
//  Pad definitions
//==============================================================================

// TODO
macro_rules! map {
    // Single instance, with optional attribute
    (
        $( #[$cfg:meta] )?
        $Sercom:ident, $PadNum:ident, $PinId:ident, $Cfg:ident
    ) => {
        $( #[$cfg] )?
        impl Map<$Sercom, $PadNum> for $PinId {
            type Id = $PinId;
            type Mode = Alternate<$Cfg>;
        }
    };
    // Multiple instances, with optional attribute
    (
        $(
            $( #[$cfg:meta] )?
            $Sercom:ident, $PadNum:ident, $PinId:ident, $Cfg:ident,
        )+
    ) => {
        $(
            map!(
                $( #[$cfg] )?
                $Sercom, $PadNum, $PinId, $Cfg
            );
        )+
    };
}

// TODO
macro_rules! pad_table {
    (
        $(
            $( #[$pinid_cfg:meta] )?
            $PinId:ident { $(
                $( #[$padnum_cfg:meta] )?
                $Cfg:ident: ($Sercom:ident, $PadNum:ident),
            )+ }
        )+
    ) => {
        $(
            $( #[$pinid_cfg] )?
            map!(
                $(
                    $( #[$padnum_cfg] )?
                    $Sercom, $PadNum, $PinId, $Cfg,
                )+
            );
        )+
    };
}

pad_table!(
    #[cfg(feature = "samd21")]
    PA00 {
        D: (Sercom1, Pad0),
    }
    #[cfg(feature = "samd21")]
    PA01 {
        D: (Sercom1, Pad1),
    }
    PA04 {
        #[cfg(feature = "samd11")]
        C: (Sercom0, Pad2),
        D: (Sercom0, Pad0),
    }
    PA05 {
        #[cfg(feature = "samd11")]
        C: (Sercom0, Pad3),
        D: (Sercom0, Pad1),
    }
    #[cfg(feature = "samd21")]
    PA06 {
        D: (Sercom0, Pad2),
    }
    #[cfg(feature = "samd21")]
    PA07 {
        D: (Sercom0, Pad3),
    }
    PA08 {
        #[cfg(feature = "samd11")]
        C: (Sercom1, Pad2),
        #[cfg(feature = "samd11")]
        D: (Sercom0, Pad2),
        #[cfg(feature = "samd21")]
        C: (Sercom0, Pad0),
        #[cfg(feature = "samd21")]
        D: (Sercom2, Pad0),
    }
    PA09 {
        #[cfg(feature = "samd11")]
        C: (Sercom1, Pad3),
        #[cfg(feature = "samd11")]
        D: (Sercom0, Pad3),
        #[cfg(feature = "samd21")]
        C: (Sercom0, Pad1),
        #[cfg(feature = "samd21")]
        D: (Sercom2, Pad1),
    }
    #[cfg(feature = "samd21")]
    PA10 {
        C: (Sercom0, Pad2),
        D: (Sercom2, Pad2),
    }
    #[cfg(feature = "samd21")]
    PA11 {
        C: (Sercom0, Pad3),
        D: (Sercom2, Pad3),
    }
    #[cfg(feature = "min-samd21g")]
    PA12 {
        C: (Sercom2, Pad0),
        D: (Sercom4, Pad0),
    }
    #[cfg(feature = "min-samd21g")]
    PA13 {
        C: (Sercom2, Pad1),
        D: (Sercom4, Pad1),
    }
    PA14 {
        #[cfg(feature = "samd11")]
        C: (Sercom0, Pad0),
        #[cfg(feature = "samd21")]
        C: (Sercom2, Pad2),
        #[cfg(feature = "min-samd21g")]
        D: (Sercom4, Pad2),
    }
    PA15 {
        #[cfg(feature = "samd11")]
        C: (Sercom0, Pad1),
        #[cfg(feature = "samd21")]
        C: (Sercom2, Pad3),
        #[cfg(feature = "min-samd21g")]
        D: (Sercom4, Pad3),
    }
    #[cfg(feature = "samd21")]
    PA16 {
        C: (Sercom1, Pad0),
        D: (Sercom3, Pad0),
    }
    #[cfg(feature = "samd21")]
    PA17 {
        C: (Sercom1, Pad1),
        D: (Sercom3, Pad1),
    }
    #[cfg(feature = "samd21")]
    PA18 {
        C: (Sercom1, Pad2),
        D: (Sercom3, Pad2),
    }
    #[cfg(feature = "samd21")]
    PA19 {
        C: (Sercom1, Pad3),
        D: (Sercom3, Pad3),
    }
    #[cfg(feature = "min-samd21g")]
    PA20 {
        C: (Sercom5, Pad2),
        D: (Sercom3, Pad2),
    }
    #[cfg(feature = "min-samd21g")]
    PA21 {
        C: (Sercom5, Pad3),
        D: (Sercom3, Pad3),
    }
    PA22 {
        #[cfg(feature = "samd21")]
        C: (Sercom3, Pad0),
        #[cfg(feature = "min-samd21g")]
        D: (Sercom5, Pad0),
    }
    PA23 {
        #[cfg(feature = "samd21")]
        C: (Sercom3, Pad1),
        #[cfg(feature = "min-samd21g")]
        D: (Sercom5, Pad1),
    }
    PA24 {
        #[cfg(feature = "samd11")]
        C: (Sercom1, Pad2),
        #[cfg(feature = "samd21")]
        C: (Sercom3, Pad2),
        #[cfg(feature = "min-samd21g")]
        D: (Sercom5, Pad2),
    }
    PA25 {
        #[cfg(feature = "samd11")]
        C: (Sercom1, Pad3),
        #[cfg(feature = "samd21")]
        C: (Sercom3, Pad3),
        #[cfg(feature = "min-samd21g")]
        D: (Sercom5, Pad3),
    }
    PA30 {
        #[cfg(feature = "samd11")]
        C: (Sercom1, Pad0),
        D: (Sercom1, Pad2),
    }
    PA31 {
        #[cfg(feature = "samd11")]
        C: (Sercom1, Pad1),
        D: (Sercom1, Pad3),
    }
    #[cfg(feature = "min-samd21j")]
    PB00 {
        D: (Sercom5, Pad2),
    }
    #[cfg(feature = "min-samd21j")]
    PB01 {
        D: (Sercom5, Pad3),
    }
    #[cfg(feature = "min-samd21g")]
    PB02 {
        D: (Sercom5, Pad0),
    }
    #[cfg(feature = "min-samd21g")]
    PB03 {
        D: (Sercom5, Pad1),
    }
    #[cfg(feature = "min-samd21g")]
    PB08 {
        D: (Sercom4, Pad0),
    }
    #[cfg(feature = "min-samd21g")]
    PB09 {
        D: (Sercom4, Pad1),
    }
    #[cfg(feature = "min-samd21g")]
    PB10 {
        D: (Sercom4, Pad2),
    }
    #[cfg(feature = "min-samd21g")]
    PB11 {
        D: (Sercom4, Pad3),
    }
    #[cfg(feature = "min-samd21j")]
    PB12 {
        C: (Sercom4, Pad0),
    }
    #[cfg(feature = "min-samd21j")]
    PB13 {
        C: (Sercom4, Pad1),
    }
    #[cfg(feature = "min-samd21j")]
    PB14 {
        C: (Sercom4, Pad2),
    }
    #[cfg(feature = "min-samd21j")]
    PB15 {
        C: (Sercom4, Pad3),
    }
    #[cfg(feature = "min-samd21j")]
    PB16 {
        C: (Sercom5, Pad0),
    }
    #[cfg(feature = "min-samd21j")]
    PB17 {
        C: (Sercom5, Pad1),
    }
    #[cfg(feature = "min-samd21g")]
    PB22 {
        D: (Sercom5, Pad2),
    }
    #[cfg(feature = "min-samd21g")]
    PB23 {
        D: (Sercom5, Pad3),
    }
    #[cfg(feature = "min-samd21j")]
    PB30 {
        D: (Sercom5, Pad0),
    }
    #[cfg(feature = "min-samd21j")]
    PB31 {
        D: (Sercom5, Pad1),
    }
);
