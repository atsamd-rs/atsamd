#[doc = "Register `GMAC_ST2RPQ[%s]` reader"]
pub struct R(crate::R<GMAC_ST2RPQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GMAC_ST2RPQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GMAC_ST2RPQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GMAC_ST2RPQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GMAC_ST2RPQ[%s]` writer"]
pub struct W(crate::W<GMAC_ST2RPQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GMAC_ST2RPQ_SPEC>;
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
impl From<crate::W<GMAC_ST2RPQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GMAC_ST2RPQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `QNB` reader - Queue Number (0-5)"]
pub struct QNB_R(crate::FieldReader<u8, u8>);
impl QNB_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        QNB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for QNB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QNB` writer - Queue Number (0-5)"]
pub struct QNB_W<'a> {
    w: &'a mut W,
}
impl<'a> QNB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `VLANP` reader - VLAN Priority"]
pub struct VLANP_R(crate::FieldReader<u8, u8>);
impl VLANP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        VLANP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VLANP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VLANP` writer - VLAN Priority"]
pub struct VLANP_W<'a> {
    w: &'a mut W,
}
impl<'a> VLANP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `VLANE` reader - VLAN Enable"]
pub struct VLANE_R(crate::FieldReader<bool, bool>);
impl VLANE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VLANE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VLANE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VLANE` writer - VLAN Enable"]
pub struct VLANE_W<'a> {
    w: &'a mut W,
}
impl<'a> VLANE_W<'a> {
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
#[doc = "Field `I2ETH` reader - Index of Screening Type 2 EtherType register x"]
pub struct I2ETH_R(crate::FieldReader<u8, u8>);
impl I2ETH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        I2ETH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2ETH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2ETH` writer - Index of Screening Type 2 EtherType register x"]
pub struct I2ETH_W<'a> {
    w: &'a mut W,
}
impl<'a> I2ETH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | ((value as u32 & 0x07) << 9);
        self.w
    }
}
#[doc = "Field `ETHE` reader - EtherType Enable"]
pub struct ETHE_R(crate::FieldReader<bool, bool>);
impl ETHE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ETHE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETHE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETHE` writer - EtherType Enable"]
pub struct ETHE_W<'a> {
    w: &'a mut W,
}
impl<'a> ETHE_W<'a> {
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
#[doc = "Field `COMPA` reader - Index of Screening Type 2 Compare Word 0/Word 1 register x"]
pub struct COMPA_R(crate::FieldReader<u8, u8>);
impl COMPA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        COMPA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMPA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPA` writer - Index of Screening Type 2 Compare Word 0/Word 1 register x"]
pub struct COMPA_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 13)) | ((value as u32 & 0x1f) << 13);
        self.w
    }
}
#[doc = "Field `COMPAE` reader - Compare A Enable"]
pub struct COMPAE_R(crate::FieldReader<bool, bool>);
impl COMPAE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMPAE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMPAE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPAE` writer - Compare A Enable"]
pub struct COMPAE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPAE_W<'a> {
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
#[doc = "Field `COMPB` reader - Index of Screening Type 2 Compare Word 0/Word 1 register x"]
pub struct COMPB_R(crate::FieldReader<u8, u8>);
impl COMPB_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        COMPB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMPB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPB` writer - Index of Screening Type 2 Compare Word 0/Word 1 register x"]
pub struct COMPB_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 19)) | ((value as u32 & 0x1f) << 19);
        self.w
    }
}
#[doc = "Field `COMPBE` reader - Compare B Enable"]
pub struct COMPBE_R(crate::FieldReader<bool, bool>);
impl COMPBE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMPBE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMPBE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPBE` writer - Compare B Enable"]
pub struct COMPBE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPBE_W<'a> {
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
#[doc = "Field `COMPC` reader - Index of Screening Type 2 Compare Word 0/Word 1 register x"]
pub struct COMPC_R(crate::FieldReader<u8, u8>);
impl COMPC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        COMPC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMPC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPC` writer - Index of Screening Type 2 Compare Word 0/Word 1 register x"]
pub struct COMPC_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 25)) | ((value as u32 & 0x1f) << 25);
        self.w
    }
}
#[doc = "Field `COMPCE` reader - Compare C Enable"]
pub struct COMPCE_R(crate::FieldReader<bool, bool>);
impl COMPCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMPCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMPCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPCE` writer - Compare C Enable"]
pub struct COMPCE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Queue Number (0-5)"]
    #[inline(always)]
    pub fn qnb(&self) -> QNB_R {
        QNB_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - VLAN Priority"]
    #[inline(always)]
    pub fn vlanp(&self) -> VLANP_R {
        VLANP_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 8 - VLAN Enable"]
    #[inline(always)]
    pub fn vlane(&self) -> VLANE_R {
        VLANE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:11 - Index of Screening Type 2 EtherType register x"]
    #[inline(always)]
    pub fn i2eth(&self) -> I2ETH_R {
        I2ETH_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bit 12 - EtherType Enable"]
    #[inline(always)]
    pub fn ethe(&self) -> ETHE_R {
        ETHE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 13:17 - Index of Screening Type 2 Compare Word 0/Word 1 register x"]
    #[inline(always)]
    pub fn compa(&self) -> COMPA_R {
        COMPA_R::new(((self.bits >> 13) & 0x1f) as u8)
    }
    #[doc = "Bit 18 - Compare A Enable"]
    #[inline(always)]
    pub fn compae(&self) -> COMPAE_R {
        COMPAE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 19:23 - Index of Screening Type 2 Compare Word 0/Word 1 register x"]
    #[inline(always)]
    pub fn compb(&self) -> COMPB_R {
        COMPB_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - Compare B Enable"]
    #[inline(always)]
    pub fn compbe(&self) -> COMPBE_R {
        COMPBE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 25:29 - Index of Screening Type 2 Compare Word 0/Word 1 register x"]
    #[inline(always)]
    pub fn compc(&self) -> COMPC_R {
        COMPC_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - Compare C Enable"]
    #[inline(always)]
    pub fn compce(&self) -> COMPCE_R {
        COMPCE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Queue Number (0-5)"]
    #[inline(always)]
    pub fn qnb(&mut self) -> QNB_W {
        QNB_W { w: self }
    }
    #[doc = "Bits 4:6 - VLAN Priority"]
    #[inline(always)]
    pub fn vlanp(&mut self) -> VLANP_W {
        VLANP_W { w: self }
    }
    #[doc = "Bit 8 - VLAN Enable"]
    #[inline(always)]
    pub fn vlane(&mut self) -> VLANE_W {
        VLANE_W { w: self }
    }
    #[doc = "Bits 9:11 - Index of Screening Type 2 EtherType register x"]
    #[inline(always)]
    pub fn i2eth(&mut self) -> I2ETH_W {
        I2ETH_W { w: self }
    }
    #[doc = "Bit 12 - EtherType Enable"]
    #[inline(always)]
    pub fn ethe(&mut self) -> ETHE_W {
        ETHE_W { w: self }
    }
    #[doc = "Bits 13:17 - Index of Screening Type 2 Compare Word 0/Word 1 register x"]
    #[inline(always)]
    pub fn compa(&mut self) -> COMPA_W {
        COMPA_W { w: self }
    }
    #[doc = "Bit 18 - Compare A Enable"]
    #[inline(always)]
    pub fn compae(&mut self) -> COMPAE_W {
        COMPAE_W { w: self }
    }
    #[doc = "Bits 19:23 - Index of Screening Type 2 Compare Word 0/Word 1 register x"]
    #[inline(always)]
    pub fn compb(&mut self) -> COMPB_W {
        COMPB_W { w: self }
    }
    #[doc = "Bit 24 - Compare B Enable"]
    #[inline(always)]
    pub fn compbe(&mut self) -> COMPBE_W {
        COMPBE_W { w: self }
    }
    #[doc = "Bits 25:29 - Index of Screening Type 2 Compare Word 0/Word 1 register x"]
    #[inline(always)]
    pub fn compc(&mut self) -> COMPC_W {
        COMPC_W { w: self }
    }
    #[doc = "Bit 30 - Compare C Enable"]
    #[inline(always)]
    pub fn compce(&mut self) -> COMPCE_W {
        COMPCE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Screening Type 2 Register Priority Queue\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_st2rpq](index.html) module"]
pub struct GMAC_ST2RPQ_SPEC;
impl crate::RegisterSpec for GMAC_ST2RPQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gmac_st2rpq::R](R) reader structure"]
impl crate::Readable for GMAC_ST2RPQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gmac_st2rpq::W](W) writer structure"]
impl crate::Writable for GMAC_ST2RPQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GMAC_ST2RPQ[%s]
to value 0"]
impl crate::Resettable for GMAC_ST2RPQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
