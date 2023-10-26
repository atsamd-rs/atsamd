#[doc = "Register `CACR` reader"]
pub type R = crate::R<CACR_SPEC>;
#[doc = "Register `CACR` writer"]
pub type W = crate::W<CACR_SPEC>;
#[doc = "Field `CAPWREN` reader - Capabilities Registers Write Enable (Required to write the correct frequencies in the Capabilities Registers)"]
pub type CAPWREN_R = crate::BitReader;
#[doc = "Field `CAPWREN` writer - Capabilities Registers Write Enable (Required to write the correct frequencies in the Capabilities Registers)"]
pub type CAPWREN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `KEY` reader - Key (0x46)"]
pub type KEY_R = crate::FieldReader;
#[doc = "Field `KEY` writer - Key (0x46)"]
pub type KEY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bit 0 - Capabilities Registers Write Enable (Required to write the correct frequencies in the Capabilities Registers)"]
    #[inline(always)]
    pub fn capwren(&self) -> CAPWREN_R {
        CAPWREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:15 - Key (0x46)"]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Capabilities Registers Write Enable (Required to write the correct frequencies in the Capabilities Registers)"]
    #[inline(always)]
    #[must_use]
    pub fn capwren(&mut self) -> CAPWREN_W<CACR_SPEC, 0> {
        CAPWREN_W::new(self)
    }
    #[doc = "Bits 8:15 - Key (0x46)"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<CACR_SPEC, 8> {
        KEY_W::new(self)
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
#[doc = "Capabilities Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cacr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cacr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACR_SPEC;
impl crate::RegisterSpec for CACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cacr::R`](R) reader structure"]
impl crate::Readable for CACR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cacr::W`](W) writer structure"]
impl crate::Writable for CACR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CACR to value 0"]
impl crate::Resettable for CACR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
