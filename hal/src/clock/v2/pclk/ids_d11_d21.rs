//! Module containing only the types implementing [`PclkId`]
//!
//! Because there are so many types that implement [`PclkId`], it is helpful to
//! have them defined in a separate module, so that you can import all of them
//! using a wildcard (`*`) without importing anything else, i.e.
//!
//! ```
//! use atsamd_hal::clock::v2::pclk::ids::*;
//! ```

use paste::paste;
use seq_macro::seq;

pub use super::super::dfll::DfllId;
pub use super::super::dpll::Dpll0Id;
pub use super::super::types::{
    AcAna, AcDig, Adc0, Dac, Eic, EvSys0, EvSys1, EvSys2, EvSys3, EvSys4, EvSys5, Ptc, Rtc,
    SercomSlow, SlowClk, Usb, Wdt,
};

seq!(N in 0..=5 {
    paste! {
        #[cfg(feature = "has-" sercom~N)]
        pub use crate::sercom::Sercom~N;
    }
});

#[cfg(feature = "samd11")]
pub use super::super::types::{Tc1Tc2, Tcc0};

#[cfg(all(feature = "has-tc6", feature = "has-tc7"))]
pub use super::super::types::Tc6Tc7;
#[cfg(feature = "samd21")]
pub use super::super::types::{
    EvSys10, EvSys11, EvSys6, EvSys7, EvSys8, EvSys9, Tc4Tc5, Tcc0Tcc1, Tcc2Tc3, I2S0, I2S1,
};

#[cfg(feature = "samd21")]
macro_rules! with_pclk_types_ids {
    ( $some_macro:ident ! ( $( $args:tt )* ) ) => {
        $some_macro!(
            $( $args )*
            (DfllId = 0, dfll)
            (Dpll0Id = 1, dpll)
            (SlowClk = 2, slow)
            (Wdt = 3, wdt)
            (Rtc = 4, rtc)
            (Eic = 5, eic)
            (Usb = 6, usb)
            (EvSys0 = 7, ev_sys0)
            (EvSys1 = 8, ev_sys1)
            (EvSys2 = 9, ev_sys2)
            (EvSys3 = 10, ev_sys3)
            (EvSys4 = 11, ev_sys4)
            (EvSys5 = 12, ev_sys5)
            (EvSys6 = 13, ev_sys6)
            (EvSys7 = 14, ev_sys7)
            (EvSys8 = 15, ev_sys8)
            (EvSys9 = 16, ev_sys9)
            (EvSys10 = 17, ev_sys10)
            (EvSys11 = 18, ev_sys11)
            (SercomSlow = 19, sercom_slow)
            (Sercom0 = 20, sercom0)
            (Sercom1 = 21, sercom1)
            #[cfg(feature = "has-sercom2")]
            (Sercom2 = 22, sercom2)
            #[cfg(feature = "has-sercom3")]
            (Sercom3 = 23, sercom3)
            #[cfg(feature = "has-sercom4")]
            (Sercom4 = 24, sercom4)
            #[cfg(feature = "has-sercom5")]
            (Sercom5 = 25, sercom5)
            (Tcc0Tcc1 = 26, tcc0_tcc1)
            (Tcc2Tc3 = 27, tcc2_tc3)
            (Tc4Tc5 = 28, tc4_tc5)
            #[cfg(all(feature = "has-tc6", feature = "has-tc7"))]
            (Tc6Tc7 = 29, tc6_tc7)
            (Adc0 = 30, adc)
            (AcDig = 31, ac_dig)
            (AcAna = 32, ac_ana)
            (Dac = 33, dac)
            (Ptc = 34, ptc)
            #[cfg(feature = "has-i2s")]
            (I2S0 = 35, i2s0)
            #[cfg(feature = "has-i2s")]
            (I2S1 = 36, i2s1)
        );
    };
}

#[cfg(feature = "samd11")]
macro_rules! with_pclk_types_ids {
    ( $some_macro:ident ! ( $( $args:tt )* ) ) => {
        $some_macro!(
            $( $args )*
            (DfllId = 0, dfll)
            (Dpll0Id = 1, dpll)
            (SlowClk = 2, slow)
            (Wdt = 3, wdt)
            (Rtc = 4, rtc)
            (Eic = 5, eic)
            (Usb = 6, usb)
            (EvSys0 = 7, ev_sys0)
            (EvSys1 = 8, ev_sys1)
            (EvSys2 = 9, ev_sys2)
            (EvSys3 = 10, ev_sys3)
            (EvSys4 = 11, ev_sys4)
            (EvSys5 = 12, ev_sys5)
            (SercomSlow = 13, sercom_slow)
            (Sercom0 = 14, sercom0)
            (Sercom1 = 15, sercom1)
            #[cfg(feature = "has-sercom2")]
            (Sercom2 = 16, sercom2)
            (Tcc0 = 17, tcc0)
            (Tc1Tc2 = 18, tc1_tc2)
            (Adc0 = 19, adc)
            (AcDig = 20, ac_dig)
            (AcAna = 21, ac_ana)
            (Dac = 22, dac)
            (Ptc = 23, ptc)
        );
    };
}

pub(super) use with_pclk_types_ids;
