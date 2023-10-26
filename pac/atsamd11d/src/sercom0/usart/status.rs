#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<STATUS_SPEC>;
#[doc = "Field `PERR` reader - Parity Error"]
pub type PERR_R = crate::BitReader;
#[doc = "Field `PERR` writer - Parity Error"]
pub type PERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FERR` reader - Frame Error"]
pub type FERR_R = crate::BitReader;
#[doc = "Field `FERR` writer - Frame Error"]
pub type FERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BUFOVF` reader - Buffer Overflow"]
pub type BUFOVF_R = crate::BitReader;
#[doc = "Field `BUFOVF` writer - Buffer Overflow"]
pub type BUFOVF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CTS` reader - Clear To Send"]
pub type CTS_R = crate::BitReader;
#[doc = "Field `ISF` reader - Inconsistent Sync Field"]
pub type ISF_R = crate::BitReader;
#[doc = "Field `ISF` writer - Inconsistent Sync Field"]
pub type ISF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `COLL` reader - Collision Detected"]
pub type COLL_R = crate::BitReader;
#[doc = "Field `COLL` writer - Collision Detected"]
pub type COLL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Parity Error"]
    #[inline(always)]
    pub fn perr(&self) -> PERR_R {
        PERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Frame Error"]
    #[inline(always)]
    pub fn ferr(&self) -> FERR_R {
        FERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Buffer Overflow"]
    #[inline(always)]
    pub fn bufovf(&self) -> BUFOVF_R {
        BUFOVF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clear To Send"]
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Inconsistent Sync Field"]
    #[inline(always)]
    pub fn isf(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Collision Detected"]
    #[inline(always)]
    pub fn coll(&self) -> COLL_R {
        COLL_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Parity Error"]
    #[inline(always)]
    #[must_use]
    pub fn perr(&mut self) -> PERR_W<STATUS_SPEC, 0> {
        PERR_W::new(self)
    }
    #[doc = "Bit 1 - Frame Error"]
    #[inline(always)]
    #[must_use]
    pub fn ferr(&mut self) -> FERR_W<STATUS_SPEC, 1> {
        FERR_W::new(self)
    }
    #[doc = "Bit 2 - Buffer Overflow"]
    #[inline(always)]
    #[must_use]
    pub fn bufovf(&mut self) -> BUFOVF_W<STATUS_SPEC, 2> {
        BUFOVF_W::new(self)
    }
    #[doc = "Bit 4 - Inconsistent Sync Field"]
    #[inline(always)]
    #[must_use]
    pub fn isf(&mut self) -> ISF_W<STATUS_SPEC, 4> {
        ISF_W::new(self)
    }
    #[doc = "Bit 5 - Collision Detected"]
    #[inline(always)]
    #[must_use]
    pub fn coll(&mut self) -> COLL_W<STATUS_SPEC, 5> {
        COLL_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "USART Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
