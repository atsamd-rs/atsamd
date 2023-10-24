#[doc = "Register `INSTRCTRL` reader"]
pub type R = crate::R<INSTRCTRL_SPEC>;
#[doc = "Register `INSTRCTRL` writer"]
pub type W = crate::W<INSTRCTRL_SPEC>;
#[doc = "Field `INSTR` reader - Instruction Code"]
pub type INSTR_R = crate::FieldReader;
#[doc = "Field `INSTR` writer - Instruction Code"]
pub type INSTR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `OPTCODE` reader - Option Code"]
pub type OPTCODE_R = crate::FieldReader;
#[doc = "Field `OPTCODE` writer - Option Code"]
pub type OPTCODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Instruction Code"]
    #[inline(always)]
    pub fn instr(&self) -> INSTR_R {
        INSTR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Option Code"]
    #[inline(always)]
    pub fn optcode(&self) -> OPTCODE_R {
        OPTCODE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Instruction Code"]
    #[inline(always)]
    #[must_use]
    pub fn instr(&mut self) -> INSTR_W<INSTRCTRL_SPEC, 0> {
        INSTR_W::new(self)
    }
    #[doc = "Bits 16:23 - Option Code"]
    #[inline(always)]
    #[must_use]
    pub fn optcode(&mut self) -> OPTCODE_W<INSTRCTRL_SPEC, 16> {
        OPTCODE_W::new(self)
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
#[doc = "Instruction Code\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`instrctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`instrctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INSTRCTRL_SPEC;
impl crate::RegisterSpec for INSTRCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`instrctrl::R`](R) reader structure"]
impl crate::Readable for INSTRCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`instrctrl::W`](W) writer structure"]
impl crate::Writable for INSTRCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INSTRCTRL to value 0"]
impl crate::Resettable for INSTRCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
