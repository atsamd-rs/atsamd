#[doc = "Register `PCFG` reader"]
pub type R = crate::R<PCFG_SPEC>;
#[doc = "Register `PCFG` writer"]
pub type W = crate::W<PCFG_SPEC>;
#[doc = "Field `PTOKEN` reader - Pipe Token"]
pub type PTOKEN_R = crate::FieldReader;
#[doc = "Field `PTOKEN` writer - Pipe Token"]
pub type PTOKEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `BK` reader - Pipe Bank"]
pub type BK_R = crate::BitReader;
#[doc = "Field `BK` writer - Pipe Bank"]
pub type BK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PTYPE` reader - Pipe Type"]
pub type PTYPE_R = crate::FieldReader;
#[doc = "Field `PTYPE` writer - Pipe Type"]
pub type PTYPE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 0:1 - Pipe Token"]
    #[inline(always)]
    pub fn ptoken(&self) -> PTOKEN_R {
        PTOKEN_R::new(self.bits & 3)
    }
    #[doc = "Bit 2 - Pipe Bank"]
    #[inline(always)]
    pub fn bk(&self) -> BK_R {
        BK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - Pipe Type"]
    #[inline(always)]
    pub fn ptype(&self) -> PTYPE_R {
        PTYPE_R::new((self.bits >> 3) & 7)
    }
}
impl W {
    #[doc = "Bits 0:1 - Pipe Token"]
    #[inline(always)]
    #[must_use]
    pub fn ptoken(&mut self) -> PTOKEN_W<PCFG_SPEC, 0> {
        PTOKEN_W::new(self)
    }
    #[doc = "Bit 2 - Pipe Bank"]
    #[inline(always)]
    #[must_use]
    pub fn bk(&mut self) -> BK_W<PCFG_SPEC, 2> {
        BK_W::new(self)
    }
    #[doc = "Bits 3:5 - Pipe Type"]
    #[inline(always)]
    #[must_use]
    pub fn ptype(&mut self) -> PTYPE_W<PCFG_SPEC, 3> {
        PTYPE_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "HOST_PIPE End Point Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCFG_SPEC;
impl crate::RegisterSpec for PCFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pcfg::R`](R) reader structure"]
impl crate::Readable for PCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pcfg::W`](W) writer structure"]
impl crate::Writable for PCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCFG to value 0"]
impl crate::Resettable for PCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
