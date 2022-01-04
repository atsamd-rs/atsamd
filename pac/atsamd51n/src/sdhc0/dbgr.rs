#[doc = "Register `DBGR` reader"]
pub struct R(crate::R<DBGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DBGR` writer"]
pub struct W(crate::W<DBGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DBGR_SPEC>;
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
impl From<crate::W<DBGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DBGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Non-intrusive debug enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NIDBG_A {
    #[doc = "0: Debugging is intrusive (reads of BDPR from debugger are considered and increment the internal buffer pointer)"]
    IDBG = 0,
    #[doc = "1: Debugging is not intrusive (reads of BDPR from debugger are discarded and do not increment the internal buffer pointer)"]
    NIDBG = 1,
}
impl From<NIDBG_A> for bool {
    #[inline(always)]
    fn from(variant: NIDBG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NIDBG` reader - Non-intrusive debug enable"]
pub struct NIDBG_R(crate::FieldReader<bool, NIDBG_A>);
impl NIDBG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NIDBG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NIDBG_A {
        match self.bits {
            false => NIDBG_A::IDBG,
            true => NIDBG_A::NIDBG,
        }
    }
    #[doc = "Checks if the value of the field is `IDBG`"]
    #[inline(always)]
    pub fn is_idbg(&self) -> bool {
        **self == NIDBG_A::IDBG
    }
    #[doc = "Checks if the value of the field is `NIDBG`"]
    #[inline(always)]
    pub fn is_nidbg(&self) -> bool {
        **self == NIDBG_A::NIDBG
    }
}
impl core::ops::Deref for NIDBG_R {
    type Target = crate::FieldReader<bool, NIDBG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NIDBG` writer - Non-intrusive debug enable"]
pub struct NIDBG_W<'a> {
    w: &'a mut W,
}
impl<'a> NIDBG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NIDBG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Debugging is intrusive (reads of BDPR from debugger are considered and increment the internal buffer pointer)"]
    #[inline(always)]
    pub fn idbg(self) -> &'a mut W {
        self.variant(NIDBG_A::IDBG)
    }
    #[doc = "Debugging is not intrusive (reads of BDPR from debugger are discarded and do not increment the internal buffer pointer)"]
    #[inline(always)]
    pub fn nidbg(self) -> &'a mut W {
        self.variant(NIDBG_A::NIDBG)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u8 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Non-intrusive debug enable"]
    #[inline(always)]
    pub fn nidbg(&self) -> NIDBG_R {
        NIDBG_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Non-intrusive debug enable"]
    #[inline(always)]
    pub fn nidbg(&mut self) -> NIDBG_W {
        NIDBG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Debug\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbgr](index.html) module"]
pub struct DBGR_SPEC;
impl crate::RegisterSpec for DBGR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dbgr::R](R) reader structure"]
impl crate::Readable for DBGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dbgr::W](W) writer structure"]
impl crate::Writable for DBGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DBGR to value 0"]
impl crate::Resettable for DBGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
