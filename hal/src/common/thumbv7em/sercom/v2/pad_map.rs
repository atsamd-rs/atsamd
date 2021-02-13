//! Implementations of the [`pads`] [`Map`] trait
//!
//! This module provides implementations of [`Map`] on each [`IoSet`]. For a
//! given [`Sercom`] and [`PadNum`], the [`Map`] trait identifies the
//! corresponding [`PinId`] and [`PinMode`] for the given [`IoSet`].
//!
//! [`pads`]: crate::sercom::v2::pads
//! [`Sercom`]: crate::sercom::v2::pads::Sercom
//! [`PadNum`]: crate::sercom::v2::pads::PadNum
//! [`IoSet`]: crate::sercom::v2::pads::IoSet
//! [`Map`]: crate::sercom::v2::pads::Map
//! [`PinId`]: crate::gpio::v2::PinId
//! [`PinMode`]: crate::gpio::v2::PinMode

use crate::gpio::v2::*;
use crate::sercom::v2::pads::*;

//==============================================================================
//  Pad definitions
//==============================================================================

// This macro accepts three different patterns to handle configuration
// attributes applied to both individual PadNums as well as entire IoSets
macro_rules! map {
    // Single instance, with optional attribute
    (
        $( #[$cfg:meta] )?
        $Sercom:ident, $IoSet:ident, $PadNum:ident, $PinId:ident, $Cfg:ident
    ) => {
        $( #[$cfg] )?
        impl Map<$Sercom, $PadNum> for $IoSet {
            type Id = $PinId;
            type Mode = Alternate<$Cfg>;
        }
    };
    // Multiple instances, with optional attribute
    (
        $(
            $( #[$cfg:meta] )?
            $Sercom:ident, $IoSet:ident, $PadNum:ident, $PinId:ident, $Cfg:ident,
        )+
    ) => {
        $(
            map!(
                $( #[$cfg] )?
                $Sercom, $IoSet, $PadNum, $PinId, $Cfg
            );
        )+
    };
    // Single attribute for multiple instances. No individual attributes allowed
    (
        #[$cfg:meta]:
        $(
            $Sercom:ident, $IoSet:ident, $PadNum:ident, $PinId:ident, $Cfg:ident,
        )+
    ) => {
        $(
            #[$cfg]
            map!($Sercom, $IoSet, $PadNum, $PinId, $Cfg);
        )+
    };
}

// If all four PadNums of an IoSet have the same configuration attribute,
// then you can place a the attribute above the IoSet. Otherwise, each
// PadNum must have its own configuration attribute.
macro_rules! pad_table {
    (
        $Sercom:ident { $(
            $( #[$ioset_cfg:meta] )?
            $IoSet:ident { $(
                $( #[$padnum_cfg:meta] )?
                $PadNum:ident: ( $PinId:ident, $Cfg:ident ),
            )+ }
        )+ }
    ) => {
        $(
            map!(
                $( #[$ioset_cfg]: )?
                $(
                    $( #[$padnum_cfg] )?
                    $Sercom, $IoSet, $PadNum, $PinId, $Cfg,
                )+
            );
        )+
    };
}

pad_table!(
    Sercom0 {
        IoSet1 {
            Pad0: (PA08, C),
            Pad1: (PA09, C),
            Pad2: (PA10, C),
            Pad3: (PA11, C),
        }
        #[cfg(feature = "min-samd51n")]
        IoSet2 {
            Pad0: (PB24, C),
            Pad1: (PB25, C),
            Pad2: (PC24, C),
            Pad3: (PC25, C),
        }
        IoSet3 {
            Pad0: (PA04, D),
            Pad1: (PA05, D),
            Pad2: (PA06, D),
            Pad3: (PA07, D),
        }
        #[cfg(feature = "min-samd51n")]
        IoSet4 {
            Pad0: (PC17, D),
            Pad1: (PC16, D),
            Pad2: (PC18, D),
            Pad3: (PC19, D),
        }
    }
);

pad_table!(
    Sercom1 {
        IoSet1 {
            Pad0: (PA16, C),
            Pad1: (PA17, C),
            Pad2: (PA18, C),
            Pad3: (PA19, C),
        }
        #[cfg(feature = "min-samd51p")]
        IoSet2 {
            Pad0: (PC22, C),
            Pad1: (PC23, C),
            Pad2: (PD20, C),
            Pad3: (PD21, C),
        }
        IoSet3 {
            #[cfg(feature = "min-samd51n")]
            Pad0: (PC27, C),
            #[cfg(feature = "min-samd51n")]
            Pad1: (PC28, C),
            Pad2: (PB22, C),
            Pad3: (PB23, C),
        }
        IoSet4 {
            Pad0: (PA00, D),
            Pad1: (PA01, D),
            Pad2: (PA30, D),
            Pad3: (PA31, D),
        }
    }
);

pad_table!(
    Sercom2 {
        IoSet1 {
            Pad0: (PA12, C),
            Pad1: (PA13, C),
            Pad2: (PA14, C),
            Pad3: (PA15, C),
        }
        #[cfg(feature = "min-samd51p")]
        IoSet2 {
            Pad0: (PB26, C),
            Pad1: (PB27, C),
            Pad2: (PB28, C),
            Pad3: (PB29, C),
        }
        IoSet3 {
            Pad0: (PA09, D),
            Pad1: (PA08, D),
            Pad2: (PA10, D),
            Pad3: (PA11, D),
        }
        #[cfg(feature = "min-samd51n")]
        IoSet4 {
            Pad0: (PB25, D),
            Pad1: (PB24, D),
            Pad2: (PC24, D),
            Pad3: (PC25, D),
        }
    }
);

pad_table!(
    Sercom3 {
        IoSet1 {
            Pad0: (PA22, C),
            Pad1: (PA23, C),
            Pad2: (PA24, C),
            Pad3: (PA25, C),
        }
        IoSet2 {
            #[cfg(feature = "min-samd51n")]
            Pad0: (PB20, C),
            #[cfg(feature = "min-samd51n")]
            Pad1: (PB21, C),
            Pad2: (PA20, D),
            Pad3: (PA21, D),
        }
        IoSet3 {
            Pad0: (PA17, D),
            Pad1: (PA16, D),
            Pad2: (PA18, D),
            Pad3: (PA19, D),
        }
        #[cfg(feature = "min-samd51p")]
        IoSet4 {
            Pad0: (PC23, D),
            Pad1: (PC22, D),
            Pad2: (PD20, D),
            Pad3: (PD21, D),
        }
    }
);

pad_table!(
    Sercom4 {
        #[cfg(feature = "min-samd51j")]
        IoSet1 {
            Pad0: (PB12, C),
            Pad1: (PB13, C),
            Pad2: (PB14, C),
            Pad3: (PB15, C),
        }
        IoSet2 {
            Pad0: (PB08, D),
            Pad1: (PB09, D),
            Pad2: (PB10, D),
            Pad3: (PB11, D),
        }
        IoSet3 {
            Pad0: (PA13, D),
            Pad1: (PA12, D),
            Pad2: (PA14, D),
            Pad3: (PA15, D),
        }
        #[cfg(feature = "min-samd51p")]
        IoSet4 {
            Pad0: (PB27, D),
            Pad1: (PB26, D),
            Pad2: (PB28, D),
            Pad3: (PB29, D),
        }
    }
);

pad_table!(
    Sercom5 {
        IoSet1 {
            #[cfg(feature = "min-samd51j")]
            Pad0: (PB16, C),
            #[cfg(feature = "min-samd51j")]
            Pad1: (PB17, C),
            #[cfg(feature = "min-samd51n")]
            Pad2: (PB18, C),
            #[cfg(feature = "min-samd51n")]
            Pad3: (PB19, C),
        }
        IoSet2 {
            Pad0: (PA23, D),
            Pad1: (PA22, D),
            Pad2: (PA20, C),
            Pad3: (PA21, C),
        }
        IoSet3 {
            Pad0: (PA23, D),
            Pad1: (PA22, D),
            Pad2: (PA24, D),
            Pad3: (PA25, D),
        }
        IoSet4 {
            Pad0: (PA23, D),
            Pad1: (PA22, D),
            Pad2: (PB22, D),
            Pad3: (PB23, D),
        }
        #[cfg(feature = "min-samd51j")]
        IoSet5 {
            Pad0: (PB31, D),
            Pad1: (PB30, D),
            Pad2: (PB00, D),
            Pad3: (PB01, D),
        }
        IoSet6 {
            Pad0: (PB02, D),
            Pad1: (PB03, D),
            #[cfg(feature = "min-samd51j")]
            Pad2: (PB00, D),
            #[cfg(feature = "min-samd51j")]
            Pad3: (PB01, D),
        }
    }
);

#[cfg(feature = "min-samd51n")]
pad_table!(
    Sercom6 {
        IoSet1 {
            Pad0: (PC16, C),
            Pad1: (PC17, C),
            Pad2: (PC18, C),
            Pad3: (PC19, C),
        }
        IoSet2 {
            #[cfg(feature = "min-samd51p")]
            Pad0: (PC04, C),
            Pad1: (PC05, C),
            Pad2: (PC06, C),
            Pad3: (PC07, C),
        }
        #[cfg(feature = "min-samd51p")]
        IoSet3 {
            Pad0: (PD09, D),
            Pad1: (PD08, D),
            Pad2: (PD10, D),
            Pad3: (PD11, D),
        }
        IoSet4 {
            Pad0: (PC13, D),
            Pad1: (PC12, D),
            Pad2: (PC14, D),
            Pad3: (PC15, D),
        }
        IoSet5 {
            Pad0: (PC13, D),
            Pad1: (PC12, D),
            Pad2: (PC10, C),
            Pad3: (PC11, C),
        }
    }
);

#[cfg(feature = "min-samd51n")]
pad_table!(
    Sercom7 {
        IoSet1 {
            Pad0: (PC12, C),
            Pad1: (PC13, C),
            Pad2: (PC14, C),
            Pad3: (PC15, C),
        }
        #[cfg(feature = "min-samd51p")]
        IoSet2 {
            Pad0: (PD08, C),
            Pad1: (PD09, C),
            Pad2: (PD10, C),
            Pad3: (PD11, C),
        }
        IoSet3 {
            Pad0: (PC12, C),
            Pad1: (PC13, C),
            Pad2: (PC10, D),
            Pad3: (PC11, D),
        }
        IoSet4 {
            Pad0: (PB21, D),
            Pad1: (PB20, D),
            Pad2: (PB18, D),
            Pad3: (PB19, D),
        }
        IoSet5 {
            Pad0: (PB30, C),
            Pad1: (PB31, C),
            Pad2: (PA30, C),
            Pad3: (PA31, C),
        }
    }
);
