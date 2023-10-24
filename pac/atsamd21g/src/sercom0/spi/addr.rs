#[doc = "Register `ADDR` reader"]
pub type R = crate::R<ADDR_SPEC>;
#[doc = "Register `ADDR` writer"]
pub type W = crate::W<ADDR_SPEC>;
#[doc = "Field `ADDR` reader - Address Value"]
pub type ADDR_R = crate::FieldReader;
#[doc = "Field `ADDR` writer - Address Value"]
pub type ADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `ADDRMASK` reader - Address Mask"]
pub type ADDRMASK_R = crate::FieldReader;
#[doc = "Field `ADDRMASK` writer - Address Mask"]
pub type ADDRMASK_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Address Value"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Address Mask"]
    #[inline(always)]
    pub fn addrmask(&self) -> ADDRMASK_R {
        ADDRMASK_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Address Value"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<ADDR_SPEC, 0> {
        ADDR_W::new(self)
    }
    #[doc = "Bits 16:23 - Address Mask"]
    #[inline(always)]
    #[must_use]
    pub fn addrmask(&mut self) -> ADDRMASK_W<ADDR_SPEC, 16> {
        ADDRMASK_W::new(self)
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
#[doc = "SPI Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADDR_SPEC;
impl crate::RegisterSpec for ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr::R`](R) reader structure"]
impl crate::Readable for ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`addr::W`](W) writer structure"]
impl crate::Writable for ADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADDR to value 0"]
impl crate::Resettable for ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
