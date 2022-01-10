#[doc = "Register `TXESC` reader"]
pub struct R(crate::R<TXESC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXESC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXESC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXESC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXESC` writer"]
pub struct W(crate::W<TXESC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXESC_SPEC>;
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
impl From<crate::W<TXESC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXESC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Tx Buffer Data Field Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TBDS_A {
    #[doc = "0: 8 byte data field"]
    DATA8 = 0,
    #[doc = "1: 12 byte data field"]
    DATA12 = 1,
    #[doc = "2: 16 byte data field"]
    DATA16 = 2,
    #[doc = "3: 20 byte data field"]
    DATA20 = 3,
    #[doc = "4: 24 byte data field"]
    DATA24 = 4,
    #[doc = "5: 32 byte data field"]
    DATA32 = 5,
    #[doc = "6: 48 byte data field"]
    DATA48 = 6,
    #[doc = "7: 64 byte data field"]
    DATA64 = 7,
}
impl From<TBDS_A> for u8 {
    #[inline(always)]
    fn from(variant: TBDS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TBDS` reader - Tx Buffer Data Field Size"]
pub struct TBDS_R(crate::FieldReader<u8, TBDS_A>);
impl TBDS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TBDS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBDS_A {
        match self.bits {
            0 => TBDS_A::DATA8,
            1 => TBDS_A::DATA12,
            2 => TBDS_A::DATA16,
            3 => TBDS_A::DATA20,
            4 => TBDS_A::DATA24,
            5 => TBDS_A::DATA32,
            6 => TBDS_A::DATA48,
            7 => TBDS_A::DATA64,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DATA8`"]
    #[inline(always)]
    pub fn is_data8(&self) -> bool {
        **self == TBDS_A::DATA8
    }
    #[doc = "Checks if the value of the field is `DATA12`"]
    #[inline(always)]
    pub fn is_data12(&self) -> bool {
        **self == TBDS_A::DATA12
    }
    #[doc = "Checks if the value of the field is `DATA16`"]
    #[inline(always)]
    pub fn is_data16(&self) -> bool {
        **self == TBDS_A::DATA16
    }
    #[doc = "Checks if the value of the field is `DATA20`"]
    #[inline(always)]
    pub fn is_data20(&self) -> bool {
        **self == TBDS_A::DATA20
    }
    #[doc = "Checks if the value of the field is `DATA24`"]
    #[inline(always)]
    pub fn is_data24(&self) -> bool {
        **self == TBDS_A::DATA24
    }
    #[doc = "Checks if the value of the field is `DATA32`"]
    #[inline(always)]
    pub fn is_data32(&self) -> bool {
        **self == TBDS_A::DATA32
    }
    #[doc = "Checks if the value of the field is `DATA48`"]
    #[inline(always)]
    pub fn is_data48(&self) -> bool {
        **self == TBDS_A::DATA48
    }
    #[doc = "Checks if the value of the field is `DATA64`"]
    #[inline(always)]
    pub fn is_data64(&self) -> bool {
        **self == TBDS_A::DATA64
    }
}
impl core::ops::Deref for TBDS_R {
    type Target = crate::FieldReader<u8, TBDS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TBDS` writer - Tx Buffer Data Field Size"]
pub struct TBDS_W<'a> {
    w: &'a mut W,
}
impl<'a> TBDS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TBDS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "8 byte data field"]
    #[inline(always)]
    pub fn data8(self) -> &'a mut W {
        self.variant(TBDS_A::DATA8)
    }
    #[doc = "12 byte data field"]
    #[inline(always)]
    pub fn data12(self) -> &'a mut W {
        self.variant(TBDS_A::DATA12)
    }
    #[doc = "16 byte data field"]
    #[inline(always)]
    pub fn data16(self) -> &'a mut W {
        self.variant(TBDS_A::DATA16)
    }
    #[doc = "20 byte data field"]
    #[inline(always)]
    pub fn data20(self) -> &'a mut W {
        self.variant(TBDS_A::DATA20)
    }
    #[doc = "24 byte data field"]
    #[inline(always)]
    pub fn data24(self) -> &'a mut W {
        self.variant(TBDS_A::DATA24)
    }
    #[doc = "32 byte data field"]
    #[inline(always)]
    pub fn data32(self) -> &'a mut W {
        self.variant(TBDS_A::DATA32)
    }
    #[doc = "48 byte data field"]
    #[inline(always)]
    pub fn data48(self) -> &'a mut W {
        self.variant(TBDS_A::DATA48)
    }
    #[doc = "64 byte data field"]
    #[inline(always)]
    pub fn data64(self) -> &'a mut W {
        self.variant(TBDS_A::DATA64)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Tx Buffer Data Field Size"]
    #[inline(always)]
    pub fn tbds(&self) -> TBDS_R {
        TBDS_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Tx Buffer Data Field Size"]
    #[inline(always)]
    pub fn tbds(&mut self) -> TBDS_W {
        TBDS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Tx Buffer Element Size Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txesc](index.html) module"]
pub struct TXESC_SPEC;
impl crate::RegisterSpec for TXESC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txesc::R](R) reader structure"]
impl crate::Readable for TXESC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txesc::W](W) writer structure"]
impl crate::Writable for TXESC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXESC to value 0"]
impl crate::Resettable for TXESC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
