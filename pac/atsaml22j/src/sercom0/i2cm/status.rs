#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATUS` writer"]
pub struct W(crate::W<STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUSERR` reader - Bus Error"]
pub type BUSERR_R = crate::BitReader<bool>;
#[doc = "Field `BUSERR` writer - Bus Error"]
pub type BUSERR_W<'a, const O: u8> = crate::BitWriter<'a, u16, STATUS_SPEC, bool, O>;
#[doc = "Field `ARBLOST` reader - Arbitration Lost"]
pub type ARBLOST_R = crate::BitReader<bool>;
#[doc = "Field `ARBLOST` writer - Arbitration Lost"]
pub type ARBLOST_W<'a, const O: u8> = crate::BitWriter<'a, u16, STATUS_SPEC, bool, O>;
#[doc = "Field `RXNACK` reader - Received Not Acknowledge"]
pub type RXNACK_R = crate::BitReader<bool>;
#[doc = "Field `BUSSTATE` reader - Bus State"]
pub type BUSSTATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BUSSTATE` writer - Bus State"]
pub type BUSSTATE_W<'a, const O: u8> = crate::FieldWriter<'a, u16, STATUS_SPEC, u8, u8, 2, O>;
#[doc = "Field `LOWTOUT` reader - SCL Low Timeout"]
pub type LOWTOUT_R = crate::BitReader<bool>;
#[doc = "Field `LOWTOUT` writer - SCL Low Timeout"]
pub type LOWTOUT_W<'a, const O: u8> = crate::BitWriter<'a, u16, STATUS_SPEC, bool, O>;
#[doc = "Field `CLKHOLD` reader - Clock Hold"]
pub type CLKHOLD_R = crate::BitReader<bool>;
#[doc = "Field `MEXTTOUT` reader - Master SCL Low Extend Timeout"]
pub type MEXTTOUT_R = crate::BitReader<bool>;
#[doc = "Field `MEXTTOUT` writer - Master SCL Low Extend Timeout"]
pub type MEXTTOUT_W<'a, const O: u8> = crate::BitWriter<'a, u16, STATUS_SPEC, bool, O>;
#[doc = "Field `SEXTTOUT` reader - Slave SCL Low Extend Timeout"]
pub type SEXTTOUT_R = crate::BitReader<bool>;
#[doc = "Field `SEXTTOUT` writer - Slave SCL Low Extend Timeout"]
pub type SEXTTOUT_W<'a, const O: u8> = crate::BitWriter<'a, u16, STATUS_SPEC, bool, O>;
#[doc = "Field `LENERR` reader - Length Error"]
pub type LENERR_R = crate::BitReader<bool>;
#[doc = "Field `LENERR` writer - Length Error"]
pub type LENERR_W<'a, const O: u8> = crate::BitWriter<'a, u16, STATUS_SPEC, bool, O>;
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
    pub fn buserr(&mut self) -> BUSERR_W<0> {
        BUSERR_W::new(self)
    }
    #[doc = "Bit 1 - Arbitration Lost"]
    #[inline(always)]
    #[must_use]
    pub fn arblost(&mut self) -> ARBLOST_W<1> {
        ARBLOST_W::new(self)
    }
    #[doc = "Bits 4:5 - Bus State"]
    #[inline(always)]
    #[must_use]
    pub fn busstate(&mut self) -> BUSSTATE_W<4> {
        BUSSTATE_W::new(self)
    }
    #[doc = "Bit 6 - SCL Low Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn lowtout(&mut self) -> LOWTOUT_W<6> {
        LOWTOUT_W::new(self)
    }
    #[doc = "Bit 8 - Master SCL Low Extend Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn mexttout(&mut self) -> MEXTTOUT_W<8> {
        MEXTTOUT_W::new(self)
    }
    #[doc = "Bit 9 - Slave SCL Low Extend Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn sexttout(&mut self) -> SEXTTOUT_W<9> {
        SEXTTOUT_W::new(self)
    }
    #[doc = "Bit 10 - Length Error"]
    #[inline(always)]
    #[must_use]
    pub fn lenerr(&mut self) -> LENERR_W<10> {
        LENERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2CM Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [status::W](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
