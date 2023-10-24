#[doc = "Register `POSITION` reader"]
pub type R = crate::R<POSITION_SPEC>;
#[doc = "Register `POSITION` writer"]
pub type W = crate::W<POSITION_SPEC>;
#[doc = "Field `WRAP` reader - Pointer Value Wraps"]
pub type WRAP_R = crate::BitReader;
#[doc = "Field `WRAP` writer - Pointer Value Wraps"]
pub type WRAP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `POINTER` reader - Trace Packet Location Pointer"]
pub type POINTER_R = crate::FieldReader<u32>;
#[doc = "Field `POINTER` writer - Trace Packet Location Pointer"]
pub type POINTER_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 29, O, u32>;
impl R {
    #[doc = "Bit 2 - Pointer Value Wraps"]
    #[inline(always)]
    pub fn wrap(&self) -> WRAP_R {
        WRAP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31 - Trace Packet Location Pointer"]
    #[inline(always)]
    pub fn pointer(&self) -> POINTER_R {
        POINTER_R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bit 2 - Pointer Value Wraps"]
    #[inline(always)]
    #[must_use]
    pub fn wrap(&mut self) -> WRAP_W<POSITION_SPEC, 2> {
        WRAP_W::new(self)
    }
    #[doc = "Bits 3:31 - Trace Packet Location Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn pointer(&mut self) -> POINTER_W<POSITION_SPEC, 3> {
        POINTER_W::new(self)
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
#[doc = "MTB Position\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`position::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`position::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct POSITION_SPEC;
impl crate::RegisterSpec for POSITION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`position::R`](R) reader structure"]
impl crate::Readable for POSITION_SPEC {}
#[doc = "`write(|w| ..)` method takes [`position::W`](W) writer structure"]
impl crate::Writable for POSITION_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets POSITION to value 0"]
impl crate::Resettable for POSITION_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
