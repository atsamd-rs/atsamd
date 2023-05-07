#[doc = "Register `CTRLA` reader"]
pub struct R(crate::R<CTRLA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLA` writer"]
pub struct W(crate::W<CTRLA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLA_SPEC>;
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
impl From<crate::W<CTRLA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AUTOWS` reader - Auto Wait State Enable"]
pub type AUTOWS_R = crate::BitReader<bool>;
#[doc = "Field `AUTOWS` writer - Auto Wait State Enable"]
pub type AUTOWS_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRLA_SPEC, bool, O>;
#[doc = "Field `SUSPEN` reader - Suspend Enable"]
pub type SUSPEN_R = crate::BitReader<bool>;
#[doc = "Field `SUSPEN` writer - Suspend Enable"]
pub type SUSPEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRLA_SPEC, bool, O>;
#[doc = "Field `WMODE` reader - Write Mode"]
pub type WMODE_R = crate::FieldReader<u8, WMODESELECT_A>;
#[doc = "Write Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WMODESELECT_A {
    #[doc = "0: Manual Write"]
    MAN = 0,
    #[doc = "1: Automatic Double Word Write"]
    ADW = 1,
    #[doc = "2: Automatic Quad Word"]
    AQW = 2,
    #[doc = "3: Automatic Page Write"]
    AP = 3,
}
impl From<WMODESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: WMODESELECT_A) -> Self {
        variant as _
    }
}
impl WMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WMODESELECT_A {
        match self.bits {
            0 => WMODESELECT_A::MAN,
            1 => WMODESELECT_A::ADW,
            2 => WMODESELECT_A::AQW,
            3 => WMODESELECT_A::AP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MAN`"]
    #[inline(always)]
    pub fn is_man(&self) -> bool {
        *self == WMODESELECT_A::MAN
    }
    #[doc = "Checks if the value of the field is `ADW`"]
    #[inline(always)]
    pub fn is_adw(&self) -> bool {
        *self == WMODESELECT_A::ADW
    }
    #[doc = "Checks if the value of the field is `AQW`"]
    #[inline(always)]
    pub fn is_aqw(&self) -> bool {
        *self == WMODESELECT_A::AQW
    }
    #[doc = "Checks if the value of the field is `AP`"]
    #[inline(always)]
    pub fn is_ap(&self) -> bool {
        *self == WMODESELECT_A::AP
    }
}
#[doc = "Field `WMODE` writer - Write Mode"]
pub type WMODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, CTRLA_SPEC, u8, WMODESELECT_A, 2, O>;
impl<'a, const O: u8> WMODE_W<'a, O> {
    #[doc = "Manual Write"]
    #[inline(always)]
    pub fn man(self) -> &'a mut W {
        self.variant(WMODESELECT_A::MAN)
    }
    #[doc = "Automatic Double Word Write"]
    #[inline(always)]
    pub fn adw(self) -> &'a mut W {
        self.variant(WMODESELECT_A::ADW)
    }
    #[doc = "Automatic Quad Word"]
    #[inline(always)]
    pub fn aqw(self) -> &'a mut W {
        self.variant(WMODESELECT_A::AQW)
    }
    #[doc = "Automatic Page Write"]
    #[inline(always)]
    pub fn ap(self) -> &'a mut W {
        self.variant(WMODESELECT_A::AP)
    }
}
#[doc = "Field `PRM` reader - Power Reduction Mode during Sleep"]
pub type PRM_R = crate::FieldReader<u8, PRMSELECT_A>;
#[doc = "Power Reduction Mode during Sleep\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRMSELECT_A {
    #[doc = "0: NVM block enters low-power mode when entering standby mode. NVM block enters low-power mode when SPRM command is issued. NVM block exits low-power mode upon first access."]
    SEMIAUTO = 0,
    #[doc = "1: NVM block enters low-power mode when entering standby mode. NVM block enters low-power mode when SPRM command is issued. NVM block exits low-power mode when system is not in standby mode."]
    FULLAUTO = 1,
    #[doc = "3: NVM block does not enter low-power mode when entering standby mode. NVM block enters low-power mode when SPRM command is issued. NVM block exits low-power mode upon first access."]
    MANUAL = 3,
}
impl From<PRMSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PRMSELECT_A) -> Self {
        variant as _
    }
}
impl PRM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PRMSELECT_A> {
        match self.bits {
            0 => Some(PRMSELECT_A::SEMIAUTO),
            1 => Some(PRMSELECT_A::FULLAUTO),
            3 => Some(PRMSELECT_A::MANUAL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SEMIAUTO`"]
    #[inline(always)]
    pub fn is_semiauto(&self) -> bool {
        *self == PRMSELECT_A::SEMIAUTO
    }
    #[doc = "Checks if the value of the field is `FULLAUTO`"]
    #[inline(always)]
    pub fn is_fullauto(&self) -> bool {
        *self == PRMSELECT_A::FULLAUTO
    }
    #[doc = "Checks if the value of the field is `MANUAL`"]
    #[inline(always)]
    pub fn is_manual(&self) -> bool {
        *self == PRMSELECT_A::MANUAL
    }
}
#[doc = "Field `PRM` writer - Power Reduction Mode during Sleep"]
pub type PRM_W<'a, const O: u8> = crate::FieldWriter<'a, u16, CTRLA_SPEC, u8, PRMSELECT_A, 2, O>;
impl<'a, const O: u8> PRM_W<'a, O> {
    #[doc = "NVM block enters low-power mode when entering standby mode. NVM block enters low-power mode when SPRM command is issued. NVM block exits low-power mode upon first access."]
    #[inline(always)]
    pub fn semiauto(self) -> &'a mut W {
        self.variant(PRMSELECT_A::SEMIAUTO)
    }
    #[doc = "NVM block enters low-power mode when entering standby mode. NVM block enters low-power mode when SPRM command is issued. NVM block exits low-power mode when system is not in standby mode."]
    #[inline(always)]
    pub fn fullauto(self) -> &'a mut W {
        self.variant(PRMSELECT_A::FULLAUTO)
    }
    #[doc = "NVM block does not enter low-power mode when entering standby mode. NVM block enters low-power mode when SPRM command is issued. NVM block exits low-power mode upon first access."]
    #[inline(always)]
    pub fn manual(self) -> &'a mut W {
        self.variant(PRMSELECT_A::MANUAL)
    }
}
#[doc = "Field `RWS` reader - NVM Read Wait States"]
pub type RWS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RWS` writer - NVM Read Wait States"]
pub type RWS_W<'a, const O: u8> = crate::FieldWriter<'a, u16, CTRLA_SPEC, u8, u8, 4, O>;
#[doc = "Field `AHBNS0` reader - Force AHB0 access to NONSEQ, burst transfers are continuously rearbitrated"]
pub type AHBNS0_R = crate::BitReader<bool>;
#[doc = "Field `AHBNS0` writer - Force AHB0 access to NONSEQ, burst transfers are continuously rearbitrated"]
pub type AHBNS0_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRLA_SPEC, bool, O>;
#[doc = "Field `AHBNS1` reader - Force AHB1 access to NONSEQ, burst transfers are continuously rearbitrated"]
pub type AHBNS1_R = crate::BitReader<bool>;
#[doc = "Field `AHBNS1` writer - Force AHB1 access to NONSEQ, burst transfers are continuously rearbitrated"]
pub type AHBNS1_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRLA_SPEC, bool, O>;
#[doc = "Field `CACHEDIS0` reader - AHB0 Cache Disable"]
pub type CACHEDIS0_R = crate::BitReader<bool>;
#[doc = "Field `CACHEDIS0` writer - AHB0 Cache Disable"]
pub type CACHEDIS0_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRLA_SPEC, bool, O>;
#[doc = "Field `CACHEDIS1` reader - AHB1 Cache Disable"]
pub type CACHEDIS1_R = crate::BitReader<bool>;
#[doc = "Field `CACHEDIS1` writer - AHB1 Cache Disable"]
pub type CACHEDIS1_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRLA_SPEC, bool, O>;
impl R {
    #[doc = "Bit 2 - Auto Wait State Enable"]
    #[inline(always)]
    pub fn autows(&self) -> AUTOWS_R {
        AUTOWS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Suspend Enable"]
    #[inline(always)]
    pub fn suspen(&self) -> SUSPEN_R {
        SUSPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Write Mode"]
    #[inline(always)]
    pub fn wmode(&self) -> WMODE_R {
        WMODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Power Reduction Mode during Sleep"]
    #[inline(always)]
    pub fn prm(&self) -> PRM_R {
        PRM_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - NVM Read Wait States"]
    #[inline(always)]
    pub fn rws(&self) -> RWS_R {
        RWS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Force AHB0 access to NONSEQ, burst transfers are continuously rearbitrated"]
    #[inline(always)]
    pub fn ahbns0(&self) -> AHBNS0_R {
        AHBNS0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Force AHB1 access to NONSEQ, burst transfers are continuously rearbitrated"]
    #[inline(always)]
    pub fn ahbns1(&self) -> AHBNS1_R {
        AHBNS1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - AHB0 Cache Disable"]
    #[inline(always)]
    pub fn cachedis0(&self) -> CACHEDIS0_R {
        CACHEDIS0_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - AHB1 Cache Disable"]
    #[inline(always)]
    pub fn cachedis1(&self) -> CACHEDIS1_R {
        CACHEDIS1_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Auto Wait State Enable"]
    #[inline(always)]
    #[must_use]
    pub fn autows(&mut self) -> AUTOWS_W<2> {
        AUTOWS_W::new(self)
    }
    #[doc = "Bit 3 - Suspend Enable"]
    #[inline(always)]
    #[must_use]
    pub fn suspen(&mut self) -> SUSPEN_W<3> {
        SUSPEN_W::new(self)
    }
    #[doc = "Bits 4:5 - Write Mode"]
    #[inline(always)]
    #[must_use]
    pub fn wmode(&mut self) -> WMODE_W<4> {
        WMODE_W::new(self)
    }
    #[doc = "Bits 6:7 - Power Reduction Mode during Sleep"]
    #[inline(always)]
    #[must_use]
    pub fn prm(&mut self) -> PRM_W<6> {
        PRM_W::new(self)
    }
    #[doc = "Bits 8:11 - NVM Read Wait States"]
    #[inline(always)]
    #[must_use]
    pub fn rws(&mut self) -> RWS_W<8> {
        RWS_W::new(self)
    }
    #[doc = "Bit 12 - Force AHB0 access to NONSEQ, burst transfers are continuously rearbitrated"]
    #[inline(always)]
    #[must_use]
    pub fn ahbns0(&mut self) -> AHBNS0_W<12> {
        AHBNS0_W::new(self)
    }
    #[doc = "Bit 13 - Force AHB1 access to NONSEQ, burst transfers are continuously rearbitrated"]
    #[inline(always)]
    #[must_use]
    pub fn ahbns1(&mut self) -> AHBNS1_W<13> {
        AHBNS1_W::new(self)
    }
    #[doc = "Bit 14 - AHB0 Cache Disable"]
    #[inline(always)]
    #[must_use]
    pub fn cachedis0(&mut self) -> CACHEDIS0_W<14> {
        CACHEDIS0_W::new(self)
    }
    #[doc = "Bit 15 - AHB1 Cache Disable"]
    #[inline(always)]
    #[must_use]
    pub fn cachedis1(&mut self) -> CACHEDIS1_W<15> {
        CACHEDIS1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrla](index.html) module"]
pub struct CTRLA_SPEC;
impl crate::RegisterSpec for CTRLA_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ctrla::R](R) reader structure"]
impl crate::Readable for CTRLA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrla::W](W) writer structure"]
impl crate::Writable for CTRLA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLA to value 0x04"]
impl crate::Resettable for CTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
