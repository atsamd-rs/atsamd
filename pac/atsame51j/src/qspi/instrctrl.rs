#[doc = "Register `INSTRCTRL` reader"]
pub struct R(crate::R<INSTRCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INSTRCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INSTRCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INSTRCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INSTRCTRL` writer"]
pub struct W(crate::W<INSTRCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INSTRCTRL_SPEC>;
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
impl From<crate::W<INSTRCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INSTRCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INSTR` reader - Instruction Code"]
pub type INSTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INSTR` writer - Instruction Code"]
pub type INSTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INSTRCTRL_SPEC, u8, u8, 8, O>;
#[doc = "Field `OPTCODE` reader - Option Code"]
pub type OPTCODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OPTCODE` writer - Option Code"]
pub type OPTCODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INSTRCTRL_SPEC, u8, u8, 8, O>;
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
    pub fn instr(&mut self) -> INSTR_W<0> {
        INSTR_W::new(self)
    }
    #[doc = "Bits 16:23 - Option Code"]
    #[inline(always)]
    #[must_use]
    pub fn optcode(&mut self) -> OPTCODE_W<16> {
        OPTCODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Instruction Code\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [instrctrl](index.html) module"]
pub struct INSTRCTRL_SPEC;
impl crate::RegisterSpec for INSTRCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [instrctrl::R](R) reader structure"]
impl crate::Readable for INSTRCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [instrctrl::W](W) writer structure"]
impl crate::Writable for INSTRCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INSTRCTRL to value 0"]
impl crate::Resettable for INSTRCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
