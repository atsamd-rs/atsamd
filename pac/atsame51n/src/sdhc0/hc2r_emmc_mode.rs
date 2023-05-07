#[doc = "Register `HC2R_EMMC_MODE` reader"]
pub struct R(crate::R<HC2R_EMMC_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HC2R_EMMC_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HC2R_EMMC_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HC2R_EMMC_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HC2R_EMMC_MODE` writer"]
pub struct W(crate::W<HC2R_EMMC_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HC2R_EMMC_MODE_SPEC>;
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
impl From<crate::W<HC2R_EMMC_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HC2R_EMMC_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HS200EN` reader - HS200 Mode Enable"]
pub type HS200EN_R = crate::FieldReader<u8, HS200ENSELECT_A>;
#[doc = "HS200 Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HS200ENSELECT_A {
    #[doc = "0: SDR12"]
    SDR12 = 0,
    #[doc = "1: SDR25"]
    SDR25 = 1,
    #[doc = "2: SDR50"]
    SDR50 = 2,
    #[doc = "3: SDR104"]
    SDR104 = 3,
    #[doc = "4: DDR50"]
    DDR50 = 4,
}
impl From<HS200ENSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: HS200ENSELECT_A) -> Self {
        variant as _
    }
}
impl HS200EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<HS200ENSELECT_A> {
        match self.bits {
            0 => Some(HS200ENSELECT_A::SDR12),
            1 => Some(HS200ENSELECT_A::SDR25),
            2 => Some(HS200ENSELECT_A::SDR50),
            3 => Some(HS200ENSELECT_A::SDR104),
            4 => Some(HS200ENSELECT_A::DDR50),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SDR12`"]
    #[inline(always)]
    pub fn is_sdr12(&self) -> bool {
        *self == HS200ENSELECT_A::SDR12
    }
    #[doc = "Checks if the value of the field is `SDR25`"]
    #[inline(always)]
    pub fn is_sdr25(&self) -> bool {
        *self == HS200ENSELECT_A::SDR25
    }
    #[doc = "Checks if the value of the field is `SDR50`"]
    #[inline(always)]
    pub fn is_sdr50(&self) -> bool {
        *self == HS200ENSELECT_A::SDR50
    }
    #[doc = "Checks if the value of the field is `SDR104`"]
    #[inline(always)]
    pub fn is_sdr104(&self) -> bool {
        *self == HS200ENSELECT_A::SDR104
    }
    #[doc = "Checks if the value of the field is `DDR50`"]
    #[inline(always)]
    pub fn is_ddr50(&self) -> bool {
        *self == HS200ENSELECT_A::DDR50
    }
}
#[doc = "Field `HS200EN` writer - HS200 Mode Enable"]
pub type HS200EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, HC2R_EMMC_MODE_SPEC, u8, HS200ENSELECT_A, 4, O>;
impl<'a, const O: u8> HS200EN_W<'a, O> {
    #[doc = "SDR12"]
    #[inline(always)]
    pub fn sdr12(self) -> &'a mut W {
        self.variant(HS200ENSELECT_A::SDR12)
    }
    #[doc = "SDR25"]
    #[inline(always)]
    pub fn sdr25(self) -> &'a mut W {
        self.variant(HS200ENSELECT_A::SDR25)
    }
    #[doc = "SDR50"]
    #[inline(always)]
    pub fn sdr50(self) -> &'a mut W {
        self.variant(HS200ENSELECT_A::SDR50)
    }
    #[doc = "SDR104"]
    #[inline(always)]
    pub fn sdr104(self) -> &'a mut W {
        self.variant(HS200ENSELECT_A::SDR104)
    }
    #[doc = "DDR50"]
    #[inline(always)]
    pub fn ddr50(self) -> &'a mut W {
        self.variant(HS200ENSELECT_A::DDR50)
    }
}
#[doc = "Field `DRVSEL` reader - Driver Strength Select"]
pub type DRVSEL_R = crate::FieldReader<u8, DRVSELSELECT_A>;
#[doc = "Driver Strength Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DRVSELSELECT_A {
    #[doc = "0: Driver Type B is Selected (Default)"]
    B = 0,
    #[doc = "1: Driver Type A is Selected"]
    A = 1,
    #[doc = "2: Driver Type C is Selected"]
    C = 2,
    #[doc = "3: Driver Type D is Selected"]
    D = 3,
}
impl From<DRVSELSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: DRVSELSELECT_A) -> Self {
        variant as _
    }
}
impl DRVSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRVSELSELECT_A {
        match self.bits {
            0 => DRVSELSELECT_A::B,
            1 => DRVSELSELECT_A::A,
            2 => DRVSELSELECT_A::C,
            3 => DRVSELSELECT_A::D,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B`"]
    #[inline(always)]
    pub fn is_b(&self) -> bool {
        *self == DRVSELSELECT_A::B
    }
    #[doc = "Checks if the value of the field is `A`"]
    #[inline(always)]
    pub fn is_a(&self) -> bool {
        *self == DRVSELSELECT_A::A
    }
    #[doc = "Checks if the value of the field is `C`"]
    #[inline(always)]
    pub fn is_c(&self) -> bool {
        *self == DRVSELSELECT_A::C
    }
    #[doc = "Checks if the value of the field is `D`"]
    #[inline(always)]
    pub fn is_d(&self) -> bool {
        *self == DRVSELSELECT_A::D
    }
}
#[doc = "Field `DRVSEL` writer - Driver Strength Select"]
pub type DRVSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, HC2R_EMMC_MODE_SPEC, u8, DRVSELSELECT_A, 2, O>;
impl<'a, const O: u8> DRVSEL_W<'a, O> {
    #[doc = "Driver Type B is Selected (Default)"]
    #[inline(always)]
    pub fn b(self) -> &'a mut W {
        self.variant(DRVSELSELECT_A::B)
    }
    #[doc = "Driver Type A is Selected"]
    #[inline(always)]
    pub fn a(self) -> &'a mut W {
        self.variant(DRVSELSELECT_A::A)
    }
    #[doc = "Driver Type C is Selected"]
    #[inline(always)]
    pub fn c(self) -> &'a mut W {
        self.variant(DRVSELSELECT_A::C)
    }
    #[doc = "Driver Type D is Selected"]
    #[inline(always)]
    pub fn d(self) -> &'a mut W {
        self.variant(DRVSELSELECT_A::D)
    }
}
#[doc = "Field `EXTUN` reader - Execute Tuning"]
pub type EXTUN_R = crate::BitReader<EXTUNSELECT_A>;
#[doc = "Execute Tuning\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXTUNSELECT_A {
    #[doc = "0: Not Tuned or Tuning Completed"]
    NO = 0,
    #[doc = "1: Execute Tuning"]
    REQUESTED = 1,
}
impl From<EXTUNSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: EXTUNSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl EXTUN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTUNSELECT_A {
        match self.bits {
            false => EXTUNSELECT_A::NO,
            true => EXTUNSELECT_A::REQUESTED,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == EXTUNSELECT_A::NO
    }
    #[doc = "Checks if the value of the field is `REQUESTED`"]
    #[inline(always)]
    pub fn is_requested(&self) -> bool {
        *self == EXTUNSELECT_A::REQUESTED
    }
}
#[doc = "Field `EXTUN` writer - Execute Tuning"]
pub type EXTUN_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, HC2R_EMMC_MODE_SPEC, EXTUNSELECT_A, O>;
impl<'a, const O: u8> EXTUN_W<'a, O> {
    #[doc = "Not Tuned or Tuning Completed"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(EXTUNSELECT_A::NO)
    }
    #[doc = "Execute Tuning"]
    #[inline(always)]
    pub fn requested(self) -> &'a mut W {
        self.variant(EXTUNSELECT_A::REQUESTED)
    }
}
#[doc = "Field `SLCKSEL` reader - Sampling Clock Select"]
pub type SLCKSEL_R = crate::BitReader<SLCKSELSELECT_A>;
#[doc = "Sampling Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLCKSELSELECT_A {
    #[doc = "0: Fixed clock is used to sample data"]
    FIXED = 0,
    #[doc = "1: Tuned clock is used to sample data"]
    TUNED = 1,
}
impl From<SLCKSELSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: SLCKSELSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl SLCKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLCKSELSELECT_A {
        match self.bits {
            false => SLCKSELSELECT_A::FIXED,
            true => SLCKSELSELECT_A::TUNED,
        }
    }
    #[doc = "Checks if the value of the field is `FIXED`"]
    #[inline(always)]
    pub fn is_fixed(&self) -> bool {
        *self == SLCKSELSELECT_A::FIXED
    }
    #[doc = "Checks if the value of the field is `TUNED`"]
    #[inline(always)]
    pub fn is_tuned(&self) -> bool {
        *self == SLCKSELSELECT_A::TUNED
    }
}
#[doc = "Field `SLCKSEL` writer - Sampling Clock Select"]
pub type SLCKSEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, HC2R_EMMC_MODE_SPEC, SLCKSELSELECT_A, O>;
impl<'a, const O: u8> SLCKSEL_W<'a, O> {
    #[doc = "Fixed clock is used to sample data"]
    #[inline(always)]
    pub fn fixed(self) -> &'a mut W {
        self.variant(SLCKSELSELECT_A::FIXED)
    }
    #[doc = "Tuned clock is used to sample data"]
    #[inline(always)]
    pub fn tuned(self) -> &'a mut W {
        self.variant(SLCKSELSELECT_A::TUNED)
    }
}
#[doc = "Field `PVALEN` reader - Preset Value Enable"]
pub type PVALEN_R = crate::BitReader<PVALENSELECT_A>;
#[doc = "Preset Value Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVALENSELECT_A {
    #[doc = "0: SDCLK and Driver Strength are controlled by Host Controller"]
    HOST = 0,
    #[doc = "1: Automatic Selection by Preset Value is Enabled"]
    AUTO = 1,
}
impl From<PVALENSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: PVALENSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl PVALEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PVALENSELECT_A {
        match self.bits {
            false => PVALENSELECT_A::HOST,
            true => PVALENSELECT_A::AUTO,
        }
    }
    #[doc = "Checks if the value of the field is `HOST`"]
    #[inline(always)]
    pub fn is_host(&self) -> bool {
        *self == PVALENSELECT_A::HOST
    }
    #[doc = "Checks if the value of the field is `AUTO`"]
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        *self == PVALENSELECT_A::AUTO
    }
}
#[doc = "Field `PVALEN` writer - Preset Value Enable"]
pub type PVALEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, HC2R_EMMC_MODE_SPEC, PVALENSELECT_A, O>;
impl<'a, const O: u8> PVALEN_W<'a, O> {
    #[doc = "SDCLK and Driver Strength are controlled by Host Controller"]
    #[inline(always)]
    pub fn host(self) -> &'a mut W {
        self.variant(PVALENSELECT_A::HOST)
    }
    #[doc = "Automatic Selection by Preset Value is Enabled"]
    #[inline(always)]
    pub fn auto(self) -> &'a mut W {
        self.variant(PVALENSELECT_A::AUTO)
    }
}
impl R {
    #[doc = "Bits 0:3 - HS200 Mode Enable"]
    #[inline(always)]
    pub fn hs200en(&self) -> HS200EN_R {
        HS200EN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Driver Strength Select"]
    #[inline(always)]
    pub fn drvsel(&self) -> DRVSEL_R {
        DRVSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Execute Tuning"]
    #[inline(always)]
    pub fn extun(&self) -> EXTUN_R {
        EXTUN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Sampling Clock Select"]
    #[inline(always)]
    pub fn slcksel(&self) -> SLCKSEL_R {
        SLCKSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 15 - Preset Value Enable"]
    #[inline(always)]
    pub fn pvalen(&self) -> PVALEN_R {
        PVALEN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - HS200 Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hs200en(&mut self) -> HS200EN_W<0> {
        HS200EN_W::new(self)
    }
    #[doc = "Bits 4:5 - Driver Strength Select"]
    #[inline(always)]
    #[must_use]
    pub fn drvsel(&mut self) -> DRVSEL_W<4> {
        DRVSEL_W::new(self)
    }
    #[doc = "Bit 6 - Execute Tuning"]
    #[inline(always)]
    #[must_use]
    pub fn extun(&mut self) -> EXTUN_W<6> {
        EXTUN_W::new(self)
    }
    #[doc = "Bit 7 - Sampling Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn slcksel(&mut self) -> SLCKSEL_W<7> {
        SLCKSEL_W::new(self)
    }
    #[doc = "Bit 15 - Preset Value Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pvalen(&mut self) -> PVALEN_W<15> {
        PVALEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Control 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc2r_emmc_mode](index.html) module"]
pub struct HC2R_EMMC_MODE_SPEC;
impl crate::RegisterSpec for HC2R_EMMC_MODE_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [hc2r_emmc_mode::R](R) reader structure"]
impl crate::Readable for HC2R_EMMC_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hc2r_emmc_mode::W](W) writer structure"]
impl crate::Writable for HC2R_EMMC_MODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HC2R_EMMC_MODE to value 0"]
impl crate::Resettable for HC2R_EMMC_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
