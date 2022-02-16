#[doc = "Register `PMC_MCKR` reader"]
pub struct R(crate::R<PMC_MCKR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMC_MCKR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMC_MCKR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMC_MCKR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMC_MCKR` writer"]
pub struct W(crate::W<PMC_MCKR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMC_MCKR_SPEC>;
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
impl From<crate::W<PMC_MCKR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMC_MCKR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Master Clock Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CSS_A {
    #[doc = "0: Slow Clock is selected"]
    SLOW_CLK = 0,
    #[doc = "1: Main Clock is selected"]
    MAIN_CLK = 1,
    #[doc = "2: PLLA Clock is selected"]
    PLLA_CLK = 2,
    #[doc = "3: Divided UPLL Clock is selected"]
    UPLL_CLK = 3,
}
impl From<CSS_A> for u8 {
    #[inline(always)]
    fn from(variant: CSS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CSS` reader - Master Clock Source Selection"]
pub struct CSS_R(crate::FieldReader<u8, CSS_A>);
impl CSS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CSS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSS_A {
        match self.bits {
            0 => CSS_A::SLOW_CLK,
            1 => CSS_A::MAIN_CLK,
            2 => CSS_A::PLLA_CLK,
            3 => CSS_A::UPLL_CLK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SLOW_CLK`"]
    #[inline(always)]
    pub fn is_slow_clk(&self) -> bool {
        **self == CSS_A::SLOW_CLK
    }
    #[doc = "Checks if the value of the field is `MAIN_CLK`"]
    #[inline(always)]
    pub fn is_main_clk(&self) -> bool {
        **self == CSS_A::MAIN_CLK
    }
    #[doc = "Checks if the value of the field is `PLLA_CLK`"]
    #[inline(always)]
    pub fn is_plla_clk(&self) -> bool {
        **self == CSS_A::PLLA_CLK
    }
    #[doc = "Checks if the value of the field is `UPLL_CLK`"]
    #[inline(always)]
    pub fn is_upll_clk(&self) -> bool {
        **self == CSS_A::UPLL_CLK
    }
}
impl core::ops::Deref for CSS_R {
    type Target = crate::FieldReader<u8, CSS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSS` writer - Master Clock Source Selection"]
pub struct CSS_W<'a> {
    w: &'a mut W,
}
impl<'a> CSS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Slow Clock is selected"]
    #[inline(always)]
    pub fn slow_clk(self) -> &'a mut W {
        self.variant(CSS_A::SLOW_CLK)
    }
    #[doc = "Main Clock is selected"]
    #[inline(always)]
    pub fn main_clk(self) -> &'a mut W {
        self.variant(CSS_A::MAIN_CLK)
    }
    #[doc = "PLLA Clock is selected"]
    #[inline(always)]
    pub fn plla_clk(self) -> &'a mut W {
        self.variant(CSS_A::PLLA_CLK)
    }
    #[doc = "Divided UPLL Clock is selected"]
    #[inline(always)]
    pub fn upll_clk(self) -> &'a mut W {
        self.variant(CSS_A::UPLL_CLK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Processor Clock Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRES_A {
    #[doc = "0: Selected clock"]
    CLK_1 = 0,
    #[doc = "1: Selected clock divided by 2"]
    CLK_2 = 1,
    #[doc = "2: Selected clock divided by 4"]
    CLK_4 = 2,
    #[doc = "3: Selected clock divided by 8"]
    CLK_8 = 3,
    #[doc = "4: Selected clock divided by 16"]
    CLK_16 = 4,
    #[doc = "5: Selected clock divided by 32"]
    CLK_32 = 5,
    #[doc = "6: Selected clock divided by 64"]
    CLK_64 = 6,
    #[doc = "7: Selected clock divided by 3"]
    CLK_3 = 7,
}
impl From<PRES_A> for u8 {
    #[inline(always)]
    fn from(variant: PRES_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PRES` reader - Processor Clock Prescaler"]
pub struct PRES_R(crate::FieldReader<u8, PRES_A>);
impl PRES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRES_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRES_A {
        match self.bits {
            0 => PRES_A::CLK_1,
            1 => PRES_A::CLK_2,
            2 => PRES_A::CLK_4,
            3 => PRES_A::CLK_8,
            4 => PRES_A::CLK_16,
            5 => PRES_A::CLK_32,
            6 => PRES_A::CLK_64,
            7 => PRES_A::CLK_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CLK_1`"]
    #[inline(always)]
    pub fn is_clk_1(&self) -> bool {
        **self == PRES_A::CLK_1
    }
    #[doc = "Checks if the value of the field is `CLK_2`"]
    #[inline(always)]
    pub fn is_clk_2(&self) -> bool {
        **self == PRES_A::CLK_2
    }
    #[doc = "Checks if the value of the field is `CLK_4`"]
    #[inline(always)]
    pub fn is_clk_4(&self) -> bool {
        **self == PRES_A::CLK_4
    }
    #[doc = "Checks if the value of the field is `CLK_8`"]
    #[inline(always)]
    pub fn is_clk_8(&self) -> bool {
        **self == PRES_A::CLK_8
    }
    #[doc = "Checks if the value of the field is `CLK_16`"]
    #[inline(always)]
    pub fn is_clk_16(&self) -> bool {
        **self == PRES_A::CLK_16
    }
    #[doc = "Checks if the value of the field is `CLK_32`"]
    #[inline(always)]
    pub fn is_clk_32(&self) -> bool {
        **self == PRES_A::CLK_32
    }
    #[doc = "Checks if the value of the field is `CLK_64`"]
    #[inline(always)]
    pub fn is_clk_64(&self) -> bool {
        **self == PRES_A::CLK_64
    }
    #[doc = "Checks if the value of the field is `CLK_3`"]
    #[inline(always)]
    pub fn is_clk_3(&self) -> bool {
        **self == PRES_A::CLK_3
    }
}
impl core::ops::Deref for PRES_R {
    type Target = crate::FieldReader<u8, PRES_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRES` writer - Processor Clock Prescaler"]
pub struct PRES_W<'a> {
    w: &'a mut W,
}
impl<'a> PRES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRES_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Selected clock"]
    #[inline(always)]
    pub fn clk_1(self) -> &'a mut W {
        self.variant(PRES_A::CLK_1)
    }
    #[doc = "Selected clock divided by 2"]
    #[inline(always)]
    pub fn clk_2(self) -> &'a mut W {
        self.variant(PRES_A::CLK_2)
    }
    #[doc = "Selected clock divided by 4"]
    #[inline(always)]
    pub fn clk_4(self) -> &'a mut W {
        self.variant(PRES_A::CLK_4)
    }
    #[doc = "Selected clock divided by 8"]
    #[inline(always)]
    pub fn clk_8(self) -> &'a mut W {
        self.variant(PRES_A::CLK_8)
    }
    #[doc = "Selected clock divided by 16"]
    #[inline(always)]
    pub fn clk_16(self) -> &'a mut W {
        self.variant(PRES_A::CLK_16)
    }
    #[doc = "Selected clock divided by 32"]
    #[inline(always)]
    pub fn clk_32(self) -> &'a mut W {
        self.variant(PRES_A::CLK_32)
    }
    #[doc = "Selected clock divided by 64"]
    #[inline(always)]
    pub fn clk_64(self) -> &'a mut W {
        self.variant(PRES_A::CLK_64)
    }
    #[doc = "Selected clock divided by 3"]
    #[inline(always)]
    pub fn clk_3(self) -> &'a mut W {
        self.variant(PRES_A::CLK_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Master Clock Division\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MDIV_A {
    #[doc = "0: Master Clock is Prescaler Output Clock divided by 1."]
    EQ_PCK = 0,
    #[doc = "1: Master Clock is Prescaler Output Clock divided by 2."]
    PCK_DIV2 = 1,
    #[doc = "2: Master Clock is Prescaler Output Clock divided by 4."]
    PCK_DIV4 = 2,
    #[doc = "3: Master Clock is Prescaler Output Clock divided by 3."]
    PCK_DIV3 = 3,
}
impl From<MDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: MDIV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MDIV` reader - Master Clock Division"]
pub struct MDIV_R(crate::FieldReader<u8, MDIV_A>);
impl MDIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MDIV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MDIV_A {
        match self.bits {
            0 => MDIV_A::EQ_PCK,
            1 => MDIV_A::PCK_DIV2,
            2 => MDIV_A::PCK_DIV4,
            3 => MDIV_A::PCK_DIV3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EQ_PCK`"]
    #[inline(always)]
    pub fn is_eq_pck(&self) -> bool {
        **self == MDIV_A::EQ_PCK
    }
    #[doc = "Checks if the value of the field is `PCK_DIV2`"]
    #[inline(always)]
    pub fn is_pck_div2(&self) -> bool {
        **self == MDIV_A::PCK_DIV2
    }
    #[doc = "Checks if the value of the field is `PCK_DIV4`"]
    #[inline(always)]
    pub fn is_pck_div4(&self) -> bool {
        **self == MDIV_A::PCK_DIV4
    }
    #[doc = "Checks if the value of the field is `PCK_DIV3`"]
    #[inline(always)]
    pub fn is_pck_div3(&self) -> bool {
        **self == MDIV_A::PCK_DIV3
    }
}
impl core::ops::Deref for MDIV_R {
    type Target = crate::FieldReader<u8, MDIV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MDIV` writer - Master Clock Division"]
pub struct MDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> MDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MDIV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Master Clock is Prescaler Output Clock divided by 1."]
    #[inline(always)]
    pub fn eq_pck(self) -> &'a mut W {
        self.variant(MDIV_A::EQ_PCK)
    }
    #[doc = "Master Clock is Prescaler Output Clock divided by 2."]
    #[inline(always)]
    pub fn pck_div2(self) -> &'a mut W {
        self.variant(MDIV_A::PCK_DIV2)
    }
    #[doc = "Master Clock is Prescaler Output Clock divided by 4."]
    #[inline(always)]
    pub fn pck_div4(self) -> &'a mut W {
        self.variant(MDIV_A::PCK_DIV4)
    }
    #[doc = "Master Clock is Prescaler Output Clock divided by 3."]
    #[inline(always)]
    pub fn pck_div3(self) -> &'a mut W {
        self.variant(MDIV_A::PCK_DIV3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `UPLLDIV2` reader - UPLL Divider by 2"]
pub struct UPLLDIV2_R(crate::FieldReader<bool, bool>);
impl UPLLDIV2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UPLLDIV2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UPLLDIV2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UPLLDIV2` writer - UPLL Divider by 2"]
pub struct UPLLDIV2_W<'a> {
    w: &'a mut W,
}
impl<'a> UPLLDIV2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Master Clock Source Selection"]
    #[inline(always)]
    pub fn css(&self) -> CSS_R {
        CSS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:6 - Processor Clock Prescaler"]
    #[inline(always)]
    pub fn pres(&self) -> PRES_R {
        PRES_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:9 - Master Clock Division"]
    #[inline(always)]
    pub fn mdiv(&self) -> MDIV_R {
        MDIV_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 13 - UPLL Divider by 2"]
    #[inline(always)]
    pub fn uplldiv2(&self) -> UPLLDIV2_R {
        UPLLDIV2_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Master Clock Source Selection"]
    #[inline(always)]
    pub fn css(&mut self) -> CSS_W {
        CSS_W { w: self }
    }
    #[doc = "Bits 4:6 - Processor Clock Prescaler"]
    #[inline(always)]
    pub fn pres(&mut self) -> PRES_W {
        PRES_W { w: self }
    }
    #[doc = "Bits 8:9 - Master Clock Division"]
    #[inline(always)]
    pub fn mdiv(&mut self) -> MDIV_W {
        MDIV_W { w: self }
    }
    #[doc = "Bit 13 - UPLL Divider by 2"]
    #[inline(always)]
    pub fn uplldiv2(&mut self) -> UPLLDIV2_W {
        UPLLDIV2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_mckr](index.html) module"]
pub struct PMC_MCKR_SPEC;
impl crate::RegisterSpec for PMC_MCKR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmc_mckr::R](R) reader structure"]
impl crate::Readable for PMC_MCKR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmc_mckr::W](W) writer structure"]
impl crate::Writable for PMC_MCKR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMC_MCKR to value 0"]
impl crate::Resettable for PMC_MCKR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
