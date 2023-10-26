#[doc = "Register `TSCC` reader"]
pub type R = crate::R<TSCC_SPEC>;
#[doc = "Register `TSCC` writer"]
pub type W = crate::W<TSCC_SPEC>;
#[doc = "Field `TSS` reader - Timestamp Select"]
pub type TSS_R = crate::FieldReader<TSSSELECT_A>;
#[doc = "Timestamp Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TSSSELECT_A {
    #[doc = "0: Timestamp counter value always 0x0000"]
    ZERO = 0,
    #[doc = "1: Timestamp counter value incremented by TCP"]
    INC = 1,
    #[doc = "2: External timestamp counter value used"]
    EXT = 2,
}
impl From<TSSSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: TSSSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TSSSELECT_A {
    type Ux = u8;
}
impl TSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TSSSELECT_A> {
        match self.bits {
            0 => Some(TSSSELECT_A::ZERO),
            1 => Some(TSSSELECT_A::INC),
            2 => Some(TSSSELECT_A::EXT),
            _ => None,
        }
    }
    #[doc = "Timestamp counter value always 0x0000"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == TSSSELECT_A::ZERO
    }
    #[doc = "Timestamp counter value incremented by TCP"]
    #[inline(always)]
    pub fn is_inc(&self) -> bool {
        *self == TSSSELECT_A::INC
    }
    #[doc = "External timestamp counter value used"]
    #[inline(always)]
    pub fn is_ext(&self) -> bool {
        *self == TSSSELECT_A::EXT
    }
}
#[doc = "Field `TSS` writer - Timestamp Select"]
pub type TSS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, TSSSELECT_A>;
impl<'a, REG, const O: u8> TSS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timestamp counter value always 0x0000"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(TSSSELECT_A::ZERO)
    }
    #[doc = "Timestamp counter value incremented by TCP"]
    #[inline(always)]
    pub fn inc(self) -> &'a mut crate::W<REG> {
        self.variant(TSSSELECT_A::INC)
    }
    #[doc = "External timestamp counter value used"]
    #[inline(always)]
    pub fn ext(self) -> &'a mut crate::W<REG> {
        self.variant(TSSSELECT_A::EXT)
    }
}
#[doc = "Field `TCP` reader - Timestamp Counter Prescaler"]
pub type TCP_R = crate::FieldReader;
#[doc = "Field `TCP` writer - Timestamp Counter Prescaler"]
pub type TCP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:1 - Timestamp Select"]
    #[inline(always)]
    pub fn tss(&self) -> TSS_R {
        TSS_R::new((self.bits & 3) as u8)
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
    #[must_use]
    pub fn tss(&mut self) -> TSS_W<TSCC_SPEC, 0> {
        TSS_W::new(self)
    }
    #[doc = "Bits 16:19 - Timestamp Counter Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn tcp(&mut self) -> TCP_W<TSCC_SPEC, 16> {
        TCP_W::new(self)
    }
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
#[doc = "Timestamp Counter Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tscc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tscc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSCC_SPEC;
impl crate::RegisterSpec for TSCC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tscc::R`](R) reader structure"]
impl crate::Readable for TSCC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tscc::W`](W) writer structure"]
impl crate::Writable for TSCC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TSCC to value 0"]
impl crate::Resettable for TSCC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
