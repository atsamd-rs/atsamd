#[doc = "Register `CTRLB` reader"]
pub type R = crate::R<CTRLB_SPEC>;
#[doc = "Register `CTRLB` writer"]
pub type W = crate::W<CTRLB_SPEC>;
#[doc = "Field `MODE` reader - Serial Memory Mode"]
pub type MODE_R = crate::BitReader<MODESELECT_A>;
#[doc = "Serial Memory Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODESELECT_A {
    #[doc = "0: SPI operating mode"]
    SPI = 0,
    #[doc = "1: Serial Memory operating mode"]
    MEMORY = 1,
}
impl From<MODESELECT_A> for bool {
    #[inline(always)]
    fn from(variant: MODESELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODESELECT_A {
        match self.bits {
            false => MODESELECT_A::SPI,
            true => MODESELECT_A::MEMORY,
        }
    }
    #[doc = "SPI operating mode"]
    #[inline(always)]
    pub fn is_spi(&self) -> bool {
        *self == MODESELECT_A::SPI
    }
    #[doc = "Serial Memory operating mode"]
    #[inline(always)]
    pub fn is_memory(&self) -> bool {
        *self == MODESELECT_A::MEMORY
    }
}
#[doc = "Field `MODE` writer - Serial Memory Mode"]
pub type MODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MODESELECT_A>;
impl<'a, REG, const O: u8> MODE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI operating mode"]
    #[inline(always)]
    pub fn spi(self) -> &'a mut crate::W<REG> {
        self.variant(MODESELECT_A::SPI)
    }
    #[doc = "Serial Memory operating mode"]
    #[inline(always)]
    pub fn memory(self) -> &'a mut crate::W<REG> {
        self.variant(MODESELECT_A::MEMORY)
    }
}
#[doc = "Field `LOOPEN` reader - Local Loopback Enable"]
pub type LOOPEN_R = crate::BitReader;
#[doc = "Field `LOOPEN` writer - Local Loopback Enable"]
pub type LOOPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WDRBT` reader - Wait Data Read Before Transfer"]
pub type WDRBT_R = crate::BitReader;
#[doc = "Field `WDRBT` writer - Wait Data Read Before Transfer"]
pub type WDRBT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SMEMREG` reader - Serial Memory reg"]
pub type SMEMREG_R = crate::BitReader;
#[doc = "Field `SMEMREG` writer - Serial Memory reg"]
pub type SMEMREG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CSMODE` reader - Chip Select Mode"]
pub type CSMODE_R = crate::FieldReader<CSMODESELECT_A>;
#[doc = "Chip Select Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CSMODESELECT_A {
    #[doc = "0: The chip select is deasserted if TD has not been reloaded before the end of the current transfer."]
    NORELOAD = 0,
    #[doc = "1: The chip select is deasserted when the bit LASTXFER is written at 1 and the character written in TD has been transferred."]
    LASTXFER = 1,
    #[doc = "2: The chip select is deasserted systematically after each transfer."]
    SYSTEMATICALLY = 2,
}
impl From<CSMODESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CSMODESELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CSMODESELECT_A {
    type Ux = u8;
}
impl CSMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CSMODESELECT_A> {
        match self.bits {
            0 => Some(CSMODESELECT_A::NORELOAD),
            1 => Some(CSMODESELECT_A::LASTXFER),
            2 => Some(CSMODESELECT_A::SYSTEMATICALLY),
            _ => None,
        }
    }
    #[doc = "The chip select is deasserted if TD has not been reloaded before the end of the current transfer."]
    #[inline(always)]
    pub fn is_noreload(&self) -> bool {
        *self == CSMODESELECT_A::NORELOAD
    }
    #[doc = "The chip select is deasserted when the bit LASTXFER is written at 1 and the character written in TD has been transferred."]
    #[inline(always)]
    pub fn is_lastxfer(&self) -> bool {
        *self == CSMODESELECT_A::LASTXFER
    }
    #[doc = "The chip select is deasserted systematically after each transfer."]
    #[inline(always)]
    pub fn is_systematically(&self) -> bool {
        *self == CSMODESELECT_A::SYSTEMATICALLY
    }
}
#[doc = "Field `CSMODE` writer - Chip Select Mode"]
pub type CSMODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, CSMODESELECT_A>;
impl<'a, REG, const O: u8> CSMODE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The chip select is deasserted if TD has not been reloaded before the end of the current transfer."]
    #[inline(always)]
    pub fn noreload(self) -> &'a mut crate::W<REG> {
        self.variant(CSMODESELECT_A::NORELOAD)
    }
    #[doc = "The chip select is deasserted when the bit LASTXFER is written at 1 and the character written in TD has been transferred."]
    #[inline(always)]
    pub fn lastxfer(self) -> &'a mut crate::W<REG> {
        self.variant(CSMODESELECT_A::LASTXFER)
    }
    #[doc = "The chip select is deasserted systematically after each transfer."]
    #[inline(always)]
    pub fn systematically(self) -> &'a mut crate::W<REG> {
        self.variant(CSMODESELECT_A::SYSTEMATICALLY)
    }
}
#[doc = "Field `DATALEN` reader - Data Length"]
pub type DATALEN_R = crate::FieldReader<DATALENSELECT_A>;
#[doc = "Data Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DATALENSELECT_A {
    #[doc = "0: 8-bits transfer"]
    _8BITS = 0,
    #[doc = "1: 9 bits transfer"]
    _9BITS = 1,
    #[doc = "2: 10-bits transfer"]
    _10BITS = 2,
    #[doc = "3: 11-bits transfer"]
    _11BITS = 3,
    #[doc = "4: 12-bits transfer"]
    _12BITS = 4,
    #[doc = "5: 13-bits transfer"]
    _13BITS = 5,
    #[doc = "6: 14-bits transfer"]
    _14BITS = 6,
    #[doc = "7: 15-bits transfer"]
    _15BITS = 7,
    #[doc = "8: 16-bits transfer"]
    _16BITS = 8,
}
impl From<DATALENSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: DATALENSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DATALENSELECT_A {
    type Ux = u8;
}
impl DATALEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DATALENSELECT_A> {
        match self.bits {
            0 => Some(DATALENSELECT_A::_8BITS),
            1 => Some(DATALENSELECT_A::_9BITS),
            2 => Some(DATALENSELECT_A::_10BITS),
            3 => Some(DATALENSELECT_A::_11BITS),
            4 => Some(DATALENSELECT_A::_12BITS),
            5 => Some(DATALENSELECT_A::_13BITS),
            6 => Some(DATALENSELECT_A::_14BITS),
            7 => Some(DATALENSELECT_A::_15BITS),
            8 => Some(DATALENSELECT_A::_16BITS),
            _ => None,
        }
    }
    #[doc = "8-bits transfer"]
    #[inline(always)]
    pub fn is_8bits(&self) -> bool {
        *self == DATALENSELECT_A::_8BITS
    }
    #[doc = "9 bits transfer"]
    #[inline(always)]
    pub fn is_9bits(&self) -> bool {
        *self == DATALENSELECT_A::_9BITS
    }
    #[doc = "10-bits transfer"]
    #[inline(always)]
    pub fn is_10bits(&self) -> bool {
        *self == DATALENSELECT_A::_10BITS
    }
    #[doc = "11-bits transfer"]
    #[inline(always)]
    pub fn is_11bits(&self) -> bool {
        *self == DATALENSELECT_A::_11BITS
    }
    #[doc = "12-bits transfer"]
    #[inline(always)]
    pub fn is_12bits(&self) -> bool {
        *self == DATALENSELECT_A::_12BITS
    }
    #[doc = "13-bits transfer"]
    #[inline(always)]
    pub fn is_13bits(&self) -> bool {
        *self == DATALENSELECT_A::_13BITS
    }
    #[doc = "14-bits transfer"]
    #[inline(always)]
    pub fn is_14bits(&self) -> bool {
        *self == DATALENSELECT_A::_14BITS
    }
    #[doc = "15-bits transfer"]
    #[inline(always)]
    pub fn is_15bits(&self) -> bool {
        *self == DATALENSELECT_A::_15BITS
    }
    #[doc = "16-bits transfer"]
    #[inline(always)]
    pub fn is_16bits(&self) -> bool {
        *self == DATALENSELECT_A::_16BITS
    }
}
#[doc = "Field `DATALEN` writer - Data Length"]
pub type DATALEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, DATALENSELECT_A>;
impl<'a, REG, const O: u8> DATALEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8-bits transfer"]
    #[inline(always)]
    pub fn _8bits(self) -> &'a mut crate::W<REG> {
        self.variant(DATALENSELECT_A::_8BITS)
    }
    #[doc = "9 bits transfer"]
    #[inline(always)]
    pub fn _9bits(self) -> &'a mut crate::W<REG> {
        self.variant(DATALENSELECT_A::_9BITS)
    }
    #[doc = "10-bits transfer"]
    #[inline(always)]
    pub fn _10bits(self) -> &'a mut crate::W<REG> {
        self.variant(DATALENSELECT_A::_10BITS)
    }
    #[doc = "11-bits transfer"]
    #[inline(always)]
    pub fn _11bits(self) -> &'a mut crate::W<REG> {
        self.variant(DATALENSELECT_A::_11BITS)
    }
    #[doc = "12-bits transfer"]
    #[inline(always)]
    pub fn _12bits(self) -> &'a mut crate::W<REG> {
        self.variant(DATALENSELECT_A::_12BITS)
    }
    #[doc = "13-bits transfer"]
    #[inline(always)]
    pub fn _13bits(self) -> &'a mut crate::W<REG> {
        self.variant(DATALENSELECT_A::_13BITS)
    }
    #[doc = "14-bits transfer"]
    #[inline(always)]
    pub fn _14bits(self) -> &'a mut crate::W<REG> {
        self.variant(DATALENSELECT_A::_14BITS)
    }
    #[doc = "15-bits transfer"]
    #[inline(always)]
    pub fn _15bits(self) -> &'a mut crate::W<REG> {
        self.variant(DATALENSELECT_A::_15BITS)
    }
    #[doc = "16-bits transfer"]
    #[inline(always)]
    pub fn _16bits(self) -> &'a mut crate::W<REG> {
        self.variant(DATALENSELECT_A::_16BITS)
    }
}
#[doc = "Field `DLYBCT` reader - Delay Between Consecutive Transfers"]
pub type DLYBCT_R = crate::FieldReader;
#[doc = "Field `DLYBCT` writer - Delay Between Consecutive Transfers"]
pub type DLYBCT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `DLYCS` reader - Minimum Inactive CS Delay"]
pub type DLYCS_R = crate::FieldReader;
#[doc = "Field `DLYCS` writer - Minimum Inactive CS Delay"]
pub type DLYCS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bit 0 - Serial Memory Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Local Loopback Enable"]
    #[inline(always)]
    pub fn loopen(&self) -> LOOPEN_R {
        LOOPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wait Data Read Before Transfer"]
    #[inline(always)]
    pub fn wdrbt(&self) -> WDRBT_R {
        WDRBT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Serial Memory reg"]
    #[inline(always)]
    pub fn smemreg(&self) -> SMEMREG_R {
        SMEMREG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Chip Select Mode"]
    #[inline(always)]
    pub fn csmode(&self) -> CSMODE_R {
        CSMODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:11 - Data Length"]
    #[inline(always)]
    pub fn datalen(&self) -> DATALEN_R {
        DATALEN_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - Delay Between Consecutive Transfers"]
    #[inline(always)]
    pub fn dlybct(&self) -> DLYBCT_R {
        DLYBCT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Minimum Inactive CS Delay"]
    #[inline(always)]
    pub fn dlycs(&self) -> DLYCS_R {
        DLYCS_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Serial Memory Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<CTRLB_SPEC, 0> {
        MODE_W::new(self)
    }
    #[doc = "Bit 1 - Local Loopback Enable"]
    #[inline(always)]
    #[must_use]
    pub fn loopen(&mut self) -> LOOPEN_W<CTRLB_SPEC, 1> {
        LOOPEN_W::new(self)
    }
    #[doc = "Bit 2 - Wait Data Read Before Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn wdrbt(&mut self) -> WDRBT_W<CTRLB_SPEC, 2> {
        WDRBT_W::new(self)
    }
    #[doc = "Bit 3 - Serial Memory reg"]
    #[inline(always)]
    #[must_use]
    pub fn smemreg(&mut self) -> SMEMREG_W<CTRLB_SPEC, 3> {
        SMEMREG_W::new(self)
    }
    #[doc = "Bits 4:5 - Chip Select Mode"]
    #[inline(always)]
    #[must_use]
    pub fn csmode(&mut self) -> CSMODE_W<CTRLB_SPEC, 4> {
        CSMODE_W::new(self)
    }
    #[doc = "Bits 8:11 - Data Length"]
    #[inline(always)]
    #[must_use]
    pub fn datalen(&mut self) -> DATALEN_W<CTRLB_SPEC, 8> {
        DATALEN_W::new(self)
    }
    #[doc = "Bits 16:23 - Delay Between Consecutive Transfers"]
    #[inline(always)]
    #[must_use]
    pub fn dlybct(&mut self) -> DLYBCT_W<CTRLB_SPEC, 16> {
        DLYBCT_W::new(self)
    }
    #[doc = "Bits 24:31 - Minimum Inactive CS Delay"]
    #[inline(always)]
    #[must_use]
    pub fn dlycs(&mut self) -> DLYCS_W<CTRLB_SPEC, 24> {
        DLYCS_W::new(self)
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
#[doc = "Control B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLB_SPEC;
impl crate::RegisterSpec for CTRLB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlb::R`](R) reader structure"]
impl crate::Readable for CTRLB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrlb::W`](W) writer structure"]
impl crate::Writable for CTRLB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for CTRLB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
