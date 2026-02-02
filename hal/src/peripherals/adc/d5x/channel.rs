use core::marker::PhantomData;
use crate::adc::*;
use crate::{
    pac::adc0::inputctrl::{Muxposselect, Muxnegselect},
    typelevel::Sealed,
};

macro_rules! channel {
    (
        $(
            $CH:ident: ($($PMUX:path)?, $($NMUX:path)?)
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
                    }
                )?
                $(
                    impl<I: AdcInstance> NegChannel<I> for $CH<I> {
                        const MUXVAL: Muxnegselect = $NMUX;
                    }
                )?

                impl<I: AdcInstance> $CH<I> {
                    pub fn get_channel() -> Self {
                        Self {
                            adc: PhantomData
                        }
                    }
                }
            )+
        }
    };
}

channel! {
    AIN0: (Muxposselect::Ain0, Muxnegselect::Ain0),
    AIN1: (Muxposselect::Ain1, Muxnegselect::Ain1),
    AIN2: (Muxposselect::Ain2, Muxnegselect::Ain2),
    AIN3: (Muxposselect::Ain3, Muxnegselect::Ain3),
    AIN4: (Muxposselect::Ain4, Muxnegselect::Ain4),
    AIN5: (Muxposselect::Ain5, Muxnegselect::Ain5),
    AIN6: (Muxposselect::Ain6, Muxnegselect::Ain6),
    AIN7: (Muxposselect::Ain7, Muxnegselect::Ain7),
    AIN8: (Muxposselect::Ain8, ),
    AIN9: (Muxposselect::Ain9, ),
    AIN10: (Muxposselect::Ain10, ),
    AIN11: (Muxposselect::Ain11, ),
    AIN12: (Muxposselect::Ain12, ),
    AIN13: (Muxposselect::Ain13, ),
    AIN14: (Muxposselect::Ain14, ),
    AIN15: (Muxposselect::Ain15, ),
    SCALEDCOREVCC: (Muxposselect::Scaledcorevcc, ),
    SCALEDVBAT: (Muxposselect::Scaledvbat, ),
    SCALEDIOVCC: (Muxposselect::Scalediovcc, ),
    BANDGAP: (Muxposselect::Bandgap, ),
    PTAT: (Muxposselect::Ptat, ),
    CTAT: (Muxposselect::Ctat, ),
    GND: (, Muxnegselect::Gnd),
}
