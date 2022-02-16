#[doc = "Register `GMAC_ST2CW1` reader"]
pub struct R(crate::R<GMAC_ST2CW1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GMAC_ST2CW1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GMAC_ST2CW1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GMAC_ST2CW1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GMAC_ST2CW1` writer"]
pub struct W(crate::W<GMAC_ST2CW1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GMAC_ST2CW1_SPEC>;
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
impl From<crate::W<GMAC_ST2CW1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GMAC_ST2CW1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OFFSVAL` reader - Offset Value in Bytes"]
pub struct OFFSVAL_R(crate::FieldReader<u8, u8>);
impl OFFSVAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OFFSVAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFFSVAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFFSVAL` writer - Offset Value in Bytes"]
pub struct OFFSVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
#[doc = "Ethernet Frame Offset Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OFFSSTRT_A {
    #[doc = "0: Offset from the start of the frame"]
    FRAMESTART = 0,
    #[doc = "1: Offset from the byte after the EtherType field"]
    ETHERTYPE = 1,
    #[doc = "2: Offset from the byte after the IP header field"]
    IP = 2,
    #[doc = "3: Offset from the byte after the TCP/UDP header field"]
    TCP_UDP = 3,
}
impl From<OFFSSTRT_A> for u8 {
    #[inline(always)]
    fn from(variant: OFFSSTRT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `OFFSSTRT` reader - Ethernet Frame Offset Start"]
pub struct OFFSSTRT_R(crate::FieldReader<u8, OFFSSTRT_A>);
impl OFFSSTRT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OFFSSTRT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFFSSTRT_A {
        match self.bits {
            0 => OFFSSTRT_A::FRAMESTART,
            1 => OFFSSTRT_A::ETHERTYPE,
            2 => OFFSSTRT_A::IP,
            3 => OFFSSTRT_A::TCP_UDP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FRAMESTART`"]
    #[inline(always)]
    pub fn is_framestart(&self) -> bool {
        **self == OFFSSTRT_A::FRAMESTART
    }
    #[doc = "Checks if the value of the field is `ETHERTYPE`"]
    #[inline(always)]
    pub fn is_ethertype(&self) -> bool {
        **self == OFFSSTRT_A::ETHERTYPE
    }
    #[doc = "Checks if the value of the field is `IP`"]
    #[inline(always)]
    pub fn is_ip(&self) -> bool {
        **self == OFFSSTRT_A::IP
    }
    #[doc = "Checks if the value of the field is `TCP_UDP`"]
    #[inline(always)]
    pub fn is_tcp_udp(&self) -> bool {
        **self == OFFSSTRT_A::TCP_UDP
    }
}
impl core::ops::Deref for OFFSSTRT_R {
    type Target = crate::FieldReader<u8, OFFSSTRT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFFSSTRT` writer - Ethernet Frame Offset Start"]
pub struct OFFSSTRT_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSSTRT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OFFSSTRT_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Offset from the start of the frame"]
    #[inline(always)]
    pub fn framestart(self) -> &'a mut W {
        self.variant(OFFSSTRT_A::FRAMESTART)
    }
    #[doc = "Offset from the byte after the EtherType field"]
    #[inline(always)]
    pub fn ethertype(self) -> &'a mut W {
        self.variant(OFFSSTRT_A::ETHERTYPE)
    }
    #[doc = "Offset from the byte after the IP header field"]
    #[inline(always)]
    pub fn ip(self) -> &'a mut W {
        self.variant(OFFSSTRT_A::IP)
    }
    #[doc = "Offset from the byte after the TCP/UDP header field"]
    #[inline(always)]
    pub fn tcp_udp(self) -> &'a mut W {
        self.variant(OFFSSTRT_A::TCP_UDP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 7)) | ((value as u32 & 0x03) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Offset Value in Bytes"]
    #[inline(always)]
    pub fn offsval(&self) -> OFFSVAL_R {
        OFFSVAL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:8 - Ethernet Frame Offset Start"]
    #[inline(always)]
    pub fn offsstrt(&self) -> OFFSSTRT_R {
        OFFSSTRT_R::new(((self.bits >> 7) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Offset Value in Bytes"]
    #[inline(always)]
    pub fn offsval(&mut self) -> OFFSVAL_W {
        OFFSVAL_W { w: self }
    }
    #[doc = "Bits 7:8 - Ethernet Frame Offset Start"]
    #[inline(always)]
    pub fn offsstrt(&mut self) -> OFFSSTRT_W {
        OFFSSTRT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Screening Type 2 Compare Word 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_st2cw1](index.html) module"]
pub struct GMAC_ST2CW1_SPEC;
impl crate::RegisterSpec for GMAC_ST2CW1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gmac_st2cw1::R](R) reader structure"]
impl crate::Readable for GMAC_ST2CW1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gmac_st2cw1::W](W) writer structure"]
impl crate::Writable for GMAC_ST2CW1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GMAC_ST2CW1 to value 0"]
impl crate::Resettable for GMAC_ST2CW1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
