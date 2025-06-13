use crate::adc::*;
use crate::gpio::pin::*;
use atsamd_hal_macros::hal_cfg;

macro_rules! adc_pins {
    (
        $(
            $( #[$cfg:meta] )?
            $PinId:ident: ($Adc:ident, $CHAN:literal)
        ),+
        $(,)?
    ) => {
        crate::paste::item! {
            $(
                $( #[$cfg] )?
                impl AdcPin<$Adc> for Pin<$PinId, AlternateB> {
                    const CHANNEL: u8 = $CHAN;
                }
            )+
        }
    };
}

#[hal_cfg(any("adc-d21"))]
adc_pins! {
    #[hal_cfg("pa02")]
    PA02: (Adc0, 0),
    #[hal_cfg("pa03")]
    PA03: (Adc0, 1),

    #[hal_cfg("pb08")]
    PB08: (Adc0, 2),
    #[hal_cfg("pb09")]
    PB09: (Adc0, 3),

    #[hal_cfg("pa04")]
    PA04: (Adc0, 4),
    #[hal_cfg("pa05")]
    PA05: (Adc0, 5),
    #[hal_cfg("pa06")]
    PA06: (Adc0, 6),
    #[hal_cfg("pa07")]
    PA07: (Adc0, 7),

    #[hal_cfg("pb00")]
    PB00: (Adc0, 8),
    #[hal_cfg("pb01")]
    PB01: (Adc0, 9),
    #[hal_cfg("pb02")]
    PB02: (Adc0, 10),
    #[hal_cfg("pb03")]
    PB03: (Adc0, 11),

    #[hal_cfg("pb04")]
    PB04: (Adc0, 12),
    #[hal_cfg("pb05")]
    PB05: (Adc0, 13),
    #[hal_cfg("pb06")]
    PB06: (Adc0, 14),
    #[hal_cfg("pb07")]
    PB07: (Adc0, 15),

    #[hal_cfg("pa08")]
    PA08: (Adc0, 16),
    #[hal_cfg("pa09")]
    PA09: (Adc0, 17),
    #[hal_cfg("pa10")]
    PA10: (Adc0, 18),
    #[hal_cfg("pa11")]
    PA11: (Adc0, 19),
}

#[hal_cfg(any("adc-d11"))]
adc_pins! {
    #[hal_cfg("pa02")]
    PA02: (Adc0, 0),
    #[hal_cfg("pa03")]
    PA03: (Adc0, 1),
    #[hal_cfg("pa04")]
    PA04: (Adc0, 2),
    #[hal_cfg("pa05")]
    PA05: (Adc0, 3),
    #[hal_cfg("pa06")]
    PA06: (Adc0, 4),
    #[hal_cfg("pa07")]
    PA07: (Adc0, 5),
    #[hal_cfg("pa14")]
    PA14: (Adc0, 6),
    #[hal_cfg("pa15")]
    PA15: (Adc0, 7),
    #[hal_cfg("pa10")]
    PA10: (Adc0, 8),
    #[hal_cfg("pa11")]
    PA11: (Adc0, 9),
}
