#[doc = "Register `PRESCBUF` reader"]
pub type R = crate::R<PRESCBUF_SPEC>;
#[doc = "Register `PRESCBUF` writer"]
pub type W = crate::W<PRESCBUF_SPEC>;
#[doc = "Field `PRESCBUF` reader - Prescaler Buffer Value"]
pub type PRESCBUF_R = crate::FieldReader<PRESCBUFSELECT_A>;
#[doc = "Prescaler Buffer Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRESCBUFSELECT_A {
    #[doc = "0: No division"]
    DIV1 = 0,
    #[doc = "1: Divide by 2"]
    DIV2 = 1,
    #[doc = "2: Divide by 4"]
    DIV4 = 2,
    #[doc = "3: Divide by 8"]
    DIV8 = 3,
    #[doc = "4: Divide by 16"]
    DIV16 = 4,
    #[doc = "5: Divide by 32"]
    DIV32 = 5,
    #[doc = "6: Divide by 64"]
    DIV64 = 6,
    #[doc = "7: Divide by 128"]
    DIV128 = 7,
    #[doc = "8: Divide by 256"]
    DIV256 = 8,
    #[doc = "9: Divide by 512"]
    DIV512 = 9,
    #[doc = "10: Divide by 1024"]
    DIV1024 = 10,
}
impl From<PRESCBUFSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESCBUFSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRESCBUFSELECT_A {
    type Ux = u8;
}
impl PRESCBUF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PRESCBUFSELECT_A> {
        match self.bits {
            0 => Some(PRESCBUFSELECT_A::DIV1),
            1 => Some(PRESCBUFSELECT_A::DIV2),
            2 => Some(PRESCBUFSELECT_A::DIV4),
            3 => Some(PRESCBUFSELECT_A::DIV8),
            4 => Some(PRESCBUFSELECT_A::DIV16),
            5 => Some(PRESCBUFSELECT_A::DIV32),
            6 => Some(PRESCBUFSELECT_A::DIV64),
            7 => Some(PRESCBUFSELECT_A::DIV128),
            8 => Some(PRESCBUFSELECT_A::DIV256),
            9 => Some(PRESCBUFSELECT_A::DIV512),
            10 => Some(PRESCBUFSELECT_A::DIV1024),
            _ => None,
        }
    }
    #[doc = "No division"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PRESCBUFSELECT_A::DIV1
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PRESCBUFSELECT_A::DIV2
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PRESCBUFSELECT_A::DIV4
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PRESCBUFSELECT_A::DIV8
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PRESCBUFSELECT_A::DIV16
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PRESCBUFSELECT_A::DIV32
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PRESCBUFSELECT_A::DIV64
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == PRESCBUFSELECT_A::DIV128
    }
    #[doc = "Divide by 256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == PRESCBUFSELECT_A::DIV256
    }
    #[doc = "Divide by 512"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == PRESCBUFSELECT_A::DIV512
    }
    #[doc = "Divide by 1024"]
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == PRESCBUFSELECT_A::DIV1024
    }
}
#[doc = "Field `PRESCBUF` writer - Prescaler Buffer Value"]
pub type PRESCBUF_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, PRESCBUFSELECT_A>;
impl<'a, REG, const O: u8> PRESCBUF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No division"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCBUFSELECT_A::DIV1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCBUFSELECT_A::DIV2)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCBUFSELECT_A::DIV4)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCBUFSELECT_A::DIV8)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCBUFSELECT_A::DIV16)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCBUFSELECT_A::DIV32)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCBUFSELECT_A::DIV64)
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCBUFSELECT_A::DIV128)
    }
    #[doc = "Divide by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCBUFSELECT_A::DIV256)
    }
    #[doc = "Divide by 512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCBUFSELECT_A::DIV512)
    }
    #[doc = "Divide by 1024"]
    #[inline(always)]
    pub fn div1024(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCBUFSELECT_A::DIV1024)
    }
}
impl R {
    #[doc = "Bits 0:3 - Prescaler Buffer Value"]
    #[inline(always)]
    pub fn prescbuf(&self) -> PRESCBUF_R {
        PRESCBUF_R::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Prescaler Buffer Value"]
    #[inline(always)]
    #[must_use]
    pub fn prescbuf(&mut self) -> PRESCBUF_W<PRESCBUF_SPEC, 0> {
        PRESCBUF_W::new(self)
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
#[doc = "Prescaler Buffer Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prescbuf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prescbuf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRESCBUF_SPEC;
impl crate::RegisterSpec for PRESCBUF_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`prescbuf::R`](R) reader structure"]
impl crate::Readable for PRESCBUF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`prescbuf::W`](W) writer structure"]
impl crate::Writable for PRESCBUF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRESCBUF to value 0"]
impl crate::Resettable for PRESCBUF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
