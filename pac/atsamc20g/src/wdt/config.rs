#[doc = "Register `CONFIG` reader"]
pub struct R(crate::R<CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFIG` writer"]
pub struct W(crate::W<CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFIG_SPEC>;
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
impl From<crate::W<CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Time-Out Period\n\nValue on reset: 11"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PER_A {
    #[doc = "0: 8 clock cycles"]
    CYC8 = 0,
    #[doc = "1: 16 clock cycles"]
    CYC16 = 1,
    #[doc = "2: 32 clock cycles"]
    CYC32 = 2,
    #[doc = "3: 64 clock cycles"]
    CYC64 = 3,
    #[doc = "4: 128 clock cycles"]
    CYC128 = 4,
    #[doc = "5: 256 clock cycles"]
    CYC256 = 5,
    #[doc = "6: 512 clock cycles"]
    CYC512 = 6,
    #[doc = "7: 1024 clock cycles"]
    CYC1024 = 7,
    #[doc = "8: 2048 clock cycles"]
    CYC2048 = 8,
    #[doc = "9: 4096 clock cycles"]
    CYC4096 = 9,
    #[doc = "10: 8192 clock cycles"]
    CYC8192 = 10,
    #[doc = "11: 16384 clock cycles"]
    CYC16384 = 11,
}
impl From<PER_A> for u8 {
    #[inline(always)]
    fn from(variant: PER_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PER` reader - Time-Out Period"]
pub struct PER_R(crate::FieldReader<u8, PER_A>);
impl PER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PER_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PER_A> {
        match self.bits {
            0 => Some(PER_A::CYC8),
            1 => Some(PER_A::CYC16),
            2 => Some(PER_A::CYC32),
            3 => Some(PER_A::CYC64),
            4 => Some(PER_A::CYC128),
            5 => Some(PER_A::CYC256),
            6 => Some(PER_A::CYC512),
            7 => Some(PER_A::CYC1024),
            8 => Some(PER_A::CYC2048),
            9 => Some(PER_A::CYC4096),
            10 => Some(PER_A::CYC8192),
            11 => Some(PER_A::CYC16384),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CYC8`"]
    #[inline(always)]
    pub fn is_cyc8(&self) -> bool {
        **self == PER_A::CYC8
    }
    #[doc = "Checks if the value of the field is `CYC16`"]
    #[inline(always)]
    pub fn is_cyc16(&self) -> bool {
        **self == PER_A::CYC16
    }
    #[doc = "Checks if the value of the field is `CYC32`"]
    #[inline(always)]
    pub fn is_cyc32(&self) -> bool {
        **self == PER_A::CYC32
    }
    #[doc = "Checks if the value of the field is `CYC64`"]
    #[inline(always)]
    pub fn is_cyc64(&self) -> bool {
        **self == PER_A::CYC64
    }
    #[doc = "Checks if the value of the field is `CYC128`"]
    #[inline(always)]
    pub fn is_cyc128(&self) -> bool {
        **self == PER_A::CYC128
    }
    #[doc = "Checks if the value of the field is `CYC256`"]
    #[inline(always)]
    pub fn is_cyc256(&self) -> bool {
        **self == PER_A::CYC256
    }
    #[doc = "Checks if the value of the field is `CYC512`"]
    #[inline(always)]
    pub fn is_cyc512(&self) -> bool {
        **self == PER_A::CYC512
    }
    #[doc = "Checks if the value of the field is `CYC1024`"]
    #[inline(always)]
    pub fn is_cyc1024(&self) -> bool {
        **self == PER_A::CYC1024
    }
    #[doc = "Checks if the value of the field is `CYC2048`"]
    #[inline(always)]
    pub fn is_cyc2048(&self) -> bool {
        **self == PER_A::CYC2048
    }
    #[doc = "Checks if the value of the field is `CYC4096`"]
    #[inline(always)]
    pub fn is_cyc4096(&self) -> bool {
        **self == PER_A::CYC4096
    }
    #[doc = "Checks if the value of the field is `CYC8192`"]
    #[inline(always)]
    pub fn is_cyc8192(&self) -> bool {
        **self == PER_A::CYC8192
    }
    #[doc = "Checks if the value of the field is `CYC16384`"]
    #[inline(always)]
    pub fn is_cyc16384(&self) -> bool {
        **self == PER_A::CYC16384
    }
}
impl core::ops::Deref for PER_R {
    type Target = crate::FieldReader<u8, PER_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PER` writer - Time-Out Period"]
pub struct PER_W<'a> {
    w: &'a mut W,
}
impl<'a> PER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PER_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "8 clock cycles"]
    #[inline(always)]
    pub fn cyc8(self) -> &'a mut W {
        self.variant(PER_A::CYC8)
    }
    #[doc = "16 clock cycles"]
    #[inline(always)]
    pub fn cyc16(self) -> &'a mut W {
        self.variant(PER_A::CYC16)
    }
    #[doc = "32 clock cycles"]
    #[inline(always)]
    pub fn cyc32(self) -> &'a mut W {
        self.variant(PER_A::CYC32)
    }
    #[doc = "64 clock cycles"]
    #[inline(always)]
    pub fn cyc64(self) -> &'a mut W {
        self.variant(PER_A::CYC64)
    }
    #[doc = "128 clock cycles"]
    #[inline(always)]
    pub fn cyc128(self) -> &'a mut W {
        self.variant(PER_A::CYC128)
    }
    #[doc = "256 clock cycles"]
    #[inline(always)]
    pub fn cyc256(self) -> &'a mut W {
        self.variant(PER_A::CYC256)
    }
    #[doc = "512 clock cycles"]
    #[inline(always)]
    pub fn cyc512(self) -> &'a mut W {
        self.variant(PER_A::CYC512)
    }
    #[doc = "1024 clock cycles"]
    #[inline(always)]
    pub fn cyc1024(self) -> &'a mut W {
        self.variant(PER_A::CYC1024)
    }
    #[doc = "2048 clock cycles"]
    #[inline(always)]
    pub fn cyc2048(self) -> &'a mut W {
        self.variant(PER_A::CYC2048)
    }
    #[doc = "4096 clock cycles"]
    #[inline(always)]
    pub fn cyc4096(self) -> &'a mut W {
        self.variant(PER_A::CYC4096)
    }
    #[doc = "8192 clock cycles"]
    #[inline(always)]
    pub fn cyc8192(self) -> &'a mut W {
        self.variant(PER_A::CYC8192)
    }
    #[doc = "16384 clock cycles"]
    #[inline(always)]
    pub fn cyc16384(self) -> &'a mut W {
        self.variant(PER_A::CYC16384)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u8 & 0x0f);
        self.w
    }
}
#[doc = "Window Mode Time-Out Period\n\nValue on reset: 11"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WINDOW_A {
    #[doc = "0: 8 clock cycles"]
    CYC8 = 0,
    #[doc = "1: 16 clock cycles"]
    CYC16 = 1,
    #[doc = "2: 32 clock cycles"]
    CYC32 = 2,
    #[doc = "3: 64 clock cycles"]
    CYC64 = 3,
    #[doc = "4: 128 clock cycles"]
    CYC128 = 4,
    #[doc = "5: 256 clock cycles"]
    CYC256 = 5,
    #[doc = "6: 512 clock cycles"]
    CYC512 = 6,
    #[doc = "7: 1024 clock cycles"]
    CYC1024 = 7,
    #[doc = "8: 2048 clock cycles"]
    CYC2048 = 8,
    #[doc = "9: 4096 clock cycles"]
    CYC4096 = 9,
    #[doc = "10: 8192 clock cycles"]
    CYC8192 = 10,
    #[doc = "11: 16384 clock cycles"]
    CYC16384 = 11,
}
impl From<WINDOW_A> for u8 {
    #[inline(always)]
    fn from(variant: WINDOW_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WINDOW` reader - Window Mode Time-Out Period"]
pub struct WINDOW_R(crate::FieldReader<u8, WINDOW_A>);
impl WINDOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WINDOW_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WINDOW_A> {
        match self.bits {
            0 => Some(WINDOW_A::CYC8),
            1 => Some(WINDOW_A::CYC16),
            2 => Some(WINDOW_A::CYC32),
            3 => Some(WINDOW_A::CYC64),
            4 => Some(WINDOW_A::CYC128),
            5 => Some(WINDOW_A::CYC256),
            6 => Some(WINDOW_A::CYC512),
            7 => Some(WINDOW_A::CYC1024),
            8 => Some(WINDOW_A::CYC2048),
            9 => Some(WINDOW_A::CYC4096),
            10 => Some(WINDOW_A::CYC8192),
            11 => Some(WINDOW_A::CYC16384),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CYC8`"]
    #[inline(always)]
    pub fn is_cyc8(&self) -> bool {
        **self == WINDOW_A::CYC8
    }
    #[doc = "Checks if the value of the field is `CYC16`"]
    #[inline(always)]
    pub fn is_cyc16(&self) -> bool {
        **self == WINDOW_A::CYC16
    }
    #[doc = "Checks if the value of the field is `CYC32`"]
    #[inline(always)]
    pub fn is_cyc32(&self) -> bool {
        **self == WINDOW_A::CYC32
    }
    #[doc = "Checks if the value of the field is `CYC64`"]
    #[inline(always)]
    pub fn is_cyc64(&self) -> bool {
        **self == WINDOW_A::CYC64
    }
    #[doc = "Checks if the value of the field is `CYC128`"]
    #[inline(always)]
    pub fn is_cyc128(&self) -> bool {
        **self == WINDOW_A::CYC128
    }
    #[doc = "Checks if the value of the field is `CYC256`"]
    #[inline(always)]
    pub fn is_cyc256(&self) -> bool {
        **self == WINDOW_A::CYC256
    }
    #[doc = "Checks if the value of the field is `CYC512`"]
    #[inline(always)]
    pub fn is_cyc512(&self) -> bool {
        **self == WINDOW_A::CYC512
    }
    #[doc = "Checks if the value of the field is `CYC1024`"]
    #[inline(always)]
    pub fn is_cyc1024(&self) -> bool {
        **self == WINDOW_A::CYC1024
    }
    #[doc = "Checks if the value of the field is `CYC2048`"]
    #[inline(always)]
    pub fn is_cyc2048(&self) -> bool {
        **self == WINDOW_A::CYC2048
    }
    #[doc = "Checks if the value of the field is `CYC4096`"]
    #[inline(always)]
    pub fn is_cyc4096(&self) -> bool {
        **self == WINDOW_A::CYC4096
    }
    #[doc = "Checks if the value of the field is `CYC8192`"]
    #[inline(always)]
    pub fn is_cyc8192(&self) -> bool {
        **self == WINDOW_A::CYC8192
    }
    #[doc = "Checks if the value of the field is `CYC16384`"]
    #[inline(always)]
    pub fn is_cyc16384(&self) -> bool {
        **self == WINDOW_A::CYC16384
    }
}
impl core::ops::Deref for WINDOW_R {
    type Target = crate::FieldReader<u8, WINDOW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WINDOW` writer - Window Mode Time-Out Period"]
pub struct WINDOW_W<'a> {
    w: &'a mut W,
}
impl<'a> WINDOW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WINDOW_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "8 clock cycles"]
    #[inline(always)]
    pub fn cyc8(self) -> &'a mut W {
        self.variant(WINDOW_A::CYC8)
    }
    #[doc = "16 clock cycles"]
    #[inline(always)]
    pub fn cyc16(self) -> &'a mut W {
        self.variant(WINDOW_A::CYC16)
    }
    #[doc = "32 clock cycles"]
    #[inline(always)]
    pub fn cyc32(self) -> &'a mut W {
        self.variant(WINDOW_A::CYC32)
    }
    #[doc = "64 clock cycles"]
    #[inline(always)]
    pub fn cyc64(self) -> &'a mut W {
        self.variant(WINDOW_A::CYC64)
    }
    #[doc = "128 clock cycles"]
    #[inline(always)]
    pub fn cyc128(self) -> &'a mut W {
        self.variant(WINDOW_A::CYC128)
    }
    #[doc = "256 clock cycles"]
    #[inline(always)]
    pub fn cyc256(self) -> &'a mut W {
        self.variant(WINDOW_A::CYC256)
    }
    #[doc = "512 clock cycles"]
    #[inline(always)]
    pub fn cyc512(self) -> &'a mut W {
        self.variant(WINDOW_A::CYC512)
    }
    #[doc = "1024 clock cycles"]
    #[inline(always)]
    pub fn cyc1024(self) -> &'a mut W {
        self.variant(WINDOW_A::CYC1024)
    }
    #[doc = "2048 clock cycles"]
    #[inline(always)]
    pub fn cyc2048(self) -> &'a mut W {
        self.variant(WINDOW_A::CYC2048)
    }
    #[doc = "4096 clock cycles"]
    #[inline(always)]
    pub fn cyc4096(self) -> &'a mut W {
        self.variant(WINDOW_A::CYC4096)
    }
    #[doc = "8192 clock cycles"]
    #[inline(always)]
    pub fn cyc8192(self) -> &'a mut W {
        self.variant(WINDOW_A::CYC8192)
    }
    #[doc = "16384 clock cycles"]
    #[inline(always)]
    pub fn cyc16384(self) -> &'a mut W {
        self.variant(WINDOW_A::CYC16384)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u8 & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Time-Out Period"]
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Window Mode Time-Out Period"]
    #[inline(always)]
    pub fn window(&self) -> WINDOW_R {
        WINDOW_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Time-Out Period"]
    #[inline(always)]
    pub fn per(&mut self) -> PER_W {
        PER_W { w: self }
    }
    #[doc = "Bits 4:7 - Window Mode Time-Out Period"]
    #[inline(always)]
    pub fn window(&mut self) -> WINDOW_W {
        WINDOW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config](index.html) module"]
pub struct CONFIG_SPEC;
impl crate::RegisterSpec for CONFIG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [config::R](R) reader structure"]
impl crate::Readable for CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [config::W](W) writer structure"]
impl crate::Writable for CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONFIG to value 0xbb"]
impl crate::Resettable for CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xbb
    }
}
