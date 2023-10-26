#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<STATUS_SPEC>;
#[doc = "Field `STOP` reader - Stop Status Flag"]
pub type STOP_R = crate::BitReader;
#[doc = "Field `STOP` writer - Stop Status Flag"]
pub type STOP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLAVE` reader - Slave Status Flag"]
pub type SLAVE_R = crate::BitReader;
#[doc = "Field `SLAVE` writer - Slave Status Flag"]
pub type SLAVE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PERBUFV` reader - Synchronization Busy Status"]
pub type PERBUFV_R = crate::BitReader;
#[doc = "Field `PERBUFV` writer - Synchronization Busy Status"]
pub type PERBUFV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CCBUFV0` reader - Compare channel buffer 0 valid"]
pub type CCBUFV0_R = crate::BitReader;
#[doc = "Field `CCBUFV0` writer - Compare channel buffer 0 valid"]
pub type CCBUFV0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CCBUFV1` reader - Compare channel buffer 1 valid"]
pub type CCBUFV1_R = crate::BitReader;
#[doc = "Field `CCBUFV1` writer - Compare channel buffer 1 valid"]
pub type CCBUFV1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Stop Status Flag"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Slave Status Flag"]
    #[inline(always)]
    pub fn slave(&self) -> SLAVE_R {
        SLAVE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Synchronization Busy Status"]
    #[inline(always)]
    pub fn perbufv(&self) -> PERBUFV_R {
        PERBUFV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Compare channel buffer 0 valid"]
    #[inline(always)]
    pub fn ccbufv0(&self) -> CCBUFV0_R {
        CCBUFV0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Compare channel buffer 1 valid"]
    #[inline(always)]
    pub fn ccbufv1(&self) -> CCBUFV1_R {
        CCBUFV1_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Stop Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<STATUS_SPEC, 0> {
        STOP_W::new(self)
    }
    #[doc = "Bit 1 - Slave Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn slave(&mut self) -> SLAVE_W<STATUS_SPEC, 1> {
        SLAVE_W::new(self)
    }
    #[doc = "Bit 3 - Synchronization Busy Status"]
    #[inline(always)]
    #[must_use]
    pub fn perbufv(&mut self) -> PERBUFV_W<STATUS_SPEC, 3> {
        PERBUFV_W::new(self)
    }
    #[doc = "Bit 4 - Compare channel buffer 0 valid"]
    #[inline(always)]
    #[must_use]
    pub fn ccbufv0(&mut self) -> CCBUFV0_W<STATUS_SPEC, 4> {
        CCBUFV0_W::new(self)
    }
    #[doc = "Bit 5 - Compare channel buffer 1 valid"]
    #[inline(always)]
    #[must_use]
    pub fn ccbufv1(&mut self) -> CCBUFV1_W<STATUS_SPEC, 5> {
        CCBUFV1_W::new(self)
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
#[doc = "Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATUS to value 0x01"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
