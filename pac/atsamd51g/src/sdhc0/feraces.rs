#[doc = "Register `FERACES` writer"]
pub struct W(crate::W<FERACES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FERACES_SPEC>;
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
impl From<crate::W<FERACES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FERACES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Force Event for Auto CMD12 Not Executed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACMD12NESELECT_AW {
    #[doc = "0: No Interrupt"]
    NO = 0,
    #[doc = "1: Interrupt is generated"]
    YES = 1,
}
impl From<ACMD12NESELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: ACMD12NESELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMD12NE` writer - Force Event for Auto CMD12 Not Executed"]
pub type ACMD12NE_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, FERACES_SPEC, ACMD12NESELECT_AW, O>;
impl<'a, const O: u8> ACMD12NE_W<'a, O> {
    #[doc = "No Interrupt"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(ACMD12NESELECT_AW::NO)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut W {
        self.variant(ACMD12NESELECT_AW::YES)
    }
}
#[doc = "Force Event for Auto CMD Timeout Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACMDTEOSELECT_AW {
    #[doc = "0: No Interrupt"]
    NO = 0,
    #[doc = "1: Interrupt is generated"]
    YES = 1,
}
impl From<ACMDTEOSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: ACMDTEOSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMDTEO` writer - Force Event for Auto CMD Timeout Error"]
pub type ACMDTEO_W<'a, const O: u8> = crate::BitWriter<'a, u16, FERACES_SPEC, ACMDTEOSELECT_AW, O>;
impl<'a, const O: u8> ACMDTEO_W<'a, O> {
    #[doc = "No Interrupt"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(ACMDTEOSELECT_AW::NO)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut W {
        self.variant(ACMDTEOSELECT_AW::YES)
    }
}
#[doc = "Force Event for Auto CMD CRC Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACMDCRCSELECT_AW {
    #[doc = "0: No Interrupt"]
    NO = 0,
    #[doc = "1: Interrupt is generated"]
    YES = 1,
}
impl From<ACMDCRCSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: ACMDCRCSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMDCRC` writer - Force Event for Auto CMD CRC Error"]
pub type ACMDCRC_W<'a, const O: u8> = crate::BitWriter<'a, u16, FERACES_SPEC, ACMDCRCSELECT_AW, O>;
impl<'a, const O: u8> ACMDCRC_W<'a, O> {
    #[doc = "No Interrupt"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(ACMDCRCSELECT_AW::NO)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut W {
        self.variant(ACMDCRCSELECT_AW::YES)
    }
}
#[doc = "Force Event for Auto CMD End Bit Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACMDENDSELECT_AW {
    #[doc = "0: No Interrupt"]
    NO = 0,
    #[doc = "1: Interrupt is generated"]
    YES = 1,
}
impl From<ACMDENDSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: ACMDENDSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMDEND` writer - Force Event for Auto CMD End Bit Error"]
pub type ACMDEND_W<'a, const O: u8> = crate::BitWriter<'a, u16, FERACES_SPEC, ACMDENDSELECT_AW, O>;
impl<'a, const O: u8> ACMDEND_W<'a, O> {
    #[doc = "No Interrupt"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(ACMDENDSELECT_AW::NO)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut W {
        self.variant(ACMDENDSELECT_AW::YES)
    }
}
#[doc = "Force Event for Auto CMD Index Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACMDIDXSELECT_AW {
    #[doc = "0: No Interrupt"]
    NO = 0,
    #[doc = "1: Interrupt is generated"]
    YES = 1,
}
impl From<ACMDIDXSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: ACMDIDXSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMDIDX` writer - Force Event for Auto CMD Index Error"]
pub type ACMDIDX_W<'a, const O: u8> = crate::BitWriter<'a, u16, FERACES_SPEC, ACMDIDXSELECT_AW, O>;
impl<'a, const O: u8> ACMDIDX_W<'a, O> {
    #[doc = "No Interrupt"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(ACMDIDXSELECT_AW::NO)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut W {
        self.variant(ACMDIDXSELECT_AW::YES)
    }
}
#[doc = "Force Event for Command Not Issued By Auto CMD12 Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMDNISELECT_AW {
    #[doc = "0: No Interrupt"]
    NO = 0,
    #[doc = "1: Interrupt is generated"]
    YES = 1,
}
impl From<CMDNISELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: CMDNISELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDNI` writer - Force Event for Command Not Issued By Auto CMD12 Error"]
pub type CMDNI_W<'a, const O: u8> = crate::BitWriter<'a, u16, FERACES_SPEC, CMDNISELECT_AW, O>;
impl<'a, const O: u8> CMDNI_W<'a, O> {
    #[doc = "No Interrupt"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(CMDNISELECT_AW::NO)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut W {
        self.variant(CMDNISELECT_AW::YES)
    }
}
impl W {
    #[doc = "Bit 0 - Force Event for Auto CMD12 Not Executed"]
    #[inline(always)]
    #[must_use]
    pub fn acmd12ne(&mut self) -> ACMD12NE_W<0> {
        ACMD12NE_W::new(self)
    }
    #[doc = "Bit 1 - Force Event for Auto CMD Timeout Error"]
    #[inline(always)]
    #[must_use]
    pub fn acmdteo(&mut self) -> ACMDTEO_W<1> {
        ACMDTEO_W::new(self)
    }
    #[doc = "Bit 2 - Force Event for Auto CMD CRC Error"]
    #[inline(always)]
    #[must_use]
    pub fn acmdcrc(&mut self) -> ACMDCRC_W<2> {
        ACMDCRC_W::new(self)
    }
    #[doc = "Bit 3 - Force Event for Auto CMD End Bit Error"]
    #[inline(always)]
    #[must_use]
    pub fn acmdend(&mut self) -> ACMDEND_W<3> {
        ACMDEND_W::new(self)
    }
    #[doc = "Bit 4 - Force Event for Auto CMD Index Error"]
    #[inline(always)]
    #[must_use]
    pub fn acmdidx(&mut self) -> ACMDIDX_W<4> {
        ACMDIDX_W::new(self)
    }
    #[doc = "Bit 7 - Force Event for Command Not Issued By Auto CMD12 Error"]
    #[inline(always)]
    #[must_use]
    pub fn cmdni(&mut self) -> CMDNI_W<7> {
        CMDNI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Force Event for Auto CMD Error Status\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [feraces](index.html) module"]
pub struct FERACES_SPEC;
impl crate::RegisterSpec for FERACES_SPEC {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [feraces::W](W) writer structure"]
impl crate::Writable for FERACES_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FERACES to value 0"]
impl crate::Resettable for FERACES_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
