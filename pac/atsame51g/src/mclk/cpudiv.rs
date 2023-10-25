#[doc = "Register `CPUDIV` reader"]
pub type R = crate::R<CPUDIV_SPEC>;
#[doc = "Register `CPUDIV` writer"]
pub type W = crate::W<CPUDIV_SPEC>;
#[doc = "Field `DIV` reader - Low-Power Clock Division Factor"]
pub type DIV_R = crate::FieldReader<DIVSELECT_A>;
#[doc = "Low-Power Clock Division Factor\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DIVSELECT_A {
    #[doc = "1: Divide by 1"]
    DIV1 = 1,
    #[doc = "2: Divide by 2"]
    DIV2 = 2,
    #[doc = "4: Divide by 4"]
    DIV4 = 4,
    #[doc = "8: Divide by 8"]
    DIV8 = 8,
    #[doc = "16: Divide by 16"]
    DIV16 = 16,
    #[doc = "32: Divide by 32"]
    DIV32 = 32,
    #[doc = "64: Divide by 64"]
    DIV64 = 64,
    #[doc = "128: Divide by 128"]
    DIV128 = 128,
}
impl From<DIVSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DIVSELECT_A {
    type Ux = u8;
}
impl DIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DIVSELECT_A> {
        match self.bits {
            1 => Some(DIVSELECT_A::DIV1),
            2 => Some(DIVSELECT_A::DIV2),
            4 => Some(DIVSELECT_A::DIV4),
            8 => Some(DIVSELECT_A::DIV8),
            16 => Some(DIVSELECT_A::DIV16),
            32 => Some(DIVSELECT_A::DIV32),
            64 => Some(DIVSELECT_A::DIV64),
            128 => Some(DIVSELECT_A::DIV128),
            _ => None,
        }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == DIVSELECT_A::DIV1
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == DIVSELECT_A::DIV2
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == DIVSELECT_A::DIV4
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == DIVSELECT_A::DIV8
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == DIVSELECT_A::DIV16
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == DIVSELECT_A::DIV32
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == DIVSELECT_A::DIV64
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == DIVSELECT_A::DIV128
    }
}
#[doc = "Field `DIV` writer - Low-Power Clock Division Factor"]
pub type DIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O, DIVSELECT_A>;
impl<'a, REG, const O: u8> DIV_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(DIVSELECT_A::DIV1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(DIVSELECT_A::DIV2)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(DIVSELECT_A::DIV4)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(DIVSELECT_A::DIV8)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(DIVSELECT_A::DIV16)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(DIVSELECT_A::DIV32)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(DIVSELECT_A::DIV64)
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(DIVSELECT_A::DIV128)
    }
}
impl R {
    #[doc = "Bits 0:7 - Low-Power Clock Division Factor"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Low-Power Clock Division Factor"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DIV_W<CPUDIV_SPEC, 0> {
        DIV_W::new(self)
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
#[doc = "CPU Clock Division\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpudiv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpudiv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPUDIV_SPEC;
impl crate::RegisterSpec for CPUDIV_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cpudiv::R`](R) reader structure"]
impl crate::Readable for CPUDIV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cpudiv::W`](W) writer structure"]
impl crate::Writable for CPUDIV_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPUDIV to value 0x01"]
impl crate::Resettable for CPUDIV_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
