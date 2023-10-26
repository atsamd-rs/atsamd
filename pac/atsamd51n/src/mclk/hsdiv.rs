#[doc = "Register `HSDIV` reader"]
pub type R = crate::R<HSDIV_SPEC>;
#[doc = "Field `DIV` reader - CPU Clock Division Factor"]
pub type DIV_R = crate::FieldReader<DIVSELECT_A>;
#[doc = "CPU Clock Division Factor\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DIVSELECT_A {
    #[doc = "1: Divide by 1"]
    DIV1 = 1,
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
            _ => None,
        }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == DIVSELECT_A::DIV1
    }
}
impl R {
    #[doc = "Bits 0:7 - CPU Clock Division Factor"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new(self.bits)
    }
}
#[doc = "HS Clock Division\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsdiv::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSDIV_SPEC;
impl crate::RegisterSpec for HSDIV_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hsdiv::R`](R) reader structure"]
impl crate::Readable for HSDIV_SPEC {}
#[doc = "`reset()` method sets HSDIV to value 0x01"]
impl crate::Resettable for HSDIV_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
