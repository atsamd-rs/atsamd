use super::{I2cClient, InactiveTimeout, PadSet, Registers};
use crate::{
    sercom::{Sercom, APB_CLK_CTRL},
    time::Hertz,
    typelevel::{Is, Sealed},
};

/// A configurable, disabled I2C peripheral
///
/// This `struct` represents a configurable I2C peripheral in its disabled
/// state. It is generic over the set of [`Pads`].
/// Upon creation, the [`Config`] takes ownership of the
/// [`Sercom`] and resets it, returning it configured as an I2C peripheral
/// with a default configuration in Master mode.
///
/// [`Config`] uses a builder-pattern API to configure the peripheral,
/// culminating in a call to [`enable`], which consumes the [`Config`] and
/// returns an enabled [`I2c`].
///
/// [`enable`]: Config::enable
/// [`Pads`]: super::Pads
pub struct ClientConfig<P>
where
    P: PadSet,
{
    pub(super) registers: Registers<P::Sercom>,
    pads: P,
    freq: Hertz,
}

impl<P: PadSet> ClientConfig<P> {
    /// Create a new [`Config`] in the default configuration.
    #[inline]
    fn default(sercom: P::Sercom, pads: P, freq: impl Into<Hertz>) -> Self {
        let mut registers = Registers::new(sercom);
        registers.swrst();
        registers.set_op_mode();
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
    /// configuration. The only available operating mode is currently Master.
    ///
    /// Note that [`Config`] takes ownership of both the
    /// PAC [`Sercom`] struct as well as the [`Pads`](super::Pads).
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

impl<P: PadSet> ClientConfig<P> {
    /// Obtain a reference to the PAC `SERCOM` struct
    ///
    /// # Safety
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
    pub fn reset(self) -> Self {
        Self::default(self.registers.sercom, self.pads, self.freq)
    }

    /// Consume the [`Config`], reset the peripheral, and return the
    /// [`Sercom`] and [`Pads`](super::Pads)
    #[inline]
    pub fn free(mut self) -> (P::Sercom, P) {
        self.registers.swrst();
        (self.registers.free(), self.pads)
    }

    /// Run in standby mode (builder pattern version)
    ///
    /// When set, the I2C peripheral will run in standby mode. See the
    /// datasheet for more details.
    #[inline]
    pub fn run_in_standby(mut self, set: bool) -> Self {
        self.set_run_in_standby(set);
        self
    }

    /// Run in standby mode (setter version)
    ///
    /// When set, the I2C peripheral will run in standby mode. See the
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

    /// Enable the I2C peripheral
    ///
    /// I2C transactions are not possible until the peripheral is enabled.
    #[inline]
    pub fn enable(mut self) -> I2cClient<Self>
    where
        Self: ClientAnyConfig,
    {
        self.registers.enable();

        I2cClient { config: self }
    }
}

//=============================================================================
// AnyConfig
//=============================================================================

/// Type class for all possible [`Config`] types
///
/// This trait uses the [`AnyKind`] trait pattern to create a [type class] for
/// [`Config`] types. See the [`AnyKind`] documentation for more details on the
/// pattern.
///
/// In addition to the normal, [`AnyKind`] associated types. This trait also
/// copies the [`Sercom`] type, to make it easier
/// to apply bounds to these types at the next level of abstraction.
///
/// [`AnyKind`]: crate::typelevel#anykind-trait-pattern
/// [type class]: crate::typelevel#type-classes
pub trait ClientAnyConfig: Is<Type = ClientSpecificConfig<Self>> {
    type Sercom: Sercom;
    type Pads: PadSet<Sercom = Self::Sercom>;
}

/// Type alias to recover the specific [`Config`] type from an implementation of
/// [`AnyConfig`]
pub type ClientSpecificConfig<C> = ClientConfig<<C as ClientAnyConfig>::Pads>;

/// Type alias to recover the specific [`Sercom`] type from an implementation of
/// [`AnyConfig`]
pub type ConfigSercom<C> = <C as ClientAnyConfig>::Sercom;

impl<P: PadSet> Sealed for ClientConfig<P> {}

impl<P: PadSet> ClientAnyConfig for ClientConfig<P> {
    type Sercom = P::Sercom;
    type Pads = P;
}

impl<P: PadSet> AsRef<Self> for ClientConfig<P> {
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<P: PadSet> AsMut<Self> for ClientConfig<P> {
    #[inline]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}
