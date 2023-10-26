#[doc = "Register `CCB%s_DITH4` reader"]
pub type R = crate::R<CCB_DITH4_SPEC>;
#[doc = "Register `CCB%s_DITH4` writer"]
pub type W = crate::W<CCB_DITH4_SPEC>;
#[doc = "Field `DITHERCYB` reader - Dithering Buffer Cycle Number"]
pub type DITHERCYB_R = crate::FieldReader;
#[doc = "Field `DITHERCYB` writer - Dithering Buffer Cycle Number"]
pub type DITHERCYB_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `CCB` reader - Channel Compare/Capture Buffer Value"]
pub type CCB_R = crate::FieldReader<u32>;
#[doc = "Field `CCB` writer - Channel Compare/Capture Buffer Value"]
pub type CCB_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 20, O, u32>;
impl R {
    #[doc = "Bits 0:3 - Dithering Buffer Cycle Number"]
    #[inline(always)]
    pub fn dithercyb(&self) -> DITHERCYB_R {
        DITHERCYB_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:23 - Channel Compare/Capture Buffer Value"]
    #[inline(always)]
    pub fn ccb(&self) -> CCB_R {
        CCB_R::new((self.bits >> 4) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - Dithering Buffer Cycle Number"]
    #[inline(always)]
    #[must_use]
    pub fn dithercyb(&mut self) -> DITHERCYB_W<CCB_DITH4_SPEC, 0> {
        DITHERCYB_W::new(self)
    }
    #[doc = "Bits 4:23 - Channel Compare/Capture Buffer Value"]
    #[inline(always)]
    #[must_use]
    pub fn ccb(&mut self) -> CCB_W<CCB_DITH4_SPEC, 4> {
        CCB_W::new(self)
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
#[doc = "Compare and Capture Buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccb_dith4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccb_dith4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCB_DITH4_SPEC;
impl crate::RegisterSpec for CCB_DITH4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccb_dith4::R`](R) reader structure"]
impl crate::Readable for CCB_DITH4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccb_dith4::W`](W) writer structure"]
impl crate::Writable for CCB_DITH4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCB%s_DITH4 to value 0"]
impl crate::Resettable for CCB_DITH4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
