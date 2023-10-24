#[doc = "Register `PARAM` reader"]
pub type R = crate::R<PARAM_SPEC>;
#[doc = "Register `PARAM` writer"]
pub type W = crate::W<PARAM_SPEC>;
#[doc = "Field `NVMP` reader - NVM Pages"]
pub type NVMP_R = crate::FieldReader<u16>;
#[doc = "Field `PSZ` reader - Page Size"]
pub type PSZ_R = crate::FieldReader<PSZSELECT_A>;
#[doc = "Page Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PSZSELECT_A {
    #[doc = "0: 8 bytes"]
    _8 = 0,
    #[doc = "1: 16 bytes"]
    _16 = 1,
    #[doc = "2: 32 bytes"]
    _32 = 2,
    #[doc = "3: 64 bytes"]
    _64 = 3,
    #[doc = "4: 128 bytes"]
    _128 = 4,
    #[doc = "5: 256 bytes"]
    _256 = 5,
    #[doc = "6: 512 bytes"]
    _512 = 6,
    #[doc = "7: 1024 bytes"]
    _1024 = 7,
}
impl From<PSZSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PSZSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PSZSELECT_A {
    type Ux = u8;
}
impl PSZ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PSZSELECT_A {
        match self.bits {
            0 => PSZSELECT_A::_8,
            1 => PSZSELECT_A::_16,
            2 => PSZSELECT_A::_32,
            3 => PSZSELECT_A::_64,
            4 => PSZSELECT_A::_128,
            5 => PSZSELECT_A::_256,
            6 => PSZSELECT_A::_512,
            7 => PSZSELECT_A::_1024,
            _ => unreachable!(),
        }
    }
    #[doc = "8 bytes"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == PSZSELECT_A::_8
    }
    #[doc = "16 bytes"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == PSZSELECT_A::_16
    }
    #[doc = "32 bytes"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == PSZSELECT_A::_32
    }
    #[doc = "64 bytes"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == PSZSELECT_A::_64
    }
    #[doc = "128 bytes"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == PSZSELECT_A::_128
    }
    #[doc = "256 bytes"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == PSZSELECT_A::_256
    }
    #[doc = "512 bytes"]
    #[inline(always)]
    pub fn is_512(&self) -> bool {
        *self == PSZSELECT_A::_512
    }
    #[doc = "1024 bytes"]
    #[inline(always)]
    pub fn is_1024(&self) -> bool {
        *self == PSZSELECT_A::_1024
    }
}
impl R {
    #[doc = "Bits 0:15 - NVM Pages"]
    #[inline(always)]
    pub fn nvmp(&self) -> NVMP_R {
        NVMP_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - Page Size"]
    #[inline(always)]
    pub fn psz(&self) -> PSZ_R {
        PSZ_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "NVM Parameter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`param::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`param::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PARAM_SPEC;
impl crate::RegisterSpec for PARAM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`param::R`](R) reader structure"]
impl crate::Readable for PARAM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`param::W`](W) writer structure"]
impl crate::Writable for PARAM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PARAM to value 0"]
impl crate::Resettable for PARAM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
