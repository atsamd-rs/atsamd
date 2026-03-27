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
#[doc = "Field `STOP` reader - Stop Status Flag"]
pub type STOP_R = crate::BitReader<bool>;
#[doc = "Field `SLAVE` reader - Slave Status Flag"]
pub type SLAVE_R = crate::BitReader<bool>;
#[doc = "Field `PERBUFV` reader - Synchronization Busy Status"]
pub type PERBUFV_R = crate::BitReader<bool>;
#[doc = "Field `PERBUFV` writer - Synchronization Busy Status"]
pub type PERBUFV_W<'a, const O: u8> = crate::BitWriter<'a, u8, STATUS_SPEC, bool, O>;
#[doc = "Field `CCBUFV0` reader - Compare channel buffer 0 valid"]
pub type CCBUFV0_R = crate::BitReader<bool>;
#[doc = "Field `CCBUFV0` writer - Compare channel buffer 0 valid"]
pub type CCBUFV0_W<'a, const O: u8> = crate::BitWriter<'a, u8, STATUS_SPEC, bool, O>;
#[doc = "Field `CCBUFV1` reader - Compare channel buffer 1 valid"]
pub type CCBUFV1_R = crate::BitReader<bool>;
#[doc = "Field `CCBUFV1` writer - Compare channel buffer 1 valid"]
pub type CCBUFV1_W<'a, const O: u8> = crate::BitWriter<'a, u8, STATUS_SPEC, bool, O>;
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
    #[doc = "Bit 3 - Synchronization Busy Status"]
    #[inline(always)]
    #[must_use]
    pub fn perbufv(&mut self) -> PERBUFV_W<3> {
        PERBUFV_W::new(self)
    }
    #[doc = "Bit 4 - Compare channel buffer 0 valid"]
    #[inline(always)]
    #[must_use]
    pub fn ccbufv0(&mut self) -> CCBUFV0_W<4> {
        CCBUFV0_W::new(self)
    }
    #[doc = "Bit 5 - Compare channel buffer 1 valid"]
    #[inline(always)]
    #[must_use]
    pub fn ccbufv1(&mut self) -> CCBUFV1_W<5> {
        CCBUFV1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u8;
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
#[doc = "`reset()` method sets STATUS to value 0x01"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
