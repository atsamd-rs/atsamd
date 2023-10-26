#[doc = "Register `TMR` reader"]
pub type R = crate::R<TMR_SPEC>;
#[doc = "Register `TMR` writer"]
pub type W = crate::W<TMR_SPEC>;
#[doc = "Field `DMAEN` reader - DMA Enable"]
pub type DMAEN_R = crate::BitReader<DMAENSELECT_A>;
#[doc = "DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAENSELECT_A {
    #[doc = "0: No data transfer or Non DMA data transfer"]
    DISABLE = 0,
    #[doc = "1: DMA data transfer"]
    ENABLE = 1,
}
impl From<DMAENSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: DMAENSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMAENSELECT_A {
        match self.bits {
            false => DMAENSELECT_A::DISABLE,
            true => DMAENSELECT_A::ENABLE,
        }
    }
    #[doc = "No data transfer or Non DMA data transfer"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DMAENSELECT_A::DISABLE
    }
    #[doc = "DMA data transfer"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DMAENSELECT_A::ENABLE
    }
}
#[doc = "Field `DMAEN` writer - DMA Enable"]
pub type DMAEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DMAENSELECT_A>;
impl<'a, REG, const O: u8> DMAEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No data transfer or Non DMA data transfer"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(DMAENSELECT_A::DISABLE)
    }
    #[doc = "DMA data transfer"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(DMAENSELECT_A::ENABLE)
    }
}
#[doc = "Field `BCEN` reader - Block Count Enable"]
pub type BCEN_R = crate::BitReader<BCENSELECT_A>;
#[doc = "Block Count Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BCENSELECT_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<BCENSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: BCENSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl BCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BCENSELECT_A {
        match self.bits {
            false => BCENSELECT_A::DISABLE,
            true => BCENSELECT_A::ENABLE,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BCENSELECT_A::DISABLE
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BCENSELECT_A::ENABLE
    }
}
#[doc = "Field `BCEN` writer - Block Count Enable"]
pub type BCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, BCENSELECT_A>;
impl<'a, REG, const O: u8> BCEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(BCENSELECT_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(BCENSELECT_A::ENABLE)
    }
}
#[doc = "Field `ACMDEN` reader - Auto Command Enable"]
pub type ACMDEN_R = crate::FieldReader<ACMDENSELECT_A>;
#[doc = "Auto Command Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ACMDENSELECT_A {
    #[doc = "0: Auto Command Disabled"]
    DISABLED = 0,
    #[doc = "1: Auto CMD12 Enable"]
    CMD12 = 1,
    #[doc = "2: Auto CMD23 Enable"]
    CMD23 = 2,
}
impl From<ACMDENSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: ACMDENSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ACMDENSELECT_A {
    type Ux = u8;
}
impl ACMDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ACMDENSELECT_A> {
        match self.bits {
            0 => Some(ACMDENSELECT_A::DISABLED),
            1 => Some(ACMDENSELECT_A::CMD12),
            2 => Some(ACMDENSELECT_A::CMD23),
            _ => None,
        }
    }
    #[doc = "Auto Command Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ACMDENSELECT_A::DISABLED
    }
    #[doc = "Auto CMD12 Enable"]
    #[inline(always)]
    pub fn is_cmd12(&self) -> bool {
        *self == ACMDENSELECT_A::CMD12
    }
    #[doc = "Auto CMD23 Enable"]
    #[inline(always)]
    pub fn is_cmd23(&self) -> bool {
        *self == ACMDENSELECT_A::CMD23
    }
}
#[doc = "Field `ACMDEN` writer - Auto Command Enable"]
pub type ACMDEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, ACMDENSELECT_A>;
impl<'a, REG, const O: u8> ACMDEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Auto Command Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ACMDENSELECT_A::DISABLED)
    }
    #[doc = "Auto CMD12 Enable"]
    #[inline(always)]
    pub fn cmd12(self) -> &'a mut crate::W<REG> {
        self.variant(ACMDENSELECT_A::CMD12)
    }
    #[doc = "Auto CMD23 Enable"]
    #[inline(always)]
    pub fn cmd23(self) -> &'a mut crate::W<REG> {
        self.variant(ACMDENSELECT_A::CMD23)
    }
}
#[doc = "Field `DTDSEL` reader - Data Transfer Direction Selection"]
pub type DTDSEL_R = crate::BitReader<DTDSELSELECT_A>;
#[doc = "Data Transfer Direction Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTDSELSELECT_A {
    #[doc = "0: Write (Host to Card)"]
    WRITE = 0,
    #[doc = "1: Read (Card to Host)"]
    READ = 1,
}
impl From<DTDSELSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: DTDSELSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl DTDSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DTDSELSELECT_A {
        match self.bits {
            false => DTDSELSELECT_A::WRITE,
            true => DTDSELSELECT_A::READ,
        }
    }
    #[doc = "Write (Host to Card)"]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        *self == DTDSELSELECT_A::WRITE
    }
    #[doc = "Read (Card to Host)"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == DTDSELSELECT_A::READ
    }
}
#[doc = "Field `DTDSEL` writer - Data Transfer Direction Selection"]
pub type DTDSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DTDSELSELECT_A>;
impl<'a, REG, const O: u8> DTDSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write (Host to Card)"]
    #[inline(always)]
    pub fn write(self) -> &'a mut crate::W<REG> {
        self.variant(DTDSELSELECT_A::WRITE)
    }
    #[doc = "Read (Card to Host)"]
    #[inline(always)]
    pub fn read(self) -> &'a mut crate::W<REG> {
        self.variant(DTDSELSELECT_A::READ)
    }
}
#[doc = "Field `MSBSEL` reader - Multi/Single Block Selection"]
pub type MSBSEL_R = crate::BitReader<MSBSELSELECT_A>;
#[doc = "Multi/Single Block Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSBSELSELECT_A {
    #[doc = "0: Single Block"]
    SINGLE = 0,
    #[doc = "1: Multiple Block"]
    MULTIPLE = 1,
}
impl From<MSBSELSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: MSBSELSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl MSBSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MSBSELSELECT_A {
        match self.bits {
            false => MSBSELSELECT_A::SINGLE,
            true => MSBSELSELECT_A::MULTIPLE,
        }
    }
    #[doc = "Single Block"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == MSBSELSELECT_A::SINGLE
    }
    #[doc = "Multiple Block"]
    #[inline(always)]
    pub fn is_multiple(&self) -> bool {
        *self == MSBSELSELECT_A::MULTIPLE
    }
}
#[doc = "Field `MSBSEL` writer - Multi/Single Block Selection"]
pub type MSBSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MSBSELSELECT_A>;
impl<'a, REG, const O: u8> MSBSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single Block"]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(MSBSELSELECT_A::SINGLE)
    }
    #[doc = "Multiple Block"]
    #[inline(always)]
    pub fn multiple(self) -> &'a mut crate::W<REG> {
        self.variant(MSBSELSELECT_A::MULTIPLE)
    }
}
impl R {
    #[doc = "Bit 0 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Block Count Enable"]
    #[inline(always)]
    pub fn bcen(&self) -> BCEN_R {
        BCEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Auto Command Enable"]
    #[inline(always)]
    pub fn acmden(&self) -> ACMDEN_R {
        ACMDEN_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Data Transfer Direction Selection"]
    #[inline(always)]
    pub fn dtdsel(&self) -> DTDSEL_R {
        DTDSEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Multi/Single Block Selection"]
    #[inline(always)]
    pub fn msbsel(&self) -> MSBSEL_R {
        MSBSEL_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<TMR_SPEC, 0> {
        DMAEN_W::new(self)
    }
    #[doc = "Bit 1 - Block Count Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bcen(&mut self) -> BCEN_W<TMR_SPEC, 1> {
        BCEN_W::new(self)
    }
    #[doc = "Bits 2:3 - Auto Command Enable"]
    #[inline(always)]
    #[must_use]
    pub fn acmden(&mut self) -> ACMDEN_W<TMR_SPEC, 2> {
        ACMDEN_W::new(self)
    }
    #[doc = "Bit 4 - Data Transfer Direction Selection"]
    #[inline(always)]
    #[must_use]
    pub fn dtdsel(&mut self) -> DTDSEL_W<TMR_SPEC, 4> {
        DTDSEL_W::new(self)
    }
    #[doc = "Bit 5 - Multi/Single Block Selection"]
    #[inline(always)]
    #[must_use]
    pub fn msbsel(&mut self) -> MSBSEL_W<TMR_SPEC, 5> {
        MSBSEL_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Transfer Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TMR_SPEC;
impl crate::RegisterSpec for TMR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tmr::R`](R) reader structure"]
impl crate::Readable for TMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tmr::W`](W) writer structure"]
impl crate::Writable for TMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TMR to value 0"]
impl crate::Resettable for TMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
