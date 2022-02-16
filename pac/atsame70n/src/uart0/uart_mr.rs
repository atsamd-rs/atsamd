#[doc = "Register `UART_MR` reader"]
pub struct R(crate::R<UART_MR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_MR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_MR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_MR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_MR` writer"]
pub struct W(crate::W<UART_MR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_MR_SPEC>;
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
impl From<crate::W<UART_MR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_MR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Receiver Digital Filter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FILTER_A {
    #[doc = "0: UART does not filter the receive line."]
    DISABLED = 0,
    #[doc = "1: UART filters the receive line using a three-sample filter (16x-bit clock) (2 over 3 majority)."]
    ENABLED = 1,
}
impl From<FILTER_A> for bool {
    #[inline(always)]
    fn from(variant: FILTER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FILTER` reader - Receiver Digital Filter"]
pub struct FILTER_R(crate::FieldReader<bool, FILTER_A>);
impl FILTER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FILTER_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FILTER_A {
        match self.bits {
            false => FILTER_A::DISABLED,
            true => FILTER_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == FILTER_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == FILTER_A::ENABLED
    }
}
impl core::ops::Deref for FILTER_R {
    type Target = crate::FieldReader<bool, FILTER_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FILTER` writer - Receiver Digital Filter"]
pub struct FILTER_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FILTER_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "UART does not filter the receive line."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FILTER_A::DISABLED)
    }
    #[doc = "UART filters the receive line using a three-sample filter (16x-bit clock) (2 over 3 majority)."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FILTER_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Parity Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAR_A {
    #[doc = "0: Even Parity"]
    EVEN = 0,
    #[doc = "1: Odd Parity"]
    ODD = 1,
    #[doc = "2: Space: parity forced to 0"]
    SPACE = 2,
    #[doc = "3: Mark: parity forced to 1"]
    MARK = 3,
    #[doc = "4: No parity"]
    NO = 4,
}
impl From<PAR_A> for u8 {
    #[inline(always)]
    fn from(variant: PAR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PAR` reader - Parity Type"]
pub struct PAR_R(crate::FieldReader<u8, PAR_A>);
impl PAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAR_A> {
        match self.bits {
            0 => Some(PAR_A::EVEN),
            1 => Some(PAR_A::ODD),
            2 => Some(PAR_A::SPACE),
            3 => Some(PAR_A::MARK),
            4 => Some(PAR_A::NO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `EVEN`"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        **self == PAR_A::EVEN
    }
    #[doc = "Checks if the value of the field is `ODD`"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        **self == PAR_A::ODD
    }
    #[doc = "Checks if the value of the field is `SPACE`"]
    #[inline(always)]
    pub fn is_space(&self) -> bool {
        **self == PAR_A::SPACE
    }
    #[doc = "Checks if the value of the field is `MARK`"]
    #[inline(always)]
    pub fn is_mark(&self) -> bool {
        **self == PAR_A::MARK
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == PAR_A::NO
    }
}
impl core::ops::Deref for PAR_R {
    type Target = crate::FieldReader<u8, PAR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAR` writer - Parity Type"]
pub struct PAR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Even Parity"]
    #[inline(always)]
    pub fn even(self) -> &'a mut W {
        self.variant(PAR_A::EVEN)
    }
    #[doc = "Odd Parity"]
    #[inline(always)]
    pub fn odd(self) -> &'a mut W {
        self.variant(PAR_A::ODD)
    }
    #[doc = "Space: parity forced to 0"]
    #[inline(always)]
    pub fn space(self) -> &'a mut W {
        self.variant(PAR_A::SPACE)
    }
    #[doc = "Mark: parity forced to 1"]
    #[inline(always)]
    pub fn mark(self) -> &'a mut W {
        self.variant(PAR_A::MARK)
    }
    #[doc = "No parity"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(PAR_A::NO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | ((value as u32 & 0x07) << 9);
        self.w
    }
}
#[doc = "Baud Rate Source Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRSRCCK_A {
    #[doc = "0: The baud rate is driven by the peripheral clock"]
    PERIPH_CLK = 0,
    #[doc = "1: The baud rate is driven by a PMC-programmable clock PCK (see section Power Management Controller (PMC))."]
    PMC_PCK = 1,
}
impl From<BRSRCCK_A> for bool {
    #[inline(always)]
    fn from(variant: BRSRCCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRSRCCK` reader - Baud Rate Source Clock"]
pub struct BRSRCCK_R(crate::FieldReader<bool, BRSRCCK_A>);
impl BRSRCCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BRSRCCK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRSRCCK_A {
        match self.bits {
            false => BRSRCCK_A::PERIPH_CLK,
            true => BRSRCCK_A::PMC_PCK,
        }
    }
    #[doc = "Checks if the value of the field is `PERIPH_CLK`"]
    #[inline(always)]
    pub fn is_periph_clk(&self) -> bool {
        **self == BRSRCCK_A::PERIPH_CLK
    }
    #[doc = "Checks if the value of the field is `PMC_PCK`"]
    #[inline(always)]
    pub fn is_pmc_pck(&self) -> bool {
        **self == BRSRCCK_A::PMC_PCK
    }
}
impl core::ops::Deref for BRSRCCK_R {
    type Target = crate::FieldReader<bool, BRSRCCK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRSRCCK` writer - Baud Rate Source Clock"]
pub struct BRSRCCK_W<'a> {
    w: &'a mut W,
}
impl<'a> BRSRCCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRSRCCK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The baud rate is driven by the peripheral clock"]
    #[inline(always)]
    pub fn periph_clk(self) -> &'a mut W {
        self.variant(BRSRCCK_A::PERIPH_CLK)
    }
    #[doc = "The baud rate is driven by a PMC-programmable clock PCK (see section Power Management Controller (PMC))."]
    #[inline(always)]
    pub fn pmc_pck(self) -> &'a mut W {
        self.variant(BRSRCCK_A::PMC_PCK)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Channel Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CHMODE_A {
    #[doc = "0: Normal mode"]
    NORMAL = 0,
    #[doc = "1: Automatic echo"]
    AUTOMATIC = 1,
    #[doc = "2: Local loopback"]
    LOCAL_LOOPBACK = 2,
    #[doc = "3: Remote loopback"]
    REMOTE_LOOPBACK = 3,
}
impl From<CHMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CHMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CHMODE` reader - Channel Mode"]
pub struct CHMODE_R(crate::FieldReader<u8, CHMODE_A>);
impl CHMODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CHMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHMODE_A {
        match self.bits {
            0 => CHMODE_A::NORMAL,
            1 => CHMODE_A::AUTOMATIC,
            2 => CHMODE_A::LOCAL_LOOPBACK,
            3 => CHMODE_A::REMOTE_LOOPBACK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == CHMODE_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `AUTOMATIC`"]
    #[inline(always)]
    pub fn is_automatic(&self) -> bool {
        **self == CHMODE_A::AUTOMATIC
    }
    #[doc = "Checks if the value of the field is `LOCAL_LOOPBACK`"]
    #[inline(always)]
    pub fn is_local_loopback(&self) -> bool {
        **self == CHMODE_A::LOCAL_LOOPBACK
    }
    #[doc = "Checks if the value of the field is `REMOTE_LOOPBACK`"]
    #[inline(always)]
    pub fn is_remote_loopback(&self) -> bool {
        **self == CHMODE_A::REMOTE_LOOPBACK
    }
}
impl core::ops::Deref for CHMODE_R {
    type Target = crate::FieldReader<u8, CHMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHMODE` writer - Channel Mode"]
pub struct CHMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CHMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHMODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(CHMODE_A::NORMAL)
    }
    #[doc = "Automatic echo"]
    #[inline(always)]
    pub fn automatic(self) -> &'a mut W {
        self.variant(CHMODE_A::AUTOMATIC)
    }
    #[doc = "Local loopback"]
    #[inline(always)]
    pub fn local_loopback(self) -> &'a mut W {
        self.variant(CHMODE_A::LOCAL_LOOPBACK)
    }
    #[doc = "Remote loopback"]
    #[inline(always)]
    pub fn remote_loopback(self) -> &'a mut W {
        self.variant(CHMODE_A::REMOTE_LOOPBACK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bit 4 - Receiver Digital Filter"]
    #[inline(always)]
    pub fn filter(&self) -> FILTER_R {
        FILTER_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 9:11 - Parity Type"]
    #[inline(always)]
    pub fn par(&self) -> PAR_R {
        PAR_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bit 12 - Baud Rate Source Clock"]
    #[inline(always)]
    pub fn brsrcck(&self) -> BRSRCCK_R {
        BRSRCCK_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - Channel Mode"]
    #[inline(always)]
    pub fn chmode(&self) -> CHMODE_R {
        CHMODE_R::new(((self.bits >> 14) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 4 - Receiver Digital Filter"]
    #[inline(always)]
    pub fn filter(&mut self) -> FILTER_W {
        FILTER_W { w: self }
    }
    #[doc = "Bits 9:11 - Parity Type"]
    #[inline(always)]
    pub fn par(&mut self) -> PAR_W {
        PAR_W { w: self }
    }
    #[doc = "Bit 12 - Baud Rate Source Clock"]
    #[inline(always)]
    pub fn brsrcck(&mut self) -> BRSRCCK_W {
        BRSRCCK_W { w: self }
    }
    #[doc = "Bits 14:15 - Channel Mode"]
    #[inline(always)]
    pub fn chmode(&mut self) -> CHMODE_W {
        CHMODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_mr](index.html) module"]
pub struct UART_MR_SPEC;
impl crate::RegisterSpec for UART_MR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_mr::R](R) reader structure"]
impl crate::Readable for UART_MR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_mr::W](W) writer structure"]
impl crate::Writable for UART_MR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_MR to value 0"]
impl crate::Resettable for UART_MR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
