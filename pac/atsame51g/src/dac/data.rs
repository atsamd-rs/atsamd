#[doc = "Register `DATA[%s]` writer"]
pub struct W(crate::W<DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA_SPEC>;
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
impl From<crate::W<DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` writer - DAC0 Data"]
pub struct DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = value as u16;
        self.w
    }
}
impl W {
    #[doc = "Bits 0:15 - DAC0 Data"]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC n Data\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data](index.html) module"]
pub struct DATA_SPEC;
impl crate::RegisterSpec for DATA_SPEC {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [data::W](W) writer structure"]
impl crate::Writable for DATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATA[%s]
to value 0"]
impl crate::Resettable for DATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
