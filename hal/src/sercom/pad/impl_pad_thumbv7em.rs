//! Implementations of the [`IsPad`], [`GetPad`] and [`InIoSet`] traits

use crate::gpio::*;
use crate::sercom::*;

use sorted_hlist::mk_hlist;

//==============================================================================
//  Pad definitions
//==============================================================================

macro_rules! pad_info {

    (
        $PinId:ident,
        $Cfg:ident,
        $Sercom:ident,
        $PadNum:ident,
        $( $IoSet:ident ),+
    ) => {
        pad_info!(@impl_pad, $PinId, $Cfg, $Sercom, $PadNum, $( $IoSet ),+ );
    };

    (
        $PinId:ident,
        $Cfg:ident,
        $Sercom:ident,
        $PadNum:ident,
        $( $IoSet:ident ),+
        + I2C
    ) => {
        impl IsI2cPad for Pin<$PinId, Alternate<$Cfg>> {}
        pad_info!(@impl_pad, $PinId, $Cfg, $Sercom, $PadNum, $( $IoSet ),+ );
    };

    (
        @impl_pad,
        $PinId:ident,
        $Cfg:ident,
        $Sercom:ident,
        $PadNum:ident,
        $( $IoSet:ident ),+
    ) => {
        impl GetPad<$Sercom> for $PinId {
            type PadNum = $PadNum;
            type PinMode = Alternate<$Cfg>;
        }
        $(
            impl InIoSet<$IoSet> for Pin<$PinId, Alternate<$Cfg>> {}
        )+
        impl IsPad for Pin<$PinId, Alternate<$Cfg>> {
            type Sercom = $Sercom;
            type PadNum = $PadNum;
        }
        impl IoSets for Pin<$PinId, Alternate<$Cfg>> {
            type SetList = mk_hlist!($( <$IoSet as IoSet>::Order ),+);
        }
    };
}

