//! Implementations of the [`IsPad`], [`GetPad`] and [`InIoSet`] traits

use crate::gpio::*;
use crate::sercom::*;

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
    };
}

macro_rules! pad_table {
    (
        #[$cfg:meta]
        $PinId:ident {
            $(
                $Cfg:ident: ( $Sercom:ident, $PadNum:ident, $( $IoSet:ident ),+ ) $( + $I2C:ident )?,
            )+
        }
    ) => {
        $(
            #[$cfg]
            pad_info!( $PinId, $Cfg, $Sercom, $PadNum, $( $IoSet ),+ $( + $I2C )?);
        )+
    };
    (
        $PinId:ident {
            $(
                $( #[$cfg:meta] )?
                $Cfg:ident: ( $Sercom:ident, $PadNum:ident, $( $IoSet:ident ),+ ) $( + $I2C:ident )?,
            )+
        }
    ) => {
        $(
            $( #[$cfg] )?
            pad_info!( $PinId, $Cfg, $Sercom, $PadNum, $( $IoSet ),+ $( + $I2C )?);
        )+
    };
    (
        $(
            $( #[$id_cfg:meta] )?
            $PinId:ident {
                $(
                    $( #[$sercom_cfg:meta] )?
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
                        $( #[$sercom_cfg] )?
                        $Cfg: ( $Sercom, $PadNum, $( $IoSet),+ ) $( + $I2C )?,
                    )+
                }
            );
        )+
    };
}

// Because we presume to be #[cfg(feature = "thumbv7")], we can also presume a
// minimum of #[cfg(feature = "pins-48a")], which cuts down on the number of
// required features below.
pad_table!(
    PA00 {
        D: (Sercom1, Pad0, IoSet4),
    }
    PA01 {
        D: (Sercom1, Pad1, IoSet4),
    }
    PA04 {
        D: (Sercom0, Pad0, IoSet3),
    }
    PA05 {
        D: (Sercom0, Pad1, IoSet3),
    }
    PA06 {
        D: (Sercom0, Pad2, IoSet3),
    }
    PA07 {
        D: (Sercom0, Pad3, IoSet3),
    }
    PA08 {
        C: (Sercom0, Pad0, IoSet1) + I2C,
        D: (Sercom2, Pad1, IoSet3) + I2C,
    }
    PA09 {
        C: (Sercom0, Pad1, IoSet1) + I2C,
        D: (Sercom2, Pad0, IoSet3) + I2C,
    }
    PA10 {
        C: (Sercom0, Pad2, IoSet1),
        D: (Sercom2, Pad2, IoSet3),
    }
    PA11 {
        C: (Sercom0, Pad3, IoSet1),
        D: (Sercom2, Pad3, IoSet3),
    }
    PA12 {
        C: (Sercom2, Pad0, IoSet1) + I2C,
        D: (Sercom4, Pad1, IoSet3) + I2C,
    }
    PA13 {
        C: (Sercom2, Pad1, IoSet1) + I2C,
        D: (Sercom4, Pad0, IoSet3) + I2C,
    }
    PA14 {
        C: (Sercom2, Pad2, IoSet1),
        D: (Sercom4, Pad2, IoSet3),
    }
    PA15 {
        C: (Sercom2, Pad3, IoSet1),
        D: (Sercom4, Pad3, IoSet3),
    }
    PA16 {
        C: (Sercom1, Pad0, IoSet1) + I2C,
        D: (Sercom3, Pad1, IoSet3) + I2C,
    }
    PA17 {
        C: (Sercom1, Pad1, IoSet1) + I2C,
        D: (Sercom3, Pad0, IoSet3) + I2C,
    }
    PA18 {
        C: (Sercom1, Pad2, IoSet1),
        D: (Sercom3, Pad2, IoSet3),
    }
    PA19 {
        C: (Sercom1, Pad3, IoSet1),
        D: (Sercom3, Pad3, IoSet3),
    }
    PA20 {
        C: (Sercom5, Pad2, IoSet2),
        D: (Sercom3, Pad2, IoSet2),
    }
    PA21 {
        C: (Sercom5, Pad3, IoSet2),
        D: (Sercom3, Pad3, IoSet2),
    }
    PA22 {
        C: (Sercom3, Pad0, IoSet1) + I2C,
        D: (Sercom5, Pad1, IoSet2, IoSet3, IoSet4) + I2C,
    }
    PA23 {
        C: (Sercom3, Pad1, IoSet1) + I2C,
        D: (Sercom5, Pad0, IoSet2, IoSet3, IoSet4) + I2C,
    }
    PA24 {
        C: (Sercom3, Pad2, IoSet1),
        D: (Sercom5, Pad2, IoSet3),
    }
    PA25 {
        C: (Sercom3, Pad3, IoSet1),
        D: (Sercom5, Pad3, IoSet3),
    }
    PA30 {
        #[cfg(feature = "has-sercom7")]
        C: (Sercom7, Pad2, IoSet5),
        D: (Sercom1, Pad2, IoSet4),
    }
    PA31 {
        #[cfg(feature = "has-sercom7")]
        C: (Sercom7, Pad3, IoSet5),
        D: (Sercom1, Pad3, IoSet4),
    }
    #[cfg(feature = "has-pb00")]
    PB00 {
        D: (Sercom5, Pad2, IoSet5, IoSet6),
    }
    #[cfg(feature = "has-pb01")]
    PB01 {
        D: (Sercom5, Pad3, IoSet5, IoSet6),
    }
    PB02 {
        // According to Metro M4, PB02 is I2C-capable. This disagrees with datasheet table 6-8.
        D: (Sercom5, Pad0, IoSet6) + I2C,
    }
    PB03 {
        // According to Metro M4, PB03 is I2C-capable. This disagrees with datasheet table 6-8.
        D: (Sercom5, Pad1, IoSet6) + I2C,
    }
    PB08 {
        D: (Sercom4, Pad0, IoSet2),
    }
    PB09 {
        D: (Sercom4, Pad1, IoSet2),
    }
    PB10 {
        D: (Sercom4, Pad2, IoSet2),
    }
    PB11 {
        D: (Sercom4, Pad3, IoSet2),
    }
    #[cfg(feature = "pins-64")]
    PB12 {
        C: (Sercom4, Pad0, IoSet1),
    }
    #[cfg(feature = "pins-64")]
    PB13 {
        C: (Sercom4, Pad1, IoSet1),
    }
    #[cfg(feature = "pins-64")]
    PB14 {
        C: (Sercom4, Pad2, IoSet1),
    }
    #[cfg(feature = "pins-64")]
    PB15 {
        C: (Sercom4, Pad3, IoSet1),
    }
    #[cfg(feature = "pins-64")]
    PB16 {
        C: (Sercom5, Pad0, IoSet1),
    }
    #[cfg(feature = "pins-64")]
    PB17 {
        C: (Sercom5, Pad1, IoSet1),
    }
    #[cfg(feature = "pins-100")]
    PB18 {
        C: (Sercom5, Pad2, IoSet1),
        D: (Sercom7, Pad2, IoSet4),
    }
    #[cfg(feature = "pins-100")]
    PB19 {
        C: (Sercom5, Pad3, IoSet1),
        D: (Sercom7, Pad3, IoSet4),
    }
    #[cfg(feature = "pins-100")]
    PB20 {
        // According to Grand Central M4, PB20 is I2C-capable. This disagrees with datasheet
        // table 6-8.
        C: (Sercom3, Pad0, IoSet2) + I2C,
        D: (Sercom7, Pad1, IoSet4),
    }
    #[cfg(feature = "pins-100")]
    PB21 {
        // According to Grand Central M4, PB21 is I2C-capable. This disagrees with datasheet
        // table 6-8.
        C: (Sercom3, Pad1, IoSet2) + I2C,
        D: (Sercom7, Pad0, IoSet4),
    }
    PB22 {
        C: (Sercom1, Pad2, IoSet3),
        D: (Sercom5, Pad2, IoSet4),
    }
    PB23 {
        C: (Sercom1, Pad3, IoSet3),
        D: (Sercom5, Pad3, IoSet4),
    }
    #[cfg(feature = "pins-100")]
    PB24 {
        C: (Sercom0, Pad0, IoSet2),
        D: (Sercom2, Pad1, IoSet4),
    }
    #[cfg(feature = "pins-100")]
    PB25 {
        C: (Sercom0, Pad1, IoSet2),
        D: (Sercom2, Pad0, IoSet4),
    }
    #[cfg(feature = "pins-128")]
    PB26 {
        C: (Sercom2, Pad0, IoSet2),
        D: (Sercom4, Pad1, IoSet4),
    }
    #[cfg(feature = "pins-128")]
    PB27 {
        C: (Sercom2, Pad1, IoSet2),
        D: (Sercom4, Pad0, IoSet4),
    }
    #[cfg(feature = "pins-128")]
    PB28 {
        C: (Sercom2, Pad2, IoSet2),
        D: (Sercom4, Pad2, IoSet4),
    }
    #[cfg(feature = "pins-128")]
    PB29 {
        C: (Sercom2, Pad3, IoSet2),
        D: (Sercom4, Pad3, IoSet4),
    }
    PB30 {
        #[cfg(feature = "has-sercom7")]
        C: (Sercom7, Pad0, IoSet5),
        #[cfg(feature = "pins-64")]
        D: (Sercom5, Pad1, IoSet5),
    }
    PB31 {
        #[cfg(feature = "has-sercom7")]
        C: (Sercom7, Pad1, IoSet5),
        #[cfg(feature = "pins-64")]
        D: (Sercom5, Pad0, IoSet5),
    }
    #[cfg(feature = "pins-128")]
    PC04 {
        C: (Sercom6, Pad0, IoSet2),
    }
    #[cfg(feature = "pins-100")]
    PC05 {
        C: (Sercom6, Pad1, IoSet2),
    }
    #[cfg(feature = "pins-100")]
    PC06 {
        C: (Sercom6, Pad2, IoSet2),
    }
    #[cfg(feature = "pins-100")]
    PC07 {
        C: (Sercom6, Pad3, IoSet2),
    }
    #[cfg(feature = "pins-100")]
    PC10 {
        C: (Sercom6, Pad2, IoSet5),
        D: (Sercom7, Pad2, IoSet3),
    }
    #[cfg(feature = "pins-100")]
    PC11 {
        C: (Sercom6, Pad3, IoSet5),
        D: (Sercom7, Pad3, IoSet3),
    }
    #[cfg(feature = "pins-100")]
    PC12 {
        C: (Sercom7, Pad0, IoSet1, IoSet3),
        D: (Sercom6, Pad1, IoSet4, IoSet5),
    }
    #[cfg(feature = "pins-100")]
    PC13 {
        C: (Sercom7, Pad1, IoSet1, IoSet3),
        D: (Sercom6, Pad0, IoSet4, IoSet5),
    }
    #[cfg(feature = "pins-100")]
    PC14 {
        C: (Sercom7, Pad2, IoSet1),
        D: (Sercom6, Pad2, IoSet4),
    }
    #[cfg(feature = "pins-100")]
    PC15 {
        C: (Sercom7, Pad3, IoSet1),
        D: (Sercom6, Pad3, IoSet4),
    }
    #[cfg(feature = "pins-100")]
    PC16 {
        C: (Sercom6, Pad0, IoSet1),
        D: (Sercom0, Pad1, IoSet4),
    }
    #[cfg(feature = "pins-100")]
    PC17 {
        C: (Sercom6, Pad1, IoSet1),
        D: (Sercom0, Pad0, IoSet4),
    }
    #[cfg(feature = "pins-100")]
    PC18 {
        C: (Sercom6, Pad2, IoSet1),
        D: (Sercom0, Pad2, IoSet4),
    }
    #[cfg(feature = "pins-100")]
    PC19 {
        C: (Sercom6, Pad3, IoSet1),
        D: (Sercom0, Pad3, IoSet4),
    }
    #[cfg(feature = "pins-128")]
    PC22 {
        C: (Sercom1, Pad0, IoSet2),
        D: (Sercom3, Pad1, IoSet4),
    }
    #[cfg(feature = "pins-128")]
    PC23 {
        C: (Sercom1, Pad1, IoSet2),
        D: (Sercom3, Pad0, IoSet4),
    }
    #[cfg(feature = "pins-100")]
    PC24 {
        C: (Sercom0, Pad2, IoSet2),
        D: (Sercom2, Pad2, IoSet4),
    }
    #[cfg(feature = "pins-100")]
    PC25 {
        C: (Sercom0, Pad3, IoSet2),
        D: (Sercom2, Pad3, IoSet4),
    }
    #[cfg(feature = "pins-100")]
    PC27 {
        C: (Sercom1, Pad0, IoSet3),
    }
    #[cfg(feature = "pins-100")]
    PC28 {
        C: (Sercom1, Pad1, IoSet3),
    }
    #[cfg(feature = "pins-128")]
    PD08 {
        C: (Sercom7, Pad0, IoSet2) + I2C,
        D: (Sercom6, Pad1, IoSet3) + I2C,
    }
    #[cfg(feature = "pins-128")]
    PD09 {
        C: (Sercom7, Pad1, IoSet2) + I2C,
        D: (Sercom6, Pad0, IoSet3) + I2C,
    }
    #[cfg(feature = "pins-128")]
    PD10 {
        C: (Sercom7, Pad2, IoSet2),
        D: (Sercom6, Pad2, IoSet3),
    }
    #[cfg(feature = "pins-128")]
    PD11 {
        C: (Sercom7, Pad3, IoSet2),
        D: (Sercom6, Pad3, IoSet3),
    }
    #[cfg(feature = "pins-128")]
    PD20 {
        C: (Sercom1, Pad2, IoSet2),
        D: (Sercom3, Pad2, IoSet4),
    }
    #[cfg(feature = "pins-128")]
    PD21 {
        C: (Sercom1, Pad3, IoSet2),
        D: (Sercom3, Pad3, IoSet4),
    }
);

// Implement an undocumented `IoSet` for PA16, PA17, PB22 & PB23 configured for
// `Sercom1`. The pygamer & feather_m4 uses this combination, but it is not
// listed as valid in the datasheet.
impl InIoSet<UndocIoSet1> for Pin<PA16, Alternate<C>> {}
impl InIoSet<UndocIoSet1> for Pin<PA17, Alternate<C>> {}
impl InIoSet<UndocIoSet1> for Pin<PB22, Alternate<C>> {}
impl InIoSet<UndocIoSet1> for Pin<PB23, Alternate<C>> {}

// Implement an undocumented `IoSet` for PA00, PA01, PB22 & PB23 configured for
// `Sercom1`. The itsybitsy_m4 uses this combination, but it is not
// listed as valid in the datasheet.
impl InIoSet<UndocIoSet2> for Pin<PA00, Alternate<D>> {}
impl InIoSet<UndocIoSet2> for Pin<PA01, Alternate<D>> {}
impl InIoSet<UndocIoSet2> for Pin<PB22, Alternate<C>> {}
impl InIoSet<UndocIoSet2> for Pin<PB23, Alternate<C>> {}
