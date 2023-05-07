#[doc = "Register `NISIER_EMMC_MODE` reader"]
pub struct R(crate::R<NISIER_EMMC_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NISIER_EMMC_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NISIER_EMMC_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NISIER_EMMC_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NISIER_EMMC_MODE` writer"]
pub struct W(crate::W<NISIER_EMMC_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NISIER_EMMC_MODE_SPEC>;
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
impl From<crate::W<NISIER_EMMC_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NISIER_EMMC_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMDC` reader - Command Complete Signal Enable"]
pub type CMDC_R = crate::BitReader<CMDCSELECT_A>;
#[doc = "Command Complete Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMDCSELECT_A {
    #[doc = "0: Masked"]
    MASKED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<CMDCSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CMDCSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CMDC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMDCSELECT_A {
        match self.bits {
            false => CMDCSELECT_A::MASKED,
            true => CMDCSELECT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == CMDCSELECT_A::MASKED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CMDCSELECT_A::ENABLED
    }
}
#[doc = "Field `CMDC` writer - Command Complete Signal Enable"]
pub type CMDC_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, NISIER_EMMC_MODE_SPEC, CMDCSELECT_A, O>;
impl<'a, const O: u8> CMDC_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(CMDCSELECT_A::MASKED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CMDCSELECT_A::ENABLED)
    }
}
#[doc = "Field `TRFC` reader - Transfer Complete Signal Enable"]
pub type TRFC_R = crate::BitReader<TRFCSELECT_A>;
#[doc = "Transfer Complete Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRFCSELECT_A {
    #[doc = "0: Masked"]
    MASKED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<TRFCSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: TRFCSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl TRFC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRFCSELECT_A {
        match self.bits {
            false => TRFCSELECT_A::MASKED,
            true => TRFCSELECT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == TRFCSELECT_A::MASKED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TRFCSELECT_A::ENABLED
    }
}
#[doc = "Field `TRFC` writer - Transfer Complete Signal Enable"]
pub type TRFC_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, NISIER_EMMC_MODE_SPEC, TRFCSELECT_A, O>;
impl<'a, const O: u8> TRFC_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(TRFCSELECT_A::MASKED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRFCSELECT_A::ENABLED)
    }
}
#[doc = "Field `BLKGE` reader - Block Gap Event Signal Enable"]
pub type BLKGE_R = crate::BitReader<BLKGESELECT_A>;
#[doc = "Block Gap Event Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLKGESELECT_A {
    #[doc = "0: Masked"]
    MASKED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<BLKGESELECT_A> for bool {
    #[inline(always)]
    fn from(variant: BLKGESELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl BLKGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLKGESELECT_A {
        match self.bits {
            false => BLKGESELECT_A::MASKED,
            true => BLKGESELECT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == BLKGESELECT_A::MASKED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BLKGESELECT_A::ENABLED
    }
}
#[doc = "Field `BLKGE` writer - Block Gap Event Signal Enable"]
pub type BLKGE_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, NISIER_EMMC_MODE_SPEC, BLKGESELECT_A, O>;
impl<'a, const O: u8> BLKGE_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(BLKGESELECT_A::MASKED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BLKGESELECT_A::ENABLED)
    }
}
#[doc = "Field `DMAINT` reader - DMA Interrupt Signal Enable"]
pub type DMAINT_R = crate::BitReader<DMAINTSELECT_A>;
#[doc = "DMA Interrupt Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAINTSELECT_A {
    #[doc = "0: Masked"]
    MASKED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<DMAINTSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: DMAINTSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAINT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAINTSELECT_A {
        match self.bits {
            false => DMAINTSELECT_A::MASKED,
            true => DMAINTSELECT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == DMAINTSELECT_A::MASKED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAINTSELECT_A::ENABLED
    }
}
#[doc = "Field `DMAINT` writer - DMA Interrupt Signal Enable"]
pub type DMAINT_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, NISIER_EMMC_MODE_SPEC, DMAINTSELECT_A, O>;
impl<'a, const O: u8> DMAINT_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(DMAINTSELECT_A::MASKED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAINTSELECT_A::ENABLED)
    }
}
#[doc = "Field `BWRRDY` reader - Buffer Write Ready Signal Enable"]
pub type BWRRDY_R = crate::BitReader<BWRRDYSELECT_A>;
#[doc = "Buffer Write Ready Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BWRRDYSELECT_A {
    #[doc = "0: Masked"]
    MASKED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<BWRRDYSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: BWRRDYSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl BWRRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BWRRDYSELECT_A {
        match self.bits {
            false => BWRRDYSELECT_A::MASKED,
            true => BWRRDYSELECT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == BWRRDYSELECT_A::MASKED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BWRRDYSELECT_A::ENABLED
    }
}
#[doc = "Field `BWRRDY` writer - Buffer Write Ready Signal Enable"]
pub type BWRRDY_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, NISIER_EMMC_MODE_SPEC, BWRRDYSELECT_A, O>;
impl<'a, const O: u8> BWRRDY_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(BWRRDYSELECT_A::MASKED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BWRRDYSELECT_A::ENABLED)
    }
}
#[doc = "Field `BRDRDY` reader - Buffer Read Ready Signal Enable"]
pub type BRDRDY_R = crate::BitReader<BRDRDYSELECT_A>;
#[doc = "Buffer Read Ready Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRDRDYSELECT_A {
    #[doc = "0: Masked"]
    MASKED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<BRDRDYSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: BRDRDYSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl BRDRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRDRDYSELECT_A {
        match self.bits {
            false => BRDRDYSELECT_A::MASKED,
            true => BRDRDYSELECT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == BRDRDYSELECT_A::MASKED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BRDRDYSELECT_A::ENABLED
    }
}
#[doc = "Field `BRDRDY` writer - Buffer Read Ready Signal Enable"]
pub type BRDRDY_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, NISIER_EMMC_MODE_SPEC, BRDRDYSELECT_A, O>;
impl<'a, const O: u8> BRDRDY_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(BRDRDYSELECT_A::MASKED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BRDRDYSELECT_A::ENABLED)
    }
}
#[doc = "Field `BOOTAR` reader - Boot Acknowledge Received Signal Enable"]
pub type BOOTAR_R = crate::BitReader<bool>;
#[doc = "Field `BOOTAR` writer - Boot Acknowledge Received Signal Enable"]
pub type BOOTAR_W<'a, const O: u8> = crate::BitWriter<'a, u16, NISIER_EMMC_MODE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Command Complete Signal Enable"]
    #[inline(always)]
    pub fn cmdc(&self) -> CMDC_R {
        CMDC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer Complete Signal Enable"]
    #[inline(always)]
    pub fn trfc(&self) -> TRFC_R {
        TRFC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Block Gap Event Signal Enable"]
    #[inline(always)]
    pub fn blkge(&self) -> BLKGE_R {
        BLKGE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Interrupt Signal Enable"]
    #[inline(always)]
    pub fn dmaint(&self) -> DMAINT_R {
        DMAINT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Buffer Write Ready Signal Enable"]
    #[inline(always)]
    pub fn bwrrdy(&self) -> BWRRDY_R {
        BWRRDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Buffer Read Ready Signal Enable"]
    #[inline(always)]
    pub fn brdrdy(&self) -> BRDRDY_R {
        BRDRDY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 14 - Boot Acknowledge Received Signal Enable"]
    #[inline(always)]
    pub fn bootar(&self) -> BOOTAR_R {
        BOOTAR_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Complete Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmdc(&mut self) -> CMDC_W<0> {
        CMDC_W::new(self)
    }
    #[doc = "Bit 1 - Transfer Complete Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trfc(&mut self) -> TRFC_W<1> {
        TRFC_W::new(self)
    }
    #[doc = "Bit 2 - Block Gap Event Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn blkge(&mut self) -> BLKGE_W<2> {
        BLKGE_W::new(self)
    }
    #[doc = "Bit 3 - DMA Interrupt Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaint(&mut self) -> DMAINT_W<3> {
        DMAINT_W::new(self)
    }
    #[doc = "Bit 4 - Buffer Write Ready Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bwrrdy(&mut self) -> BWRRDY_W<4> {
        BWRRDY_W::new(self)
    }
    #[doc = "Bit 5 - Buffer Read Ready Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn brdrdy(&mut self) -> BRDRDY_W<5> {
        BRDRDY_W::new(self)
    }
    #[doc = "Bit 14 - Boot Acknowledge Received Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bootar(&mut self) -> BOOTAR_W<14> {
        BOOTAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Normal Interrupt Signal Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nisier_emmc_mode](index.html) module"]
pub struct NISIER_EMMC_MODE_SPEC;
impl crate::RegisterSpec for NISIER_EMMC_MODE_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [nisier_emmc_mode::R](R) reader structure"]
impl crate::Readable for NISIER_EMMC_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nisier_emmc_mode::W](W) writer structure"]
impl crate::Writable for NISIER_EMMC_MODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NISIER_EMMC_MODE to value 0"]
impl crate::Resettable for NISIER_EMMC_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
