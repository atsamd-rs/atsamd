//! I2C host/

use crate::pac;
use crate::typelevel::Sealed;
use crate::sercom::Sercom;
use super::{Flags, ClientFlags};

/// Representation of an I2C mode: host or client
pub trait I2cMode: Sealed {
    // Allows for some method reuse
    #[doc(hidden)]
    type Flag;
    /// Register block type
    #[doc(hidden)]
    type RegBlock;

    fn get_regblock<S:Sercom>(sercom: &S) -> Self::RegBlock;
}

/// Marker type for I2C host mode
pub struct Host;

impl Sealed for Host {}

impl I2cMode for Host {
    type Flag = Flags;
    type RegBlock = crate::pac::sercom0::I2CM;

    fn get_regblock<S:Sercom>(sercom: &S) -> Self::RegBlock{
        sercom.i2cm()
    }
}

/// Marker type for I2C client mode
pub struct Client;

impl Sealed for Client {}

impl I2cMode for Client {
    type Flag = ClientFlags;
    type RegBlock = crate::pac::sercom0::I2CM;

    fn get_regblock<S:Sercom>(sercom: &S) -> Self::RegBlock {
        sercom.i2cs()
    }
}
