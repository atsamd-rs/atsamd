#[doc = "Register `CLKCTRL%s` reader"]
pub struct R(crate::R<CLKCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKCTRL%s` writer"]
pub struct W(crate::W<CLKCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CLKCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLOTSIZE` reader - Slot Size"]
pub type SLOTSIZE_R = crate::FieldReader<u8, SLOTSIZESELECT_A>;
#[doc = "Slot Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SLOTSIZESELECT_A {
    #[doc = "0: 8-bit Slot for Clock Unit n"]
    _8 = 0,
    #[doc = "1: 16-bit Slot for Clock Unit n"]
    _16 = 1,
    #[doc = "2: 24-bit Slot for Clock Unit n"]
    _24 = 2,
    #[doc = "3: 32-bit Slot for Clock Unit n"]
    _32 = 3,
}
impl From<SLOTSIZESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: SLOTSIZESELECT_A) -> Self {
        variant as _
    }
}
impl SLOTSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLOTSIZESELECT_A {
        match self.bits {
            0 => SLOTSIZESELECT_A::_8,
            1 => SLOTSIZESELECT_A::_16,
            2 => SLOTSIZESELECT_A::_24,
            3 => SLOTSIZESELECT_A::_32,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == SLOTSIZESELECT_A::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == SLOTSIZESELECT_A::_16
    }
    #[doc = "Checks if the value of the field is `_24`"]
    #[inline(always)]
    pub fn is_24(&self) -> bool {
        *self == SLOTSIZESELECT_A::_24
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == SLOTSIZESELECT_A::_32
    }
}
#[doc = "Field `SLOTSIZE` writer - Slot Size"]
pub type SLOTSIZE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CLKCTRL_SPEC, u8, SLOTSIZESELECT_A, 2, O>;
impl<'a, const O: u8> SLOTSIZE_W<'a, O> {
    #[doc = "8-bit Slot for Clock Unit n"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(SLOTSIZESELECT_A::_8)
    }
    #[doc = "16-bit Slot for Clock Unit n"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(SLOTSIZESELECT_A::_16)
    }
    #[doc = "24-bit Slot for Clock Unit n"]
    #[inline(always)]
    pub fn _24(self) -> &'a mut W {
        self.variant(SLOTSIZESELECT_A::_24)
    }
    #[doc = "32-bit Slot for Clock Unit n"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(SLOTSIZESELECT_A::_32)
    }
}
#[doc = "Field `NBSLOTS` reader - Number of Slots in Frame"]
pub type NBSLOTS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NBSLOTS` writer - Number of Slots in Frame"]
pub type NBSLOTS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLKCTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `FSWIDTH` reader - Frame Sync Width"]
pub type FSWIDTH_R = crate::FieldReader<u8, FSWIDTHSELECT_A>;
#[doc = "Frame Sync Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSWIDTHSELECT_A {
    #[doc = "0: Frame Sync Pulse is 1 Slot wide (default for I2S protocol)"]
    SLOT = 0,
    #[doc = "1: Frame Sync Pulse is half a Frame wide"]
    HALF = 1,
    #[doc = "2: Frame Sync Pulse is 1 Bit wide"]
    BIT = 2,
    #[doc = "3: Clock Unit n operates in Burst mode, with a 1-bit wide Frame Sync pulse per Data sample, only when Data transfer is requested"]
    BURST = 3,
}
impl From<FSWIDTHSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: FSWIDTHSELECT_A) -> Self {
        variant as _
    }
}
impl FSWIDTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSWIDTHSELECT_A {
        match self.bits {
            0 => FSWIDTHSELECT_A::SLOT,
            1 => FSWIDTHSELECT_A::HALF,
            2 => FSWIDTHSELECT_A::BIT,
            3 => FSWIDTHSELECT_A::BURST,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SLOT`"]
    #[inline(always)]
    pub fn is_slot(&self) -> bool {
        *self == FSWIDTHSELECT_A::SLOT
    }
    #[doc = "Checks if the value of the field is `HALF`"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == FSWIDTHSELECT_A::HALF
    }
    #[doc = "Checks if the value of the field is `BIT`"]
    #[inline(always)]
    pub fn is_bit_(&self) -> bool {
        *self == FSWIDTHSELECT_A::BIT
    }
    #[doc = "Checks if the value of the field is `BURST`"]
    #[inline(always)]
    pub fn is_burst(&self) -> bool {
        *self == FSWIDTHSELECT_A::BURST
    }
}
#[doc = "Field `FSWIDTH` writer - Frame Sync Width"]
pub type FSWIDTH_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CLKCTRL_SPEC, u8, FSWIDTHSELECT_A, 2, O>;
impl<'a, const O: u8> FSWIDTH_W<'a, O> {
    #[doc = "Frame Sync Pulse is 1 Slot wide (default for I2S protocol)"]
    #[inline(always)]
    pub fn slot(self) -> &'a mut W {
        self.variant(FSWIDTHSELECT_A::SLOT)
    }
    #[doc = "Frame Sync Pulse is half a Frame wide"]
    #[inline(always)]
    pub fn half(self) -> &'a mut W {
        self.variant(FSWIDTHSELECT_A::HALF)
    }
    #[doc = "Frame Sync Pulse is 1 Bit wide"]
    #[inline(always)]
    pub fn bit_(self) -> &'a mut W {
        self.variant(FSWIDTHSELECT_A::BIT)
    }
    #[doc = "Clock Unit n operates in Burst mode, with a 1-bit wide Frame Sync pulse per Data sample, only when Data transfer is requested"]
    #[inline(always)]
    pub fn burst(self) -> &'a mut W {
        self.variant(FSWIDTHSELECT_A::BURST)
    }
}
#[doc = "Field `BITDELAY` reader - Data Delay from Frame Sync"]
pub type BITDELAY_R = crate::BitReader<BITDELAYSELECT_A>;
#[doc = "Data Delay from Frame Sync\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BITDELAYSELECT_A {
    #[doc = "0: Left Justified (0 Bit Delay)"]
    LJ = 0,
    #[doc = "1: I2S (1 Bit Delay)"]
    I2S = 1,
}
impl From<BITDELAYSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: BITDELAYSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl BITDELAY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BITDELAYSELECT_A {
        match self.bits {
            false => BITDELAYSELECT_A::LJ,
            true => BITDELAYSELECT_A::I2S,
        }
    }
    #[doc = "Checks if the value of the field is `LJ`"]
    #[inline(always)]
    pub fn is_lj(&self) -> bool {
        *self == BITDELAYSELECT_A::LJ
    }
    #[doc = "Checks if the value of the field is `I2S`"]
    #[inline(always)]
    pub fn is_i2s(&self) -> bool {
        *self == BITDELAYSELECT_A::I2S
    }
}
#[doc = "Field `BITDELAY` writer - Data Delay from Frame Sync"]
pub type BITDELAY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKCTRL_SPEC, BITDELAYSELECT_A, O>;
impl<'a, const O: u8> BITDELAY_W<'a, O> {
    #[doc = "Left Justified (0 Bit Delay)"]
    #[inline(always)]
    pub fn lj(self) -> &'a mut W {
        self.variant(BITDELAYSELECT_A::LJ)
    }
    #[doc = "I2S (1 Bit Delay)"]
    #[inline(always)]
    pub fn i2s(self) -> &'a mut W {
        self.variant(BITDELAYSELECT_A::I2S)
    }
}
#[doc = "Field `FSSEL` reader - Frame Sync Select"]
pub type FSSEL_R = crate::BitReader<FSSELSELECT_A>;
#[doc = "Frame Sync Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FSSELSELECT_A {
    #[doc = "0: Divided Serial Clock n is used as Frame Sync n source"]
    SCKDIV = 0,
    #[doc = "1: FSn input pin is used as Frame Sync n source"]
    FSPIN = 1,
}
impl From<FSSELSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: FSSELSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl FSSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSSELSELECT_A {
        match self.bits {
            false => FSSELSELECT_A::SCKDIV,
            true => FSSELSELECT_A::FSPIN,
        }
    }
    #[doc = "Checks if the value of the field is `SCKDIV`"]
    #[inline(always)]
    pub fn is_sckdiv(&self) -> bool {
        *self == FSSELSELECT_A::SCKDIV
    }
    #[doc = "Checks if the value of the field is `FSPIN`"]
    #[inline(always)]
    pub fn is_fspin(&self) -> bool {
        *self == FSSELSELECT_A::FSPIN
    }
}
#[doc = "Field `FSSEL` writer - Frame Sync Select"]
pub type FSSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKCTRL_SPEC, FSSELSELECT_A, O>;
impl<'a, const O: u8> FSSEL_W<'a, O> {
    #[doc = "Divided Serial Clock n is used as Frame Sync n source"]
    #[inline(always)]
    pub fn sckdiv(self) -> &'a mut W {
        self.variant(FSSELSELECT_A::SCKDIV)
    }
    #[doc = "FSn input pin is used as Frame Sync n source"]
    #[inline(always)]
    pub fn fspin(self) -> &'a mut W {
        self.variant(FSSELSELECT_A::FSPIN)
    }
}
#[doc = "Field `FSINV` reader - Frame Sync Invert"]
pub type FSINV_R = crate::BitReader<bool>;
#[doc = "Field `FSINV` writer - Frame Sync Invert"]
pub type FSINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKCTRL_SPEC, bool, O>;
#[doc = "Field `SCKSEL` reader - Serial Clock Select"]
pub type SCKSEL_R = crate::BitReader<SCKSELSELECT_A>;
#[doc = "Serial Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCKSELSELECT_A {
    #[doc = "0: Divided Master Clock n is used as Serial Clock n source"]
    MCKDIV = 0,
    #[doc = "1: SCKn input pin is used as Serial Clock n source"]
    SCKPIN = 1,
}
impl From<SCKSELSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: SCKSELSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl SCKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCKSELSELECT_A {
        match self.bits {
            false => SCKSELSELECT_A::MCKDIV,
            true => SCKSELSELECT_A::SCKPIN,
        }
    }
    #[doc = "Checks if the value of the field is `MCKDIV`"]
    #[inline(always)]
    pub fn is_mckdiv(&self) -> bool {
        *self == SCKSELSELECT_A::MCKDIV
    }
    #[doc = "Checks if the value of the field is `SCKPIN`"]
    #[inline(always)]
    pub fn is_sckpin(&self) -> bool {
        *self == SCKSELSELECT_A::SCKPIN
    }
}
#[doc = "Field `SCKSEL` writer - Serial Clock Select"]
pub type SCKSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKCTRL_SPEC, SCKSELSELECT_A, O>;
impl<'a, const O: u8> SCKSEL_W<'a, O> {
    #[doc = "Divided Master Clock n is used as Serial Clock n source"]
    #[inline(always)]
    pub fn mckdiv(self) -> &'a mut W {
        self.variant(SCKSELSELECT_A::MCKDIV)
    }
    #[doc = "SCKn input pin is used as Serial Clock n source"]
    #[inline(always)]
    pub fn sckpin(self) -> &'a mut W {
        self.variant(SCKSELSELECT_A::SCKPIN)
    }
}
#[doc = "Field `MCKSEL` reader - Master Clock Select"]
pub type MCKSEL_R = crate::BitReader<MCKSELSELECT_A>;
#[doc = "Master Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCKSELSELECT_A {
    #[doc = "0: GCLK_I2S_n is used as Master Clock n source"]
    GCLK = 0,
    #[doc = "1: MCKn input pin is used as Master Clock n source"]
    MCKPIN = 1,
}
impl From<MCKSELSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: MCKSELSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl MCKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCKSELSELECT_A {
        match self.bits {
            false => MCKSELSELECT_A::GCLK,
            true => MCKSELSELECT_A::MCKPIN,
        }
    }
    #[doc = "Checks if the value of the field is `GCLK`"]
    #[inline(always)]
    pub fn is_gclk(&self) -> bool {
        *self == MCKSELSELECT_A::GCLK
    }
    #[doc = "Checks if the value of the field is `MCKPIN`"]
    #[inline(always)]
    pub fn is_mckpin(&self) -> bool {
        *self == MCKSELSELECT_A::MCKPIN
    }
}
#[doc = "Field `MCKSEL` writer - Master Clock Select"]
pub type MCKSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKCTRL_SPEC, MCKSELSELECT_A, O>;
impl<'a, const O: u8> MCKSEL_W<'a, O> {
    #[doc = "GCLK_I2S_n is used as Master Clock n source"]
    #[inline(always)]
    pub fn gclk(self) -> &'a mut W {
        self.variant(MCKSELSELECT_A::GCLK)
    }
    #[doc = "MCKn input pin is used as Master Clock n source"]
    #[inline(always)]
    pub fn mckpin(self) -> &'a mut W {
        self.variant(MCKSELSELECT_A::MCKPIN)
    }
}
#[doc = "Field `MCKEN` reader - Master Clock Enable"]
pub type MCKEN_R = crate::BitReader<bool>;
#[doc = "Field `MCKEN` writer - Master Clock Enable"]
pub type MCKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKCTRL_SPEC, bool, O>;
#[doc = "Field `MCKDIV` reader - Master Clock Division Factor"]
pub type MCKDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MCKDIV` writer - Master Clock Division Factor"]
pub type MCKDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLKCTRL_SPEC, u8, u8, 5, O>;
#[doc = "Field `MCKOUTDIV` reader - Master Clock Output Division Factor"]
pub type MCKOUTDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MCKOUTDIV` writer - Master Clock Output Division Factor"]
pub type MCKOUTDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLKCTRL_SPEC, u8, u8, 5, O>;
#[doc = "Field `FSOUTINV` reader - Frame Sync Output Invert"]
pub type FSOUTINV_R = crate::BitReader<bool>;
#[doc = "Field `FSOUTINV` writer - Frame Sync Output Invert"]
pub type FSOUTINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKCTRL_SPEC, bool, O>;
#[doc = "Field `SCKOUTINV` reader - Serial Clock Output Invert"]
pub type SCKOUTINV_R = crate::BitReader<bool>;
#[doc = "Field `SCKOUTINV` writer - Serial Clock Output Invert"]
pub type SCKOUTINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKCTRL_SPEC, bool, O>;
#[doc = "Field `MCKOUTINV` reader - Master Clock Output Invert"]
pub type MCKOUTINV_R = crate::BitReader<bool>;
#[doc = "Field `MCKOUTINV` writer - Master Clock Output Invert"]
pub type MCKOUTINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Slot Size"]
    #[inline(always)]
    pub fn slotsize(&self) -> SLOTSIZE_R {
        SLOTSIZE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4 - Number of Slots in Frame"]
    #[inline(always)]
    pub fn nbslots(&self) -> NBSLOTS_R {
        NBSLOTS_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:6 - Frame Sync Width"]
    #[inline(always)]
    pub fn fswidth(&self) -> FSWIDTH_R {
        FSWIDTH_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Data Delay from Frame Sync"]
    #[inline(always)]
    pub fn bitdelay(&self) -> BITDELAY_R {
        BITDELAY_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Frame Sync Select"]
    #[inline(always)]
    pub fn fssel(&self) -> FSSEL_R {
        FSSEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Frame Sync Invert"]
    #[inline(always)]
    pub fn fsinv(&self) -> FSINV_R {
        FSINV_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Serial Clock Select"]
    #[inline(always)]
    pub fn scksel(&self) -> SCKSEL_R {
        SCKSEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Master Clock Select"]
    #[inline(always)]
    pub fn mcksel(&self) -> MCKSEL_R {
        MCKSEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Master Clock Enable"]
    #[inline(always)]
    pub fn mcken(&self) -> MCKEN_R {
        MCKEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:23 - Master Clock Division Factor"]
    #[inline(always)]
    pub fn mckdiv(&self) -> MCKDIV_R {
        MCKDIV_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Master Clock Output Division Factor"]
    #[inline(always)]
    pub fn mckoutdiv(&self) -> MCKOUTDIV_R {
        MCKOUTDIV_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 29 - Frame Sync Output Invert"]
    #[inline(always)]
    pub fn fsoutinv(&self) -> FSOUTINV_R {
        FSOUTINV_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Serial Clock Output Invert"]
    #[inline(always)]
    pub fn sckoutinv(&self) -> SCKOUTINV_R {
        SCKOUTINV_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Master Clock Output Invert"]
    #[inline(always)]
    pub fn mckoutinv(&self) -> MCKOUTINV_R {
        MCKOUTINV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Slot Size"]
    #[inline(always)]
    #[must_use]
    pub fn slotsize(&mut self) -> SLOTSIZE_W<0> {
        SLOTSIZE_W::new(self)
    }
    #[doc = "Bits 2:4 - Number of Slots in Frame"]
    #[inline(always)]
    #[must_use]
    pub fn nbslots(&mut self) -> NBSLOTS_W<2> {
        NBSLOTS_W::new(self)
    }
    #[doc = "Bits 5:6 - Frame Sync Width"]
    #[inline(always)]
    #[must_use]
    pub fn fswidth(&mut self) -> FSWIDTH_W<5> {
        FSWIDTH_W::new(self)
    }
    #[doc = "Bit 7 - Data Delay from Frame Sync"]
    #[inline(always)]
    #[must_use]
    pub fn bitdelay(&mut self) -> BITDELAY_W<7> {
        BITDELAY_W::new(self)
    }
    #[doc = "Bit 8 - Frame Sync Select"]
    #[inline(always)]
    #[must_use]
    pub fn fssel(&mut self) -> FSSEL_W<8> {
        FSSEL_W::new(self)
    }
    #[doc = "Bit 11 - Frame Sync Invert"]
    #[inline(always)]
    #[must_use]
    pub fn fsinv(&mut self) -> FSINV_W<11> {
        FSINV_W::new(self)
    }
    #[doc = "Bit 12 - Serial Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn scksel(&mut self) -> SCKSEL_W<12> {
        SCKSEL_W::new(self)
    }
    #[doc = "Bit 16 - Master Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn mcksel(&mut self) -> MCKSEL_W<16> {
        MCKSEL_W::new(self)
    }
    #[doc = "Bit 18 - Master Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mcken(&mut self) -> MCKEN_W<18> {
        MCKEN_W::new(self)
    }
    #[doc = "Bits 19:23 - Master Clock Division Factor"]
    #[inline(always)]
    #[must_use]
    pub fn mckdiv(&mut self) -> MCKDIV_W<19> {
        MCKDIV_W::new(self)
    }
    #[doc = "Bits 24:28 - Master Clock Output Division Factor"]
    #[inline(always)]
    #[must_use]
    pub fn mckoutdiv(&mut self) -> MCKOUTDIV_W<24> {
        MCKOUTDIV_W::new(self)
    }
    #[doc = "Bit 29 - Frame Sync Output Invert"]
    #[inline(always)]
    #[must_use]
    pub fn fsoutinv(&mut self) -> FSOUTINV_W<29> {
        FSOUTINV_W::new(self)
    }
    #[doc = "Bit 30 - Serial Clock Output Invert"]
    #[inline(always)]
    #[must_use]
    pub fn sckoutinv(&mut self) -> SCKOUTINV_W<30> {
        SCKOUTINV_W::new(self)
    }
    #[doc = "Bit 31 - Master Clock Output Invert"]
    #[inline(always)]
    #[must_use]
    pub fn mckoutinv(&mut self) -> MCKOUTINV_W<31> {
        MCKOUTINV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Unit n Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkctrl](index.html) module"]
pub struct CLKCTRL_SPEC;
impl crate::RegisterSpec for CLKCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkctrl::R](R) reader structure"]
impl crate::Readable for CLKCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkctrl::W](W) writer structure"]
impl crate::Writable for CLKCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKCTRL%s to value 0"]
impl crate::Resettable for CLKCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
