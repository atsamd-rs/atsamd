use crate::adc::*;
use crate::gpio::pin::*;
use atsamd_hal_macros::hal_cfg;

macro_rules! pos_adc_pins {
    (
        $(
            $( #[$cfg:meta] )?
            $PinId:ident: ($Adc:ident, $CHAN:ident)
        ),+
        $(,)?
    ) => {
        crate::paste::item! {
            $(
                $( #[$cfg] )?
                impl PosAdcPin<$Adc> for Pin<$PinId, AlternateB> {
                    type Channel = $CHAN<$Adc>;
                }
            )+
        }
    };
}

macro_rules! neg_adc_pins {
    (
        $(
            $( #[$cfg:meta] )?
            $PinId:ident: ($Adc:ident, $CHAN:ident)
        ),+
        $(,)?
    ) => {
        crate::paste::item! {
            $(
                $( #[$cfg] )?
                impl NegAdcPin<$Adc> for Pin<$PinId, AlternateB> {
                    type Channel = $CHAN<$Adc>;
                }
            )+
        }
    };
}

pos_adc_pins! {
    #[hal_cfg("pa02")]
    PA02: (Adc0, AIN0),
    #[hal_cfg("pa03")]
    PA03: (Adc0, AIN1),
    #[hal_cfg("pb08")]
    PB08: (Adc0, AIN2),
    #[hal_cfg("pb09")]
    PB09: (Adc0, AIN3),
    #[hal_cfg("pa04")]
    PA04: (Adc0, AIN4),
    #[hal_cfg("pa05")]
    PA05: (Adc0, AIN5),
    #[hal_cfg("pa06")]
    PA06: (Adc0, AIN6),
    #[hal_cfg("pa07")]
    PA07: (Adc0, AIN7),
    #[hal_cfg("pa08")]
    PA08: (Adc0, AIN8),
    #[hal_cfg("pa09")]
    PA09: (Adc0, AIN9),
    #[hal_cfg("pa10")]
    PA10: (Adc0, AIN10),
    #[hal_cfg("pa11")]
    PA11: (Adc0, AIN11),
    #[hal_cfg("pb00")]
    PB00: (Adc0, AIN12),
    #[hal_cfg("pb01")]
    PB01: (Adc0, AIN13),
    #[hal_cfg("pb02")]
    PB02: (Adc0, AIN14),
    #[hal_cfg("pb03")]
    PB03: (Adc0, AIN15),

    #[hal_cfg("pb08")]
    PB08: (Adc1, AIN0),
    #[hal_cfg("pb09")]
    PB09: (Adc1, AIN1),
    #[hal_cfg("pa08")]
    PA08: (Adc1, AIN2),
    #[hal_cfg("pa09")]
    PA09: (Adc1, AIN3),
    #[hal_cfg("pc02")]
    PC02: (Adc1, AIN4),
    #[hal_cfg("pc03")]
    PC03: (Adc1, AIN5),
    #[hal_cfg("pb04")]
    PB04: (Adc1, AIN6),
    #[hal_cfg("pb05")]
    PB05: (Adc1, AIN7),
    #[hal_cfg("pb06")]
    PB06: (Adc1, AIN8),
    #[hal_cfg("pb07")]
    PB07: (Adc1, AIN9),
    #[hal_cfg("pc00")]
    PC00: (Adc1, AIN10),
    #[hal_cfg("pc01")]
    PC01: (Adc1, AIN11),
    #[hal_cfg("pc30")]
    PC30: (Adc1, AIN12),
    #[hal_cfg("pc31")]
    PC31: (Adc1, AIN13),
    #[hal_cfg("pd00")]
    PD00: (Adc1, AIN14),
    #[hal_cfg("pd01")]
    PD01: (Adc1, AIN15),
}

neg_adc_pins! {
    #[hal_cfg("pa02")]
    PA02: (Adc0, AIN0),
    #[hal_cfg("pa03")]
    PA03: (Adc0, AIN1),
    #[hal_cfg("pb08")]
    PB08: (Adc0, AIN2),
    #[hal_cfg("pb09")]
    PB09: (Adc0, AIN3),
    #[hal_cfg("pa04")]
    PA04: (Adc0, AIN4),
    #[hal_cfg("pa05")]
    PA05: (Adc0, AIN5),
    #[hal_cfg("pa06")]
    PA06: (Adc0, AIN6),
    #[hal_cfg("pa07")]
    PA07: (Adc0, AIN7),

    #[hal_cfg("pb08")]
    PB08: (Adc1, AIN0),
    #[hal_cfg("pb09")]
    PB09: (Adc1, AIN1),
    #[hal_cfg("pa08")]
    PA08: (Adc1, AIN2),
    #[hal_cfg("pa09")]
    PA09: (Adc1, AIN3),
    #[hal_cfg("pc02")]
    PC02: (Adc1, AIN4),
    #[hal_cfg("pc03")]
    PC03: (Adc1, AIN5),
    #[hal_cfg("pb04")]
    PB04: (Adc1, AIN6),
    #[hal_cfg("pb05")]
    PB05: (Adc1, AIN7),
}
