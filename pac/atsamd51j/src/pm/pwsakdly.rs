#[doc = "Register `PWSAKDLY` reader"]
pub type R = crate::R<PWSAKDLY_SPEC>;
#[doc = "Register `PWSAKDLY` writer"]
pub type W = crate::W<PWSAKDLY_SPEC>;
#[doc = "Field `DLYVAL` reader - Delay Value"]
pub type DLYVAL_R = crate::FieldReader;
#[doc = "Field `DLYVAL` writer - Delay Value"]
pub type DLYVAL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `IGNACK` reader - Ignore Acknowledge"]
pub type IGNACK_R = crate::BitReader;
#[doc = "Field `IGNACK` writer - Ignore Acknowledge"]
pub type IGNACK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:6 - Delay Value"]
    #[inline(always)]
    pub fn dlyval(&self) -> DLYVAL_R {
        DLYVAL_R::new(self.bits & 0x7f)
    }
    #[doc = "Bit 7 - Ignore Acknowledge"]
    #[inline(always)]
    pub fn ignack(&self) -> IGNACK_R {
        IGNACK_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Delay Value"]
    #[inline(always)]
    #[must_use]
    pub fn dlyval(&mut self) -> DLYVAL_W<PWSAKDLY_SPEC, 0> {
        DLYVAL_W::new(self)
    }
    #[doc = "Bit 7 - Ignore Acknowledge"]
    #[inline(always)]
    #[must_use]
    pub fn ignack(&mut self) -> IGNACK_W<PWSAKDLY_SPEC, 7> {
        IGNACK_W::new(self)
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
#[doc = "Power Switch Acknowledge Delay\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwsakdly::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwsakdly::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWSAKDLY_SPEC;
impl crate::RegisterSpec for PWSAKDLY_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pwsakdly::R`](R) reader structure"]
impl crate::Readable for PWSAKDLY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pwsakdly::W`](W) writer structure"]
impl crate::Writable for PWSAKDLY_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWSAKDLY to value 0"]
impl crate::Resettable for PWSAKDLY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
