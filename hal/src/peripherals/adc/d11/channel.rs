use core::marker::PhantomData;
use atsamd_hal_macros::hal_cfg;
use crate::{
    adc::*,
    pac::adc::inputctrl::{Muxposselect, Muxnegselect},
    typelevel::Sealed,
};

macro_rules! channel {
    (
        $(
            $CH:ident: ($($PMUX:path)?, $($NMUX:path)?) $(+ $MARKER:ident)*
        ),+
        $(,)?
    ) => {
        crate::paste::paste!{
            $(
                pub struct $CH<I: AdcInstance> {
                    adc: PhantomData<I>,
                }

                impl<I: AdcInstance> Sealed for $CH<I> {}

                $(
                    impl<I: AdcInstance> PosChannel<I> for $CH<I> {
                        const MUXVAL: Muxposselect = $PMUX;

                        fn get_channel() -> Self {
                            Self {
                                adc: PhantomData
                            }
                        }
                    }
                )?
                $(
                    impl<I: AdcInstance> NegChannel<I> for $CH<I> {
                        const MUXVAL: Muxnegselect = $NMUX;

                        fn get_channel() -> Self {
                            Self {
                                adc: PhantomData
                            }
                        }
                    }
                )?
                $(
                    impl<I: AdcInstance> $MARKER<I> for $CH<I> {}
                )*
            )+
        }
    };
}

#[hal_cfg("adc-d21")]
channel! {
    AIN0: (Muxposselect::Pin0, Muxnegselect::Pin0),
    AIN1: (Muxposselect::Pin1, Muxnegselect::Pin1),
    AIN2: (Muxposselect::Pin2, Muxnegselect::Pin2),
    AIN3: (Muxposselect::Pin3, Muxnegselect::Pin3),
    AIN4: (Muxposselect::Pin4, Muxnegselect::Pin4),
    AIN5: (Muxposselect::Pin5, Muxnegselect::Pin5),
    AIN6: (Muxposselect::Pin6, Muxnegselect::Pin6),
    AIN7: (Muxposselect::Pin7, Muxnegselect::Pin7),
    AIN8: (Muxposselect::Pin8, ),
    AIN9: (Muxposselect::Pin9, ),
    AIN10: (Muxposselect::Pin10, ),
    AIN11: (Muxposselect::Pin11, ),
    AIN12: (Muxposselect::Pin12, ),
    AIN13: (Muxposselect::Pin13, ),
    AIN14: (Muxposselect::Pin14, ),
    AIN15: (Muxposselect::Pin15, ),
    AIN16: (Muxposselect::Pin15, ),
    AIN17: (Muxposselect::Pin15, ),
    AIN18: (Muxposselect::Pin15, ),
    AIN19: (Muxposselect::Pin15, ),
    TEMP: (Muxposselect::Temp, ),
    SCALEDCOREVCC: (Muxposselect::Scaledcorevcc, ) + CpuVoltageSource,
    SCALEDIOVCC: (Muxposselect::Scalediovcc, ) + CpuVoltageSource,
    BANDGAP: (Muxposselect::Bandgap, ) + CpuVoltageSource,
    DAC: (Muxposselect::Dac, ),
    GND: (, Muxnegselect::Gnd),
    IOGND: (, Muxnegselect::Iognd),
}

#[hal_cfg("adc-d11")]
channel! {
    AIN0: (Muxposselect::Pin0, Muxnegselect::Pin0),
    AIN1: (Muxposselect::Pin1, Muxnegselect::Pin1),
    AIN2: (Muxposselect::Pin2, Muxnegselect::Pin2),
    AIN3: (Muxposselect::Pin3, Muxnegselect::Pin3),
    AIN4: (Muxposselect::Pin4, Muxnegselect::Pin4),
    AIN5: (Muxposselect::Pin5, Muxnegselect::Pin5),
    AIN6: (Muxposselect::Pin6, Muxnegselect::Pin6),
    AIN7: (Muxposselect::Pin7, Muxnegselect::Pin7),
    AIN8: (Muxposselect::Pin8, ),
    AIN9: (Muxposselect::Pin9, ),
    TEMP: (Muxposselect::Temp, ),
    SCALEDCOREVCC: (Muxposselect::Scaledcorevcc, ) + CpuVoltageSource,
    SCALEDIOVCC: (Muxposselect::Scalediovcc, ) + CpuVoltageSource,
    BANDGAP: (Muxposselect::Bandgap, ) + CpuVoltageSource,
    DAC: (Muxposselect::Dac, ),
    GND: (, Muxnegselect::Gnd),
    IOGND: (, Muxnegselect::Iognd),
}
