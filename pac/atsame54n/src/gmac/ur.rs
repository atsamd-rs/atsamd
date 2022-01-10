#[doc = "Register `UR` reader"]
pub struct R(crate::R<UR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UR` writer"]
pub struct W(crate::W<UR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UR_SPEC>;
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
impl From<crate::W<UR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MII` reader - MII Mode"]
pub struct MII_R(crate::FieldReader<bool, bool>);
impl MII_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MII_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MII_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MII` writer - MII Mode"]
pub struct MII_W<'a> {
    w: &'a mut W,
}
impl<'a> MII_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - MII Mode"]
    #[inline(always)]
    pub fn mii(&self) -> MII_R {
        MII_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MII Mode"]
    #[inline(always)]
    pub fn mii(&mut self) -> MII_W {
        MII_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "User Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ur](index.html) module"]
pub struct UR_SPEC;
impl crate::RegisterSpec for UR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ur::R](R) reader structure"]
impl crate::Readable for UR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ur::W](W) writer structure"]
impl crate::Writable for UR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UR to value 0"]
impl crate::Resettable for UR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
