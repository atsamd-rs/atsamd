#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ICDIS` reader - Instruction Cache Disable"]
pub type ICDIS_R = crate::BitReader<bool>;
#[doc = "Field `ICDIS` writer - Instruction Cache Disable"]
pub type ICDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `DCDIS` reader - Data Cache Disable"]
pub type DCDIS_R = crate::BitReader<bool>;
#[doc = "Field `DCDIS` writer - Data Cache Disable"]
pub type DCDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `CSIZESW` reader - Cache size configured by software"]
pub type CSIZESW_R = crate::FieldReader<u8, CSIZESWSELECT_A>;
#[doc = "Cache size configured by software\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CSIZESWSELECT_A {
    #[doc = "0: The Cache Size is configured to 1KB"]
    CONF_CSIZE_1KB = 0,
    #[doc = "1: The Cache Size is configured to 2KB"]
    CONF_CSIZE_2KB = 1,
    #[doc = "2: The Cache Size is configured to 4KB"]
    CONF_CSIZE_4KB = 2,
    #[doc = "3: The Cache Size is configured to 8KB"]
    CONF_CSIZE_8KB = 3,
    #[doc = "4: The Cache Size is configured to 16KB"]
    CONF_CSIZE_16KB = 4,
    #[doc = "5: The Cache Size is configured to 32KB"]
    CONF_CSIZE_32KB = 5,
    #[doc = "6: The Cache Size is configured to 64KB"]
    CONF_CSIZE_64KB = 6,
}
impl From<CSIZESWSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CSIZESWSELECT_A) -> Self {
        variant as _
    }
}
impl CSIZESW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CSIZESWSELECT_A> {
        match self.bits {
            0 => Some(CSIZESWSELECT_A::CONF_CSIZE_1KB),
            1 => Some(CSIZESWSELECT_A::CONF_CSIZE_2KB),
            2 => Some(CSIZESWSELECT_A::CONF_CSIZE_4KB),
            3 => Some(CSIZESWSELECT_A::CONF_CSIZE_8KB),
            4 => Some(CSIZESWSELECT_A::CONF_CSIZE_16KB),
            5 => Some(CSIZESWSELECT_A::CONF_CSIZE_32KB),
            6 => Some(CSIZESWSELECT_A::CONF_CSIZE_64KB),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CONF_CSIZE_1KB`"]
    #[inline(always)]
    pub fn is_conf_csize_1kb(&self) -> bool {
        *self == CSIZESWSELECT_A::CONF_CSIZE_1KB
    }
    #[doc = "Checks if the value of the field is `CONF_CSIZE_2KB`"]
    #[inline(always)]
    pub fn is_conf_csize_2kb(&self) -> bool {
        *self == CSIZESWSELECT_A::CONF_CSIZE_2KB
    }
    #[doc = "Checks if the value of the field is `CONF_CSIZE_4KB`"]
    #[inline(always)]
    pub fn is_conf_csize_4kb(&self) -> bool {
        *self == CSIZESWSELECT_A::CONF_CSIZE_4KB
    }
    #[doc = "Checks if the value of the field is `CONF_CSIZE_8KB`"]
    #[inline(always)]
    pub fn is_conf_csize_8kb(&self) -> bool {
        *self == CSIZESWSELECT_A::CONF_CSIZE_8KB
    }
    #[doc = "Checks if the value of the field is `CONF_CSIZE_16KB`"]
    #[inline(always)]
    pub fn is_conf_csize_16kb(&self) -> bool {
        *self == CSIZESWSELECT_A::CONF_CSIZE_16KB
    }
    #[doc = "Checks if the value of the field is `CONF_CSIZE_32KB`"]
    #[inline(always)]
    pub fn is_conf_csize_32kb(&self) -> bool {
        *self == CSIZESWSELECT_A::CONF_CSIZE_32KB
    }
    #[doc = "Checks if the value of the field is `CONF_CSIZE_64KB`"]
    #[inline(always)]
    pub fn is_conf_csize_64kb(&self) -> bool {
        *self == CSIZESWSELECT_A::CONF_CSIZE_64KB
    }
}
#[doc = "Field `CSIZESW` writer - Cache size configured by software"]
pub type CSIZESW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CFG_SPEC, u8, CSIZESWSELECT_A, 3, O>;
impl<'a, const O: u8> CSIZESW_W<'a, O> {
    #[doc = "The Cache Size is configured to 1KB"]
    #[inline(always)]
    pub fn conf_csize_1kb(self) -> &'a mut W {
        self.variant(CSIZESWSELECT_A::CONF_CSIZE_1KB)
    }
    #[doc = "The Cache Size is configured to 2KB"]
    #[inline(always)]
    pub fn conf_csize_2kb(self) -> &'a mut W {
        self.variant(CSIZESWSELECT_A::CONF_CSIZE_2KB)
    }
    #[doc = "The Cache Size is configured to 4KB"]
    #[inline(always)]
    pub fn conf_csize_4kb(self) -> &'a mut W {
        self.variant(CSIZESWSELECT_A::CONF_CSIZE_4KB)
    }
    #[doc = "The Cache Size is configured to 8KB"]
    #[inline(always)]
    pub fn conf_csize_8kb(self) -> &'a mut W {
        self.variant(CSIZESWSELECT_A::CONF_CSIZE_8KB)
    }
    #[doc = "The Cache Size is configured to 16KB"]
    #[inline(always)]
    pub fn conf_csize_16kb(self) -> &'a mut W {
        self.variant(CSIZESWSELECT_A::CONF_CSIZE_16KB)
    }
    #[doc = "The Cache Size is configured to 32KB"]
    #[inline(always)]
    pub fn conf_csize_32kb(self) -> &'a mut W {
        self.variant(CSIZESWSELECT_A::CONF_CSIZE_32KB)
    }
    #[doc = "The Cache Size is configured to 64KB"]
    #[inline(always)]
    pub fn conf_csize_64kb(self) -> &'a mut W {
        self.variant(CSIZESWSELECT_A::CONF_CSIZE_64KB)
    }
}
impl R {
    #[doc = "Bit 1 - Instruction Cache Disable"]
    #[inline(always)]
    pub fn icdis(&self) -> ICDIS_R {
        ICDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Data Cache Disable"]
    #[inline(always)]
    pub fn dcdis(&self) -> DCDIS_R {
        DCDIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Cache size configured by software"]
    #[inline(always)]
    pub fn csizesw(&self) -> CSIZESW_R {
        CSIZESW_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Instruction Cache Disable"]
    #[inline(always)]
    #[must_use]
    pub fn icdis(&mut self) -> ICDIS_W<1> {
        ICDIS_W::new(self)
    }
    #[doc = "Bit 2 - Data Cache Disable"]
    #[inline(always)]
    #[must_use]
    pub fn dcdis(&mut self) -> DCDIS_W<2> {
        DCDIS_W::new(self)
    }
    #[doc = "Bits 4:6 - Cache size configured by software"]
    #[inline(always)]
    #[must_use]
    pub fn csizesw(&mut self) -> CSIZESW_W<4> {
        CSIZESW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cache Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG to value 0x20"]
impl crate::Resettable for CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x20;
}
