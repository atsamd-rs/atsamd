//! USB Device support

use crate::gpio;

use private::UsbSealed;
pub use usb_device;

mod bus;
pub use self::bus::UsbBus;

mod devicedesc;
use self::devicedesc::Descriptors;

/// Default SOF pad
pub type SofPad = gpio::v1::Pa23<gpio::v1::PfH>;
/// Default USB D- pad
pub type DmPad = gpio::v1::Pa24<gpio::v1::PfH>;
/// Default USB D+ pad
pub type DpPad = gpio::v1::Pa25<gpio::v1::PfH>;

/// Indicates pin can be used as USB D-
pub trait UsbPadDm: Send + private::UsbSealed {}

/// Indicates pin can be used as USB D+
pub trait UsbPadDp: Send + private::UsbSealed {}

/// Indicates pin can be used as SOF (1kHz signal)
pub trait UsbPadSof: Send + private::UsbSealed {}

// v1 pin impls
impl UsbPadSof for gpio::v1::Pa23<gpio::v1::PfH> {}
impl UsbPadDm for gpio::v1::Pa24<gpio::v1::PfH> {}
impl UsbPadDp for gpio::v1::Pa25<gpio::v1::PfH> {}

// v2 pin impls
impl UsbPadSof for gpio::v2::Pin<gpio::v2::PA23, gpio::v2::AlternateH> {}
impl UsbPadDm for gpio::v2::Pin<gpio::v2::PA24, gpio::v2::AlternateH> {}
impl UsbPadDp for gpio::v2::Pin<gpio::v2::PA25, gpio::v2::AlternateH> {}

mod private {
    use crate::gpio;

    pub trait UsbSealed {}

    // v1 pin impls
    impl UsbSealed for gpio::v1::Pa23<gpio::v1::PfH> {}
    impl UsbSealed for gpio::v1::Pa24<gpio::v1::PfH> {}
    impl UsbSealed for gpio::v1::Pa25<gpio::v1::PfH> {}

    // v2 pin impls
    impl UsbSealed for gpio::v2::Pin<gpio::v2::PA23, gpio::v2::AlternateH> {}
    impl UsbSealed for gpio::v2::Pin<gpio::v2::PA24, gpio::v2::AlternateH> {}
    impl UsbSealed for gpio::v2::Pin<gpio::v2::PA25, gpio::v2::AlternateH> {}
}
