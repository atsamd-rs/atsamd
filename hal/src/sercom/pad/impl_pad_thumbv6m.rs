//! Implementations of the [`IsPad`], [`IsI2cPad`], [`GetPad`] traits

use atsamd_hal_macros::{hal_cfg, hal_macro_helper};

use crate::gpio::*;
use crate::sercom::*;

//==============================================================================
//  Pad definitions
//==============================================================================

macro_rules! pad_info {
    // This is the entry pattern when the pad is not I2C enabled.
    (
        $PinId:ident,
        $Cfg:ident,
        $Sercom:ident,
        $PadNum:ident
    ) => {
        pad_info!(@impl_pad, $PinId, $Cfg, $Sercom, $PadNum);
    };

    // This is the entry pattern when the pad is I2C enabled.
    (
        $PinId:ident,
        $Cfg:ident,
        $Sercom:ident,
        $PadNum:ident
        + I2C
    ) => {
        impl IsI2cPad for Pin<$PinId, Alternate<$Cfg>> {}
        pad_info!(@impl_pad, $PinId, $Cfg, $Sercom, $PadNum);
    };

    (
        @impl_pad,
        $PinId:ident,
        $Cfg:ident,
        $Sercom:ident,
        $PadNum:ident
    ) => {
        #[hal_cfg("sercom0-d11")]
        impl GetPad<$Sercom, $PadNum> for $PinId {
            type PinMode = Alternate<$Cfg>;
        }

        #[hal_cfg("sercom0-d21")]
        impl GetPad<$Sercom> for $PinId {
            type PadNum = $PadNum;
            type PinMode = Alternate<$Cfg>;
        }

        impl IsPad for Pin<$PinId, Alternate<$Cfg>> {
            type Sercom = $Sercom;
            type PadNum = $PadNum;
        }
    };
}

