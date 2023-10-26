#[doc = "Register `CTRLB` reader"]
pub type R = crate::R<CTRLB_SPEC>;
#[doc = "Register `CTRLB` writer"]
pub type W = crate::W<CTRLB_SPEC>;
#[doc = "Field `CHSIZE` reader - Character Size"]
pub type CHSIZE_R = crate::FieldReader<CHSIZESELECT_A>;
#[doc = "Character Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CHSIZESELECT_A {
    #[doc = "0: 8 bits"]
    _8_BIT = 0,
    #[doc = "1: 9 bits"]
    _9_BIT = 1,
}
impl From<CHSIZESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CHSIZESELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CHSIZESELECT_A {
    type Ux = u8;
}
impl CHSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CHSIZESELECT_A> {
        match self.bits {
            0 => Some(CHSIZESELECT_A::_8_BIT),
            1 => Some(CHSIZESELECT_A::_9_BIT),
            _ => None,
        }
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn is_8_bit(&self) -> bool {
        *self == CHSIZESELECT_A::_8_BIT
    }
    #[doc = "9 bits"]
    #[inline(always)]
    pub fn is_9_bit(&self) -> bool {
        *self == CHSIZESELECT_A::_9_BIT
    }
}
#[doc = "Field `CHSIZE` writer - Character Size"]
pub type CHSIZE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, CHSIZESELECT_A>;
impl<'a, REG, const O: u8> CHSIZE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn _8_bit(self) -> &'a mut crate::W<REG> {
        self.variant(CHSIZESELECT_A::_8_BIT)
    }
    #[doc = "9 bits"]
    #[inline(always)]
    pub fn _9_bit(self) -> &'a mut crate::W<REG> {
        self.variant(CHSIZESELECT_A::_9_BIT)
    }
}
#[doc = "Field `PLOADEN` reader - Data Preload Enable"]
pub type PLOADEN_R = crate::BitReader;
#[doc = "Field `PLOADEN` writer - Data Preload Enable"]
pub type PLOADEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SSDE` reader - Slave Select Low Detect Enable"]
pub type SSDE_R = crate::BitReader;
#[doc = "Field `SSDE` writer - Slave Select Low Detect Enable"]
pub type SSDE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MSSEN` reader - Master Slave Select Enable"]
pub type MSSEN_R = crate::BitReader;
#[doc = "Field `MSSEN` writer - Master Slave Select Enable"]
pub type MSSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AMODE` reader - Address Mode"]
pub type AMODE_R = crate::FieldReader<AMODESELECT_A>;
#[doc = "Address Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AMODESELECT_A {
    #[doc = "0: SPI Address mask"]
    MASK = 0,
    #[doc = "1: Two unique Addressess"]
    _2_ADDRESSES = 1,
    #[doc = "2: Address Range"]
    RANGE = 2,
}
impl From<AMODESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: AMODESELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AMODESELECT_A {
    type Ux = u8;
}
impl AMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<AMODESELECT_A> {
        match self.bits {
            0 => Some(AMODESELECT_A::MASK),
            1 => Some(AMODESELECT_A::_2_ADDRESSES),
            2 => Some(AMODESELECT_A::RANGE),
            _ => None,
        }
    }
    #[doc = "SPI Address mask"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == AMODESELECT_A::MASK
    }
    #[doc = "Two unique Addressess"]
    #[inline(always)]
    pub fn is_2_addresses(&self) -> bool {
        *self == AMODESELECT_A::_2_ADDRESSES
    }
    #[doc = "Address Range"]
    #[inline(always)]
    pub fn is_range(&self) -> bool {
        *self == AMODESELECT_A::RANGE
    }
}
#[doc = "Field `AMODE` writer - Address Mode"]
pub type AMODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, AMODESELECT_A>;
impl<'a, REG, const O: u8> AMODE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SPI Address mask"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut crate::W<REG> {
        self.variant(AMODESELECT_A::MASK)
    }
    #[doc = "Two unique Addressess"]
    #[inline(always)]
    pub fn _2_addresses(self) -> &'a mut crate::W<REG> {
        self.variant(AMODESELECT_A::_2_ADDRESSES)
    }
    #[doc = "Address Range"]
    #[inline(always)]
    pub fn range(self) -> &'a mut crate::W<REG> {
        self.variant(AMODESELECT_A::RANGE)
    }
}
#[doc = "Field `RXEN` reader - Receiver Enable"]
pub type RXEN_R = crate::BitReader;
#[doc = "Field `RXEN` writer - Receiver Enable"]
pub type RXEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:2 - Character Size"]
    #[inline(always)]
    pub fn chsize(&self) -> CHSIZE_R {
        CHSIZE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 6 - Data Preload Enable"]
    #[inline(always)]
    pub fn ploaden(&self) -> PLOADEN_R {
        PLOADEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - Slave Select Low Detect Enable"]
    #[inline(always)]
    pub fn ssde(&self) -> SSDE_R {
        SSDE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 13 - Master Slave Select Enable"]
    #[inline(always)]
    pub fn mssen(&self) -> MSSEN_R {
        MSSEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Address Mode"]
    #[inline(always)]
    pub fn amode(&self) -> AMODE_R {
        AMODE_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 17 - Receiver Enable"]
    #[inline(always)]
    pub fn rxen(&self) -> RXEN_R {
        RXEN_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Character Size"]
    #[inline(always)]
    #[must_use]
    pub fn chsize(&mut self) -> CHSIZE_W<CTRLB_SPEC, 0> {
        CHSIZE_W::new(self)
    }
    #[doc = "Bit 6 - Data Preload Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ploaden(&mut self) -> PLOADEN_W<CTRLB_SPEC, 6> {
        PLOADEN_W::new(self)
    }
    #[doc = "Bit 9 - Slave Select Low Detect Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ssde(&mut self) -> SSDE_W<CTRLB_SPEC, 9> {
        SSDE_W::new(self)
    }
    #[doc = "Bit 13 - Master Slave Select Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mssen(&mut self) -> MSSEN_W<CTRLB_SPEC, 13> {
        MSSEN_W::new(self)
    }
    #[doc = "Bits 14:15 - Address Mode"]
    #[inline(always)]
    #[must_use]
    pub fn amode(&mut self) -> AMODE_W<CTRLB_SPEC, 14> {
        AMODE_W::new(self)
    }
    #[doc = "Bit 17 - Receiver Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxen(&mut self) -> RXEN_W<CTRLB_SPEC, 17> {
        RXEN_W::new(self)
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
#[doc = "SPIM Control B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
