#[doc = "Register `CRCCTRL` reader"]
pub type R = crate::R<CRCCTRL_SPEC>;
#[doc = "Register `CRCCTRL` writer"]
pub type W = crate::W<CRCCTRL_SPEC>;
#[doc = "Field `CRCBEATSIZE` reader - CRC Beat Size"]
pub type CRCBEATSIZE_R = crate::FieldReader<CRCBEATSIZESELECT_A>;
#[doc = "CRC Beat Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CRCBEATSIZESELECT_A {
    #[doc = "0: 8-bit bus transfer"]
    BYTE = 0,
    #[doc = "1: 16-bit bus transfer"]
    HWORD = 1,
    #[doc = "2: 32-bit bus transfer"]
    WORD = 2,
}
impl From<CRCBEATSIZESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CRCBEATSIZESELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CRCBEATSIZESELECT_A {
    type Ux = u8;
}
impl CRCBEATSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CRCBEATSIZESELECT_A> {
        match self.bits {
            0 => Some(CRCBEATSIZESELECT_A::BYTE),
            1 => Some(CRCBEATSIZESELECT_A::HWORD),
            2 => Some(CRCBEATSIZESELECT_A::WORD),
            _ => None,
        }
    }
    #[doc = "8-bit bus transfer"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == CRCBEATSIZESELECT_A::BYTE
    }
    #[doc = "16-bit bus transfer"]
    #[inline(always)]
    pub fn is_hword(&self) -> bool {
        *self == CRCBEATSIZESELECT_A::HWORD
    }
    #[doc = "32-bit bus transfer"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == CRCBEATSIZESELECT_A::WORD
    }
}
#[doc = "Field `CRCBEATSIZE` writer - CRC Beat Size"]
pub type CRCBEATSIZE_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O, CRCBEATSIZESELECT_A>;
impl<'a, REG, const O: u8> CRCBEATSIZE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8-bit bus transfer"]
    #[inline(always)]
    pub fn byte(self) -> &'a mut crate::W<REG> {
        self.variant(CRCBEATSIZESELECT_A::BYTE)
    }
    #[doc = "16-bit bus transfer"]
    #[inline(always)]
    pub fn hword(self) -> &'a mut crate::W<REG> {
        self.variant(CRCBEATSIZESELECT_A::HWORD)
    }
    #[doc = "32-bit bus transfer"]
    #[inline(always)]
    pub fn word(self) -> &'a mut crate::W<REG> {
        self.variant(CRCBEATSIZESELECT_A::WORD)
    }
}
#[doc = "Field `CRCPOLY` reader - CRC Polynomial Type"]
pub type CRCPOLY_R = crate::FieldReader<CRCPOLYSELECT_A>;
#[doc = "CRC Polynomial Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CRCPOLYSELECT_A {
    #[doc = "0: CRC-16 (CRC-CCITT)"]
    CRC16 = 0,
    #[doc = "1: CRC32 (IEEE 802.3)"]
    CRC32 = 1,
}
impl From<CRCPOLYSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CRCPOLYSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CRCPOLYSELECT_A {
    type Ux = u8;
}
impl CRCPOLY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CRCPOLYSELECT_A> {
        match self.bits {
            0 => Some(CRCPOLYSELECT_A::CRC16),
            1 => Some(CRCPOLYSELECT_A::CRC32),
            _ => None,
        }
    }
    #[doc = "CRC-16 (CRC-CCITT)"]
    #[inline(always)]
    pub fn is_crc16(&self) -> bool {
        *self == CRCPOLYSELECT_A::CRC16
    }
    #[doc = "CRC32 (IEEE 802.3)"]
    #[inline(always)]
    pub fn is_crc32(&self) -> bool {
        *self == CRCPOLYSELECT_A::CRC32
    }
}
#[doc = "Field `CRCPOLY` writer - CRC Polynomial Type"]
pub type CRCPOLY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, CRCPOLYSELECT_A>;
impl<'a, REG, const O: u8> CRCPOLY_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CRC-16 (CRC-CCITT)"]
    #[inline(always)]
    pub fn crc16(self) -> &'a mut crate::W<REG> {
        self.variant(CRCPOLYSELECT_A::CRC16)
    }
    #[doc = "CRC32 (IEEE 802.3)"]
    #[inline(always)]
    pub fn crc32(self) -> &'a mut crate::W<REG> {
        self.variant(CRCPOLYSELECT_A::CRC32)
    }
}
#[doc = "Field `CRCSRC` reader - CRC Input Source"]
pub type CRCSRC_R = crate::FieldReader<CRCSRCSELECT_A>;
#[doc = "CRC Input Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CRCSRCSELECT_A {
    #[doc = "0: No action"]
    NOACT = 0,
    #[doc = "1: I/O interface"]
    IO = 1,
}
impl From<CRCSRCSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CRCSRCSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CRCSRCSELECT_A {
    type Ux = u8;
}
impl CRCSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CRCSRCSELECT_A> {
        match self.bits {
            0 => Some(CRCSRCSELECT_A::NOACT),
            1 => Some(CRCSRCSELECT_A::IO),
            _ => None,
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn is_noact(&self) -> bool {
        *self == CRCSRCSELECT_A::NOACT
    }
    #[doc = "I/O interface"]
    #[inline(always)]
    pub fn is_io(&self) -> bool {
        *self == CRCSRCSELECT_A::IO
    }
}
#[doc = "Field `CRCSRC` writer - CRC Input Source"]
pub type CRCSRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O, CRCSRCSELECT_A>;
impl<'a, REG, const O: u8> CRCSRC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn noact(self) -> &'a mut crate::W<REG> {
        self.variant(CRCSRCSELECT_A::NOACT)
    }
    #[doc = "I/O interface"]
    #[inline(always)]
    pub fn io(self) -> &'a mut crate::W<REG> {
        self.variant(CRCSRCSELECT_A::IO)
    }
}
impl R {
    #[doc = "Bits 0:1 - CRC Beat Size"]
    #[inline(always)]
    pub fn crcbeatsize(&self) -> CRCBEATSIZE_R {
        CRCBEATSIZE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - CRC Polynomial Type"]
    #[inline(always)]
    pub fn crcpoly(&self) -> CRCPOLY_R {
        CRCPOLY_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 8:13 - CRC Input Source"]
    #[inline(always)]
    pub fn crcsrc(&self) -> CRCSRC_R {
        CRCSRC_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CRC Beat Size"]
    #[inline(always)]
    #[must_use]
    pub fn crcbeatsize(&mut self) -> CRCBEATSIZE_W<CRCCTRL_SPEC, 0> {
        CRCBEATSIZE_W::new(self)
    }
    #[doc = "Bits 2:3 - CRC Polynomial Type"]
    #[inline(always)]
    #[must_use]
    pub fn crcpoly(&mut self) -> CRCPOLY_W<CRCCTRL_SPEC, 2> {
        CRCPOLY_W::new(self)
    }
    #[doc = "Bits 8:13 - CRC Input Source"]
    #[inline(always)]
    #[must_use]
    pub fn crcsrc(&mut self) -> CRCSRC_W<CRCCTRL_SPEC, 8> {
        CRCSRC_W::new(self)
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
#[doc = "CRC Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crcctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crcctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRCCTRL_SPEC;
impl crate::RegisterSpec for CRCCTRL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`crcctrl::R`](R) reader structure"]
impl crate::Readable for CRCCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`crcctrl::W`](W) writer structure"]
impl crate::Writable for CRCCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRCCTRL to value 0"]
impl crate::Resettable for CRCCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
