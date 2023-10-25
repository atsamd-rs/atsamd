#[doc = "Register `READREQ` reader"]
pub type R = crate::R<READREQ_SPEC>;
#[doc = "Register `READREQ` writer"]
pub type W = crate::W<READREQ_SPEC>;
#[doc = "Field `ADDR` reader - Address"]
pub type ADDR_R = crate::FieldReader;
#[doc = "Field `RCONT` reader - Read Continuously"]
pub type RCONT_R = crate::BitReader;
#[doc = "Field `RCONT` writer - Read Continuously"]
pub type RCONT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RREQ` writer - Read Request"]
pub type RREQ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:5 - Address"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 14 - Read Continuously"]
    #[inline(always)]
    pub fn rcont(&self) -> RCONT_R {
        RCONT_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - Read Continuously"]
    #[inline(always)]
    #[must_use]
    pub fn rcont(&mut self) -> RCONT_W<READREQ_SPEC, 14> {
        RCONT_W::new(self)
    }
    #[doc = "Bit 15 - Read Request"]
    #[inline(always)]
    #[must_use]
    pub fn rreq(&mut self) -> RREQ_W<READREQ_SPEC, 15> {
        RREQ_W::new(self)
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
#[doc = "Read Request\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`readreq::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`readreq::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct READREQ_SPEC;
impl crate::RegisterSpec for READREQ_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`readreq::R`](R) reader structure"]
impl crate::Readable for READREQ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`readreq::W`](W) writer structure"]
impl crate::Writable for READREQ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets READREQ to value 0x10"]
impl crate::Resettable for READREQ_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
