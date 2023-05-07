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
#[doc = "Field `TBDS` reader - Tx Buffer Data Field Size"]
pub type TBDS_R = crate::FieldReader<u8, TBDSSELECT_A>;
#[doc = "Tx Buffer Data Field Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TBDSSELECT_A {
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
impl From<TBDSSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: TBDSSELECT_A) -> Self {
        variant as _
    }
}
impl TBDS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBDSSELECT_A {
        match self.bits {
            0 => TBDSSELECT_A::DATA8,
            1 => TBDSSELECT_A::DATA12,
            2 => TBDSSELECT_A::DATA16,
            3 => TBDSSELECT_A::DATA20,
            4 => TBDSSELECT_A::DATA24,
            5 => TBDSSELECT_A::DATA32,
            6 => TBDSSELECT_A::DATA48,
            7 => TBDSSELECT_A::DATA64,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DATA8`"]
    #[inline(always)]
    pub fn is_data8(&self) -> bool {
        *self == TBDSSELECT_A::DATA8
    }
    #[doc = "Checks if the value of the field is `DATA12`"]
    #[inline(always)]
    pub fn is_data12(&self) -> bool {
        *self == TBDSSELECT_A::DATA12
    }
    #[doc = "Checks if the value of the field is `DATA16`"]
    #[inline(always)]
    pub fn is_data16(&self) -> bool {
        *self == TBDSSELECT_A::DATA16
    }
    #[doc = "Checks if the value of the field is `DATA20`"]
    #[inline(always)]
    pub fn is_data20(&self) -> bool {
        *self == TBDSSELECT_A::DATA20
    }
    #[doc = "Checks if the value of the field is `DATA24`"]
    #[inline(always)]
    pub fn is_data24(&self) -> bool {
        *self == TBDSSELECT_A::DATA24
    }
    #[doc = "Checks if the value of the field is `DATA32`"]
    #[inline(always)]
    pub fn is_data32(&self) -> bool {
        *self == TBDSSELECT_A::DATA32
    }
    #[doc = "Checks if the value of the field is `DATA48`"]
    #[inline(always)]
    pub fn is_data48(&self) -> bool {
        *self == TBDSSELECT_A::DATA48
    }
    #[doc = "Checks if the value of the field is `DATA64`"]
    #[inline(always)]
    pub fn is_data64(&self) -> bool {
        *self == TBDSSELECT_A::DATA64
    }
}
#[doc = "Field `TBDS` writer - Tx Buffer Data Field Size"]
pub type TBDS_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TXESC_SPEC, u8, TBDSSELECT_A, 3, O>;
impl<'a, const O: u8> TBDS_W<'a, O> {
    #[doc = "8 byte data field"]
    #[inline(always)]
    pub fn data8(self) -> &'a mut W {
        self.variant(TBDSSELECT_A::DATA8)
    }
    #[doc = "12 byte data field"]
    #[inline(always)]
    pub fn data12(self) -> &'a mut W {
        self.variant(TBDSSELECT_A::DATA12)
    }
    #[doc = "16 byte data field"]
    #[inline(always)]
    pub fn data16(self) -> &'a mut W {
        self.variant(TBDSSELECT_A::DATA16)
    }
    #[doc = "20 byte data field"]
    #[inline(always)]
    pub fn data20(self) -> &'a mut W {
        self.variant(TBDSSELECT_A::DATA20)
    }
    #[doc = "24 byte data field"]
    #[inline(always)]
    pub fn data24(self) -> &'a mut W {
        self.variant(TBDSSELECT_A::DATA24)
    }
    #[doc = "32 byte data field"]
    #[inline(always)]
    pub fn data32(self) -> &'a mut W {
        self.variant(TBDSSELECT_A::DATA32)
    }
    #[doc = "48 byte data field"]
    #[inline(always)]
    pub fn data48(self) -> &'a mut W {
        self.variant(TBDSSELECT_A::DATA48)
    }
    #[doc = "64 byte data field"]
    #[inline(always)]
    pub fn data64(self) -> &'a mut W {
        self.variant(TBDSSELECT_A::DATA64)
    }
}
impl R {
    #[doc = "Bits 0:2 - Tx Buffer Data Field Size"]
    #[inline(always)]
    pub fn tbds(&self) -> TBDS_R {
        TBDS_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Tx Buffer Data Field Size"]
    #[inline(always)]
    #[must_use]
    pub fn tbds(&mut self) -> TBDS_W<0> {
        TBDS_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXESC to value 0"]
impl crate::Resettable for TXESC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
