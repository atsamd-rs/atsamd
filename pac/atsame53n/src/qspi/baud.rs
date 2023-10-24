#[doc = "Register `BAUD` reader"]
pub type R = crate::R<BAUD_SPEC>;
#[doc = "Register `BAUD` writer"]
pub type W = crate::W<BAUD_SPEC>;
#[doc = "Field `CPOL` reader - Clock Polarity"]
pub type CPOL_R = crate::BitReader;
#[doc = "Field `CPOL` writer - Clock Polarity"]
pub type CPOL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CPHA` reader - Clock Phase"]
pub type CPHA_R = crate::BitReader;
#[doc = "Field `CPHA` writer - Clock Phase"]
pub type CPHA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BAUD` reader - Serial Clock Baud Rate"]
pub type BAUD_R = crate::FieldReader;
#[doc = "Field `BAUD` writer - Serial Clock Baud Rate"]
pub type BAUD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `DLYBS` reader - Delay Before SCK"]
pub type DLYBS_R = crate::FieldReader;
#[doc = "Field `DLYBS` writer - Delay Before SCK"]
pub type DLYBS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bit 0 - Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock Phase"]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Serial Clock Baud Rate"]
    #[inline(always)]
    pub fn baud(&self) -> BAUD_R {
        BAUD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Delay Before SCK"]
    #[inline(always)]
    pub fn dlybs(&self) -> DLYBS_R {
        DLYBS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cpol(&mut self) -> CPOL_W<BAUD_SPEC, 0> {
        CPOL_W::new(self)
    }
    #[doc = "Bit 1 - Clock Phase"]
    #[inline(always)]
    #[must_use]
    pub fn cpha(&mut self) -> CPHA_W<BAUD_SPEC, 1> {
        CPHA_W::new(self)
    }
    #[doc = "Bits 8:15 - Serial Clock Baud Rate"]
    #[inline(always)]
    #[must_use]
    pub fn baud(&mut self) -> BAUD_W<BAUD_SPEC, 8> {
        BAUD_W::new(self)
    }
    #[doc = "Bits 16:23 - Delay Before SCK"]
    #[inline(always)]
    #[must_use]
    pub fn dlybs(&mut self) -> DLYBS_W<BAUD_SPEC, 16> {
        DLYBS_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Baud Rate\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`baud::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`baud::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BAUD_SPEC;
impl crate::RegisterSpec for BAUD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`baud::R`](R) reader structure"]
impl crate::Readable for BAUD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`baud::W`](W) writer structure"]
impl crate::Writable for BAUD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BAUD to value 0"]
impl crate::Resettable for BAUD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