macro_rules! pad_table {
    (
        #[$id_cfg:meta]
        $PinId:ident {
            $(
                $( #[$sercom_cfg:meta] )*
                $Cfg:ident: ( $Sercom:ident, $PadNum:ident, $( $IoSet:ident ),+ ) $( + $I2C:ident )?,
            )+
        }
    ) => {
        $(
            #[$id_cfg]
            $( #[$sercom_cfg] )*
            pad_info!( $PinId, $Cfg, $Sercom, $PadNum, $( $IoSet ),+ $( + $I2C )?);
        )+
    };
    (
        $PinId:ident {
            $(
                $( #[$sercom_cfg:meta] )*
                $Cfg:ident: ( $Sercom:ident, $PadNum:ident, $( $IoSet:ident ),+ ) $( + $I2C:ident )?,
            )+
        }
    ) => {
        $(
            $( #[$sercom_cfg] )*
            pad_info!( $PinId, $Cfg, $Sercom, $PadNum, $( $IoSet ),+ $( + $I2C )?);
        )+
    };
    (
        $(
            $( #[$id_cfg:meta] )?
            $PinId:ident {
                $(
                    $( #[$sercom_cfg:meta] )*
                    $Cfg:ident: ( $Sercom:ident, $PadNum:ident, $( $IoSet:ident ),+ ) $( + $I2C:ident )?,
                )+
            }
        )+
    ) => {
        $(
            pad_table!(
                $( #[$id_cfg] )?
                $PinId{
                    $(
                        $( #[$sercom_cfg] )*
                        $Cfg: ( $Sercom, $PadNum, $( $IoSet),+ ) $( + $I2C )?,
                    )+
                }
            );
        )+
    };
}

// Implement an undocumented `IoSet` for PA16, PA17, PB22 & PB23 configured for
// `Sercom1`. The pygamer & feather_m4 uses this combination, but it is not
// listed as valid in the datasheet.

// Implement an undocumented `IoSet` for PA00, PA01, PB22 & PB23 configured for
// `Sercom1`. The itsybitsy_m4 uses this combination, but it is not
// listed as valid in the datasheet.

pad_table!(
    #[hal_cfg("pa00")]
    PA00 {
        #[cfg(not(feature = "undoc-features"))]
        #[hal_cfg("sercom1")]
        D: (Sercom1, Pad0, IoSet4),

        #[cfg(feature = "undoc-features")]
        #[hal_cfg("sercom1")]
        D: (Sercom1, Pad0, IoSet4, UndocIoSet2),
    }
    #[hal_cfg("pa01")]
    PA01 {
        #[cfg(not(feature = "undoc-features"))]
        #[hal_cfg("sercom1")]
        D: (Sercom1, Pad1, IoSet4),

        #[cfg(feature = "undoc-features")]
        #[hal_cfg("sercom1")]
        D: (Sercom1, Pad1, IoSet4, UndocIoSet2),
    }
    #[hal_cfg("pa04")]
    PA04 {
        #[hal_cfg("sercom0")]
        D: (Sercom0, Pad0, IoSet3),
    }
    #[hal_cfg("pa05")]
    PA05 {
        #[hal_cfg("sercom0")]
        D: (Sercom0, Pad1, IoSet3),
    }
    #[hal_cfg("pa06")]
    PA06 {
        #[hal_cfg("sercom0")]
        D: (Sercom0, Pad2, IoSet3),
    }
    #[hal_cfg("pa07")]
    PA07 {
        #[hal_cfg("sercom0")]
        D: (Sercom0, Pad3, IoSet3),
    }
    #[hal_cfg("pa08")]
    PA08 {
        #[hal_cfg("sercom0")]
        C: (Sercom0, Pad0, IoSet1) + I2C,
        #[hal_cfg("sercom2")]
        D: (Sercom2, Pad1, IoSet3) + I2C,
    }
    #[hal_cfg("pa09")]
    PA09 {
        #[hal_cfg("sercom0")]
        C: (Sercom0, Pad1, IoSet1) + I2C,
        #[hal_cfg("sercom2")]
        D: (Sercom2, Pad0, IoSet3) + I2C,
    }
    #[hal_cfg("pa10")]
    PA10 {
        #[hal_cfg("sercom0")]
        C: (Sercom0, Pad2, IoSet1),
        #[hal_cfg("sercom2")]
        D: (Sercom2, Pad2, IoSet3),
    }
    #[hal_cfg("pa11")]
    PA11 {
        #[hal_cfg("sercom0")]
        C: (Sercom0, Pad3, IoSet1),
        #[hal_cfg("sercom2")]
        D: (Sercom2, Pad3, IoSet3),
    }
    #[hal_cfg("pa12")]
    PA12 {
        #[hal_cfg("sercom2")]
        C: (Sercom2, Pad0, IoSet1) + I2C,
        #[hal_cfg("sercom4")]
        D: (Sercom4, Pad1, IoSet3) + I2C,
    }
    #[hal_cfg("pa13")]
    PA13 {
        #[hal_cfg("sercom2")]
        C: (Sercom2, Pad1, IoSet1) + I2C,
        #[hal_cfg("sercom4")]
        D: (Sercom4, Pad0, IoSet3) + I2C,
    }
    #[hal_cfg("pa14")]
    PA14 {
        #[hal_cfg("sercom2")]
        C: (Sercom2, Pad2, IoSet1),
        #[hal_cfg("sercom4")]
        D: (Sercom4, Pad2, IoSet3),
    }
    #[hal_cfg("pa15")]
    PA15 {
        #[hal_cfg("sercom2")]
        C: (Sercom2, Pad3, IoSet1),
        #[hal_cfg("sercom4")]
        D: (Sercom4, Pad3, IoSet3),
    }
    #[hal_cfg("pa16")]
    PA16 {
        #[hal_cfg("sercom1")]
        #[cfg(not(feature = "undoc-features"))]
        C: (Sercom1, Pad0, IoSet1) + I2C,

        #[cfg(feature = "undoc-features")]
        #[hal_cfg("sercom1")]
        C: (Sercom1, Pad0, IoSet1, UndocIoSet1) + I2C,
        #[hal_cfg("sercom3")]
        D: (Sercom3, Pad1, IoSet3) + I2C,
    }
    #[hal_cfg("pa17")]
    PA17 {
        #[cfg(not(feature = "undoc-features"))]
        #[hal_cfg("sercom1")]
        C: (Sercom1, Pad1, IoSet1) + I2C,

        #[cfg(feature = "undoc-features")]
        #[hal_cfg("sercom1")]
        C: (Sercom1, Pad1, IoSet1, UndocIoSet1) + I2C,
        #[hal_cfg("sercom3")]
        D: (Sercom3, Pad0, IoSet3) + I2C,
    }
    #[hal_cfg("pa18")]
    PA18 {
        #[hal_cfg("sercom1")]
        C: (Sercom1, Pad2, IoSet1),
        #[hal_cfg("sercom3")]
        D: (Sercom3, Pad2, IoSet3),
    }
    #[hal_cfg("pa19")]
    PA19 {
        #[hal_cfg("sercom1")]
        C: (Sercom1, Pad3, IoSet1),
        #[hal_cfg("sercom3")]
        D: (Sercom3, Pad3, IoSet3),
    }
    #[hal_cfg("pa20")]
    PA20 {
        #[hal_cfg("sercom5")]
        C: (Sercom5, Pad2, IoSet2),
        #[hal_cfg("sercom3")]
        D: (Sercom3, Pad2, IoSet2),
    }
    #[hal_cfg("pa21")]
    PA21 {
        #[hal_cfg("sercom5")]
        C: (Sercom5, Pad3, IoSet2),
        #[hal_cfg("sercom3")]
        D: (Sercom3, Pad3, IoSet2),
    }
    #[hal_cfg("pa22")]
    PA22 {
        #[hal_cfg("sercom3")]
        C: (Sercom3, Pad0, IoSet1) + I2C,
        #[hal_cfg("sercom5")]
        D: (Sercom5, Pad1, IoSet2, IoSet3, IoSet4) + I2C,
    }
    #[hal_cfg("pa23")]
    PA23 {
        #[hal_cfg("sercom3")]
        C: (Sercom3, Pad1, IoSet1) + I2C,
        #[hal_cfg("sercom5")]
        D: (Sercom5, Pad0, IoSet2, IoSet3, IoSet4) + I2C,
    }
    #[hal_cfg("pa24")]
    PA24 {
        #[hal_cfg("sercom3")]
        C: (Sercom3, Pad2, IoSet1),
        #[hal_cfg("sercom5")]
        D: (Sercom5, Pad2, IoSet3),
    }
    #[hal_cfg("pa25")]
    PA25 {
        #[hal_cfg("sercom3")]
        C: (Sercom3, Pad3, IoSet1),
        #[hal_cfg("sercom5")]
        D: (Sercom5, Pad3, IoSet3),
    }
    #[hal_cfg("pa30")]
    PA30 {
        // Pin table in 6.1 does not mention function C, but sercom table in 6.2.8.1 does
        #[hal_cfg("sercom7")]
        C: (Sercom7, Pad2, IoSet5),
        #[hal_cfg("sercom1")]
        D: (Sercom1, Pad2, IoSet4),
    }
    #[hal_cfg("pa31")]
    PA31 {
        // Pin table in 6.1 does not mention function C, but sercom table in 6.2.8.1 does
        #[hal_cfg("sercom7")]
        C: (Sercom7, Pad3, IoSet5),
        #[hal_cfg("sercom1")]
        D: (Sercom1, Pad3, IoSet4),
    }
    #[hal_cfg("pb00")]
    PB00 {
        #[hal_cfg("sercom5")]
        D: (Sercom5, Pad2, IoSet5, IoSet6),
    }
    #[hal_cfg("pb01")]
    PB01 {
        #[hal_cfg("sercom5")]
        D: (Sercom5, Pad3, IoSet5, IoSet6),
    }
    #[hal_cfg("pb02")]
    PB02 {
        #[cfg(not(feature = "undoc-features"))]
        #[hal_cfg("sercom5")]
        D: (Sercom5, Pad0, IoSet6),

        #[cfg(feature = "undoc-features")]
        #[hal_cfg("sercom5")]
        D: (Sercom5, Pad0, IoSet6) + I2C,
    }
    #[hal_cfg("pb03")]
    PB03 {
        #[cfg(not(feature = "undoc-features"))]
        #[hal_cfg("sercom5")]
        D: (Sercom5, Pad1, IoSet6),

        #[cfg(feature = "undoc-features")]
        #[hal_cfg("sercom5")]
        D: (Sercom5, Pad1, IoSet6) + I2C,
    }
    #[hal_cfg("pb08")]
    PB08 {
        #[hal_cfg("sercom4")]
        D: (Sercom4, Pad0, IoSet2),
    }
    #[hal_cfg("pb09")]
    PB09 {
        #[hal_cfg("sercom4")]
        D: (Sercom4, Pad1, IoSet2),
    }
    #[hal_cfg("pb10")]
    PB10 {
        #[hal_cfg("sercom4")]
        D: (Sercom4, Pad2, IoSet2),
    }
    #[hal_cfg("pb11")]
    PB11 {
        #[hal_cfg("sercom4")]
        D: (Sercom4, Pad3, IoSet2),
    }
    #[hal_cfg("pb12")]
    PB12 {
        #[hal_cfg("sercom4")]
        C: (Sercom4, Pad0, IoSet1),
    }
    #[hal_cfg("pb13")]
    PB13 {
        #[hal_cfg("sercom4")]
        C: (Sercom4, Pad1, IoSet1),
    }
    #[hal_cfg("pb14")]
    PB14 {
        #[hal_cfg("sercom4")]
        C: (Sercom4, Pad2, IoSet1),
    }
    #[hal_cfg("pb15")]
    PB15 {
        #[hal_cfg("sercom4")]
        C: (Sercom4, Pad3, IoSet1),
    }
    #[hal_cfg("pb16")]
    PB16 {
        #[hal_cfg("sercom5")]
        C: (Sercom5, Pad0, IoSet1),
    }
    #[hal_cfg("pb17")]
    PB17 {
        #[hal_cfg("sercom5")]
        C: (Sercom5, Pad1, IoSet1),
    }
    #[hal_cfg("pb18")]
    PB18 {
        #[hal_cfg("sercom5")]
        C: (Sercom5, Pad2, IoSet1),
        #[hal_cfg("sercom7")]
        D: (Sercom7, Pad2, IoSet4),
    }
    #[hal_cfg("pb19")]
    PB19 {
        #[hal_cfg("sercom5")]
        C: (Sercom5, Pad3, IoSet1),
        #[hal_cfg("sercom7")]
        D: (Sercom7, Pad3, IoSet4),
    }
    #[hal_cfg("pb20")]
    PB20 {
        // According to Grand Central M4, PB20 is I2C-capable. This disagrees with datasheet
        // table 6-8.
        #[hal_cfg("sercom3")]
        C: (Sercom3, Pad0, IoSet2) + I2C,
        #[hal_cfg("sercom7")]
        D: (Sercom7, Pad1, IoSet4),
    }
    #[hal_cfg("pb21")]
    PB21 {
        // According to Grand Central M4, PB21 is I2C-capable. This disagrees with datasheet
        // table 6-8.
        #[hal_cfg("sercom3")]
        C: (Sercom3, Pad1, IoSet2) + I2C,
        #[hal_cfg("sercom7")]
        D: (Sercom7, Pad0, IoSet4),
    }
    #[hal_cfg("pb22")]
    PB22 {
        #[cfg(not(feature = "undoc-features"))]
        #[hal_cfg("sercom1")]
        C: (Sercom1, Pad2, IoSet3),

        #[cfg(feature = "undoc-features")]
        #[hal_cfg("sercom1")]
        C: (Sercom1, Pad2, IoSet3, UndocIoSet1, UndocIoSet2),
        #[hal_cfg("sercom5")]
        D: (Sercom5, Pad2, IoSet4),
    }
    #[hal_cfg("pb23")]
    PB23 {
        #[cfg(not(feature = "undoc-features"))]
        #[hal_cfg("sercom1")]
        C: (Sercom1, Pad3, IoSet3),

        #[cfg(feature = "undoc-features")]
        #[hal_cfg("sercom1")]
        C: (Sercom1, Pad3, IoSet3, UndocIoSet1, UndocIoSet2),
        #[hal_cfg("sercom5")]
        D: (Sercom5, Pad3, IoSet4),
    }
    #[hal_cfg("pb24")]
    PB24 {
        #[hal_cfg("sercom0")]
        C: (Sercom0, Pad0, IoSet2),
        #[hal_cfg("sercom2")]
        D: (Sercom2, Pad1, IoSet4),
    }
    #[hal_cfg("pb25")]
    PB25 {
        #[hal_cfg("sercom0")]
        C: (Sercom0, Pad1, IoSet2),
        #[hal_cfg("sercom2")]
        D: (Sercom2, Pad0, IoSet4),
    }
    #[hal_cfg("pb26")]
    PB26 {
        #[hal_cfg("sercom2")]
        C: (Sercom2, Pad0, IoSet2),
        #[hal_cfg("sercom4")]
        D: (Sercom4, Pad1, IoSet4),
    }
    #[hal_cfg("pb27")]
    PB27 {
        #[hal_cfg("sercom2")]
        C: (Sercom2, Pad1, IoSet2),
        #[hal_cfg("sercom4")]
        D: (Sercom4, Pad0, IoSet4),
    }
    #[hal_cfg("pb28")]
    PB28 {
        #[hal_cfg("sercom2")]
        C: (Sercom2, Pad2, IoSet2),
        #[hal_cfg("sercom4")]
        D: (Sercom4, Pad2, IoSet4),
    }
    #[hal_cfg("pb29")]
    PB29 {
        #[hal_cfg("sercom2")]
        C: (Sercom2, Pad3, IoSet2),
        #[hal_cfg("sercom4")]
        D: (Sercom4, Pad3, IoSet4),
    }
    #[hal_cfg("pb30")]
    PB30 {
        // Pin table in 6.1 does not mention function C, but sercom table in 6.2.8.1 does
        #[hal_cfg("sercom7")]
        C: (Sercom7, Pad0, IoSet5),
        #[hal_cfg("sercom5")]
        D: (Sercom5, Pad1, IoSet5),
    }
    #[hal_cfg("pb31")]
    PB31 {
        // Pin table in 6.1 does not mention function C, but sercom table in 6.2.8.1 does
        #[hal_cfg("sercom7")]
        C: (Sercom7, Pad1, IoSet5),
        #[hal_cfg("sercom5")]
        D: (Sercom5, Pad0, IoSet5),
    }
    #[hal_cfg("pc04")]
    PC04 {
        #[hal_cfg("sercom6")]
        C: (Sercom6, Pad0, IoSet2),
    }
    #[hal_cfg("pc05")]
    PC05 {
        #[hal_cfg("sercom6")]
        C: (Sercom6, Pad1, IoSet2),
    }
    #[hal_cfg("pc06")]
    PC06 {
        #[hal_cfg("sercom6")]
        C: (Sercom6, Pad2, IoSet2),
    }
    #[hal_cfg("pc07")]
    PC07 {
        #[hal_cfg("sercom6")]
        C: (Sercom6, Pad3, IoSet2),
    }
    #[hal_cfg("pc10")]
    PC10 {
        #[hal_cfg("sercom6")]
        C: (Sercom6, Pad2, IoSet5),
        #[hal_cfg("sercom7")]
        D: (Sercom7, Pad2, IoSet3),
    }
    #[hal_cfg("pc11")]
    PC11 {
        #[hal_cfg("sercom6")]
        C: (Sercom6, Pad3, IoSet5),
        #[hal_cfg("sercom7")]
        D: (Sercom7, Pad3, IoSet3),
    }
    #[hal_cfg("pc12")]
    PC12 {
        #[hal_cfg("sercom7")]
        C: (Sercom7, Pad0, IoSet1, IoSet3),
        #[hal_cfg("sercom6")]
        D: (Sercom6, Pad1, IoSet4, IoSet5),
    }
    #[hal_cfg("pc13")]
    PC13 {
        #[hal_cfg("sercom7")]
        C: (Sercom7, Pad1, IoSet1, IoSet3),
        #[hal_cfg("sercom6")]
        D: (Sercom6, Pad0, IoSet4, IoSet5),
    }
    #[hal_cfg("pc14")]
    PC14 {
        #[hal_cfg("sercom7")]
        C: (Sercom7, Pad2, IoSet1),
        #[hal_cfg("sercom6")]
        D: (Sercom6, Pad2, IoSet4),
    }
    #[hal_cfg("pc15")]
    PC15 {
        #[hal_cfg("sercom7")]
        C: (Sercom7, Pad3, IoSet1),
        #[hal_cfg("sercom6")]
        D: (Sercom6, Pad3, IoSet4),
    }
    #[hal_cfg("pc16")]
    PC16 {
        #[hal_cfg("sercom6")]
        C: (Sercom6, Pad0, IoSet1),
        #[hal_cfg("sercom0")]
        D: (Sercom0, Pad1, IoSet4),
    }
    #[hal_cfg("pc17")]
    PC17 {
        #[hal_cfg("sercom6")]
        C: (Sercom6, Pad1, IoSet1),
        #[hal_cfg("sercom0")]
        D: (Sercom0, Pad0, IoSet4),
    }
    #[hal_cfg("pc18")]
    PC18 {
        #[hal_cfg("sercom6")]
        C: (Sercom6, Pad2, IoSet1),
        #[hal_cfg("sercom0")]
        D: (Sercom0, Pad2, IoSet4),
    }
    #[hal_cfg("pc19")]
    PC19 {
        #[hal_cfg("sercom6")]
        C: (Sercom6, Pad3, IoSet1),
        #[hal_cfg("sercom0")]
        D: (Sercom0, Pad3, IoSet4),
    }
    #[hal_cfg("pc22")]
    PC22 {
        #[hal_cfg("sercom1")]
        C: (Sercom1, Pad0, IoSet2),
        #[hal_cfg("sercom3")]
        D: (Sercom3, Pad1, IoSet4),
    }
    #[hal_cfg("pc23")]
    PC23 {
        #[hal_cfg("sercom1")]
        C: (Sercom1, Pad1, IoSet2),
        #[hal_cfg("sercom3")]
        D: (Sercom3, Pad0, IoSet4),
    }
    #[hal_cfg("pc24")]
    PC24 {
        #[hal_cfg("sercom0")]
        C: (Sercom0, Pad2, IoSet2),
        #[hal_cfg("sercom2")]
        D: (Sercom2, Pad2, IoSet4),
    }
    #[hal_cfg("pc25")]
    PC25 {
        #[hal_cfg("sercom0")]
        C: (Sercom0, Pad3, IoSet2),
        #[hal_cfg("sercom2")]
        D: (Sercom2, Pad3, IoSet4),
    }
    #[hal_cfg("pc27")]
    PC27 {
        #[hal_cfg("sercom1")]
        C: (Sercom1, Pad0, IoSet3),
    }
    #[hal_cfg("pc28")]
    PC28 {
        #[hal_cfg("sercom1")]
        C: (Sercom1, Pad1, IoSet3),
    }
    #[hal_cfg("pd08")]
    PD08 {
        #[hal_cfg("sercom7")]
        C: (Sercom7, Pad0, IoSet2) + I2C,
        #[hal_cfg("sercom6")]
        D: (Sercom6, Pad1, IoSet3) + I2C,
    }
    #[hal_cfg("pd09")]
    PD09 {
        #[hal_cfg("sercom7")]
        C: (Sercom7, Pad1, IoSet2) + I2C,
        #[hal_cfg("sercom6")]
        D: (Sercom6, Pad0, IoSet3) + I2C,
    }
    #[hal_cfg("pd10")]
    PD10 {
        #[hal_cfg("sercom7")]
        C: (Sercom7, Pad2, IoSet2),
        #[hal_cfg("sercom6")]
        D: (Sercom6, Pad2, IoSet3),
    }
    #[hal_cfg("pd11")]
    PD11 {
        #[hal_cfg("sercom7")]
        C: (Sercom7, Pad3, IoSet2),
        #[hal_cfg("sercom6")]
        D: (Sercom6, Pad3, IoSet3),
    }
    #[hal_cfg("pd20")]
    PD20 {
        #[hal_cfg("sercom1")]
        C: (Sercom1, Pad2, IoSet2),
        #[hal_cfg("sercom3")]
        D: (Sercom3, Pad2, IoSet4),
    }
    #[hal_cfg("pd21")]
    PD21 {
        #[hal_cfg("sercom1")]
        C: (Sercom1, Pad3, IoSet2),
        #[hal_cfg("sercom3")]
        D: (Sercom3, Pad3, IoSet4),
    }
);
