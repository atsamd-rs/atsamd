#[doc = "Register `CONFIG` reader"]
pub type R = crate::R<CONFIG_SPEC>;
#[doc = "Register `CONFIG` writer"]
pub type W = crate::W<CONFIG_SPEC>;
#[doc = "Field `PER` reader - Time-Out Period"]
pub type PER_R = crate::FieldReader<PERSELECT_A>;
#[doc = "Time-Out Period\n\nValue on reset: 11"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PERSELECT_A {
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
impl From<PERSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PERSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PERSELECT_A {
    type Ux = u8;
}
impl PER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PERSELECT_A> {
        match self.bits {
            0 => Some(PERSELECT_A::CYC8),
            1 => Some(PERSELECT_A::CYC16),
            2 => Some(PERSELECT_A::CYC32),
            3 => Some(PERSELECT_A::CYC64),
            4 => Some(PERSELECT_A::CYC128),
            5 => Some(PERSELECT_A::CYC256),
            6 => Some(PERSELECT_A::CYC512),
            7 => Some(PERSELECT_A::CYC1024),
            8 => Some(PERSELECT_A::CYC2048),
            9 => Some(PERSELECT_A::CYC4096),
            10 => Some(PERSELECT_A::CYC8192),
            11 => Some(PERSELECT_A::CYC16384),
            _ => None,
        }
    }
    #[doc = "8 clock cycles"]
    #[inline(always)]
    pub fn is_cyc8(&self) -> bool {
        *self == PERSELECT_A::CYC8
    }
    #[doc = "16 clock cycles"]
    #[inline(always)]
    pub fn is_cyc16(&self) -> bool {
        *self == PERSELECT_A::CYC16
    }
    #[doc = "32 clock cycles"]
    #[inline(always)]
    pub fn is_cyc32(&self) -> bool {
        *self == PERSELECT_A::CYC32
    }
    #[doc = "64 clock cycles"]
    #[inline(always)]
    pub fn is_cyc64(&self) -> bool {
        *self == PERSELECT_A::CYC64
    }
    #[doc = "128 clock cycles"]
    #[inline(always)]
    pub fn is_cyc128(&self) -> bool {
        *self == PERSELECT_A::CYC128
    }
    #[doc = "256 clock cycles"]
    #[inline(always)]
    pub fn is_cyc256(&self) -> bool {
        *self == PERSELECT_A::CYC256
    }
    #[doc = "512 clock cycles"]
    #[inline(always)]
    pub fn is_cyc512(&self) -> bool {
        *self == PERSELECT_A::CYC512
    }
    #[doc = "1024 clock cycles"]
    #[inline(always)]
    pub fn is_cyc1024(&self) -> bool {
        *self == PERSELECT_A::CYC1024
    }
    #[doc = "2048 clock cycles"]
    #[inline(always)]
    pub fn is_cyc2048(&self) -> bool {
        *self == PERSELECT_A::CYC2048
    }
    #[doc = "4096 clock cycles"]
    #[inline(always)]
    pub fn is_cyc4096(&self) -> bool {
        *self == PERSELECT_A::CYC4096
    }
    #[doc = "8192 clock cycles"]
    #[inline(always)]
    pub fn is_cyc8192(&self) -> bool {
        *self == PERSELECT_A::CYC8192
    }
    #[doc = "16384 clock cycles"]
    #[inline(always)]
    pub fn is_cyc16384(&self) -> bool {
        *self == PERSELECT_A::CYC16384
    }
}
#[doc = "Field `PER` writer - Time-Out Period"]
pub type PER_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, PERSELECT_A>;
impl<'a, REG, const O: u8> PER_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 clock cycles"]
    #[inline(always)]
    pub fn cyc8(self) -> &'a mut crate::W<REG> {
        self.variant(PERSELECT_A::CYC8)
    }
    #[doc = "16 clock cycles"]
    #[inline(always)]
    pub fn cyc16(self) -> &'a mut crate::W<REG> {
        self.variant(PERSELECT_A::CYC16)
    }
    #[doc = "32 clock cycles"]
    #[inline(always)]
    pub fn cyc32(self) -> &'a mut crate::W<REG> {
        self.variant(PERSELECT_A::CYC32)
    }
    #[doc = "64 clock cycles"]
    #[inline(always)]
    pub fn cyc64(self) -> &'a mut crate::W<REG> {
        self.variant(PERSELECT_A::CYC64)
    }
    #[doc = "128 clock cycles"]
    #[inline(always)]
    pub fn cyc128(self) -> &'a mut crate::W<REG> {
        self.variant(PERSELECT_A::CYC128)
    }
    #[doc = "256 clock cycles"]
    #[inline(always)]
    pub fn cyc256(self) -> &'a mut crate::W<REG> {
        self.variant(PERSELECT_A::CYC256)
    }
    #[doc = "512 clock cycles"]
    #[inline(always)]
    pub fn cyc512(self) -> &'a mut crate::W<REG> {
        self.variant(PERSELECT_A::CYC512)
    }
    #[doc = "1024 clock cycles"]
    #[inline(always)]
    pub fn cyc1024(self) -> &'a mut crate::W<REG> {
        self.variant(PERSELECT_A::CYC1024)
    }
    #[doc = "2048 clock cycles"]
    #[inline(always)]
    pub fn cyc2048(self) -> &'a mut crate::W<REG> {
        self.variant(PERSELECT_A::CYC2048)
    }
    #[doc = "4096 clock cycles"]
    #[inline(always)]
    pub fn cyc4096(self) -> &'a mut crate::W<REG> {
        self.variant(PERSELECT_A::CYC4096)
    }
    #[doc = "8192 clock cycles"]
    #[inline(always)]
    pub fn cyc8192(self) -> &'a mut crate::W<REG> {
        self.variant(PERSELECT_A::CYC8192)
    }
    #[doc = "16384 clock cycles"]
    #[inline(always)]
    pub fn cyc16384(self) -> &'a mut crate::W<REG> {
        self.variant(PERSELECT_A::CYC16384)
    }
}
#[doc = "Field `WINDOW` reader - Window Mode Time-Out Period"]
pub type WINDOW_R = crate::FieldReader<WINDOWSELECT_A>;
#[doc = "Window Mode Time-Out Period\n\nValue on reset: 11"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WINDOWSELECT_A {
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
impl From<WINDOWSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: WINDOWSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WINDOWSELECT_A {
    type Ux = u8;
}
impl WINDOW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WINDOWSELECT_A> {
        match self.bits {
            0 => Some(WINDOWSELECT_A::CYC8),
            1 => Some(WINDOWSELECT_A::CYC16),
            2 => Some(WINDOWSELECT_A::CYC32),
            3 => Some(WINDOWSELECT_A::CYC64),
            4 => Some(WINDOWSELECT_A::CYC128),
            5 => Some(WINDOWSELECT_A::CYC256),
            6 => Some(WINDOWSELECT_A::CYC512),
            7 => Some(WINDOWSELECT_A::CYC1024),
            8 => Some(WINDOWSELECT_A::CYC2048),
            9 => Some(WINDOWSELECT_A::CYC4096),
            10 => Some(WINDOWSELECT_A::CYC8192),
            11 => Some(WINDOWSELECT_A::CYC16384),
            _ => None,
        }
    }
    #[doc = "8 clock cycles"]
    #[inline(always)]
    pub fn is_cyc8(&self) -> bool {
        *self == WINDOWSELECT_A::CYC8
    }
    #[doc = "16 clock cycles"]
    #[inline(always)]
    pub fn is_cyc16(&self) -> bool {
        *self == WINDOWSELECT_A::CYC16
    }
    #[doc = "32 clock cycles"]
    #[inline(always)]
    pub fn is_cyc32(&self) -> bool {
        *self == WINDOWSELECT_A::CYC32
    }
    #[doc = "64 clock cycles"]
    #[inline(always)]
    pub fn is_cyc64(&self) -> bool {
        *self == WINDOWSELECT_A::CYC64
    }
    #[doc = "128 clock cycles"]
    #[inline(always)]
    pub fn is_cyc128(&self) -> bool {
        *self == WINDOWSELECT_A::CYC128
    }
    #[doc = "256 clock cycles"]
    #[inline(always)]
    pub fn is_cyc256(&self) -> bool {
        *self == WINDOWSELECT_A::CYC256
    }
    #[doc = "512 clock cycles"]
    #[inline(always)]
    pub fn is_cyc512(&self) -> bool {
        *self == WINDOWSELECT_A::CYC512
    }
    #[doc = "1024 clock cycles"]
    #[inline(always)]
    pub fn is_cyc1024(&self) -> bool {
        *self == WINDOWSELECT_A::CYC1024
    }
    #[doc = "2048 clock cycles"]
    #[inline(always)]
    pub fn is_cyc2048(&self) -> bool {
        *self == WINDOWSELECT_A::CYC2048
    }
    #[doc = "4096 clock cycles"]
    #[inline(always)]
    pub fn is_cyc4096(&self) -> bool {
        *self == WINDOWSELECT_A::CYC4096
    }
    #[doc = "8192 clock cycles"]
    #[inline(always)]
    pub fn is_cyc8192(&self) -> bool {
        *self == WINDOWSELECT_A::CYC8192
    }
    #[doc = "16384 clock cycles"]
    #[inline(always)]
    pub fn is_cyc16384(&self) -> bool {
        *self == WINDOWSELECT_A::CYC16384
    }
}
#[doc = "Field `WINDOW` writer - Window Mode Time-Out Period"]
pub type WINDOW_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, WINDOWSELECT_A>;
impl<'a, REG, const O: u8> WINDOW_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 clock cycles"]
    #[inline(always)]
    pub fn cyc8(self) -> &'a mut crate::W<REG> {
        self.variant(WINDOWSELECT_A::CYC8)
    }
    #[doc = "16 clock cycles"]
    #[inline(always)]
    pub fn cyc16(self) -> &'a mut crate::W<REG> {
        self.variant(WINDOWSELECT_A::CYC16)
    }
    #[doc = "32 clock cycles"]
    #[inline(always)]
    pub fn cyc32(self) -> &'a mut crate::W<REG> {
        self.variant(WINDOWSELECT_A::CYC32)
    }
    #[doc = "64 clock cycles"]
    #[inline(always)]
    pub fn cyc64(self) -> &'a mut crate::W<REG> {
        self.variant(WINDOWSELECT_A::CYC64)
    }
    #[doc = "128 clock cycles"]
    #[inline(always)]
    pub fn cyc128(self) -> &'a mut crate::W<REG> {
        self.variant(WINDOWSELECT_A::CYC128)
    }
    #[doc = "256 clock cycles"]
    #[inline(always)]
    pub fn cyc256(self) -> &'a mut crate::W<REG> {
        self.variant(WINDOWSELECT_A::CYC256)
    }
    #[doc = "512 clock cycles"]
    #[inline(always)]
    pub fn cyc512(self) -> &'a mut crate::W<REG> {
        self.variant(WINDOWSELECT_A::CYC512)
    }
    #[doc = "1024 clock cycles"]
    #[inline(always)]
    pub fn cyc1024(self) -> &'a mut crate::W<REG> {
        self.variant(WINDOWSELECT_A::CYC1024)
    }
    #[doc = "2048 clock cycles"]
    #[inline(always)]
    pub fn cyc2048(self) -> &'a mut crate::W<REG> {
        self.variant(WINDOWSELECT_A::CYC2048)
    }
    #[doc = "4096 clock cycles"]
    #[inline(always)]
    pub fn cyc4096(self) -> &'a mut crate::W<REG> {
        self.variant(WINDOWSELECT_A::CYC4096)
    }
    #[doc = "8192 clock cycles"]
    #[inline(always)]
    pub fn cyc8192(self) -> &'a mut crate::W<REG> {
        self.variant(WINDOWSELECT_A::CYC8192)
    }
    #[doc = "16384 clock cycles"]
    #[inline(always)]
    pub fn cyc16384(self) -> &'a mut crate::W<REG> {
        self.variant(WINDOWSELECT_A::CYC16384)
    }
}
impl R {
    #[doc = "Bits 0:3 - Time-Out Period"]
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - Window Mode Time-Out Period"]
    #[inline(always)]
    pub fn window(&self) -> WINDOW_R {
        WINDOW_R::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Time-Out Period"]
    #[inline(always)]
    #[must_use]
    pub fn per(&mut self) -> PER_W<CONFIG_SPEC, 0> {
        PER_W::new(self)
    }
    #[doc = "Bits 4:7 - Window Mode Time-Out Period"]
    #[inline(always)]
    #[must_use]
    pub fn window(&mut self) -> WINDOW_W<CONFIG_SPEC, 4> {
        WINDOW_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONFIG_SPEC;
impl crate::RegisterSpec for CONFIG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`config::R`](R) reader structure"]
impl crate::Readable for CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for CONFIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONFIG to value 0xbb"]
impl crate::Resettable for CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0xbb;
}
