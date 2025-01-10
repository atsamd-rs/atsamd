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
    #[hal_cfg("pa08")]
    PA08: (Adc0, 8),
    #[hal_cfg("pa09")]
    PA09: (Adc0, 9),
    #[hal_cfg("pa10")]
    PA10: (Adc0, 10),
    #[hal_cfg("pa11")]
    PA11: (Adc0, 11),
    #[hal_cfg("pb00")]
    PB00: (Adc0, 12),
    #[hal_cfg("pb01")]
    PB01: (Adc0, 13),
    #[hal_cfg("pb02")]
    PB02: (Adc0, 14),
    #[hal_cfg("pb03")]
    PB03: (Adc0, 15),

    #[hal_cfg("pb08")]
    PB08: (Adc1, 0),
    #[hal_cfg("pb09")]
    PB09: (Adc1, 1),
    #[hal_cfg("pa08")]
    PA08: (Adc1, 2),
    #[hal_cfg("pa09")]
    PA09: (Adc1, 3),
    #[hal_cfg("pc02")]
    PC02: (Adc1, 4),
    #[hal_cfg("pc03")]
    PC03: (Adc1, 5),
    #[hal_cfg("pb04")]
    PB04: (Adc1, 6),
    #[hal_cfg("pb05")]
    PB05: (Adc1, 7),
    #[hal_cfg("pb06")]
    PB06: (Adc1, 8),
    #[hal_cfg("pb07")]
    PB07: (Adc1, 9),
    #[hal_cfg("pc00")]
    PC00: (Adc1, 10),
    #[hal_cfg("pc01")]
    PC01: (Adc1, 11),
    #[hal_cfg("pc30")]
    PC30: (Adc1, 12),
    #[hal_cfg("pc31")]
    PC31: (Adc1, 13),
    #[hal_cfg("pd00")]
    PD00: (Adc1, 14),
    #[hal_cfg("pd01")]
    PD01: (Adc1, 15),
}
