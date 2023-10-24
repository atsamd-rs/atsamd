#[doc = "Register `CTRLB` reader"]
pub type R = crate::R<CTRLB_SPEC>;
#[doc = "Register `CTRLB` writer"]
pub type W = crate::W<CTRLB_SPEC>;
#[doc = "Field `DIFF` reader - Differential mode enable"]
pub type DIFF_R = crate::BitReader;
#[doc = "Field `DIFF` writer - Differential mode enable"]
pub type DIFF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `REFSEL` reader - Reference Selection for DAC0/1"]
pub type REFSEL_R = crate::FieldReader<REFSELSELECT_A>;
#[doc = "Reference Selection for DAC0/1\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REFSELSELECT_A {
    #[doc = "0: External reference unbuffered"]
    VREFPU = 0,
    #[doc = "1: Analog supply"]
    VDDANA = 1,
    #[doc = "2: External reference buffered"]
    VREFPB = 2,
    #[doc = "3: Internal bandgap reference"]
    INTREF = 3,
}
impl From<REFSELSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: REFSELSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for REFSELSELECT_A {
    type Ux = u8;
}
impl REFSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> REFSELSELECT_A {
        match self.bits {
            0 => REFSELSELECT_A::VREFPU,
            1 => REFSELSELECT_A::VDDANA,
            2 => REFSELSELECT_A::VREFPB,
            3 => REFSELSELECT_A::INTREF,
            _ => unreachable!(),
        }
    }
    #[doc = "External reference unbuffered"]
    #[inline(always)]
    pub fn is_vrefpu(&self) -> bool {
        *self == REFSELSELECT_A::VREFPU
    }
    #[doc = "Analog supply"]
    #[inline(always)]
    pub fn is_vddana(&self) -> bool {
        *self == REFSELSELECT_A::VDDANA
    }
    #[doc = "External reference buffered"]
    #[inline(always)]
    pub fn is_vrefpb(&self) -> bool {
        *self == REFSELSELECT_A::VREFPB
    }
    #[doc = "Internal bandgap reference"]
    #[inline(always)]
    pub fn is_intref(&self) -> bool {
        *self == REFSELSELECT_A::INTREF
    }
}
#[doc = "Field `REFSEL` writer - Reference Selection for DAC0/1"]
pub type REFSEL_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, REFSELSELECT_A>;
impl<'a, REG, const O: u8> REFSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External reference unbuffered"]
    #[inline(always)]
    pub fn vrefpu(self) -> &'a mut crate::W<REG> {
        self.variant(REFSELSELECT_A::VREFPU)
    }
    #[doc = "Analog supply"]
    #[inline(always)]
    pub fn vddana(self) -> &'a mut crate::W<REG> {
        self.variant(REFSELSELECT_A::VDDANA)
    }
    #[doc = "External reference buffered"]
    #[inline(always)]
    pub fn vrefpb(self) -> &'a mut crate::W<REG> {
        self.variant(REFSELSELECT_A::VREFPB)
    }
    #[doc = "Internal bandgap reference"]
    #[inline(always)]
    pub fn intref(self) -> &'a mut crate::W<REG> {
        self.variant(REFSELSELECT_A::INTREF)
    }
}
impl R {
    #[doc = "Bit 0 - Differential mode enable"]
    #[inline(always)]
    pub fn diff(&self) -> DIFF_R {
        DIFF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Reference Selection for DAC0/1"]
    #[inline(always)]
    pub fn refsel(&self) -> REFSEL_R {
        REFSEL_R::new((self.bits >> 1) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - Differential mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn diff(&mut self) -> DIFF_W<CTRLB_SPEC, 0> {
        DIFF_W::new(self)
    }
    #[doc = "Bits 1:2 - Reference Selection for DAC0/1"]
    #[inline(always)]
    #[must_use]
    pub fn refsel(&mut self) -> REFSEL_W<CTRLB_SPEC, 1> {
        REFSEL_W::new(self)
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
#[doc = "Control B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLB_SPEC;
impl crate::RegisterSpec for CTRLB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctrlb::R`](R) reader structure"]
impl crate::Readable for CTRLB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrlb::W`](W) writer structure"]
impl crate::Writable for CTRLB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLB to value 0x02"]
impl crate::Resettable for CTRLB_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
