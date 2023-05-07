#[doc = "Register `CTRLB` reader"]
pub struct R(crate::R<CTRLB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLB` writer"]
pub struct W(crate::W<CTRLB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLB_SPEC>;
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
impl From<crate::W<CTRLB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESUME` reader - Send USB Resume"]
pub type RESUME_R = crate::BitReader<bool>;
#[doc = "Field `RESUME` writer - Send USB Resume"]
pub type RESUME_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRLB_SPEC, bool, O>;
#[doc = "Field `SPDCONF` reader - Speed Configuration for Host"]
pub type SPDCONF_R = crate::FieldReader<u8, SPDCONFSELECT_A>;
#[doc = "Speed Configuration for Host\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPDCONFSELECT_A {
    #[doc = "0: Normal mode: the host starts in full-speed mode and performs a high-speed reset to switch to the high speed mode if the downstream peripheral is high-speed capable."]
    NORMAL = 0,
    #[doc = "3: Full-speed: the host remains in full-speed mode whatever is the peripheral speed capability. Relevant in UTMI mode only."]
    FS = 3,
}
impl From<SPDCONFSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: SPDCONFSELECT_A) -> Self {
        variant as _
    }
}
impl SPDCONF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SPDCONFSELECT_A> {
        match self.bits {
            0 => Some(SPDCONFSELECT_A::NORMAL),
            3 => Some(SPDCONFSELECT_A::FS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == SPDCONFSELECT_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `FS`"]
    #[inline(always)]
    pub fn is_fs(&self) -> bool {
        *self == SPDCONFSELECT_A::FS
    }
}
#[doc = "Field `SPDCONF` writer - Speed Configuration for Host"]
pub type SPDCONF_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, CTRLB_SPEC, u8, SPDCONFSELECT_A, 2, O>;
impl<'a, const O: u8> SPDCONF_W<'a, O> {
    #[doc = "Normal mode: the host starts in full-speed mode and performs a high-speed reset to switch to the high speed mode if the downstream peripheral is high-speed capable."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(SPDCONFSELECT_A::NORMAL)
    }
    #[doc = "Full-speed: the host remains in full-speed mode whatever is the peripheral speed capability. Relevant in UTMI mode only."]
    #[inline(always)]
    pub fn fs(self) -> &'a mut W {
        self.variant(SPDCONFSELECT_A::FS)
    }
}
#[doc = "Field `AUTORESUME` reader - Auto Resume Enable"]
pub type AUTORESUME_R = crate::BitReader<bool>;
#[doc = "Field `AUTORESUME` writer - Auto Resume Enable"]
pub type AUTORESUME_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRLB_SPEC, bool, O>;
#[doc = "Field `TSTJ` reader - Test mode J"]
pub type TSTJ_R = crate::BitReader<bool>;
#[doc = "Field `TSTJ` writer - Test mode J"]
pub type TSTJ_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRLB_SPEC, bool, O>;
#[doc = "Field `TSTK` reader - Test mode K"]
pub type TSTK_R = crate::BitReader<bool>;
#[doc = "Field `TSTK` writer - Test mode K"]
pub type TSTK_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRLB_SPEC, bool, O>;
#[doc = "Field `SOFE` reader - Start of Frame Generation Enable"]
pub type SOFE_R = crate::BitReader<bool>;
#[doc = "Field `SOFE` writer - Start of Frame Generation Enable"]
pub type SOFE_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRLB_SPEC, bool, O>;
#[doc = "Field `BUSRESET` reader - Send USB Reset"]
pub type BUSRESET_R = crate::BitReader<bool>;
#[doc = "Field `BUSRESET` writer - Send USB Reset"]
pub type BUSRESET_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRLB_SPEC, bool, O>;
#[doc = "Field `VBUSOK` reader - VBUS is OK"]
pub type VBUSOK_R = crate::BitReader<bool>;
#[doc = "Field `VBUSOK` writer - VBUS is OK"]
pub type VBUSOK_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRLB_SPEC, bool, O>;
#[doc = "Field `L1RESUME` reader - Send L1 Resume"]
pub type L1RESUME_R = crate::BitReader<bool>;
#[doc = "Field `L1RESUME` writer - Send L1 Resume"]
pub type L1RESUME_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRLB_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - Send USB Resume"]
    #[inline(always)]
    pub fn resume(&self) -> RESUME_R {
        RESUME_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Speed Configuration for Host"]
    #[inline(always)]
    pub fn spdconf(&self) -> SPDCONF_R {
        SPDCONF_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Auto Resume Enable"]
    #[inline(always)]
    pub fn autoresume(&self) -> AUTORESUME_R {
        AUTORESUME_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Test mode J"]
    #[inline(always)]
    pub fn tstj(&self) -> TSTJ_R {
        TSTJ_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Test mode K"]
    #[inline(always)]
    pub fn tstk(&self) -> TSTK_R {
        TSTK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Start of Frame Generation Enable"]
    #[inline(always)]
    pub fn sofe(&self) -> SOFE_R {
        SOFE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Send USB Reset"]
    #[inline(always)]
    pub fn busreset(&self) -> BUSRESET_R {
        BUSRESET_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - VBUS is OK"]
    #[inline(always)]
    pub fn vbusok(&self) -> VBUSOK_R {
        VBUSOK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Send L1 Resume"]
    #[inline(always)]
    pub fn l1resume(&self) -> L1RESUME_R {
        L1RESUME_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Send USB Resume"]
    #[inline(always)]
    #[must_use]
    pub fn resume(&mut self) -> RESUME_W<1> {
        RESUME_W::new(self)
    }
    #[doc = "Bits 2:3 - Speed Configuration for Host"]
    #[inline(always)]
    #[must_use]
    pub fn spdconf(&mut self) -> SPDCONF_W<2> {
        SPDCONF_W::new(self)
    }
    #[doc = "Bit 4 - Auto Resume Enable"]
    #[inline(always)]
    #[must_use]
    pub fn autoresume(&mut self) -> AUTORESUME_W<4> {
        AUTORESUME_W::new(self)
    }
    #[doc = "Bit 5 - Test mode J"]
    #[inline(always)]
    #[must_use]
    pub fn tstj(&mut self) -> TSTJ_W<5> {
        TSTJ_W::new(self)
    }
    #[doc = "Bit 6 - Test mode K"]
    #[inline(always)]
    #[must_use]
    pub fn tstk(&mut self) -> TSTK_W<6> {
        TSTK_W::new(self)
    }
    #[doc = "Bit 8 - Start of Frame Generation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sofe(&mut self) -> SOFE_W<8> {
        SOFE_W::new(self)
    }
    #[doc = "Bit 9 - Send USB Reset"]
    #[inline(always)]
    #[must_use]
    pub fn busreset(&mut self) -> BUSRESET_W<9> {
        BUSRESET_W::new(self)
    }
    #[doc = "Bit 10 - VBUS is OK"]
    #[inline(always)]
    #[must_use]
    pub fn vbusok(&mut self) -> VBUSOK_W<10> {
        VBUSOK_W::new(self)
    }
    #[doc = "Bit 11 - Send L1 Resume"]
    #[inline(always)]
    #[must_use]
    pub fn l1resume(&mut self) -> L1RESUME_W<11> {
        L1RESUME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HOST Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlb](index.html) module"]
pub struct CTRLB_SPEC;
impl crate::RegisterSpec for CTRLB_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ctrlb::R](R) reader structure"]
impl crate::Readable for CTRLB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrlb::W](W) writer structure"]
impl crate::Writable for CTRLB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for CTRLB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
