#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<STATUS_SPEC>;
#[doc = "Field `BUSERR` reader - Bus Error"]
pub type BUSERR_R = crate::BitReader;
#[doc = "Field `BUSERR` writer - Bus Error"]
pub type BUSERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `COLL` reader - Transmit Collision"]
pub type COLL_R = crate::BitReader;
#[doc = "Field `COLL` writer - Transmit Collision"]
pub type COLL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXNACK` reader - Received Not Acknowledge"]
pub type RXNACK_R = crate::BitReader;
#[doc = "Field `DIR` reader - Read/Write Direction"]
pub type DIR_R = crate::BitReader;
#[doc = "Field `SR` reader - Repeated Start"]
pub type SR_R = crate::BitReader;
#[doc = "Field `LOWTOUT` reader - SCL Low Timeout"]
pub type LOWTOUT_R = crate::BitReader;
#[doc = "Field `LOWTOUT` writer - SCL Low Timeout"]
pub type LOWTOUT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLKHOLD` reader - Clock Hold"]
pub type CLKHOLD_R = crate::BitReader;
#[doc = "Field `SEXTTOUT` reader - Slave SCL Low Extend Timeout"]
pub type SEXTTOUT_R = crate::BitReader;
#[doc = "Field `SEXTTOUT` writer - Slave SCL Low Extend Timeout"]
pub type SEXTTOUT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HS` reader - High Speed"]
pub type HS_R = crate::BitReader;
#[doc = "Field `HS` writer - High Speed"]
pub type HS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Bus Error"]
    #[inline(always)]
    pub fn buserr(&self) -> BUSERR_R {
        BUSERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Collision"]
    #[inline(always)]
    pub fn coll(&self) -> COLL_R {
        COLL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Received Not Acknowledge"]
    #[inline(always)]
    pub fn rxnack(&self) -> RXNACK_R {
        RXNACK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Read/Write Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Repeated Start"]
    #[inline(always)]
    pub fn sr(&self) -> SR_R {
        SR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - SCL Low Timeout"]
    #[inline(always)]
    pub fn lowtout(&self) -> LOWTOUT_R {
        LOWTOUT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Clock Hold"]
    #[inline(always)]
    pub fn clkhold(&self) -> CLKHOLD_R {
        CLKHOLD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Slave SCL Low Extend Timeout"]
    #[inline(always)]
    pub fn sexttout(&self) -> SEXTTOUT_R {
        SEXTTOUT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - High Speed"]
    #[inline(always)]
    pub fn hs(&self) -> HS_R {
        HS_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bus Error"]
    #[inline(always)]
    #[must_use]
    pub fn buserr(&mut self) -> BUSERR_W<STATUS_SPEC, 0> {
        BUSERR_W::new(self)
    }
    #[doc = "Bit 1 - Transmit Collision"]
    #[inline(always)]
    #[must_use]
    pub fn coll(&mut self) -> COLL_W<STATUS_SPEC, 1> {
        COLL_W::new(self)
    }
    #[doc = "Bit 6 - SCL Low Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn lowtout(&mut self) -> LOWTOUT_W<STATUS_SPEC, 6> {
        LOWTOUT_W::new(self)
    }
    #[doc = "Bit 9 - Slave SCL Low Extend Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn sexttout(&mut self) -> SEXTTOUT_W<STATUS_SPEC, 9> {
        SEXTTOUT_W::new(self)
    }
    #[doc = "Bit 10 - High Speed"]
    #[inline(always)]
    #[must_use]
    pub fn hs(&mut self) -> HS_W<STATUS_SPEC, 10> {
        HS_W::new(self)
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
#[doc = "I2CS Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
