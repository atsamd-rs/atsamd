#[doc = "Register `NISTER` reader"]
pub struct R(crate::R<NISTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NISTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NISTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NISTER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NISTER` writer"]
pub struct W(crate::W<NISTER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NISTER_SPEC>;
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
impl From<crate::W<NISTER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NISTER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMDC` reader - Command Complete Status Enable"]
pub type CMDC_R = crate::BitReader<CMDCSELECT_A>;
#[doc = "Command Complete Status Enable\n\nValue on reset: 0"]
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
#[doc = "Field `CMDC` writer - Command Complete Status Enable"]
pub type CMDC_W<'a, const O: u8> = crate::BitWriter<'a, u16, NISTER_SPEC, CMDCSELECT_A, O>;
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
#[doc = "Field `TRFC` reader - Transfer Complete Status Enable"]
pub type TRFC_R = crate::BitReader<TRFCSELECT_A>;
#[doc = "Transfer Complete Status Enable\n\nValue on reset: 0"]
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
#[doc = "Field `TRFC` writer - Transfer Complete Status Enable"]
pub type TRFC_W<'a, const O: u8> = crate::BitWriter<'a, u16, NISTER_SPEC, TRFCSELECT_A, O>;
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
#[doc = "Field `BLKGE` reader - Block Gap Event Status Enable"]
pub type BLKGE_R = crate::BitReader<BLKGESELECT_A>;
#[doc = "Block Gap Event Status Enable\n\nValue on reset: 0"]
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
#[doc = "Field `BLKGE` writer - Block Gap Event Status Enable"]
pub type BLKGE_W<'a, const O: u8> = crate::BitWriter<'a, u16, NISTER_SPEC, BLKGESELECT_A, O>;
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
#[doc = "Field `DMAINT` reader - DMA Interrupt Status Enable"]
pub type DMAINT_R = crate::BitReader<DMAINTSELECT_A>;
#[doc = "DMA Interrupt Status Enable\n\nValue on reset: 0"]
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
#[doc = "Field `DMAINT` writer - DMA Interrupt Status Enable"]
pub type DMAINT_W<'a, const O: u8> = crate::BitWriter<'a, u16, NISTER_SPEC, DMAINTSELECT_A, O>;
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
#[doc = "Field `BWRRDY` reader - Buffer Write Ready Status Enable"]
pub type BWRRDY_R = crate::BitReader<BWRRDYSELECT_A>;
#[doc = "Buffer Write Ready Status Enable\n\nValue on reset: 0"]
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
#[doc = "Field `BWRRDY` writer - Buffer Write Ready Status Enable"]
pub type BWRRDY_W<'a, const O: u8> = crate::BitWriter<'a, u16, NISTER_SPEC, BWRRDYSELECT_A, O>;
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
#[doc = "Field `BRDRDY` reader - Buffer Read Ready Status Enable"]
pub type BRDRDY_R = crate::BitReader<BRDRDYSELECT_A>;
#[doc = "Buffer Read Ready Status Enable\n\nValue on reset: 0"]
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
#[doc = "Field `BRDRDY` writer - Buffer Read Ready Status Enable"]
pub type BRDRDY_W<'a, const O: u8> = crate::BitWriter<'a, u16, NISTER_SPEC, BRDRDYSELECT_A, O>;
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
#[doc = "Field `CINS` reader - Card Insertion Status Enable"]
pub type CINS_R = crate::BitReader<CINSSELECT_A>;
#[doc = "Card Insertion Status Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CINSSELECT_A {
    #[doc = "0: Masked"]
    MASKED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<CINSSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CINSSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CINS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CINSSELECT_A {
        match self.bits {
            false => CINSSELECT_A::MASKED,
            true => CINSSELECT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == CINSSELECT_A::MASKED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CINSSELECT_A::ENABLED
    }
}
#[doc = "Field `CINS` writer - Card Insertion Status Enable"]
pub type CINS_W<'a, const O: u8> = crate::BitWriter<'a, u16, NISTER_SPEC, CINSSELECT_A, O>;
impl<'a, const O: u8> CINS_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(CINSSELECT_A::MASKED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CINSSELECT_A::ENABLED)
    }
}
#[doc = "Field `CREM` reader - Card Removal Status Enable"]
pub type CREM_R = crate::BitReader<CREMSELECT_A>;
#[doc = "Card Removal Status Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CREMSELECT_A {
    #[doc = "0: Masked"]
    MASKED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<CREMSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CREMSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CREM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CREMSELECT_A {
        match self.bits {
            false => CREMSELECT_A::MASKED,
            true => CREMSELECT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == CREMSELECT_A::MASKED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CREMSELECT_A::ENABLED
    }
}
#[doc = "Field `CREM` writer - Card Removal Status Enable"]
pub type CREM_W<'a, const O: u8> = crate::BitWriter<'a, u16, NISTER_SPEC, CREMSELECT_A, O>;
impl<'a, const O: u8> CREM_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(CREMSELECT_A::MASKED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CREMSELECT_A::ENABLED)
    }
}
#[doc = "Field `CINT` reader - Card Interrupt Status Enable"]
pub type CINT_R = crate::BitReader<CINTSELECT_A>;
#[doc = "Card Interrupt Status Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CINTSELECT_A {
    #[doc = "0: Masked"]
    MASKED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<CINTSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CINTSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CINT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CINTSELECT_A {
        match self.bits {
            false => CINTSELECT_A::MASKED,
            true => CINTSELECT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == CINTSELECT_A::MASKED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CINTSELECT_A::ENABLED
    }
}
#[doc = "Field `CINT` writer - Card Interrupt Status Enable"]
pub type CINT_W<'a, const O: u8> = crate::BitWriter<'a, u16, NISTER_SPEC, CINTSELECT_A, O>;
impl<'a, const O: u8> CINT_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(CINTSELECT_A::MASKED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CINTSELECT_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Command Complete Status Enable"]
    #[inline(always)]
    pub fn cmdc(&self) -> CMDC_R {
        CMDC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer Complete Status Enable"]
    #[inline(always)]
    pub fn trfc(&self) -> TRFC_R {
        TRFC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Block Gap Event Status Enable"]
    #[inline(always)]
    pub fn blkge(&self) -> BLKGE_R {
        BLKGE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Interrupt Status Enable"]
    #[inline(always)]
    pub fn dmaint(&self) -> DMAINT_R {
        DMAINT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Buffer Write Ready Status Enable"]
    #[inline(always)]
    pub fn bwrrdy(&self) -> BWRRDY_R {
        BWRRDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Buffer Read Ready Status Enable"]
    #[inline(always)]
    pub fn brdrdy(&self) -> BRDRDY_R {
        BRDRDY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Card Insertion Status Enable"]
    #[inline(always)]
    pub fn cins(&self) -> CINS_R {
        CINS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Card Removal Status Enable"]
    #[inline(always)]
    pub fn crem(&self) -> CREM_R {
        CREM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Card Interrupt Status Enable"]
    #[inline(always)]
    pub fn cint(&self) -> CINT_R {
        CINT_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Complete Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmdc(&mut self) -> CMDC_W<0> {
        CMDC_W::new(self)
    }
    #[doc = "Bit 1 - Transfer Complete Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trfc(&mut self) -> TRFC_W<1> {
        TRFC_W::new(self)
    }
    #[doc = "Bit 2 - Block Gap Event Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn blkge(&mut self) -> BLKGE_W<2> {
        BLKGE_W::new(self)
    }
    #[doc = "Bit 3 - DMA Interrupt Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaint(&mut self) -> DMAINT_W<3> {
        DMAINT_W::new(self)
    }
    #[doc = "Bit 4 - Buffer Write Ready Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bwrrdy(&mut self) -> BWRRDY_W<4> {
        BWRRDY_W::new(self)
    }
    #[doc = "Bit 5 - Buffer Read Ready Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn brdrdy(&mut self) -> BRDRDY_W<5> {
        BRDRDY_W::new(self)
    }
    #[doc = "Bit 6 - Card Insertion Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cins(&mut self) -> CINS_W<6> {
        CINS_W::new(self)
    }
    #[doc = "Bit 7 - Card Removal Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn crem(&mut self) -> CREM_W<7> {
        CREM_W::new(self)
    }
    #[doc = "Bit 8 - Card Interrupt Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cint(&mut self) -> CINT_W<8> {
        CINT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Normal Interrupt Status Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nister](index.html) module"]
pub struct NISTER_SPEC;
impl crate::RegisterSpec for NISTER_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [nister::R](R) reader structure"]
impl crate::Readable for NISTER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nister::W](W) writer structure"]
impl crate::Writable for NISTER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NISTER to value 0"]
impl crate::Resettable for NISTER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
