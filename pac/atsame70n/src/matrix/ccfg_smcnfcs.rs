#[doc = "Register `CCFG_SMCNFCS` reader"]
pub struct R(crate::R<CCFG_SMCNFCS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCFG_SMCNFCS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCFG_SMCNFCS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCFG_SMCNFCS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCFG_SMCNFCS` writer"]
pub struct W(crate::W<CCFG_SMCNFCS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCFG_SMCNFCS_SPEC>;
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
impl From<crate::W<CCFG_SMCNFCS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCFG_SMCNFCS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SMC_NFCS0` reader - SMC NAND Flash Chip Select 0 Assignment"]
pub struct SMC_NFCS0_R(crate::FieldReader<bool, bool>);
impl SMC_NFCS0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SMC_NFCS0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMC_NFCS0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMC_NFCS0` writer - SMC NAND Flash Chip Select 0 Assignment"]
pub struct SMC_NFCS0_W<'a> {
    w: &'a mut W,
}
impl<'a> SMC_NFCS0_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `SMC_NFCS1` reader - SMC NAND Flash Chip Select 1 Assignment"]
pub struct SMC_NFCS1_R(crate::FieldReader<bool, bool>);
impl SMC_NFCS1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SMC_NFCS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMC_NFCS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMC_NFCS1` writer - SMC NAND Flash Chip Select 1 Assignment"]
pub struct SMC_NFCS1_W<'a> {
    w: &'a mut W,
}
impl<'a> SMC_NFCS1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `SMC_NFCS2` reader - SMC NAND Flash Chip Select 2 Assignment"]
pub struct SMC_NFCS2_R(crate::FieldReader<bool, bool>);
impl SMC_NFCS2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SMC_NFCS2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMC_NFCS2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMC_NFCS2` writer - SMC NAND Flash Chip Select 2 Assignment"]
pub struct SMC_NFCS2_W<'a> {
    w: &'a mut W,
}
impl<'a> SMC_NFCS2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `SMC_NFCS3` reader - SMC NAND Flash Chip Select 3 Assignment"]
pub struct SMC_NFCS3_R(crate::FieldReader<bool, bool>);
impl SMC_NFCS3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SMC_NFCS3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMC_NFCS3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMC_NFCS3` writer - SMC NAND Flash Chip Select 3 Assignment"]
pub struct SMC_NFCS3_W<'a> {
    w: &'a mut W,
}
impl<'a> SMC_NFCS3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `SDRAMEN` reader - SDRAM Enable"]
pub struct SDRAMEN_R(crate::FieldReader<bool, bool>);
impl SDRAMEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDRAMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDRAMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDRAMEN` writer - SDRAM Enable"]
pub struct SDRAMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SDRAMEN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - SMC NAND Flash Chip Select 0 Assignment"]
    #[inline(always)]
    pub fn smc_nfcs0(&self) -> SMC_NFCS0_R {
        SMC_NFCS0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SMC NAND Flash Chip Select 1 Assignment"]
    #[inline(always)]
    pub fn smc_nfcs1(&self) -> SMC_NFCS1_R {
        SMC_NFCS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SMC NAND Flash Chip Select 2 Assignment"]
    #[inline(always)]
    pub fn smc_nfcs2(&self) -> SMC_NFCS2_R {
        SMC_NFCS2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SMC NAND Flash Chip Select 3 Assignment"]
    #[inline(always)]
    pub fn smc_nfcs3(&self) -> SMC_NFCS3_R {
        SMC_NFCS3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SDRAM Enable"]
    #[inline(always)]
    pub fn sdramen(&self) -> SDRAMEN_R {
        SDRAMEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SMC NAND Flash Chip Select 0 Assignment"]
    #[inline(always)]
    pub fn smc_nfcs0(&mut self) -> SMC_NFCS0_W {
        SMC_NFCS0_W { w: self }
    }
    #[doc = "Bit 1 - SMC NAND Flash Chip Select 1 Assignment"]
    #[inline(always)]
    pub fn smc_nfcs1(&mut self) -> SMC_NFCS1_W {
        SMC_NFCS1_W { w: self }
    }
    #[doc = "Bit 2 - SMC NAND Flash Chip Select 2 Assignment"]
    #[inline(always)]
    pub fn smc_nfcs2(&mut self) -> SMC_NFCS2_W {
        SMC_NFCS2_W { w: self }
    }
    #[doc = "Bit 3 - SMC NAND Flash Chip Select 3 Assignment"]
    #[inline(always)]
    pub fn smc_nfcs3(&mut self) -> SMC_NFCS3_W {
        SMC_NFCS3_W { w: self }
    }
    #[doc = "Bit 4 - SDRAM Enable"]
    #[inline(always)]
    pub fn sdramen(&mut self) -> SDRAMEN_W {
        SDRAMEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SMC NAND Flash Chip Select Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccfg_smcnfcs](index.html) module"]
pub struct CCFG_SMCNFCS_SPEC;
impl crate::RegisterSpec for CCFG_SMCNFCS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccfg_smcnfcs::R](R) reader structure"]
impl crate::Readable for CCFG_SMCNFCS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccfg_smcnfcs::W](W) writer structure"]
impl crate::Writable for CCFG_SMCNFCS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCFG_SMCNFCS to value 0"]
impl crate::Resettable for CCFG_SMCNFCS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
