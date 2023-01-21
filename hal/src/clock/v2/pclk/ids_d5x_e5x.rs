//! Module containing only the types implementing [`PclkId`]
//!
//! Because there are so many types that implement [`PclkId`], it is helpful to
//! have them defined in a separate module, so that you can import all of them
//! using a wildcard (`*`) without importing anything else, i.e.
//!
//! ```
//! use atsamd_hal::clock::v2::pclk::ids::*;
//! ```

pub use crate::sercom::{Sercom0, Sercom1, Sercom2, Sercom3, Sercom4, Sercom5};

#[cfg(feature = "has-sercom6")]
pub use crate::sercom::Sercom6;
#[cfg(feature = "has-sercom7")]
pub use crate::sercom::Sercom7;

pub use super::super::dfll::DfllId;
pub use super::super::dpll::Dpll0Id;
pub use super::super::dpll::Dpll1Id;
pub use super::super::types::{
    Ac, Adc0, Adc1, CM4Trace, Ccl, Dac, Eic, EvSys0, EvSys1, EvSys10, EvSys11, EvSys2, EvSys3,
    EvSys4, EvSys5, EvSys6, EvSys7, EvSys8, EvSys9, FreqMMeasure, FreqMReference, PDec, Sdhc0,
    SlowClk, Tc0Tc1, Tc2Tc3, Tcc0Tcc1, Tcc2Tcc3, Usb,
};

#[cfg(feature = "has-sdhc1")]
pub use super::super::types::Sdhc1;
#[cfg(all(feature = "has-tc4", feature = "has-tc5"))]
pub use super::super::types::Tc4Tc5;
#[cfg(all(feature = "has-tc6", feature = "has-tc7"))]
pub use super::super::types::Tc6Tc7;
#[cfg(feature = "has-tcc4")]
pub use super::super::types::Tcc4;
#[cfg(feature = "has-can0")]
pub use super::super::types::Can0;
#[cfg(feature = "has-can1")]
pub use super::super::types::Can1;
#[cfg(feature = "has-i2s")]
pub use super::super::types::{I2S0, I2S1};

macro_rules! with_pclk_types_ids {
    ( $some_macro:ident ! ( $( $args:tt )* ) ) => {
        $some_macro!(
            $( $args )*
            (DfllId = 0, dfll)
            (Dpll0Id = 1, dpll0)
            (Dpll1Id = 2, dpll1)
            (SlowClk = 3, slow)
            (Eic = 4, eic)
            (FreqMMeasure = 5, freq_m_measure)
            (FreqMReference = 6, freq_m_reference)
            (Sercom0 = 7, sercom0)
            (Sercom1 = 8, sercom1)
            (Tc0Tc1 = 9, tc0_tc1)
            (Usb = 10, usb)
            (EvSys0 = 11, ev_sys0)
            (EvSys1 = 12, ev_sys1)
            (EvSys2 = 13, ev_sys2)
            (EvSys3 = 14, ev_sys3)
            (EvSys4 = 15, ev_sys4)
            (EvSys5 = 16, ev_sys5)
            (EvSys6 = 17, ev_sys6)
            (EvSys7 = 18, ev_sys7)
            (EvSys8 = 19, ev_sys8)
            (EvSys9 = 20, ev_sys9)
            (EvSys10 = 21, ev_sys10)
            (EvSys11 = 22, ev_sys11)
            (Sercom2 = 23, sercom2)
            (Sercom3 = 24, sercom3)
            (Tcc0Tcc1 = 25, tcc0_tcc1)
            (Tc2Tc3 = 26, tc2_tc3)
            #[cfg(feature = "has-can0")]
            (Can0 = 27, can0)
            #[cfg(feature = "has-can1")]
            (Can1 = 28, can1)
            (Tcc2Tcc3 = 29, tcc2_tcc3)
            #[cfg(all(feature = "has-tc4", feature = "has-tc5"))]
            (Tc4Tc5 = 30, tc4_tc5)
            (PDec = 31, pdec)
            (Ac = 32, ac)
            (Ccl = 33, ccl)
            (Sercom4 = 34, sercom4)
            (Sercom5 = 35, sercom5)
            #[cfg(feature = "has-sercom6")]
            (Sercom6 = 36, sercom6)
            #[cfg(feature = "has-sercom7")]
            (Sercom7 = 37, sercom7)
            #[cfg(feature = "has-tcc4")]
            (Tcc4 = 38, tcc4)
            #[cfg(all(feature = "has-tc6", feature = "has-tc7"))]
            (Tc6Tc7 = 39, tc6_tc7)
            (Adc0 = 40, adc0)
            (Adc1 = 41, adc1)
            (Dac = 42, dac)
            #[cfg(feature = "has-i2s")]
            (I2S0 = 43, i2s0)
            #[cfg(feature = "has-i2s")]
            (I2S1 = 44, i2s1)
            (Sdhc0 = 45, sdhc0)
            #[cfg(feature = "has-sdhc1")]
            (Sdhc1 = 46, sdhc1)
            (CM4Trace = 47, cm4_trace)
        );
    };
}
