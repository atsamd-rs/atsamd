#[doc = "Register `MCAN_TSCC` reader"]
pub struct R(crate::R<MCAN_TSCC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCAN_TSCC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCAN_TSCC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCAN_TSCC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCAN_TSCC` writer"]
pub struct W(crate::W<MCAN_TSCC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCAN_TSCC_SPEC>;
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
impl From<crate::W<MCAN_TSCC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCAN_TSCC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Timestamp Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TSS_A {
    #[doc = "0: Timestamp counter value always 0x0000"]
    ALWAYS_0 = 0,
    #[doc = "1: Timestamp counter value incremented according to TCP"]
    TCP_INC = 1,
    #[doc = "2: External timestamp counter value used"]
    EXT_TIMESTAMP = 2,
}
impl From<TSS_A> for u8 {
    #[inline(always)]
    fn from(variant: TSS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TSS` reader - Timestamp Select"]
pub struct TSS_R(crate::FieldReader<u8, TSS_A>);
impl TSS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TSS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TSS_A> {
        match self.bits {
            0 => Some(TSS_A::ALWAYS_0),
            1 => Some(TSS_A::TCP_INC),
            2 => Some(TSS_A::EXT_TIMESTAMP),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ALWAYS_0`"]
    #[inline(always)]
    pub fn is_always_0(&self) -> bool {
        **self == TSS_A::ALWAYS_0
    }
    #[doc = "Checks if the value of the field is `TCP_INC`"]
    #[inline(always)]
    pub fn is_tcp_inc(&self) -> bool {
        **self == TSS_A::TCP_INC
    }
    #[doc = "Checks if the value of the field is `EXT_TIMESTAMP`"]
    #[inline(always)]
    pub fn is_ext_timestamp(&self) -> bool {
        **self == TSS_A::EXT_TIMESTAMP
    }
}
impl core::ops::Deref for TSS_R {
    type Target = crate::FieldReader<u8, TSS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSS` writer - Timestamp Select"]
pub struct TSS_W<'a> {
    w: &'a mut W,
}
impl<'a> TSS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Timestamp counter value always 0x0000"]
    #[inline(always)]
    pub fn always_0(self) -> &'a mut W {
        self.variant(TSS_A::ALWAYS_0)
    }
    #[doc = "Timestamp counter value incremented according to TCP"]
    #[inline(always)]
    pub fn tcp_inc(self) -> &'a mut W {
        self.variant(TSS_A::TCP_INC)
    }
    #[doc = "External timestamp counter value used"]
    #[inline(always)]
    pub fn ext_timestamp(self) -> &'a mut W {
        self.variant(TSS_A::EXT_TIMESTAMP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `TCP` reader - Timestamp Counter Prescaler"]
pub struct TCP_R(crate::FieldReader<u8, u8>);
impl TCP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TCP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCP` writer - Timestamp Counter Prescaler"]
pub struct TCP_W<'a> {
    w: &'a mut W,
}
impl<'a> TCP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Timestamp Select"]
    #[inline(always)]
    pub fn tss(&self) -> TSS_R {
        TSS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 16:19 - Timestamp Counter Prescaler"]
    #[inline(always)]
    pub fn tcp(&self) -> TCP_R {
        TCP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Timestamp Select"]
    #[inline(always)]
    pub fn tss(&mut self) -> TSS_W {
        TSS_W { w: self }
    }
    #[doc = "Bits 16:19 - Timestamp Counter Prescaler"]
    #[inline(always)]
    pub fn tcp(&mut self) -> TCP_W {
        TCP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timestamp Counter Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_tscc](index.html) module"]
pub struct MCAN_TSCC_SPEC;
impl crate::RegisterSpec for MCAN_TSCC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcan_tscc::R](R) reader structure"]
impl crate::Readable for MCAN_TSCC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcan_tscc::W](W) writer structure"]
impl crate::Writable for MCAN_TSCC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCAN_TSCC to value 0"]
impl crate::Resettable for MCAN_TSCC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
