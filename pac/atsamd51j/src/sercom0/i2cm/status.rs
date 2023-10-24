#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<STATUS_SPEC>;
#[doc = "Field `BUSERR` reader - Bus Error"]
pub type BUSERR_R = crate::BitReader;
#[doc = "Field `BUSERR` writer - Bus Error"]
pub type BUSERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ARBLOST` reader - Arbitration Lost"]
pub type ARBLOST_R = crate::BitReader;
#[doc = "Field `ARBLOST` writer - Arbitration Lost"]
pub type ARBLOST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXNACK` reader - Received Not Acknowledge"]
pub type RXNACK_R = crate::BitReader;
#[doc = "Field `RXNACK` writer - Received Not Acknowledge"]
pub type RXNACK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BUSSTATE` reader - Bus State"]
pub type BUSSTATE_R = crate::FieldReader;
#[doc = "Field `BUSSTATE` writer - Bus State"]
pub type BUSSTATE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `LOWTOUT` reader - SCL Low Timeout"]
pub type LOWTOUT_R = crate::BitReader;
#[doc = "Field `LOWTOUT` writer - SCL Low Timeout"]
pub type LOWTOUT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLKHOLD` reader - Clock Hold"]
pub type CLKHOLD_R = crate::BitReader;
#[doc = "Field `CLKHOLD` writer - Clock Hold"]
pub type CLKHOLD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MEXTTOUT` reader - Master SCL Low Extend Timeout"]
pub type MEXTTOUT_R = crate::BitReader;
#[doc = "Field `MEXTTOUT` writer - Master SCL Low Extend Timeout"]
pub type MEXTTOUT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SEXTTOUT` reader - Slave SCL Low Extend Timeout"]
pub type SEXTTOUT_R = crate::BitReader;
#[doc = "Field `SEXTTOUT` writer - Slave SCL Low Extend Timeout"]
pub type SEXTTOUT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LENERR` reader - Length Error"]
pub type LENERR_R = crate::BitReader;
#[doc = "Field `LENERR` writer - Length Error"]
pub type LENERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Bus Error"]
    #[inline(always)]
    pub fn buserr(&self) -> BUSERR_R {
        BUSERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Arbitration Lost"]
    #[inline(always)]
    pub fn arblost(&self) -> ARBLOST_R {
        ARBLOST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Received Not Acknowledge"]
    #[inline(always)]
    pub fn rxnack(&self) -> RXNACK_R {
        RXNACK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Bus State"]
    #[inline(always)]
    pub fn busstate(&self) -> BUSSTATE_R {
        BUSSTATE_R::new(((self.bits >> 4) & 3) as u8)
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
    #[doc = "Bit 8 - Master SCL Low Extend Timeout"]
    #[inline(always)]
    pub fn mexttout(&self) -> MEXTTOUT_R {
        MEXTTOUT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Slave SCL Low Extend Timeout"]
    #[inline(always)]
    pub fn sexttout(&self) -> SEXTTOUT_R {
        SEXTTOUT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Length Error"]
    #[inline(always)]
    pub fn lenerr(&self) -> LENERR_R {
        LENERR_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bus Error"]
    #[inline(always)]
    #[must_use]
    pub fn buserr(&mut self) -> BUSERR_W<STATUS_SPEC, 0> {
        BUSERR_W::new(self)
    }
    #[doc = "Bit 1 - Arbitration Lost"]
    #[inline(always)]
    #[must_use]
    pub fn arblost(&mut self) -> ARBLOST_W<STATUS_SPEC, 1> {
        ARBLOST_W::new(self)
    }
    #[doc = "Bit 2 - Received Not Acknowledge"]
    #[inline(always)]
    #[must_use]
    pub fn rxnack(&mut self) -> RXNACK_W<STATUS_SPEC, 2> {
        RXNACK_W::new(self)
    }
    #[doc = "Bits 4:5 - Bus State"]
    #[inline(always)]
    #[must_use]
    pub fn busstate(&mut self) -> BUSSTATE_W<STATUS_SPEC, 4> {
        BUSSTATE_W::new(self)
    }
    #[doc = "Bit 6 - SCL Low Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn lowtout(&mut self) -> LOWTOUT_W<STATUS_SPEC, 6> {
        LOWTOUT_W::new(self)
    }
    #[doc = "Bit 7 - Clock Hold"]
    #[inline(always)]
    #[must_use]
    pub fn clkhold(&mut self) -> CLKHOLD_W<STATUS_SPEC, 7> {
        CLKHOLD_W::new(self)
    }
    #[doc = "Bit 8 - Master SCL Low Extend Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn mexttout(&mut self) -> MEXTTOUT_W<STATUS_SPEC, 8> {
        MEXTTOUT_W::new(self)
    }
    #[doc = "Bit 9 - Slave SCL Low Extend Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn sexttout(&mut self) -> SEXTTOUT_W<STATUS_SPEC, 9> {
        SEXTTOUT_W::new(self)
    }
    #[doc = "Bit 10 - Length Error"]
    #[inline(always)]
    #[must_use]
    pub fn lenerr(&mut self) -> LENERR_W<STATUS_SPEC, 10> {
        LENERR_W::new(self)
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
#[doc = "I2CM Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
