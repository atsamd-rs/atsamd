#[doc = "Register `MC1R` reader"]
pub struct R(crate::R<MC1R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MC1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MC1R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MC1R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MC1R` writer"]
pub struct W(crate::W<MC1R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MC1R_SPEC>;
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
impl From<crate::W<MC1R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MC1R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMDTYP` reader - e.MMC Command Type"]
pub type CMDTYP_R = crate::FieldReader<u8, CMDTYPSELECT_A>;
#[doc = "e.MMC Command Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMDTYPSELECT_A {
    #[doc = "0: Not a MMC specific command"]
    NORMAL = 0,
    #[doc = "1: Wait IRQ Command"]
    WAITIRQ = 1,
    #[doc = "2: Stream Command"]
    STREAM = 2,
    #[doc = "3: Boot Command"]
    BOOT = 3,
}
impl From<CMDTYPSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CMDTYPSELECT_A) -> Self {
        variant as _
    }
}
impl CMDTYP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMDTYPSELECT_A {
        match self.bits {
            0 => CMDTYPSELECT_A::NORMAL,
            1 => CMDTYPSELECT_A::WAITIRQ,
            2 => CMDTYPSELECT_A::STREAM,
            3 => CMDTYPSELECT_A::BOOT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == CMDTYPSELECT_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `WAITIRQ`"]
    #[inline(always)]
    pub fn is_waitirq(&self) -> bool {
        *self == CMDTYPSELECT_A::WAITIRQ
    }
    #[doc = "Checks if the value of the field is `STREAM`"]
    #[inline(always)]
    pub fn is_stream(&self) -> bool {
        *self == CMDTYPSELECT_A::STREAM
    }
    #[doc = "Checks if the value of the field is `BOOT`"]
    #[inline(always)]
    pub fn is_boot(&self) -> bool {
        *self == CMDTYPSELECT_A::BOOT
    }
}
#[doc = "Field `CMDTYP` writer - e.MMC Command Type"]
pub type CMDTYP_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, MC1R_SPEC, u8, CMDTYPSELECT_A, 2, O>;
impl<'a, const O: u8> CMDTYP_W<'a, O> {
    #[doc = "Not a MMC specific command"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(CMDTYPSELECT_A::NORMAL)
    }
    #[doc = "Wait IRQ Command"]
    #[inline(always)]
    pub fn waitirq(self) -> &'a mut W {
        self.variant(CMDTYPSELECT_A::WAITIRQ)
    }
    #[doc = "Stream Command"]
    #[inline(always)]
    pub fn stream(self) -> &'a mut W {
        self.variant(CMDTYPSELECT_A::STREAM)
    }
    #[doc = "Boot Command"]
    #[inline(always)]
    pub fn boot(self) -> &'a mut W {
        self.variant(CMDTYPSELECT_A::BOOT)
    }
}
#[doc = "Field `DDR` reader - e.MMC HSDDR Mode"]
pub type DDR_R = crate::BitReader<bool>;
#[doc = "Field `DDR` writer - e.MMC HSDDR Mode"]
pub type DDR_W<'a, const O: u8> = crate::BitWriter<'a, u8, MC1R_SPEC, bool, O>;
#[doc = "Field `OPD` reader - e.MMC Open Drain Mode"]
pub type OPD_R = crate::BitReader<bool>;
#[doc = "Field `OPD` writer - e.MMC Open Drain Mode"]
pub type OPD_W<'a, const O: u8> = crate::BitWriter<'a, u8, MC1R_SPEC, bool, O>;
#[doc = "Field `BOOTA` reader - e.MMC Boot Acknowledge Enable"]
pub type BOOTA_R = crate::BitReader<bool>;
#[doc = "Field `BOOTA` writer - e.MMC Boot Acknowledge Enable"]
pub type BOOTA_W<'a, const O: u8> = crate::BitWriter<'a, u8, MC1R_SPEC, bool, O>;
#[doc = "Field `RSTN` reader - e.MMC Reset Signal"]
pub type RSTN_R = crate::BitReader<bool>;
#[doc = "Field `RSTN` writer - e.MMC Reset Signal"]
pub type RSTN_W<'a, const O: u8> = crate::BitWriter<'a, u8, MC1R_SPEC, bool, O>;
#[doc = "Field `FCD` reader - e.MMC Force Card Detect"]
pub type FCD_R = crate::BitReader<bool>;
#[doc = "Field `FCD` writer - e.MMC Force Card Detect"]
pub type FCD_W<'a, const O: u8> = crate::BitWriter<'a, u8, MC1R_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - e.MMC Command Type"]
    #[inline(always)]
    pub fn cmdtyp(&self) -> CMDTYP_R {
        CMDTYP_R::new(self.bits & 3)
    }
    #[doc = "Bit 3 - e.MMC HSDDR Mode"]
    #[inline(always)]
    pub fn ddr(&self) -> DDR_R {
        DDR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - e.MMC Open Drain Mode"]
    #[inline(always)]
    pub fn opd(&self) -> OPD_R {
        OPD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - e.MMC Boot Acknowledge Enable"]
    #[inline(always)]
    pub fn boota(&self) -> BOOTA_R {
        BOOTA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - e.MMC Reset Signal"]
    #[inline(always)]
    pub fn rstn(&self) -> RSTN_R {
        RSTN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - e.MMC Force Card Detect"]
    #[inline(always)]
    pub fn fcd(&self) -> FCD_R {
        FCD_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - e.MMC Command Type"]
    #[inline(always)]
    #[must_use]
    pub fn cmdtyp(&mut self) -> CMDTYP_W<0> {
        CMDTYP_W::new(self)
    }
    #[doc = "Bit 3 - e.MMC HSDDR Mode"]
    #[inline(always)]
    #[must_use]
    pub fn ddr(&mut self) -> DDR_W<3> {
        DDR_W::new(self)
    }
    #[doc = "Bit 4 - e.MMC Open Drain Mode"]
    #[inline(always)]
    #[must_use]
    pub fn opd(&mut self) -> OPD_W<4> {
        OPD_W::new(self)
    }
    #[doc = "Bit 5 - e.MMC Boot Acknowledge Enable"]
    #[inline(always)]
    #[must_use]
    pub fn boota(&mut self) -> BOOTA_W<5> {
        BOOTA_W::new(self)
    }
    #[doc = "Bit 6 - e.MMC Reset Signal"]
    #[inline(always)]
    #[must_use]
    pub fn rstn(&mut self) -> RSTN_W<6> {
        RSTN_W::new(self)
    }
    #[doc = "Bit 7 - e.MMC Force Card Detect"]
    #[inline(always)]
    #[must_use]
    pub fn fcd(&mut self) -> FCD_W<7> {
        FCD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MMC Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mc1r](index.html) module"]
pub struct MC1R_SPEC;
impl crate::RegisterSpec for MC1R_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [mc1r::R](R) reader structure"]
impl crate::Readable for MC1R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mc1r::W](W) writer structure"]
impl crate::Writable for MC1R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MC1R to value 0"]
impl crate::Resettable for MC1R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
