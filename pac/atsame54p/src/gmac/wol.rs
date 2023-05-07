#[doc = "Register `WOL` reader"]
pub struct R(crate::R<WOL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WOL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WOL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WOL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WOL` writer"]
pub struct W(crate::W<WOL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WOL_SPEC>;
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
impl From<crate::W<WOL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WOL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IP` reader - IP address"]
pub type IP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `IP` writer - IP address"]
pub type IP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WOL_SPEC, u16, u16, 16, O>;
#[doc = "Field `MAG` reader - Event enable"]
pub type MAG_R = crate::BitReader<bool>;
#[doc = "Field `MAG` writer - Event enable"]
pub type MAG_W<'a, const O: u8> = crate::BitWriter<'a, u32, WOL_SPEC, bool, O>;
#[doc = "Field `ARP` reader - LAN ARP req"]
pub type ARP_R = crate::BitReader<bool>;
#[doc = "Field `ARP` writer - LAN ARP req"]
pub type ARP_W<'a, const O: u8> = crate::BitWriter<'a, u32, WOL_SPEC, bool, O>;
#[doc = "Field `SA1` reader - WOL specific address reg 1"]
pub type SA1_R = crate::BitReader<bool>;
#[doc = "Field `SA1` writer - WOL specific address reg 1"]
pub type SA1_W<'a, const O: u8> = crate::BitWriter<'a, u32, WOL_SPEC, bool, O>;
#[doc = "Field `MTI` reader - WOL LAN multicast"]
pub type MTI_R = crate::BitReader<bool>;
#[doc = "Field `MTI` writer - WOL LAN multicast"]
pub type MTI_W<'a, const O: u8> = crate::BitWriter<'a, u32, WOL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - IP address"]
    #[inline(always)]
    pub fn ip(&self) -> IP_R {
        IP_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Event enable"]
    #[inline(always)]
    pub fn mag(&self) -> MAG_R {
        MAG_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - LAN ARP req"]
    #[inline(always)]
    pub fn arp(&self) -> ARP_R {
        ARP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - WOL specific address reg 1"]
    #[inline(always)]
    pub fn sa1(&self) -> SA1_R {
        SA1_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - WOL LAN multicast"]
    #[inline(always)]
    pub fn mti(&self) -> MTI_R {
        MTI_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - IP address"]
    #[inline(always)]
    #[must_use]
    pub fn ip(&mut self) -> IP_W<0> {
        IP_W::new(self)
    }
    #[doc = "Bit 16 - Event enable"]
    #[inline(always)]
    #[must_use]
    pub fn mag(&mut self) -> MAG_W<16> {
        MAG_W::new(self)
    }
    #[doc = "Bit 17 - LAN ARP req"]
    #[inline(always)]
    #[must_use]
    pub fn arp(&mut self) -> ARP_W<17> {
        ARP_W::new(self)
    }
    #[doc = "Bit 18 - WOL specific address reg 1"]
    #[inline(always)]
    #[must_use]
    pub fn sa1(&mut self) -> SA1_W<18> {
        SA1_W::new(self)
    }
    #[doc = "Bit 19 - WOL LAN multicast"]
    #[inline(always)]
    #[must_use]
    pub fn mti(&mut self) -> MTI_W<19> {
        MTI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Wake on LAN\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wol](index.html) module"]
pub struct WOL_SPEC;
impl crate::RegisterSpec for WOL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wol::R](R) reader structure"]
impl crate::Readable for WOL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wol::W](W) writer structure"]
impl crate::Writable for WOL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WOL to value 0"]
impl crate::Resettable for WOL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
