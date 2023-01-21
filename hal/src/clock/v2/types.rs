//! Module defining or exporting peripheral types for the ['ahb'], ['apb'] and
//! ['pclk'] modules
//!
//! The `ahb`, `apb` and `pclk` modules each define structs that are
//! generic over a type parameter representing a peripheral. Some peripheral
//! modules already define suitable types for this purpose. For example,
//! [`sercom`] defines the [`Sercom0`], [`Sercom1`], etc. types. But other
//! peripherals are either not yet implemented in the HAL or do not define a
//! suitable type. This module defines a type for such peripherals. If/when a
//! suitable type is added for a given peripheral, the type defined here should
//! be deprecated or removed.
//!
//! [`ahb`]: super::ahb
//! [`apb`]: super::apb
//! [`pclk`]: super::pclk
//! [`sercom`]: crate::sercom
//! [`Sercom0`]: crate::sercom::Sercom0
//! [`Sercom1`]: crate::sercom::Sercom1

use paste::paste;
use seq_macro::seq;

use crate::typelevel::Sealed;

macro_rules! create_types {
    (
        $(
            $Type:ident
        ),+
    ) => {
        $(
            /// Marker type representing the corresponding peripheral
            ///
            /// This type is defined by and used within the [`clock`](super::super)
            /// module. See the the [`types`](super) module documentation for
            /// more details.
            pub enum $Type {}
            impl Sealed for $Type {}
        )+
    };
}

// AHB types
create_types!(Dmac);
create_types!(Dsu);
create_types!(Hpb0, Hpb1, Hpb2);
create_types!(NvmCtrl);
create_types!(Usb);

// APB types
create_types!(Ac);
create_types!(Adc0);
create_types!(Dac);
create_types!(Eic);
create_types!(EvSys);
create_types!(Gclk);
#[cfg(feature = "has-i2s")]
create_types!(I2S);
create_types!(Pac0);
create_types!(Pm);
create_types!(Port);
create_types!(Rtc);
seq!(N in 0..=7 {
    paste! {
        #[cfg(feature = "has-" sercom~N)]
        pub use crate::sercom::Sercom~N;
    }
});
seq!(N in 0..=7 {
    paste! {
        #[cfg(feature = "has-" tc~N)]
        create_types!(Tc~N);
    }
});
seq!(N in 0..=4 {
    paste! {
        #[cfg(feature = "has-" tcc~N)]
        create_types!(Tcc~N);
    }
});
create_types!(Wdt);

// PCLK types
create_types!(EvSys0, EvSys1, EvSys2, EvSys3, EvSys4, EvSys5);
#[cfg(any(feature = "samd21", feature = "samd5xe5x"))]
create_types!(EvSys6, EvSys7, EvSys8, EvSys9, EvSys10, EvSys11);
#[cfg(feature = "has-i2s")]
create_types!(I2S0, I2S1);
create_types!(SlowClk);
#[cfg(all(feature = "has-tc4", feature = "has-tc5"))]
create_types!(Tc4Tc5);
#[cfg(all(feature = "has-tc6", feature = "has-tc7"))]
create_types!(Tc6Tc7);

#[cfg(feature = "samd5xe5x")]
mod variant_ahb_types {
    use super::Sealed;

    #[cfg(feature = "has-can0")]
    create_types!(Can0);
    #[cfg(feature = "has-can1")]
    create_types!(Can1);
    create_types!(Cmcc);
    #[cfg(feature = "has-ethernet")]
    create_types!(Gmac);
    create_types!(Hpb3);
    create_types!(Icm);
    create_types!(NvmCtrlSmeeProm, NvmCtrlCache);
    create_types!(Pukcc);
    create_types!(Qspi, Qspi2x);
    create_types!(Sdhc0);
    #[cfg(feature = "has-sdhc1")]
    create_types!(Sdhc1);
}

#[cfg(feature = "samd5xe5x")]
mod variant_apb_types {
    use super::Sealed;

    create_types!(Adc1);
    create_types!(Aes);
    create_types!(Ccl);
    create_types!(FreqM);
    create_types!(Mclk);
    create_types!(OscCtrl);
    create_types!(Osc32kCtrl);
    create_types!(Pcc);
    create_types!(PDec);
    create_types!(RamEcc);
    create_types!(RstC);
    create_types!(SupC);
    create_types!(Trng);
}

#[cfg(feature = "samd5xe5x")]
mod variant_pclk_types {
    use super::Sealed;

    create_types!(CM4Trace);
    create_types!(FreqMMeasure);
    create_types!(FreqMReference);
    create_types!(Tc0Tc1);
    create_types!(Tc2Tc3);
    create_types!(Tcc0Tcc1);
    #[cfg(all(feature = "has-tcc2", feature = "has-tcc3"))]
    create_types!(Tcc2Tcc3);
}

#[cfg(any(feature = "samd11", feature = "samd21"))]
mod variant_apb_types {
    use super::Sealed;

    #[cfg(feature = "samd21")]
    create_types!(Ac1);
    create_types!(Pac1, Pac2);
    create_types!(Ptc);
    create_types!(SysCtrl);
}

#[cfg(any(feature = "samd11", feature = "samd21"))]
mod variant_pclk_types {
    use super::Sealed;
    create_types!(AcDig);
    create_types!(AcAna);
    create_types!(SercomSlow);
    #[cfg(feature = "samd11")]
    create_types!(Tc1Tc2);
    #[cfg(feature = "samd21")]
    create_types!(Tcc0Tcc1);
    #[cfg(feature = "samd21")]
    create_types!(Tcc2Tc3);
}

#[cfg(feature = "samd5xe5x")]
pub use variant_ahb_types::*;
pub use variant_apb_types::*;
pub use variant_pclk_types::*;
