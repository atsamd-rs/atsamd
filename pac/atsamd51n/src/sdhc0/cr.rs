#[doc = "Register `CR` reader"]
pub type R = crate::R<CR_SPEC>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CR_SPEC>;
#[doc = "Field `RESPTYP` reader - Response Type"]
pub type RESPTYP_R = crate::FieldReader<RESPTYPSELECT_A>;
#[doc = "Response Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RESPTYPSELECT_A {
    #[doc = "0: No response"]
    NONE = 0,
    #[doc = "1: 136-bit response"]
    _136_BIT = 1,
    #[doc = "2: 48-bit response"]
    _48_BIT = 2,
    #[doc = "3: 48-bit response check busy after response"]
    _48_BIT_BUSY = 3,
}
impl From<RESPTYPSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: RESPTYPSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RESPTYPSELECT_A {
    type Ux = u8;
}
impl RESPTYP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RESPTYPSELECT_A {
        match self.bits {
            0 => RESPTYPSELECT_A::NONE,
            1 => RESPTYPSELECT_A::_136_BIT,
            2 => RESPTYPSELECT_A::_48_BIT,
            3 => RESPTYPSELECT_A::_48_BIT_BUSY,
            _ => unreachable!(),
        }
    }
    #[doc = "No response"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == RESPTYPSELECT_A::NONE
    }
    #[doc = "136-bit response"]
    #[inline(always)]
    pub fn is_136_bit(&self) -> bool {
        *self == RESPTYPSELECT_A::_136_BIT
    }
    #[doc = "48-bit response"]
    #[inline(always)]
    pub fn is_48_bit(&self) -> bool {
        *self == RESPTYPSELECT_A::_48_BIT
    }
    #[doc = "48-bit response check busy after response"]
    #[inline(always)]
    pub fn is_48_bit_busy(&self) -> bool {
        *self == RESPTYPSELECT_A::_48_BIT_BUSY
    }
}
#[doc = "Field `RESPTYP` writer - Response Type"]
pub type RESPTYP_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, RESPTYPSELECT_A>;
impl<'a, REG, const O: u8> RESPTYP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No response"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(RESPTYPSELECT_A::NONE)
    }
    #[doc = "136-bit response"]
    #[inline(always)]
    pub fn _136_bit(self) -> &'a mut crate::W<REG> {
        self.variant(RESPTYPSELECT_A::_136_BIT)
    }
    #[doc = "48-bit response"]
    #[inline(always)]
    pub fn _48_bit(self) -> &'a mut crate::W<REG> {
        self.variant(RESPTYPSELECT_A::_48_BIT)
    }
    #[doc = "48-bit response check busy after response"]
    #[inline(always)]
    pub fn _48_bit_busy(self) -> &'a mut crate::W<REG> {
        self.variant(RESPTYPSELECT_A::_48_BIT_BUSY)
    }
}
#[doc = "Field `CMDCCEN` reader - Command CRC Check Enable"]
pub type CMDCCEN_R = crate::BitReader<CMDCCENSELECT_A>;
#[doc = "Command CRC Check Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMDCCENSELECT_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<CMDCCENSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CMDCCENSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CMDCCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMDCCENSELECT_A {
        match self.bits {
            false => CMDCCENSELECT_A::DISABLE,
            true => CMDCCENSELECT_A::ENABLE,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CMDCCENSELECT_A::DISABLE
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CMDCCENSELECT_A::ENABLE
    }
}
#[doc = "Field `CMDCCEN` writer - Command CRC Check Enable"]
pub type CMDCCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CMDCCENSELECT_A>;
impl<'a, REG, const O: u8> CMDCCEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CMDCCENSELECT_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(CMDCCENSELECT_A::ENABLE)
    }
}
#[doc = "Field `CMDICEN` reader - Command Index Check Enable"]
pub type CMDICEN_R = crate::BitReader<CMDICENSELECT_A>;
#[doc = "Command Index Check Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMDICENSELECT_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<CMDICENSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CMDICENSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CMDICEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMDICENSELECT_A {
        match self.bits {
            false => CMDICENSELECT_A::DISABLE,
            true => CMDICENSELECT_A::ENABLE,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CMDICENSELECT_A::DISABLE
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CMDICENSELECT_A::ENABLE
    }
}
#[doc = "Field `CMDICEN` writer - Command Index Check Enable"]
pub type CMDICEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CMDICENSELECT_A>;
impl<'a, REG, const O: u8> CMDICEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CMDICENSELECT_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(CMDICENSELECT_A::ENABLE)
    }
}
#[doc = "Field `DPSEL` reader - Data Present Select"]
pub type DPSEL_R = crate::BitReader<DPSELSELECT_A>;
#[doc = "Data Present Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPSELSELECT_A {
    #[doc = "0: No Data Present"]
    NO_DATA = 0,
    #[doc = "1: Data Present"]
    DATA = 1,
}
impl From<DPSELSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: DPSELSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl DPSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DPSELSELECT_A {
        match self.bits {
            false => DPSELSELECT_A::NO_DATA,
            true => DPSELSELECT_A::DATA,
        }
    }
    #[doc = "No Data Present"]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == DPSELSELECT_A::NO_DATA
    }
    #[doc = "Data Present"]
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        *self == DPSELSELECT_A::DATA
    }
}
#[doc = "Field `DPSEL` writer - Data Present Select"]
pub type DPSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DPSELSELECT_A>;
impl<'a, REG, const O: u8> DPSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Data Present"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut crate::W<REG> {
        self.variant(DPSELSELECT_A::NO_DATA)
    }
    #[doc = "Data Present"]
    #[inline(always)]
    pub fn data(self) -> &'a mut crate::W<REG> {
        self.variant(DPSELSELECT_A::DATA)
    }
}
#[doc = "Field `CMDTYP` reader - Command Type"]
pub type CMDTYP_R = crate::FieldReader<CMDTYPSELECT_A>;
#[doc = "Command Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMDTYPSELECT_A {
    #[doc = "0: Other commands"]
    NORMAL = 0,
    #[doc = "1: CMD52 for writing Bus Suspend in CCCR"]
    SUSPEND = 1,
    #[doc = "2: CMD52 for writing Function Select in CCCR"]
    RESUME = 2,
    #[doc = "3: CMD12, CMD52 for writing I/O Abort in CCCR"]
    ABORT = 3,
}
impl From<CMDTYPSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CMDTYPSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CMDTYPSELECT_A {
    type Ux = u8;
}
impl CMDTYP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMDTYPSELECT_A {
        match self.bits {
            0 => CMDTYPSELECT_A::NORMAL,
            1 => CMDTYPSELECT_A::SUSPEND,
            2 => CMDTYPSELECT_A::RESUME,
            3 => CMDTYPSELECT_A::ABORT,
            _ => unreachable!(),
        }
    }
    #[doc = "Other commands"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == CMDTYPSELECT_A::NORMAL
    }
    #[doc = "CMD52 for writing Bus Suspend in CCCR"]
    #[inline(always)]
    pub fn is_suspend(&self) -> bool {
        *self == CMDTYPSELECT_A::SUSPEND
    }
    #[doc = "CMD52 for writing Function Select in CCCR"]
    #[inline(always)]
    pub fn is_resume(&self) -> bool {
        *self == CMDTYPSELECT_A::RESUME
    }
    #[doc = "CMD12, CMD52 for writing I/O Abort in CCCR"]
    #[inline(always)]
    pub fn is_abort(&self) -> bool {
        *self == CMDTYPSELECT_A::ABORT
    }
}
#[doc = "Field `CMDTYP` writer - Command Type"]
pub type CMDTYP_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, CMDTYPSELECT_A>;
impl<'a, REG, const O: u8> CMDTYP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Other commands"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(CMDTYPSELECT_A::NORMAL)
    }
    #[doc = "CMD52 for writing Bus Suspend in CCCR"]
    #[inline(always)]
    pub fn suspend(self) -> &'a mut crate::W<REG> {
        self.variant(CMDTYPSELECT_A::SUSPEND)
    }
    #[doc = "CMD52 for writing Function Select in CCCR"]
    #[inline(always)]
    pub fn resume(self) -> &'a mut crate::W<REG> {
        self.variant(CMDTYPSELECT_A::RESUME)
    }
    #[doc = "CMD12, CMD52 for writing I/O Abort in CCCR"]
    #[inline(always)]
    pub fn abort(self) -> &'a mut crate::W<REG> {
        self.variant(CMDTYPSELECT_A::ABORT)
    }
}
#[doc = "Field `CMDIDX` reader - Command Index"]
pub type CMDIDX_R = crate::FieldReader;
#[doc = "Field `CMDIDX` writer - Command Index"]
pub type CMDIDX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
impl R {
    #[doc = "Bits 0:1 - Response Type"]
    #[inline(always)]
    pub fn resptyp(&self) -> RESPTYP_R {
        RESPTYP_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Command CRC Check Enable"]
    #[inline(always)]
    pub fn cmdccen(&self) -> CMDCCEN_R {
        CMDCCEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Command Index Check Enable"]
    #[inline(always)]
    pub fn cmdicen(&self) -> CMDICEN_R {
        CMDICEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data Present Select"]
    #[inline(always)]
    pub fn dpsel(&self) -> DPSEL_R {
        DPSEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Command Type"]
    #[inline(always)]
    pub fn cmdtyp(&self) -> CMDTYP_R {
        CMDTYP_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:13 - Command Index"]
    #[inline(always)]
    pub fn cmdidx(&self) -> CMDIDX_R {
        CMDIDX_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Response Type"]
    #[inline(always)]
    #[must_use]
    pub fn resptyp(&mut self) -> RESPTYP_W<CR_SPEC, 0> {
        RESPTYP_W::new(self)
    }
    #[doc = "Bit 3 - Command CRC Check Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmdccen(&mut self) -> CMDCCEN_W<CR_SPEC, 3> {
        CMDCCEN_W::new(self)
    }
    #[doc = "Bit 4 - Command Index Check Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmdicen(&mut self) -> CMDICEN_W<CR_SPEC, 4> {
        CMDICEN_W::new(self)
    }
    #[doc = "Bit 5 - Data Present Select"]
    #[inline(always)]
    #[must_use]
    pub fn dpsel(&mut self) -> DPSEL_W<CR_SPEC, 5> {
        DPSEL_W::new(self)
    }
    #[doc = "Bits 6:7 - Command Type"]
    #[inline(always)]
    #[must_use]
    pub fn cmdtyp(&mut self) -> CMDTYP_W<CR_SPEC, 6> {
        CMDTYP_W::new(self)
    }
    #[doc = "Bits 8:13 - Command Index"]
    #[inline(always)]
    #[must_use]
    pub fn cmdidx(&mut self) -> CMDIDX_W<CR_SPEC, 8> {
        CMDIDX_W::new(self)
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
#[doc = "Command\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
