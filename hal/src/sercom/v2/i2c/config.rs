//! I2C [`Config`] definition and implementation

use super::{I2c, InactiveTimeout, PadSet, Registers};
use crate::{
    pac::sercom0::i2cm::ctrla::MODE_A,
    sercom::v2::*,
    time::Hertz,
    typelevel::{Is, Sealed},
};

//=============================================================================
// Operating mode
//=============================================================================

/// Type-level enum representing the I2C operating mode
///
/// See the documentation on [type-level enums] for a discussion of the pattern.
///
/// The available operating modes are [`Master`] and [`Slave`].
/// [type-level enums]: crate::typelevel#type-level-enums
pub trait OpMode: Sealed {
    /// Corresponding variant from the PAC enum
    const MODE: MODE_A;
}

/// [`OpMode`] variant for Master mode
pub enum Master {}
/// [`OpMode`] variant for Slave mode
pub enum Slave {}

impl Sealed for Master {}
impl OpMode for Master {
    const MODE: MODE_A = MODE_A::I2C_MASTER;
}

impl Sealed for Slave {}
/*
impl OpMode for Slave {
    const MODE: MODE_A = MODE_A::I2C_SLAVE;
}
*/

//=============================================================================
// Config
//=============================================================================

/// A configurable, disabled UART peripheral
///
/// This `struct` represents a configurable UART peripheral in its disabled
/// state. It is generic over the set of [`Pads`] and [`CharSize`].
/// Upon creation, the [`Config`] takes ownership of the
/// [`Sercom`] and resets it, returning it configured as an UART peripheral
/// with a default configuration:
///
/// * [`EightBit`]
/// * No parity
/// * One stop bit
/// * LSB-first
///
/// [`Config`] uses a builder-pattern API to configure the peripheral,
/// culminating in a call to [`enable`], which consumes the [`Config`] and
/// returns enabled [`Uart`].
///
/// [`enable`]: Config::enable
/// [`Pads`]: super::Pads
pub struct Config<P>
where
    P: PadSet,
{
    pub(super) registers: Registers<P::Sercom>,
    pads: P,
    freq: Hertz,
}

impl<P: PadSet> Config<P> {
    /// Create a new [`Config`] in the default configuration.
    #[inline]
    fn default(sercom: P::Sercom, pads: P, freq: impl Into<Hertz>) -> Self {
        let mut registers = Registers::new(sercom);
        registers.swrst();
        registers.set_op_mode(Master::MODE);
        Self {
            registers,
            pads,
            freq: freq.into(),
        }
    }

    /// Create a new [`Config`] in the default configuration
    ///
    /// This function will enable the corresponding APB clock, reset the
    /// [`Sercom`] peripheral, and return a [`Config`] in the default
    /// configuration. The default [`OpMode`] is [`Master`], while the default
    /// [`Size`] is an
    #[cfg_attr(
        any(feature = "samd11", feature = "samd21"),
        doc = "[`EightBit`] [`CharSize`]"
    )]
    #[cfg_attr(feature = "min-samd51g", doc = "`EightBit` `CharSize`")]
    /// for SAMD11 and SAMD21 chips or a
    #[cfg_attr(any(feature = "samd11", feature = "samd21"), doc = "`Length` of `U1`")]
    #[cfg_attr(feature = "min-samd51g", doc = "[`Length`] of `U1`")]
    /// for SAMx5x chips. Note that [`Config`] takes ownership of both the
    /// PAC [`Sercom`] struct as well as the [`Pads`].
    ///
    /// Users must configure GCLK manually. The `freq` parameter represents the
    /// GCLK frequency for this [`Sercom`] instance.
    #[inline]
    pub fn new(
        apb_clk_ctrl: &APB_CLK_CTRL,
        mut sercom: P::Sercom,
        pads: P,
        freq: impl Into<Hertz>,
    ) -> Self {
        sercom.enable_apb_clock(apb_clk_ctrl);
        Self::default(sercom, pads, freq)
    }
}

impl<P: PadSet> Config<P> {
    /// Obtain a reference to the PAC `SERCOM` struct
    ///
    /// Directly accessing the `SERCOM` could break the invariants of the
    /// type-level tracking in this module, so it is unsafe.
    #[inline]
    pub unsafe fn sercom(&self) -> &P::Sercom {
        &self.registers.sercom
    }

    /// Trigger the [`Sercom`]'s SWRST and return a [`Config`] in the
    /// default configuration.
    #[inline]
    pub fn reset(self) -> Config<P> {
        Config::default(self.registers.sercom, self.pads, self.freq)
    }

    /// Consume the [`Config`], reset the peripheral, and return the [`Sercom`]
    /// and [`Pads`]
    #[inline]
    pub fn free(mut self) -> (P::Sercom, P) {
        self.registers.swrst();
        (self.registers.free(), self.pads)
    }

    /// Run in standby mode (builder pattern version)
    ///
    /// When set, the UART peripheral will run in standby mode. See the
    /// datasheet for more details.
    #[inline]
    pub fn run_in_standby(mut self, set: bool) -> Self {
        self.set_run_in_standby(set);
        self
    }

    /// Run in standby mode (setter version)
    ///
    /// When set, the UART peripheral will run in standby mode. See the
    /// datasheet for more details.
    #[inline]
    pub fn set_run_in_standby(&mut self, set: bool) {
        self.registers.set_run_in_standby(set);
    }

    /// Get the current run in standby mode
    #[inline]
    pub fn get_run_in_standby(&self) -> bool {
        self.registers.get_run_in_standby()
    }

