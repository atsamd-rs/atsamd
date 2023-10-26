#[doc = "Register `RXF0C` reader"]
pub type R = crate::R<RXF0C_SPEC>;
#[doc = "Register `RXF0C` writer"]
pub type W = crate::W<RXF0C_SPEC>;
#[doc = "Field `F0SA` reader - Rx FIFO 0 Start Address"]
pub type F0SA_R = crate::FieldReader<u16>;
#[doc = "Field `F0SA` writer - Rx FIFO 0 Start Address"]
pub type F0SA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `F0S` reader - Rx FIFO 0 Size"]
pub type F0S_R = crate::FieldReader;
#[doc = "Field `F0S` writer - Rx FIFO 0 Size"]
pub type F0S_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `F0WM` reader - Rx FIFO 0 Watermark"]
pub type F0WM_R = crate::FieldReader;
#[doc = "Field `F0WM` writer - Rx FIFO 0 Watermark"]
pub type F0WM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `F0OM` reader - FIFO 0 Operation Mode"]
pub type F0OM_R = crate::BitReader;
#[doc = "Field `F0OM` writer - FIFO 0 Operation Mode"]
pub type F0OM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:15 - Rx FIFO 0 Start Address"]
    #[inline(always)]
    pub fn f0sa(&self) -> F0SA_R {
        F0SA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:22 - Rx FIFO 0 Size"]
    #[inline(always)]
    pub fn f0s(&self) -> F0S_R {
        F0S_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - Rx FIFO 0 Watermark"]
    #[inline(always)]
    pub fn f0wm(&self) -> F0WM_R {
        F0WM_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - FIFO 0 Operation Mode"]
    #[inline(always)]
    pub fn f0om(&self) -> F0OM_R {
        F0OM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Rx FIFO 0 Start Address"]
    #[inline(always)]
    #[must_use]
    pub fn f0sa(&mut self) -> F0SA_W<RXF0C_SPEC, 0> {
        F0SA_W::new(self)
    }
    #[doc = "Bits 16:22 - Rx FIFO 0 Size"]
    #[inline(always)]
    #[must_use]
    pub fn f0s(&mut self) -> F0S_W<RXF0C_SPEC, 16> {
        F0S_W::new(self)
    }
    #[doc = "Bits 24:30 - Rx FIFO 0 Watermark"]
    #[inline(always)]
    #[must_use]
    pub fn f0wm(&mut self) -> F0WM_W<RXF0C_SPEC, 24> {
        F0WM_W::new(self)
    }
    #[doc = "Bit 31 - FIFO 0 Operation Mode"]
    #[inline(always)]
    #[must_use]
    pub fn f0om(&mut self) -> F0OM_W<RXF0C_SPEC, 31> {
        F0OM_W::new(self)
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
#[doc = "Rx FIFO 0 Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxf0c::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxf0c::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXF0C_SPEC;
impl crate::RegisterSpec for RXF0C_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxf0c::R`](R) reader structure"]
impl crate::Readable for RXF0C_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxf0c::W`](W) writer structure"]
impl crate::Writable for RXF0C_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXF0C to value 0"]
impl crate::Resettable for RXF0C_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
