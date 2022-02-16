#[doc = "Register `HSMCI_CFG` reader"]
pub struct R(crate::R<HSMCI_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSMCI_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSMCI_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSMCI_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSMCI_CFG` writer"]
pub struct W(crate::W<HSMCI_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSMCI_CFG_SPEC>;
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
impl From<crate::W<HSMCI_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSMCI_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIFOMODE` reader - HSMCI Internal FIFO control mode"]
pub struct FIFOMODE_R(crate::FieldReader<bool, bool>);
impl FIFOMODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FIFOMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFOMODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFOMODE` writer - HSMCI Internal FIFO control mode"]
pub struct FIFOMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOMODE_W<'a> {
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
#[doc = "Field `FERRCTRL` reader - Flow Error flag reset control mode"]
pub struct FERRCTRL_R(crate::FieldReader<bool, bool>);
impl FERRCTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FERRCTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FERRCTRL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FERRCTRL` writer - Flow Error flag reset control mode"]
pub struct FERRCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> FERRCTRL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `HSMODE` reader - High Speed Mode"]
pub struct HSMODE_R(crate::FieldReader<bool, bool>);
impl HSMODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HSMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSMODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSMODE` writer - High Speed Mode"]
pub struct HSMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> HSMODE_W<'a> {
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
#[doc = "Field `LSYNC` reader - Synchronize on the last block"]
pub struct LSYNC_R(crate::FieldReader<bool, bool>);
impl LSYNC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LSYNC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSYNC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSYNC` writer - Synchronize on the last block"]
pub struct LSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> LSYNC_W<'a> {
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
impl R {
    #[doc = "Bit 0 - HSMCI Internal FIFO control mode"]
    #[inline(always)]
    pub fn fifomode(&self) -> FIFOMODE_R {
        FIFOMODE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - Flow Error flag reset control mode"]
    #[inline(always)]
    pub fn ferrctrl(&self) -> FERRCTRL_R {
        FERRCTRL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - High Speed Mode"]
    #[inline(always)]
    pub fn hsmode(&self) -> HSMODE_R {
        HSMODE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Synchronize on the last block"]
    #[inline(always)]
    pub fn lsync(&self) -> LSYNC_R {
        LSYNC_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HSMCI Internal FIFO control mode"]
    #[inline(always)]
    pub fn fifomode(&mut self) -> FIFOMODE_W {
        FIFOMODE_W { w: self }
    }
    #[doc = "Bit 4 - Flow Error flag reset control mode"]
    #[inline(always)]
    pub fn ferrctrl(&mut self) -> FERRCTRL_W {
        FERRCTRL_W { w: self }
    }
    #[doc = "Bit 8 - High Speed Mode"]
    #[inline(always)]
    pub fn hsmode(&mut self) -> HSMODE_W {
        HSMODE_W { w: self }
    }
    #[doc = "Bit 12 - Synchronize on the last block"]
    #[inline(always)]
    pub fn lsync(&mut self) -> LSYNC_W {
        LSYNC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsmci_cfg](index.html) module"]
pub struct HSMCI_CFG_SPEC;
impl crate::RegisterSpec for HSMCI_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hsmci_cfg::R](R) reader structure"]
impl crate::Readable for HSMCI_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hsmci_cfg::W](W) writer structure"]
impl crate::Writable for HSMCI_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HSMCI_CFG to value 0"]
impl crate::Resettable for HSMCI_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
