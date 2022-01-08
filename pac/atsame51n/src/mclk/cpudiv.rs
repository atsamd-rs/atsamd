#[doc = "Register `CPUDIV` reader"]
pub struct R(crate::R<CPUDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPUDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPUDIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPUDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPUDIV` writer"]
pub struct W(crate::W<CPUDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPUDIV_SPEC>;
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
impl From<crate::W<CPUDIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPUDIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Low-Power Clock Division Factor\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DIV_A {
    #[doc = "1: Divide by 1"]
    DIV1 = 1,
    #[doc = "2: Divide by 2"]
    DIV2 = 2,
    #[doc = "4: Divide by 4"]
    DIV4 = 4,
    #[doc = "8: Divide by 8"]
    DIV8 = 8,
    #[doc = "16: Divide by 16"]
    DIV16 = 16,
    #[doc = "32: Divide by 32"]
    DIV32 = 32,
    #[doc = "64: Divide by 64"]
    DIV64 = 64,
    #[doc = "128: Divide by 128"]
    DIV128 = 128,
}
impl From<DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: DIV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DIV` reader - Low-Power Clock Division Factor"]
pub struct DIV_R(crate::FieldReader<u8, DIV_A>);
impl DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DIV_A> {
        match self.bits {
            1 => Some(DIV_A::DIV1),
            2 => Some(DIV_A::DIV2),
            4 => Some(DIV_A::DIV4),
            8 => Some(DIV_A::DIV8),
            16 => Some(DIV_A::DIV16),
            32 => Some(DIV_A::DIV32),
            64 => Some(DIV_A::DIV64),
            128 => Some(DIV_A::DIV128),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        **self == DIV_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        **self == DIV_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        **self == DIV_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        **self == DIV_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        **self == DIV_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        **self == DIV_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        **self == DIV_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        **self == DIV_A::DIV128
    }
}
impl core::ops::Deref for DIV_R {
    type Target = crate::FieldReader<u8, DIV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIV` writer - Low-Power Clock Division Factor"]
pub struct DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(DIV_A::DIV1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(DIV_A::DIV2)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(DIV_A::DIV4)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(DIV_A::DIV8)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(DIV_A::DIV16)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(DIV_A::DIV32)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(DIV_A::DIV64)
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(DIV_A::DIV128)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value as u8;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Low-Power Clock Division Factor"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new(self.bits as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Low-Power Clock Division Factor"]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W {
        DIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU Clock Division\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpudiv](index.html) module"]
pub struct CPUDIV_SPEC;
impl crate::RegisterSpec for CPUDIV_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cpudiv::R](R) reader structure"]
impl crate::Readable for CPUDIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpudiv::W](W) writer structure"]
impl crate::Writable for CPUDIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CPUDIV to value 0x01"]
impl crate::Resettable for CPUDIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
