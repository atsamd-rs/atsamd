#[doc = "Register `HC1R` reader"]
pub type R = crate::R<HC1R_SPEC>;
#[doc = "Register `HC1R` writer"]
pub type W = crate::W<HC1R_SPEC>;
#[doc = "Field `LEDCTRL` reader - LED Control"]
pub type LEDCTRL_R = crate::BitReader<LEDCTRLSELECT_A>;
#[doc = "LED Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LEDCTRLSELECT_A {
    #[doc = "0: LED off"]
    OFF = 0,
    #[doc = "1: LED on"]
    ON = 1,
}
impl From<LEDCTRLSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: LEDCTRLSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl LEDCTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LEDCTRLSELECT_A {
        match self.bits {
            false => LEDCTRLSELECT_A::OFF,
            true => LEDCTRLSELECT_A::ON,
        }
    }
    #[doc = "LED off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == LEDCTRLSELECT_A::OFF
    }
    #[doc = "LED on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == LEDCTRLSELECT_A::ON
    }
}
#[doc = "Field `LEDCTRL` writer - LED Control"]
pub type LEDCTRL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, LEDCTRLSELECT_A>;
impl<'a, REG, const O: u8> LEDCTRL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LED off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(LEDCTRLSELECT_A::OFF)
    }
    #[doc = "LED on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(LEDCTRLSELECT_A::ON)
    }
}
#[doc = "Field `DW` reader - Data Width"]
pub type DW_R = crate::BitReader<DWSELECT_A>;
#[doc = "Data Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DWSELECT_A {
    #[doc = "0: 1-bit mode"]
    _1BIT = 0,
    #[doc = "1: 4-bit mode"]
    _4BIT = 1,
}
impl From<DWSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: DWSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl DW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DWSELECT_A {
        match self.bits {
            false => DWSELECT_A::_1BIT,
            true => DWSELECT_A::_4BIT,
        }
    }
    #[doc = "1-bit mode"]
    #[inline(always)]
    pub fn is_1bit(&self) -> bool {
        *self == DWSELECT_A::_1BIT
    }
    #[doc = "4-bit mode"]
    #[inline(always)]
    pub fn is_4bit(&self) -> bool {
        *self == DWSELECT_A::_4BIT
    }
}
#[doc = "Field `DW` writer - Data Width"]
pub type DW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DWSELECT_A>;
impl<'a, REG, const O: u8> DW_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "1-bit mode"]
    #[inline(always)]
    pub fn _1bit(self) -> &'a mut crate::W<REG> {
        self.variant(DWSELECT_A::_1BIT)
    }
    #[doc = "4-bit mode"]
    #[inline(always)]
    pub fn _4bit(self) -> &'a mut crate::W<REG> {
        self.variant(DWSELECT_A::_4BIT)
    }
}
#[doc = "Field `HSEN` reader - High Speed Enable"]
pub type HSEN_R = crate::BitReader<HSENSELECT_A>;
#[doc = "High Speed Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSENSELECT_A {
    #[doc = "0: Normal Speed mode"]
    NORMAL = 0,
    #[doc = "1: High Speed mode"]
    HIGH = 1,
}
impl From<HSENSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: HSENSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl HSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSENSELECT_A {
        match self.bits {
            false => HSENSELECT_A::NORMAL,
            true => HSENSELECT_A::HIGH,
        }
    }
    #[doc = "Normal Speed mode"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == HSENSELECT_A::NORMAL
    }
    #[doc = "High Speed mode"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == HSENSELECT_A::HIGH
    }
}
#[doc = "Field `HSEN` writer - High Speed Enable"]
pub type HSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, HSENSELECT_A>;
impl<'a, REG, const O: u8> HSEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal Speed mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(HSENSELECT_A::NORMAL)
    }
    #[doc = "High Speed mode"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(HSENSELECT_A::HIGH)
    }
}
#[doc = "Field `DMASEL` reader - DMA Select"]
pub type DMASEL_R = crate::FieldReader<DMASELSELECT_A>;
#[doc = "DMA Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMASELSELECT_A {
    #[doc = "0: SDMA is selected"]
    SDMA = 0,
    #[doc = "2: 32-bit Address ADMA2 is selected"]
    _32BIT = 2,
}
impl From<DMASELSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: DMASELSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DMASELSELECT_A {
    type Ux = u8;
}
impl DMASEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DMASELSELECT_A> {
        match self.bits {
            0 => Some(DMASELSELECT_A::SDMA),
            2 => Some(DMASELSELECT_A::_32BIT),
            _ => None,
        }
    }
    #[doc = "SDMA is selected"]
    #[inline(always)]
    pub fn is_sdma(&self) -> bool {
        *self == DMASELSELECT_A::SDMA
    }
    #[doc = "32-bit Address ADMA2 is selected"]
    #[inline(always)]
    pub fn is_32bit(&self) -> bool {
        *self == DMASELSELECT_A::_32BIT
    }
}
#[doc = "Field `DMASEL` writer - DMA Select"]
pub type DMASEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, DMASELSELECT_A>;
impl<'a, REG, const O: u8> DMASEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SDMA is selected"]
    #[inline(always)]
    pub fn sdma(self) -> &'a mut crate::W<REG> {
        self.variant(DMASELSELECT_A::SDMA)
    }
    #[doc = "32-bit Address ADMA2 is selected"]
    #[inline(always)]
    pub fn _32bit(self) -> &'a mut crate::W<REG> {
        self.variant(DMASELSELECT_A::_32BIT)
    }
}
#[doc = "Field `CARDDTL` reader - Card Detect Test Level"]
pub type CARDDTL_R = crate::BitReader<CARDDTLSELECT_A>;
#[doc = "Card Detect Test Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CARDDTLSELECT_A {
    #[doc = "0: No Card"]
    NO = 0,
    #[doc = "1: Card Inserted"]
    YES = 1,
}
impl From<CARDDTLSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CARDDTLSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CARDDTL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CARDDTLSELECT_A {
        match self.bits {
            false => CARDDTLSELECT_A::NO,
            true => CARDDTLSELECT_A::YES,
        }
    }
    #[doc = "No Card"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == CARDDTLSELECT_A::NO
    }
    #[doc = "Card Inserted"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == CARDDTLSELECT_A::YES
    }
}
#[doc = "Field `CARDDTL` writer - Card Detect Test Level"]
pub type CARDDTL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CARDDTLSELECT_A>;
impl<'a, REG, const O: u8> CARDDTL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Card"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(CARDDTLSELECT_A::NO)
    }
    #[doc = "Card Inserted"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(CARDDTLSELECT_A::YES)
    }
}
#[doc = "Field `CARDDSEL` reader - Card Detect Signal Selection"]
pub type CARDDSEL_R = crate::BitReader<CARDDSELSELECT_A>;
#[doc = "Card Detect Signal Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CARDDSELSELECT_A {
    #[doc = "0: SDCD# is selected (for normal use)"]
    NORMAL = 0,
    #[doc = "1: The Card Select Test Level is selected (for test purpose)"]
    TEST = 1,
}
impl From<CARDDSELSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CARDDSELSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CARDDSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CARDDSELSELECT_A {
        match self.bits {
            false => CARDDSELSELECT_A::NORMAL,
            true => CARDDSELSELECT_A::TEST,
        }
    }
    #[doc = "SDCD# is selected (for normal use)"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == CARDDSELSELECT_A::NORMAL
    }
    #[doc = "The Card Select Test Level is selected (for test purpose)"]
    #[inline(always)]
    pub fn is_test(&self) -> bool {
        *self == CARDDSELSELECT_A::TEST
    }
}
#[doc = "Field `CARDDSEL` writer - Card Detect Signal Selection"]
pub type CARDDSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CARDDSELSELECT_A>;
impl<'a, REG, const O: u8> CARDDSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SDCD# is selected (for normal use)"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(CARDDSELSELECT_A::NORMAL)
    }
    #[doc = "The Card Select Test Level is selected (for test purpose)"]
    #[inline(always)]
    pub fn test(self) -> &'a mut crate::W<REG> {
        self.variant(CARDDSELSELECT_A::TEST)
    }
}
impl R {
    #[doc = "Bit 0 - LED Control"]
    #[inline(always)]
    pub fn ledctrl(&self) -> LEDCTRL_R {
        LEDCTRL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data Width"]
    #[inline(always)]
    pub fn dw(&self) -> DW_R {
        DW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - High Speed Enable"]
    #[inline(always)]
    pub fn hsen(&self) -> HSEN_R {
        HSEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - DMA Select"]
    #[inline(always)]
    pub fn dmasel(&self) -> DMASEL_R {
        DMASEL_R::new((self.bits >> 3) & 3)
    }
    #[doc = "Bit 6 - Card Detect Test Level"]
    #[inline(always)]
    pub fn carddtl(&self) -> CARDDTL_R {
        CARDDTL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Card Detect Signal Selection"]
    #[inline(always)]
    pub fn carddsel(&self) -> CARDDSEL_R {
        CARDDSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LED Control"]
    #[inline(always)]
    #[must_use]
    pub fn ledctrl(&mut self) -> LEDCTRL_W<HC1R_SPEC, 0> {
        LEDCTRL_W::new(self)
    }
    #[doc = "Bit 1 - Data Width"]
    #[inline(always)]
    #[must_use]
    pub fn dw(&mut self) -> DW_W<HC1R_SPEC, 1> {
        DW_W::new(self)
    }
    #[doc = "Bit 2 - High Speed Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hsen(&mut self) -> HSEN_W<HC1R_SPEC, 2> {
        HSEN_W::new(self)
    }
    #[doc = "Bits 3:4 - DMA Select"]
    #[inline(always)]
    #[must_use]
    pub fn dmasel(&mut self) -> DMASEL_W<HC1R_SPEC, 3> {
        DMASEL_W::new(self)
    }
    #[doc = "Bit 6 - Card Detect Test Level"]
    #[inline(always)]
    #[must_use]
    pub fn carddtl(&mut self) -> CARDDTL_W<HC1R_SPEC, 6> {
        CARDDTL_W::new(self)
    }
    #[doc = "Bit 7 - Card Detect Signal Selection"]
    #[inline(always)]
    #[must_use]
    pub fn carddsel(&mut self) -> CARDDSEL_W<HC1R_SPEC, 7> {
        CARDDSEL_W::new(self)
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
#[doc = "Host Control 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc1r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc1r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HC1R_SPEC;
impl crate::RegisterSpec for HC1R_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hc1r::R`](R) reader structure"]
impl crate::Readable for HC1R_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hc1r::W`](W) writer structure"]
impl crate::Writable for HC1R_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HC1R to value 0"]
impl crate::Resettable for HC1R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
