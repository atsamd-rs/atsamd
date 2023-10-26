#[doc = "Register `TBQB` reader"]
pub type R = crate::R<TBQB_SPEC>;
#[doc = "Register `TBQB` writer"]
pub type W = crate::W<TBQB_SPEC>;
#[doc = "Field `ADDR` reader - Transmit Buffer Queue Base Address"]
pub type ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - Transmit Buffer Queue Base Address"]
pub type ADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 30, O, u32>;
impl R {
    #[doc = "Bits 2:31 - Transmit Buffer Queue Base Address"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Transmit Buffer Queue Base Address"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<TBQB_SPEC, 2> {
        ADDR_W::new(self)
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
#[doc = "Transmit Buffer Queue Base Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbqb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tbqb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TBQB_SPEC;
impl crate::RegisterSpec for TBQB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbqb::R`](R) reader structure"]
impl crate::Readable for TBQB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tbqb::W`](W) writer structure"]
impl crate::Writable for TBQB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TBQB to value 0"]
impl crate::Resettable for TBQB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