// Feature gates can be placed on individual configurations or on the entire pin
macro_rules! pad_table {
    (
        #[$id_cfg:meta]
        $PinId:ident {
            $(
                $( #[$sercom_cfg:meta] )*
                $Cfg:ident: ( $Sercom:ident, $PadNum:ident ) $( + $I2C:ident )?,
            )+
        }
    ) => {
        $(

            #[$id_cfg]
            $( #[$sercom_cfg] )?
            pad_info!( $PinId, $Cfg, $Sercom, $PadNum $( + $I2C )? );
        )+
    };

    (
        $PinId:ident {
            $(
                $( #[$sercom_cfg:meta] )*
                $Cfg:ident: ( $Sercom:ident, $PadNum:ident ) $( + $I2C:ident )?,
            )+
        }
    ) => {
        $(
            $( #[$sercom_cfg] )*
            pad_info!( $PinId, $Cfg, $Sercom, $PadNum $( + $I2C )?  );
        )+
    };

    (
        $(
            $( #[$id_cfg:meta] )?
            $PinId:ident {
                $(
                    $( #[$sercom_cfg:meta] )*
                    $Cfg:ident: ( $Sercom:ident, $PadNum:ident ) $( + $I2C:ident )?,
                )+
            }
        )+
    ) => {
        $(
            pad_table!(
                $( #[$id_cfg] )?
                $PinId {
                    $(
                        $( #[$sercom_cfg] )*
                        $Cfg: ( $Sercom, $PadNum ) $( + $I2C )?,
                    )+
                }
            );
        )+
    };
}

#[hal_macro_helper]
#[hal_cfg("sercom0-d11")]
pad_table!(
    #[hal_cfg("pa04")]
    PA04 {
        #[hal_cfg("sercom0")]
        C: (Sercom0, Pad2),
        #[hal_cfg("sercom0")]
        D: (Sercom0, Pad0),
    }
    #[hal_cfg("pa05")]
    PA05 {
        #[hal_cfg("sercom0")]
        C: (Sercom0, Pad3),
        #[hal_cfg("sercom0")]
        D: (Sercom0, Pad1),
    }
    #[hal_cfg("pa06")]
    PA06 {
        #[hal_cfg("sercom0")]
        C: (Sercom0, Pad0),
        #[hal_cfg("sercom0")]
        D: (Sercom0, Pad2),
    }
    #[hal_cfg("pa07")]
    PA07 {
        #[hal_cfg("sercom1")]
        C: (Sercom1, Pad1),
        #[hal_cfg("sercom0")]
        D: (Sercom0, Pad3),
    }
    #[hal_cfg("pa08")]
    PA08 {
        #[hal_cfg("sercom1")]
        C: (Sercom1, Pad2),
        #[hal_cfg("sercom0")]
        D: (Sercom0, Pad2),
    }
    #[hal_cfg("pa09")]
    PA09 {
        #[hal_cfg("sercom1")]
        C: (Sercom1, Pad3),
        #[hal_cfg("sercom0")]
        D: (Sercom0, Pad3),
    }
    #[hal_cfg("pa10")]
    PA10 {
        #[hal_cfg("sercom0")]
        C: (Sercom0, Pad2),
        #[hal_cfg("sercom2")]
        D: (Sercom2, Pad2),
    }
    #[hal_cfg("pa11")]
    PA11 {
        #[hal_cfg("sercom0")]
        C: (Sercom0, Pad3),
        #[hal_cfg("sercom2")]
        D: (Sercom2, Pad3),
    }
    #[hal_cfg("pa14")]
    PA14 {
        #[hal_cfg("sercom0")]
        C: (Sercom0, Pad0) + I2C,
        #[hal_cfg("sercom2")]
        D: (Sercom2, Pad0) + I2C,
    }
    #[hal_cfg("pa15")]
    PA15 {
        #[hal_cfg("sercom0")]
        C: (Sercom0, Pad1) + I2C,
        #[hal_cfg("sercom2")]
        D: (Sercom2, Pad1) + I2C,
    }
    #[hal_cfg("pa16")]
    PA16 {
        #[hal_cfg("sercom2")]
        C: (Sercom1, Pad2),
        #[hal_cfg("sercom2")]
        D: (Sercom2, Pad2),
    }
    #[hal_cfg("pa17")]
    PA17 {
        #[hal_cfg("sercom1")]
        C: (Sercom1, Pad3),
        #[hal_cfg("sercom2")]
        D: (Sercom2, Pad3),
    }
    #[hal_cfg("pa22")]
    PA22 {
        #[hal_cfg("sercom1")]
        C: (Sercom1, Pad0) + I2C,
        #[hal_cfg("sercom2")]
        D: (Sercom2, Pad0) + I2C,
    }
    #[hal_cfg("pa23")]
    PA23 {
        #[hal_cfg("sercom1")]
        C: (Sercom1, Pad1) + I2C,
        #[hal_cfg("sercom2")]
        D: (Sercom2, Pad1) + I2C,
    }
    #[hal_cfg("pa24")]
    PA24 {
        #[hal_cfg("sercom1")]
        C: (Sercom1, Pad2),
        #[hal_cfg("sercom2")]
        D: (Sercom2, Pad2),
    }
    #[hal_cfg("pa25")]
    PA25 {
        #[hal_cfg("sercom1")]
        C: (Sercom1, Pad3),
        #[hal_cfg("sercom2")]
        D: (Sercom2, Pad3),
    }
    #[hal_cfg("pa30")]
    PA30 {
        #[hal_cfg("sercom1")]
        C: (Sercom1, Pad0),
        #[hal_cfg("sercom1")]
        D: (Sercom1, Pad2),
    }
    #[hal_cfg("pa31")]
    PA31 {
        #[hal_cfg("sercom1")]
        C: (Sercom1, Pad1),
        #[hal_cfg("sercom1")]
        D: (Sercom1, Pad3),
    }
);

#[hal_macro_helper]
#[hal_cfg("sercom0-d21")]
pad_table!(
    #[hal_cfg("pa00")]
    PA00 {
        #[cfg(not(feature = "undoc-features"))]
        #[hal_cfg("sercom1")]
        D: (Sercom1, Pad0),

        #[cfg(feature = "undoc-features")]
        #[hal_cfg("sercom1")]
        D: (Sercom1, Pad0) + I2C,
    }
    #[hal_cfg("pa01")]
    PA01 {
        #[cfg(not(feature = "undoc-features"))]
        #[hal_cfg("sercom1")]
        D: (Sercom1, Pad1),

        #[cfg(feature = "undoc-features")]
        #[hal_cfg("sercom1")]
        D: (Sercom1, Pad1) + I2C,
    }
    #[hal_cfg("pa04")]
    PA04 {
        #[hal_cfg("sercom0")]
        D: (Sercom0, Pad0),
    }
    #[hal_cfg("pa05")]
    PA05 {
        #[hal_cfg("sercom0")]
        D: (Sercom0, Pad1),
    }
    #[hal_cfg("pa06")]
    PA06 {
        #[hal_cfg("sercom0")]
        D: (Sercom0, Pad2),
    }
    #[hal_cfg("pa07")]
    PA07 {
        #[hal_cfg("sercom0")]
        D: (Sercom0, Pad3),
    }
    #[hal_cfg("pa08")]
    PA08 {
        #[hal_cfg("sercom0")]
        C: (Sercom0, Pad0) + I2C,
        #[hal_cfg("sercom2")]
        D: (Sercom2, Pad0) + I2C,
    }
    #[hal_cfg("pa09")]
    PA09 {
        #[hal_cfg("sercom0")]
        C: (Sercom0, Pad1) + I2C,
        #[hal_cfg("sercom2")]
        D: (Sercom2, Pad1) + I2C,
    }
    #[hal_cfg("pa10")]
    PA10 {
        #[hal_cfg("sercom0")]
        C: (Sercom0, Pad2),
        #[hal_cfg("sercom2")]
        D: (Sercom2, Pad2),
    }
    #[hal_cfg("pa11")]
    PA11 {
        #[hal_cfg("sercom0")]
        C: (Sercom0, Pad3),
        #[hal_cfg("sercom2")]
        D: (Sercom2, Pad3),
    }
    #[hal_cfg("pa12")]
    PA12 {
        #[hal_cfg("sercom2")]
        C: (Sercom2, Pad0) + I2C,
        #[hal_cfg("sercom4")]
        D: (Sercom4, Pad0) + I2C,
    }
    #[hal_cfg("pa13")]
    PA13 {
        #[hal_cfg("sercom2")]
        C: (Sercom2, Pad1) + I2C,
        #[hal_cfg("sercom4")]
        D: (Sercom4, Pad1) + I2C,
    }
    #[hal_cfg("pa14")]
    PA14 {
        #[hal_cfg("sercom2")]
        C: (Sercom2, Pad2),
        #[hal_cfg("sercom4")]
        D: (Sercom4, Pad2),
    }
    #[hal_cfg("pa15")]
    PA15 {
        #[hal_cfg("sercom2")]
        C: (Sercom2, Pad3),
        #[hal_cfg("sercom4")]
        D: (Sercom4, Pad3),
    }
    #[hal_cfg("pa16")]
    PA16 {
        #[hal_cfg("sercom1")]
        C: (Sercom1, Pad0) + I2C,
        #[hal_cfg("sercom3")]
        D: (Sercom3, Pad0) + I2C,
    }
    #[hal_cfg("pa17")]
    PA17 {
        #[hal_cfg("sercom1")]
        C: (Sercom1, Pad1) + I2C,
        #[hal_cfg("sercom3")]
        D: (Sercom3, Pad1) + I2C,
    }
    #[hal_cfg("pa18")]
    PA18 {
        #[hal_cfg("sercom1")]
        C: (Sercom1, Pad2),
        #[hal_cfg("sercom3")]
        D: (Sercom3, Pad2),
    }
    #[hal_cfg("pa19")]
    PA19 {
        #[hal_cfg("sercom1")]
        C: (Sercom1, Pad3),
        #[hal_cfg("sercom3")]
        D: (Sercom3, Pad3),
    }
    #[hal_cfg("pa20")]
    PA20 {
        #[hal_cfg("sercom5")]
        C: (Sercom5, Pad2),
        #[hal_cfg("sercom3")]
        D: (Sercom3, Pad2),
    }
    #[hal_cfg("pa21")]
    PA21 {
        #[hal_cfg("sercom5")]
        C: (Sercom5, Pad3),
        #[hal_cfg("sercom3")]
        D: (Sercom3, Pad3),
    }
    #[hal_cfg("pa22")]
    PA22 {
        #[hal_cfg("sercom3")]
        C: (Sercom3, Pad0) + I2C,
        #[hal_cfg("sercom5")]
        D: (Sercom5, Pad0) + I2C,
    }
    #[hal_cfg("pa23")]
    PA23 {
        #[hal_cfg("sercom3")]
        C: (Sercom3, Pad1) + I2C,
        #[hal_cfg("sercom5")]
        D: (Sercom5, Pad1) + I2C,
    }
    #[hal_cfg("pa24")]
    PA24 {
        #[hal_cfg("sercom3")]
        C: (Sercom3, Pad2),
        #[hal_cfg("sercom5")]
        D: (Sercom5, Pad2),
    }
    #[hal_cfg("pa25")]
    PA25 {
        #[hal_cfg("sercom3")]
        C: (Sercom3, Pad3),
        #[hal_cfg("sercom5")]
        D: (Sercom5, Pad3),
    }
    #[hal_cfg("pa30")]
    PA30 {
        #[hal_cfg("sercom1")]
        D: (Sercom1, Pad2),
    }
    #[hal_cfg("pa31")]
    PA31 {
        #[hal_cfg("sercom1")]
        D: (Sercom1, Pad3),
    }
    #[hal_cfg("pb00")]
    PB00 {
        #[hal_cfg("sercom5")]
        D: (Sercom5, Pad2),
    }
    #[hal_cfg("pb01")]
    PB01 {
        #[hal_cfg("sercom5")]
        D: (Sercom5, Pad3),
    }
    #[hal_cfg("pb02")]
    PB02 {
        #[cfg(not(feature = "undoc-features"))]
        #[hal_cfg("sercom5")]
        D: (Sercom5, Pad0),

        #[cfg(feature = "undoc-features")]
        #[hal_cfg("sercom5")]
        D: (Sercom5, Pad0) + I2C,
    }
    #[hal_cfg("pb03")]
    PB03 {
        #[cfg(not(feature = "undoc-features"))]
        #[hal_cfg("sercom5")]
        D: (Sercom5, Pad1),

        #[cfg(feature = "undoc-features")]
        #[hal_cfg("sercom5")]
        D: (Sercom5, Pad1) + I2C,
    }
    #[hal_cfg("pb08")]
    PB08 {
        #[hal_cfg("sercom4")]
        D: (Sercom4, Pad0),
    }
    #[hal_cfg("pb09")]
    PB09 {
        #[hal_cfg("sercom4")]
        D: (Sercom4, Pad1),
    }
    #[hal_cfg("pb10")]
    PB10 {
        #[hal_cfg("sercom4")]
        D: (Sercom4, Pad2),
    }
    #[hal_cfg("pb11")]
    PB11 {
        #[hal_cfg("sercom4")]
        D: (Sercom4, Pad3),
    }
    #[hal_cfg("pb12")]
    PB12 {
        #[hal_cfg("sercom4")]
        C: (Sercom4, Pad0) + I2C,
    }
    #[hal_cfg("pb13")]
    PB13 {
        #[hal_cfg("sercom4")]
        C: (Sercom4, Pad1) + I2C,
    }
    #[hal_cfg("pb14")]
    PB14 {
        #[hal_cfg("sercom4")]
        C: (Sercom4, Pad2),
    }
    #[hal_cfg("pb15")]
    PB15 {
        #[hal_cfg("sercom4")]
        C: (Sercom4, Pad3),
    }
    #[hal_cfg("pb16")]
    PB16 {
        #[hal_cfg("sercom5")]
        C: (Sercom5, Pad0) + I2C,
    }
    #[hal_cfg("pb17")]
    PB17 {
        #[hal_cfg("sercom5")]
        C: (Sercom5, Pad1) + I2C,
    }
    #[hal_cfg("pb22")]
    PB22 {
        #[hal_cfg("sercom5")]
        D: (Sercom5, Pad2),
    }
    #[hal_cfg("pb23")]
    PB23 {
        #[hal_cfg("sercom5")]
        D: (Sercom5, Pad3),
    }
    #[hal_cfg("pb30")]
    PB30 {
        #[hal_cfg("sercom5")]
        D: (Sercom5, Pad0) + I2C,
    }
    #[hal_cfg("pb31")]
    PB31 {
        #[hal_cfg("sercom5")]
        D: (Sercom5, Pad1) + I2C,
    }
);