    /// Set the baud rate (builder pattern version)
    ///
    /// This function will calculate the best BAUD register setting based on the
    /// stored GCLK frequency and desired baud rate. The maximum baud rate is
    /// GCLK frequency/oversampling. Values outside this range will saturate at
    /// the maximum supported baud rate.
    ///
    /// Note that 3x oversampling is not supported.
    #[inline]
    pub fn baud(mut self, baud: impl Into<Hertz>) -> Self {
        self.set_baud(baud);
        self
    }

    /// Set the baud rate (setter version)
    ///
    /// This function will calculate the best BAUD register setting based on the
    /// stored GCLK frequency and desired baud rate. The maximum baud rate is
    /// GCLK frequency/oversampling. Values outside this range will saturate at
    /// the maximum supported baud rate.
    ///
    /// Note that 3x oversampling is not supported.
    #[inline]
    pub fn set_baud(&mut self, baud: impl Into<Hertz>) {
        self.registers.set_baud(self.freq, baud);
    }

    /// Get the contents of the `BAUD` register and the current baud mode. Note
    /// that only the CONTENTS of `BAUD` are returned, and not the actual baud
    /// rate. Refer to the datasheet to convert the `BAUD` register contents
    /// into a baud rate.
    #[inline]
    pub fn get_baud(&self) -> u32 {
        self.registers.get_baud()
    }

    /// Set SCL Low Time-Out (builder pattern version)
    ///
    /// If SCL is held low for 25ms-35ms, the master will release its clock
    /// hold, if enabled, and complete the current transaction. A stop condition
    /// will automatically be transmitted. INTFLAG.SB or INTFLAG.MB will be set
    /// as normal, but the clock hold will be released. The STATUS.LOWTOUT and
    /// STATUS.BUSERR status bits will be set.
    #[inline]
    pub fn low_timeout(mut self, set: bool) -> Self {
        self.set_low_timeout(set);
        self
    }

    /// Set SCL Low Time-Out (setter version)
    ///
    /// If SCL is held low for 25ms-35ms, the master will release its clock
    /// hold, if enabled, and complete the current transaction. A stop condition
    /// will automatically be transmitted. INTFLAG.SB or INTFLAG.MB will be set
    /// as normal, but the clock hold will be released. The STATUS.LOWTOUT and
    /// STATUS.BUSERR status bits will be set.
    #[inline]
    pub fn set_low_timeout(&mut self, set: bool) {
        self.registers.set_low_timeout(set);
    }

    /// Get SCL Low Time-Out
    ///
    /// If SCL is held low for 25ms-35ms, the master will release its clock
    /// hold, if enabled, and complete the current transaction. A stop condition
    /// will automatically be transmitted. INTFLAG.SB or INTFLAG.MB will be set
    /// as normal, but the clock hold will be released. The STATUS.LOWTOUT and
    /// STATUS.BUSERR status bits will be set.
    #[inline]
    pub fn get_low_timeout(&mut self) -> bool {
        self.registers.get_low_timeout()
    }

    /// Set the inactive timeout (builder pattern version).
    ///
    /// Timeout after which the bus state will be set to IDLE. Necessary for
    /// SMBus compatibility.
    #[inline]
    pub fn inactive_timeout(mut self, timeout: super::InactiveTimeout) -> Self {
        self.set_inactive_timeout(timeout);
        self
    }

    /// Set the inactive timeout (setter version).
    ///
    /// Timeout after which the bus state will be set to IDLE. Necessary for
    /// SMBus compatibility.
    #[inline]
    pub fn set_inactive_timeout(&mut self, timeout: super::InactiveTimeout) {
        self.registers.set_inactive_timeout(timeout);
    }

    /// Get the inactive timeout setting.
    #[inline]
    pub fn get_inactive_timeout(&mut self) -> InactiveTimeout {
        self.registers.get_inactive_timeout()
    }

    /// Enable the I2C peripheral
    ///
    /// I2C transactions are not possible until the peripheral is enabled.
    #[inline]
    pub fn enable(mut self) -> I2c<Self>
    where
        Self: AnyConfig,
    {
        self.registers.enable();

        I2c { config: self }
    }
}

//=============================================================================
// AnyConfig
//=============================================================================

/// Type class for all possible [`Config`] types
///
/// This trait uses the [`AnyKind`] trait pattern to create a [type class] for
/// [`Config`] types. See the `AnyKind` documentation for more details on the
/// pattern.
///
/// In addition to the normal, `AnyKind` associated types. This trait also
/// copies the [`Sercom`], [`Capability`] and [`Word`] types, to make it easier
/// to apply bounds to these types at the next level of abstraction.
///
/// [`AnyKind`]: crate::typelevel#anykind-trait-pattern
/// [type class]: crate::typelevel#type-classes
pub trait AnyConfig: Is<Type = SpecificConfig<Self>> {
    type Sercom: Sercom;
    type Pads: PadSet<Sercom = Self::Sercom>;
}

/// Type alias to recover the specific [`Config`] type from an implementation of
/// [`AnyConfig`]
pub type SpecificConfig<C> = Config<<C as AnyConfig>::Pads>;

impl<P: PadSet> Sealed for Config<P> {}

impl<P: PadSet> AnyConfig for Config<P> {
    type Sercom = P::Sercom;
    type Pads = P;
}

impl<P: PadSet> AsRef<Self> for Config<P> {
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<P: PadSet> AsMut<Self> for Config<P> {
    #[inline]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}
