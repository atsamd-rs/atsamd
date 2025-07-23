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

macro_rules! create_types {
    (
        $(
            $Type:ident
        ),+
    ) => {
        $(
            /// Marker type representing the corresponding peripheral
            ///
            /// This type is defined by and used within the [`clock`](super)
            /// module. See the the [`types`](self) module documentation for
            /// more details.
            pub enum $Type {}
            impl Sealed for $Type {}
        )+
    };
}

create_types!(Ac);
create_types!(Adc0);
#[hal_cfg("clock-d5x")]
create_types!(Adc1);
#[hal_cfg("clock-d5x")]
create_types!(Aes);
#[hal_cfg("can0")]
create_types!(Can0);
#[hal_cfg("can1")]
create_types!(Can1);
#[hal_cfg("clock-d5x")]
create_types!(Ccl);
#[hal_cfg("clock-d5x")]
create_types!(Cmcc);
#[hal_cfg("clock-d5x")]
create_types!(CM4Trace);
create_types!(Dac);
create_types!(Dmac);
create_types!(Dsu);
create_types!(Eic);
create_types!(
    EvSys, EvSys0, EvSys1, EvSys2, EvSys3, EvSys4, EvSys5, EvSys6, EvSys7, EvSys8, EvSys9, EvSys10,
    EvSys11
);
#[hal_cfg("clock-d5x")]
create_types!(FreqM);
#[hal_cfg("clock-d5x")]
create_types!(FreqMMeasure);
#[hal_cfg("clock-d5x")]
create_types!(FreqMReference);
create_types!(Gclk);
#[hal_cfg("gmac")]
create_types!(Gmac);
create_types!(Hpb0, Hpb1, Hpb2);
#[hal_cfg("clock-d5x")]
create_types!(Hpb3);
#[hal_cfg("clock-d5x")]
create_types!(Icm);
#[hal_cfg("clock-d5x")]
create_types!(Mclk);
create_types!(NvmCtrl);
#[hal_cfg("clock-d5x")]
create_types!(NvmCtrlSmeeProm, NvmCtrlCache);
#[hal_cfg("clock-d5x")] // TODO was min-samd51j in Bradley's work
create_types!(I2S, I2S0, I2S1);
#[hal_cfg("clock-d5x")]
create_types!(OscCtrl);
#[hal_cfg("clock-d5x")]
create_types!(Osc32kCtrl);
create_types!(Pac0);
#[hal_cfg(any("clock-d11", "clock-d21"))]
create_types!(Pac1, Pac2);
#[hal_cfg("clock-d5x")]
create_types!(Pcc);
#[hal_cfg("clock-d5x")]
create_types!(PDec);
create_types!(Pm);
create_types!(Port);
#[hal_cfg("clock-d5x")]
create_types!(Pukcc);
#[hal_cfg("clock-d5x")]
create_types!(Qspi, Qspi2x);
#[hal_cfg("clock-d5x")]
create_types!(RamEcc);
#[hal_cfg("clock-d5x")]
create_types!(RstC);
create_types!(Rtc);
#[hal_cfg("clock-d5x")]
create_types!(Sdhc0);
#[hal_cfg("sdhc1")]
create_types!(Sdhc1);
create_types!(SlowClk);
#[hal_cfg("clock-d5x")]
create_types!(SupC);
#[hal_cfg(any("clock-d11", "clock-d21"))]
create_types!(SysCtrl);
#[hal_cfg("clock-d5x")]
create_types!(Tc0Tc1, Tc0, Tc1);
#[hal_cfg("clock-d5x")]
create_types!(Tc2Tc3, Tc2, Tc3);
#[hal_cfg(all("tc4", "tc5"))]
create_types!(Tc4Tc5, Tc4, Tc5);
#[hal_cfg(all("tc6", "tc7"))]
create_types!(Tc6Tc7, Tc6, Tc7);
create_types!(Tcc0Tcc1, Tcc0, Tcc1);
#[hal_cfg("clock-d5x")]
create_types!(Tcc2Tcc3, Tcc2);
#[hal_cfg("tcc3")]
create_types!(Tcc3);
#[hal_cfg("tcc4")]
create_types!(Tcc4);
#[hal_cfg(any("clock-d11", "clock-d21"))]
create_types!(Tcc2Tc3, Tcc2, Tc3);
#[hal_cfg("clock-d5x")]
create_types!(Trng);
create_types!(Usb);
create_types!(Wdt);
