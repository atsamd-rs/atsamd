#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ETMPD` reader - ETM Power Down"]
pub type ETMPD_R = crate::BitReader<bool>;
#[doc = "Field `ETMPD` writer - ETM Power Down"]
pub type ETMPD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `PORTSIZE` reader - Port Size bits 2:0"]
pub type PORTSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PORTSIZE` writer - Port Size bits 2:0"]
pub type PORTSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 3, O>;
#[doc = "Field `STALL` reader - Stall Processor"]
pub type STALL_R = crate::BitReader<bool>;
#[doc = "Field `STALL` writer - Stall Processor"]
pub type STALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `BROUT` reader - Branch Output"]
pub type BROUT_R = crate::BitReader<bool>;
#[doc = "Field `BROUT` writer - Branch Output"]
pub type BROUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DBGRQ` reader - Debug Request Control"]
pub type DBGRQ_R = crate::BitReader<bool>;
#[doc = "Field `DBGRQ` writer - Debug Request Control"]
pub type DBGRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `PROG` reader - ETM Programming"]
pub type PROG_R = crate::BitReader<bool>;
#[doc = "Field `PROG` writer - ETM Programming"]
pub type PROG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `PORTSEL` reader - ETM Port Select"]
pub type PORTSEL_R = crate::BitReader<bool>;
#[doc = "Field `PORTSEL` writer - ETM Port Select"]
pub type PORTSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `PORTMODE2` reader - Port Mode bit 2"]
pub type PORTMODE2_R = crate::BitReader<bool>;
#[doc = "Field `PORTMODE2` writer - Port Mode bit 2"]
pub type PORTMODE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `PORTMODE` reader - Port Mode bits 1:0"]
pub type PORTMODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PORTMODE` writer - Port Mode bits 1:0"]
pub type PORTMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PORTSIZE3` reader - Port Size bit 3"]
pub type PORTSIZE3_R = crate::BitReader<bool>;
#[doc = "Field `PORTSIZE3` writer - Port Size bit 3"]
pub type PORTSIZE3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `TSEN` reader - TimeStamp Enable"]
pub type TSEN_R = crate::BitReader<bool>;
#[doc = "Field `TSEN` writer - TimeStamp Enable"]
pub type TSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - ETM Power Down"]
    #[inline(always)]
    pub fn etmpd(&self) -> ETMPD_R {
        ETMPD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6 - Port Size bits 2:0"]
    #[inline(always)]
    pub fn portsize(&self) -> PORTSIZE_R {
        PORTSIZE_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Stall Processor"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Branch Output"]
    #[inline(always)]
    pub fn brout(&self) -> BROUT_R {
        BROUT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Debug Request Control"]
    #[inline(always)]
    pub fn dbgrq(&self) -> DBGRQ_R {
        DBGRQ_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ETM Programming"]
    #[inline(always)]
    pub fn prog(&self) -> PROG_R {
        PROG_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ETM Port Select"]
    #[inline(always)]
    pub fn portsel(&self) -> PORTSEL_R {
        PORTSEL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Port Mode bit 2"]
    #[inline(always)]
    pub fn portmode2(&self) -> PORTMODE2_R {
        PORTMODE2_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Port Mode bits 1:0"]
    #[inline(always)]
    pub fn portmode(&self) -> PORTMODE_R {
        PORTMODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 21 - Port Size bit 3"]
    #[inline(always)]
    pub fn portsize3(&self) -> PORTSIZE3_R {
        PORTSIZE3_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 28 - TimeStamp Enable"]
    #[inline(always)]
    pub fn tsen(&self) -> TSEN_R {
        TSEN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ETM Power Down"]
    #[inline(always)]
    #[must_use]
    pub fn etmpd(&mut self) -> ETMPD_W<0> {
        ETMPD_W::new(self)
    }
    #[doc = "Bits 4:6 - Port Size bits 2:0"]
    #[inline(always)]
    #[must_use]
    pub fn portsize(&mut self) -> PORTSIZE_W<4> {
        PORTSIZE_W::new(self)
    }
    #[doc = "Bit 7 - Stall Processor"]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> STALL_W<7> {
        STALL_W::new(self)
    }
    #[doc = "Bit 8 - Branch Output"]
    #[inline(always)]
    #[must_use]
    pub fn brout(&mut self) -> BROUT_W<8> {
        BROUT_W::new(self)
    }
    #[doc = "Bit 9 - Debug Request Control"]
    #[inline(always)]
    #[must_use]
    pub fn dbgrq(&mut self) -> DBGRQ_W<9> {
        DBGRQ_W::new(self)
    }
    #[doc = "Bit 10 - ETM Programming"]
    #[inline(always)]
    #[must_use]
    pub fn prog(&mut self) -> PROG_W<10> {
        PROG_W::new(self)
    }
    #[doc = "Bit 11 - ETM Port Select"]
    #[inline(always)]
    #[must_use]
    pub fn portsel(&mut self) -> PORTSEL_W<11> {
        PORTSEL_W::new(self)
    }
    #[doc = "Bit 13 - Port Mode bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn portmode2(&mut self) -> PORTMODE2_W<13> {
        PORTMODE2_W::new(self)
    }
    #[doc = "Bits 16:17 - Port Mode bits 1:0"]
    #[inline(always)]
    #[must_use]
    pub fn portmode(&mut self) -> PORTMODE_W<16> {
        PORTMODE_W::new(self)
    }
    #[doc = "Bit 21 - Port Size bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn portsize3(&mut self) -> PORTSIZE3_W<21> {
        PORTSIZE3_W::new(self)
    }
    #[doc = "Bit 28 - TimeStamp Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsen(&mut self) -> TSEN_W<28> {
        TSEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ETM Main Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0x0411"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0411;
}
