#[doc = "Register `PMC_PCER1` writer"]
pub struct W(crate::W<PMC_PCER1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMC_PCER1_SPEC>;
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
impl From<crate::W<PMC_PCER1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMC_PCER1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PID32` writer - Peripheral Clock 32 Enable"]
pub struct PID32_W<'a> {
    w: &'a mut W,
}
impl<'a> PID32_W<'a> {
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
#[doc = "Field `PID33` writer - Peripheral Clock 33 Enable"]
pub struct PID33_W<'a> {
    w: &'a mut W,
}
impl<'a> PID33_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `PID34` writer - Peripheral Clock 34 Enable"]
pub struct PID34_W<'a> {
    w: &'a mut W,
}
impl<'a> PID34_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `PID35` writer - Peripheral Clock 35 Enable"]
pub struct PID35_W<'a> {
    w: &'a mut W,
}
impl<'a> PID35_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `PID37` writer - Peripheral Clock 37 Enable"]
pub struct PID37_W<'a> {
    w: &'a mut W,
}
impl<'a> PID37_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `PID39` writer - Peripheral Clock 39 Enable"]
pub struct PID39_W<'a> {
    w: &'a mut W,
}
impl<'a> PID39_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `PID40` writer - Peripheral Clock 40 Enable"]
pub struct PID40_W<'a> {
    w: &'a mut W,
}
impl<'a> PID40_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `PID41` writer - Peripheral Clock 41 Enable"]
pub struct PID41_W<'a> {
    w: &'a mut W,
}
impl<'a> PID41_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `PID43` writer - Peripheral Clock 43 Enable"]
pub struct PID43_W<'a> {
    w: &'a mut W,
}
impl<'a> PID43_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `PID44` writer - Peripheral Clock 44 Enable"]
pub struct PID44_W<'a> {
    w: &'a mut W,
}
impl<'a> PID44_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `PID45` writer - Peripheral Clock 45 Enable"]
pub struct PID45_W<'a> {
    w: &'a mut W,
}
impl<'a> PID45_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `PID46` writer - Peripheral Clock 46 Enable"]
pub struct PID46_W<'a> {
    w: &'a mut W,
}
impl<'a> PID46_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `PID47` writer - Peripheral Clock 47 Enable"]
pub struct PID47_W<'a> {
    w: &'a mut W,
}
impl<'a> PID47_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `PID48` writer - Peripheral Clock 48 Enable"]
pub struct PID48_W<'a> {
    w: &'a mut W,
}
impl<'a> PID48_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `PID49` writer - Peripheral Clock 49 Enable"]
pub struct PID49_W<'a> {
    w: &'a mut W,
}
impl<'a> PID49_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `PID50` writer - Peripheral Clock 50 Enable"]
pub struct PID50_W<'a> {
    w: &'a mut W,
}
impl<'a> PID50_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `PID51` writer - Peripheral Clock 51 Enable"]
pub struct PID51_W<'a> {
    w: &'a mut W,
}
impl<'a> PID51_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `PID52` writer - Peripheral Clock 52 Enable"]
pub struct PID52_W<'a> {
    w: &'a mut W,
}
impl<'a> PID52_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `PID56` writer - Peripheral Clock 56 Enable"]
pub struct PID56_W<'a> {
    w: &'a mut W,
}
impl<'a> PID56_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `PID57` writer - Peripheral Clock 57 Enable"]
pub struct PID57_W<'a> {
    w: &'a mut W,
}
impl<'a> PID57_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `PID58` writer - Peripheral Clock 58 Enable"]
pub struct PID58_W<'a> {
    w: &'a mut W,
}
impl<'a> PID58_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `PID59` writer - Peripheral Clock 59 Enable"]
pub struct PID59_W<'a> {
    w: &'a mut W,
}
impl<'a> PID59_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `PID60` writer - Peripheral Clock 60 Enable"]
pub struct PID60_W<'a> {
    w: &'a mut W,
}
impl<'a> PID60_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Peripheral Clock 32 Enable"]
    #[inline(always)]
    pub fn pid32(&mut self) -> PID32_W {
        PID32_W { w: self }
    }
    #[doc = "Bit 1 - Peripheral Clock 33 Enable"]
    #[inline(always)]
    pub fn pid33(&mut self) -> PID33_W {
        PID33_W { w: self }
    }
    #[doc = "Bit 2 - Peripheral Clock 34 Enable"]
    #[inline(always)]
    pub fn pid34(&mut self) -> PID34_W {
        PID34_W { w: self }
    }
    #[doc = "Bit 3 - Peripheral Clock 35 Enable"]
    #[inline(always)]
    pub fn pid35(&mut self) -> PID35_W {
        PID35_W { w: self }
    }
    #[doc = "Bit 5 - Peripheral Clock 37 Enable"]
    #[inline(always)]
    pub fn pid37(&mut self) -> PID37_W {
        PID37_W { w: self }
    }
    #[doc = "Bit 7 - Peripheral Clock 39 Enable"]
    #[inline(always)]
    pub fn pid39(&mut self) -> PID39_W {
        PID39_W { w: self }
    }
    #[doc = "Bit 8 - Peripheral Clock 40 Enable"]
    #[inline(always)]
    pub fn pid40(&mut self) -> PID40_W {
        PID40_W { w: self }
    }
    #[doc = "Bit 9 - Peripheral Clock 41 Enable"]
    #[inline(always)]
    pub fn pid41(&mut self) -> PID41_W {
        PID41_W { w: self }
    }
    #[doc = "Bit 11 - Peripheral Clock 43 Enable"]
    #[inline(always)]
    pub fn pid43(&mut self) -> PID43_W {
        PID43_W { w: self }
    }
    #[doc = "Bit 12 - Peripheral Clock 44 Enable"]
    #[inline(always)]
    pub fn pid44(&mut self) -> PID44_W {
        PID44_W { w: self }
    }
    #[doc = "Bit 13 - Peripheral Clock 45 Enable"]
    #[inline(always)]
    pub fn pid45(&mut self) -> PID45_W {
        PID45_W { w: self }
    }
    #[doc = "Bit 14 - Peripheral Clock 46 Enable"]
    #[inline(always)]
    pub fn pid46(&mut self) -> PID46_W {
        PID46_W { w: self }
    }
    #[doc = "Bit 15 - Peripheral Clock 47 Enable"]
    #[inline(always)]
    pub fn pid47(&mut self) -> PID47_W {
        PID47_W { w: self }
    }
    #[doc = "Bit 16 - Peripheral Clock 48 Enable"]
    #[inline(always)]
    pub fn pid48(&mut self) -> PID48_W {
        PID48_W { w: self }
    }
    #[doc = "Bit 17 - Peripheral Clock 49 Enable"]
    #[inline(always)]
    pub fn pid49(&mut self) -> PID49_W {
        PID49_W { w: self }
    }
    #[doc = "Bit 18 - Peripheral Clock 50 Enable"]
    #[inline(always)]
    pub fn pid50(&mut self) -> PID50_W {
        PID50_W { w: self }
    }
    #[doc = "Bit 19 - Peripheral Clock 51 Enable"]
    #[inline(always)]
    pub fn pid51(&mut self) -> PID51_W {
        PID51_W { w: self }
    }
    #[doc = "Bit 20 - Peripheral Clock 52 Enable"]
    #[inline(always)]
    pub fn pid52(&mut self) -> PID52_W {
        PID52_W { w: self }
    }
    #[doc = "Bit 24 - Peripheral Clock 56 Enable"]
    #[inline(always)]
    pub fn pid56(&mut self) -> PID56_W {
        PID56_W { w: self }
    }
    #[doc = "Bit 25 - Peripheral Clock 57 Enable"]
    #[inline(always)]
    pub fn pid57(&mut self) -> PID57_W {
        PID57_W { w: self }
    }
    #[doc = "Bit 26 - Peripheral Clock 58 Enable"]
    #[inline(always)]
    pub fn pid58(&mut self) -> PID58_W {
        PID58_W { w: self }
    }
    #[doc = "Bit 27 - Peripheral Clock 59 Enable"]
    #[inline(always)]
    pub fn pid59(&mut self) -> PID59_W {
        PID59_W { w: self }
    }
    #[doc = "Bit 28 - Peripheral Clock 60 Enable"]
    #[inline(always)]
    pub fn pid60(&mut self) -> PID60_W {
        PID60_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral Clock Enable Register 1\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_pcer1](index.html) module"]
pub struct PMC_PCER1_SPEC;
impl crate::RegisterSpec for PMC_PCER1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pmc_pcer1::W](W) writer structure"]
impl crate::Writable for PMC_PCER1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMC_PCER1 to value 0"]
impl crate::Resettable for PMC_PCER1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
