#[doc = "Register `CTRLC` reader"]
pub type R = crate::R<CTRLC_SPEC>;
#[doc = "Register `CTRLC` writer"]
pub type W = crate::W<CTRLC_SPEC>;
#[doc = "Field `SDASETUP` reader - SDA Setup Time"]
pub type SDASETUP_R = crate::FieldReader;
#[doc = "Field `SDASETUP` writer - SDA Setup Time"]
pub type SDASETUP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `DATA32B` reader - Data 32 Bit"]
pub type DATA32B_R = crate::BitReader<DATA32BSELECT_A>;
#[doc = "Data 32 Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DATA32BSELECT_A {
    #[doc = "0: Data transaction from/to DATA register are 8-bit"]
    DATA_TRANS_8BIT = 0,
    #[doc = "1: Data transaction from/to DATA register are 32-bit"]
    DATA_TRANS_32BIT = 1,
}
impl From<DATA32BSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: DATA32BSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl DATA32B_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DATA32BSELECT_A {
        match self.bits {
            false => DATA32BSELECT_A::DATA_TRANS_8BIT,
            true => DATA32BSELECT_A::DATA_TRANS_32BIT,
        }
    }
    #[doc = "Data transaction from/to DATA register are 8-bit"]
    #[inline(always)]
    pub fn is_data_trans_8bit(&self) -> bool {
        *self == DATA32BSELECT_A::DATA_TRANS_8BIT
    }
    #[doc = "Data transaction from/to DATA register are 32-bit"]
    #[inline(always)]
    pub fn is_data_trans_32bit(&self) -> bool {
        *self == DATA32BSELECT_A::DATA_TRANS_32BIT
    }
}
#[doc = "Field `DATA32B` writer - Data 32 Bit"]
pub type DATA32B_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DATA32BSELECT_A>;
impl<'a, REG, const O: u8> DATA32B_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data transaction from/to DATA register are 8-bit"]
    #[inline(always)]
    pub fn data_trans_8bit(self) -> &'a mut crate::W<REG> {
        self.variant(DATA32BSELECT_A::DATA_TRANS_8BIT)
    }
    #[doc = "Data transaction from/to DATA register are 32-bit"]
    #[inline(always)]
    pub fn data_trans_32bit(self) -> &'a mut crate::W<REG> {
        self.variant(DATA32BSELECT_A::DATA_TRANS_32BIT)
    }
}
impl R {
    #[doc = "Bits 0:3 - SDA Setup Time"]
    #[inline(always)]
    pub fn sdasetup(&self) -> SDASETUP_R {
        SDASETUP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Data 32 Bit"]
    #[inline(always)]
    pub fn data32b(&self) -> DATA32B_R {
        DATA32B_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - SDA Setup Time"]
    #[inline(always)]
    #[must_use]
    pub fn sdasetup(&mut self) -> SDASETUP_W<CTRLC_SPEC, 0> {
        SDASETUP_W::new(self)
    }
    #[doc = "Bit 24 - Data 32 Bit"]
    #[inline(always)]
    #[must_use]
    pub fn data32b(&mut self) -> DATA32B_W<CTRLC_SPEC, 24> {
        DATA32B_W::new(self)
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
#[doc = "I2CS Control C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLC_SPEC;
impl crate::RegisterSpec for CTRLC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlc::R`](R) reader structure"]
impl crate::Readable for CTRLC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrlc::W`](W) writer structure"]
impl crate::Writable for CTRLC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLC to value 0"]
impl crate::Resettable for CTRLC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
