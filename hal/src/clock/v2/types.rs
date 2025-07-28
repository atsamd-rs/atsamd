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

use atsamd_hal_macros::hal_cfg;

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
            /// This type is defined by and used within the
            /// [`clock`](super::super) module. See the the [`types`](super)
            /// module documentation for more details.
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
#[hal_cfg("i2s")]
create_types!(I2S);
create_types!(Pac0);
create_types!(Pm);
create_types!(Port);
create_types!(Rtc);
#[hal_cfg("sercom0")]
pub use crate::sercom::Sercom0;
#[hal_cfg("sercom1")]
pub use crate::sercom::Sercom1;
#[hal_cfg("sercom2")]
pub use crate::sercom::Sercom2;
#[hal_cfg("sercom3")]
pub use crate::sercom::Sercom3;
#[hal_cfg("sercom4")]
pub use crate::sercom::Sercom4;
#[hal_cfg("sercom5")]
pub use crate::sercom::Sercom5;
#[hal_cfg("sercom6")]
pub use crate::sercom::Sercom6;
#[hal_cfg("sercom7")]
pub use crate::sercom::Sercom7;

#[hal_cfg("tc0")]
create_types!(Tc0);
#[hal_cfg("tc1")]
create_types!(Tc1);
#[hal_cfg("tc2")]
create_types!(Tc2);
#[hal_cfg("tc3")]
create_types!(Tc3);
#[hal_cfg("tc4")]
create_types!(Tc4);
#[hal_cfg("tc5")]
create_types!(Tc5);
#[hal_cfg("tc6")]
create_types!(Tc6);
#[hal_cfg("tc7")]
create_types!(Tc7);

#[hal_cfg("tcc0")]
create_types!(Tcc0);
#[hal_cfg("tcc1")]
create_types!(Tcc1);
#[hal_cfg("tcc2")]
create_types!(Tcc2);
#[hal_cfg("tcc3")]
create_types!(Tcc3);
#[hal_cfg("tcc4")]
create_types!(Tcc4);

// TODO
#[hal_cfg(all("tc0", "tc1"))]
create_types!(Tc0Tc1);
#[hal_cfg(all("tc2", "tc3"))]
create_types!(Tc2Tc3);
#[hal_cfg(all("tc4", "tc5"))]
create_types!(Tc4Tc5);
#[hal_cfg(all("tc6", "tc7"))]
create_types!(Tc6Tc7);

#[hal_cfg(all("tcc0", "tcc1"))]
create_types!(Tcc0Tcc1);
#[hal_cfg(all("tcc2", "tcc3"))]
create_types!(Tcc2Tcc3);

#[hal_cfg(all("tc1-d11", "tc2-d11"))]
create_types!(Tc1Tc2);

#[hal_cfg(all("tcc2", "tc3-d21"))]
create_types!(Tcc2Tc3);

create_types!(Wdt);

// PCLK types
// TODO set channel numbers in devices.yaml
create_types!(
    EvSys0, EvSys1, EvSys2, EvSys3, EvSys4, EvSys5, EvSys6, EvSys7, EvSys8, EvSys9, EvSys10,
    EvSys11
);
#[hal_cfg("i2s")]
create_types!(I2S0, I2S1);
create_types!(SlowClk);

#[hal_cfg("clock-d5x")]
mod variant_ahb_types {
    use super::{Sealed, hal_cfg};

    #[hal_cfg("can0")]
    create_types!(Can0);
    #[hal_cfg("can1")]
    create_types!(Can1);
    create_types!(Cmcc);
    #[hal_cfg("gmac")]
    create_types!(Gmac);
    create_types!(Hpb3);
    create_types!(Icm);
    create_types!(NvmCtrlSmeeProm, NvmCtrlCache);
    create_types!(Pukcc);
    create_types!(Qspi, Qspi2x);
    #[hal_cfg("sdhc0")]
    create_types!(Sdhc0);
    #[hal_cfg("sdhc1")]
    create_types!(Sdhc1);
}

#[hal_cfg("clock-d5x")]
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

#[hal_cfg("clock-d5x")]
mod variant_pclk_types {
    use super::Sealed;

    create_types!(CM4Trace);
    create_types!(FreqMMeasure);
    create_types!(FreqMReference);
}

#[hal_cfg(any("clock-d11", "clock-d21"))]
mod variant_apb_types {
    use super::{Sealed, hal_cfg};

    #[hal_cfg("clock-d21")]
    create_types!(Ac1);
    create_types!(Pac1, Pac2);
    create_types!(Ptc);
    create_types!(SysCtrl);
}

#[hal_cfg(any("clock-d11", "clock-d21"))]
mod variant_pclk_types {
    use super::Sealed;

    create_types!(AcDig);
    create_types!(AcAna);
    create_types!(SercomSlow);
}

#[hal_cfg("clock-d5x")]
pub use variant_ahb_types::*;
pub use variant_apb_types::*;
pub use variant_pclk_types::*;
