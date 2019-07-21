#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRLB {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        self.register.set(f(&R { bits }, &mut W { bits }).bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        self.register.set(
            f(&mut W {
                bits: Self::reset_value(),
            })
            .bits,
        );
    }
    #[doc = r" Reset value of the register"]
    #[inline]
    pub const fn reset_value() -> u32 {
        0
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = "Possible values of the field `RWS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWSR {
    #[doc = "Single Auto Wait State"]
    SINGLE,
    #[doc = "Half Auto Wait State"]
    HALF,
    #[doc = "Dual Auto Wait State"]
    DUAL,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RWSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RWSR::SINGLE => 0,
            RWSR::HALF => 0x01,
            RWSR::DUAL => 0x02,
            RWSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RWSR {
        match value {
            0 => RWSR::SINGLE,
            1 => RWSR::HALF,
            2 => RWSR::DUAL,
            i => RWSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline]
    pub fn is_single(&self) -> bool {
        *self == RWSR::SINGLE
    }
    #[doc = "Checks if the value of the field is `HALF`"]
    #[inline]
    pub fn is_half(&self) -> bool {
        *self == RWSR::HALF
    }
    #[doc = "Checks if the value of the field is `DUAL`"]
    #[inline]
    pub fn is_dual(&self) -> bool {
        *self == RWSR::DUAL
    }
}
#[doc = r" Value of the field"]
pub struct MANWR {
    bits: bool,
}
impl MANWR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Possible values of the field `SLEEPPRM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEPPRMR {
    #[doc = "NVM block enters low-power mode when entering sleep.NVM block exits low-power mode upon first access."]
    WAKEONACCESS,
    #[doc = "NVM block enters low-power mode when entering sleep.NVM block exits low-power mode when exiting sleep."]
    WAKEUPINSTANT,
    #[doc = "Auto power reduction disabled."]
    DISABLED,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SLEEPPRMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SLEEPPRMR::WAKEONACCESS => 0,
            SLEEPPRMR::WAKEUPINSTANT => 0x01,
            SLEEPPRMR::DISABLED => 0x03,
            SLEEPPRMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SLEEPPRMR {
        match value {
            0 => SLEEPPRMR::WAKEONACCESS,
            1 => SLEEPPRMR::WAKEUPINSTANT,
            3 => SLEEPPRMR::DISABLED,
            i => SLEEPPRMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `WAKEONACCESS`"]
    #[inline]
    pub fn is_wakeonaccess(&self) -> bool {
        *self == SLEEPPRMR::WAKEONACCESS
    }
    #[doc = "Checks if the value of the field is `WAKEUPINSTANT`"]
    #[inline]
    pub fn is_wakeupinstant(&self) -> bool {
        *self == SLEEPPRMR::WAKEUPINSTANT
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SLEEPPRMR::DISABLED
    }
}
#[doc = "Possible values of the field `READMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READMODER {
    #[doc = "The NVM Controller (cache system) does not insert wait states on a cache miss. Gives the best system performance."]
    NO_MISS_PENALTY,
    #[doc = "Reduces power consumption of the cache system, but inserts a wait state each time there is a cache miss. This mode may not be relevant if CPU performance is required, as the application will be stalled and may lead to increase run time."]
    LOW_POWER,
    #[doc = "The cache system ensures that a cache hit or miss takes the same amount of time, determined by the number of programmed flash wait states. This mode can be used for real-time applications that require deterministic execution timings."]
    DETERMINISTIC,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl READMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            READMODER::NO_MISS_PENALTY => 0,
            READMODER::LOW_POWER => 0x01,
            READMODER::DETERMINISTIC => 0x02,
            READMODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> READMODER {
        match value {
            0 => READMODER::NO_MISS_PENALTY,
            1 => READMODER::LOW_POWER,
            2 => READMODER::DETERMINISTIC,
            i => READMODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NO_MISS_PENALTY`"]
    #[inline]
    pub fn is_no_miss_penalty(&self) -> bool {
        *self == READMODER::NO_MISS_PENALTY
    }
    #[doc = "Checks if the value of the field is `LOW_POWER`"]
    #[inline]
    pub fn is_low_power(&self) -> bool {
        *self == READMODER::LOW_POWER
    }
    #[doc = "Checks if the value of the field is `DETERMINISTIC`"]
    #[inline]
    pub fn is_deterministic(&self) -> bool {
        *self == READMODER::DETERMINISTIC
    }
}
#[doc = r" Value of the field"]
pub struct CACHEDISR {
    bits: bool,
}
impl CACHEDISR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Values that can be written to the field `RWS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWSW {
    #[doc = "Single Auto Wait State"]
    SINGLE,
    #[doc = "Half Auto Wait State"]
    HALF,
    #[doc = "Dual Auto Wait State"]
    DUAL,
}
impl RWSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RWSW::SINGLE => 0,
            RWSW::HALF => 1,
            RWSW::DUAL => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RWSW<'a> {
    w: &'a mut W,
}
impl<'a> _RWSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RWSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Single Auto Wait State"]
    #[inline]
    pub fn single(self) -> &'a mut W {
        self.variant(RWSW::SINGLE)
    }
    #[doc = "Half Auto Wait State"]
    #[inline]
    pub fn half(self) -> &'a mut W {
        self.variant(RWSW::HALF)
    }
    #[doc = "Dual Auto Wait State"]
    #[inline]
    pub fn dual(self) -> &'a mut W {
        self.variant(RWSW::DUAL)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x0f << 1);
        self.w.bits |= ((value as u32) & 0x0f) << 1;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MANWW<'a> {
    w: &'a mut W,
}
impl<'a> _MANWW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(0x01 << 7);
        self.w.bits |= ((value as u32) & 0x01) << 7;
        self.w
    }
}
#[doc = "Values that can be written to the field `SLEEPPRM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEPPRMW {
    #[doc = "NVM block enters low-power mode when entering sleep.NVM block exits low-power mode upon first access."]
    WAKEONACCESS,
    #[doc = "NVM block enters low-power mode when entering sleep.NVM block exits low-power mode when exiting sleep."]
    WAKEUPINSTANT,
    #[doc = "Auto power reduction disabled."]
    DISABLED,
}
impl SLEEPPRMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SLEEPPRMW::WAKEONACCESS => 0,
            SLEEPPRMW::WAKEUPINSTANT => 1,
            SLEEPPRMW::DISABLED => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLEEPPRMW<'a> {
    w: &'a mut W,
}
impl<'a> _SLEEPPRMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLEEPPRMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "NVM block enters low-power mode when entering sleep.NVM block exits low-power mode upon first access."]
    #[inline]
    pub fn wakeonaccess(self) -> &'a mut W {
        self.variant(SLEEPPRMW::WAKEONACCESS)
    }
    #[doc = "NVM block enters low-power mode when entering sleep.NVM block exits low-power mode when exiting sleep."]
    #[inline]
    pub fn wakeupinstant(self) -> &'a mut W {
        self.variant(SLEEPPRMW::WAKEUPINSTANT)
    }
    #[doc = "Auto power reduction disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SLEEPPRMW::DISABLED)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x03 << 8);
        self.w.bits |= ((value as u32) & 0x03) << 8;
        self.w
    }
}
#[doc = "Values that can be written to the field `READMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READMODEW {
    #[doc = "The NVM Controller (cache system) does not insert wait states on a cache miss. Gives the best system performance."]
    NO_MISS_PENALTY,
    #[doc = "Reduces power consumption of the cache system, but inserts a wait state each time there is a cache miss. This mode may not be relevant if CPU performance is required, as the application will be stalled and may lead to increase run time."]
    LOW_POWER,
    #[doc = "The cache system ensures that a cache hit or miss takes the same amount of time, determined by the number of programmed flash wait states. This mode can be used for real-time applications that require deterministic execution timings."]
    DETERMINISTIC,
}
impl READMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            READMODEW::NO_MISS_PENALTY => 0,
            READMODEW::LOW_POWER => 1,
            READMODEW::DETERMINISTIC => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _READMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _READMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: READMODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The NVM Controller (cache system) does not insert wait states on a cache miss. Gives the best system performance."]
    #[inline]
    pub fn no_miss_penalty(self) -> &'a mut W {
        self.variant(READMODEW::NO_MISS_PENALTY)
    }
    #[doc = "Reduces power consumption of the cache system, but inserts a wait state each time there is a cache miss. This mode may not be relevant if CPU performance is required, as the application will be stalled and may lead to increase run time."]
    #[inline]
    pub fn low_power(self) -> &'a mut W {
        self.variant(READMODEW::LOW_POWER)
    }
    #[doc = "The cache system ensures that a cache hit or miss takes the same amount of time, determined by the number of programmed flash wait states. This mode can be used for real-time applications that require deterministic execution timings."]
    #[inline]
    pub fn deterministic(self) -> &'a mut W {
        self.variant(READMODEW::DETERMINISTIC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x03 << 16);
        self.w.bits |= ((value as u32) & 0x03) << 16;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CACHEDISW<'a> {
    w: &'a mut W,
}
impl<'a> _CACHEDISW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(0x01 << 18);
        self.w.bits |= ((value as u32) & 0x01) << 18;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 1:4 - NVM Read Wait States"]
    #[inline]
    pub fn rws(&self) -> RWSR {
        RWSR::_from(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 7 - Manual Write"]
    #[inline]
    pub fn manw(&self) -> MANWR {
        let bits = ((self.bits >> 7) & 0x01) != 0;
        MANWR { bits }
    }
    #[doc = "Bits 8:9 - Power Reduction Mode during Sleep"]
    #[inline]
    pub fn sleepprm(&self) -> SLEEPPRMR {
        SLEEPPRMR::_from(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - NVMCTRL Read Mode"]
    #[inline]
    pub fn readmode(&self) -> READMODER {
        READMODER::_from(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 18 - Cache Disable"]
    #[inline]
    pub fn cachedis(&self) -> CACHEDISR {
        let bits = ((self.bits >> 18) & 0x01) != 0;
        CACHEDISR { bits }
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 1:4 - NVM Read Wait States"]
    #[inline]
    pub fn rws(&mut self) -> _RWSW {
        _RWSW { w: self }
    }
    #[doc = "Bit 7 - Manual Write"]
    #[inline]
    pub fn manw(&mut self) -> _MANWW {
        _MANWW { w: self }
    }
    #[doc = "Bits 8:9 - Power Reduction Mode during Sleep"]
    #[inline]
    pub fn sleepprm(&mut self) -> _SLEEPPRMW {
        _SLEEPPRMW { w: self }
    }
    #[doc = "Bits 16:17 - NVMCTRL Read Mode"]
    #[inline]
    pub fn readmode(&mut self) -> _READMODEW {
        _READMODEW { w: self }
    }
    #[doc = "Bit 18 - Cache Disable"]
    #[inline]
    pub fn cachedis(&mut self) -> _CACHEDISW {
        _CACHEDISW { w: self }
    }
}
