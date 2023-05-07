#[doc = "Register `RXESC` reader"]
pub struct R(crate::R<RXESC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXESC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXESC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXESC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXESC` writer"]
pub struct W(crate::W<RXESC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXESC_SPEC>;
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
impl From<crate::W<RXESC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXESC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `F0DS` reader - Rx FIFO 0 Data Field Size"]
pub type F0DS_R = crate::FieldReader<u8, F0DSSELECT_A>;
#[doc = "Rx FIFO 0 Data Field Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum F0DSSELECT_A {
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
impl From<F0DSSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: F0DSSELECT_A) -> Self {
        variant as _
    }
}
impl F0DS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> F0DSSELECT_A {
        match self.bits {
            0 => F0DSSELECT_A::DATA8,
            1 => F0DSSELECT_A::DATA12,
            2 => F0DSSELECT_A::DATA16,
            3 => F0DSSELECT_A::DATA20,
            4 => F0DSSELECT_A::DATA24,
            5 => F0DSSELECT_A::DATA32,
            6 => F0DSSELECT_A::DATA48,
            7 => F0DSSELECT_A::DATA64,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DATA8`"]
    #[inline(always)]
    pub fn is_data8(&self) -> bool {
        *self == F0DSSELECT_A::DATA8
    }
    #[doc = "Checks if the value of the field is `DATA12`"]
    #[inline(always)]
    pub fn is_data12(&self) -> bool {
        *self == F0DSSELECT_A::DATA12
    }
    #[doc = "Checks if the value of the field is `DATA16`"]
    #[inline(always)]
    pub fn is_data16(&self) -> bool {
        *self == F0DSSELECT_A::DATA16
    }
    #[doc = "Checks if the value of the field is `DATA20`"]
    #[inline(always)]
    pub fn is_data20(&self) -> bool {
        *self == F0DSSELECT_A::DATA20
    }
    #[doc = "Checks if the value of the field is `DATA24`"]
    #[inline(always)]
    pub fn is_data24(&self) -> bool {
        *self == F0DSSELECT_A::DATA24
    }
    #[doc = "Checks if the value of the field is `DATA32`"]
    #[inline(always)]
    pub fn is_data32(&self) -> bool {
        *self == F0DSSELECT_A::DATA32
    }
    #[doc = "Checks if the value of the field is `DATA48`"]
    #[inline(always)]
    pub fn is_data48(&self) -> bool {
        *self == F0DSSELECT_A::DATA48
    }
    #[doc = "Checks if the value of the field is `DATA64`"]
    #[inline(always)]
    pub fn is_data64(&self) -> bool {
        *self == F0DSSELECT_A::DATA64
    }
}
#[doc = "Field `F0DS` writer - Rx FIFO 0 Data Field Size"]
pub type F0DS_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, RXESC_SPEC, u8, F0DSSELECT_A, 3, O>;
impl<'a, const O: u8> F0DS_W<'a, O> {
    #[doc = "8 byte data field"]
    #[inline(always)]
    pub fn data8(self) -> &'a mut W {
        self.variant(F0DSSELECT_A::DATA8)
    }
    #[doc = "12 byte data field"]
    #[inline(always)]
    pub fn data12(self) -> &'a mut W {
        self.variant(F0DSSELECT_A::DATA12)
    }
    #[doc = "16 byte data field"]
    #[inline(always)]
    pub fn data16(self) -> &'a mut W {
        self.variant(F0DSSELECT_A::DATA16)
    }
    #[doc = "20 byte data field"]
    #[inline(always)]
    pub fn data20(self) -> &'a mut W {
        self.variant(F0DSSELECT_A::DATA20)
    }
    #[doc = "24 byte data field"]
    #[inline(always)]
    pub fn data24(self) -> &'a mut W {
        self.variant(F0DSSELECT_A::DATA24)
    }
    #[doc = "32 byte data field"]
    #[inline(always)]
    pub fn data32(self) -> &'a mut W {
        self.variant(F0DSSELECT_A::DATA32)
    }
    #[doc = "48 byte data field"]
    #[inline(always)]
    pub fn data48(self) -> &'a mut W {
        self.variant(F0DSSELECT_A::DATA48)
    }
    #[doc = "64 byte data field"]
    #[inline(always)]
    pub fn data64(self) -> &'a mut W {
        self.variant(F0DSSELECT_A::DATA64)
    }
}
#[doc = "Field `F1DS` reader - Rx FIFO 1 Data Field Size"]
pub type F1DS_R = crate::FieldReader<u8, F1DSSELECT_A>;
#[doc = "Rx FIFO 1 Data Field Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum F1DSSELECT_A {
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
impl From<F1DSSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: F1DSSELECT_A) -> Self {
        variant as _
    }
}
impl F1DS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> F1DSSELECT_A {
        match self.bits {
            0 => F1DSSELECT_A::DATA8,
            1 => F1DSSELECT_A::DATA12,
            2 => F1DSSELECT_A::DATA16,
            3 => F1DSSELECT_A::DATA20,
            4 => F1DSSELECT_A::DATA24,
            5 => F1DSSELECT_A::DATA32,
            6 => F1DSSELECT_A::DATA48,
            7 => F1DSSELECT_A::DATA64,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DATA8`"]
    #[inline(always)]
    pub fn is_data8(&self) -> bool {
        *self == F1DSSELECT_A::DATA8
    }
    #[doc = "Checks if the value of the field is `DATA12`"]
    #[inline(always)]
    pub fn is_data12(&self) -> bool {
        *self == F1DSSELECT_A::DATA12
    }
    #[doc = "Checks if the value of the field is `DATA16`"]
    #[inline(always)]
    pub fn is_data16(&self) -> bool {
        *self == F1DSSELECT_A::DATA16
    }
    #[doc = "Checks if the value of the field is `DATA20`"]
    #[inline(always)]
    pub fn is_data20(&self) -> bool {
        *self == F1DSSELECT_A::DATA20
    }
    #[doc = "Checks if the value of the field is `DATA24`"]
    #[inline(always)]
    pub fn is_data24(&self) -> bool {
        *self == F1DSSELECT_A::DATA24
    }
    #[doc = "Checks if the value of the field is `DATA32`"]
    #[inline(always)]
    pub fn is_data32(&self) -> bool {
        *self == F1DSSELECT_A::DATA32
    }
    #[doc = "Checks if the value of the field is `DATA48`"]
    #[inline(always)]
    pub fn is_data48(&self) -> bool {
        *self == F1DSSELECT_A::DATA48
    }
    #[doc = "Checks if the value of the field is `DATA64`"]
    #[inline(always)]
    pub fn is_data64(&self) -> bool {
        *self == F1DSSELECT_A::DATA64
    }
}
#[doc = "Field `F1DS` writer - Rx FIFO 1 Data Field Size"]
pub type F1DS_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, RXESC_SPEC, u8, F1DSSELECT_A, 3, O>;
impl<'a, const O: u8> F1DS_W<'a, O> {
    #[doc = "8 byte data field"]
    #[inline(always)]
    pub fn data8(self) -> &'a mut W {
        self.variant(F1DSSELECT_A::DATA8)
    }
    #[doc = "12 byte data field"]
    #[inline(always)]
    pub fn data12(self) -> &'a mut W {
        self.variant(F1DSSELECT_A::DATA12)
    }
    #[doc = "16 byte data field"]
    #[inline(always)]
    pub fn data16(self) -> &'a mut W {
        self.variant(F1DSSELECT_A::DATA16)
    }
    #[doc = "20 byte data field"]
    #[inline(always)]
    pub fn data20(self) -> &'a mut W {
        self.variant(F1DSSELECT_A::DATA20)
    }
    #[doc = "24 byte data field"]
    #[inline(always)]
    pub fn data24(self) -> &'a mut W {
        self.variant(F1DSSELECT_A::DATA24)
    }
    #[doc = "32 byte data field"]
    #[inline(always)]
    pub fn data32(self) -> &'a mut W {
        self.variant(F1DSSELECT_A::DATA32)
    }
    #[doc = "48 byte data field"]
    #[inline(always)]
    pub fn data48(self) -> &'a mut W {
        self.variant(F1DSSELECT_A::DATA48)
    }
    #[doc = "64 byte data field"]
    #[inline(always)]
    pub fn data64(self) -> &'a mut W {
        self.variant(F1DSSELECT_A::DATA64)
    }
}
#[doc = "Field `RBDS` reader - Rx Buffer Data Field Size"]
pub type RBDS_R = crate::FieldReader<u8, RBDSSELECT_A>;
#[doc = "Rx Buffer Data Field Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RBDSSELECT_A {
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
impl From<RBDSSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: RBDSSELECT_A) -> Self {
        variant as _
    }
}
impl RBDS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RBDSSELECT_A {
        match self.bits {
            0 => RBDSSELECT_A::DATA8,
            1 => RBDSSELECT_A::DATA12,
            2 => RBDSSELECT_A::DATA16,
            3 => RBDSSELECT_A::DATA20,
            4 => RBDSSELECT_A::DATA24,
            5 => RBDSSELECT_A::DATA32,
            6 => RBDSSELECT_A::DATA48,
            7 => RBDSSELECT_A::DATA64,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DATA8`"]
    #[inline(always)]
    pub fn is_data8(&self) -> bool {
        *self == RBDSSELECT_A::DATA8
    }
    #[doc = "Checks if the value of the field is `DATA12`"]
    #[inline(always)]
    pub fn is_data12(&self) -> bool {
        *self == RBDSSELECT_A::DATA12
    }
    #[doc = "Checks if the value of the field is `DATA16`"]
    #[inline(always)]
    pub fn is_data16(&self) -> bool {
        *self == RBDSSELECT_A::DATA16
    }
    #[doc = "Checks if the value of the field is `DATA20`"]
    #[inline(always)]
    pub fn is_data20(&self) -> bool {
        *self == RBDSSELECT_A::DATA20
    }
    #[doc = "Checks if the value of the field is `DATA24`"]
    #[inline(always)]
    pub fn is_data24(&self) -> bool {
        *self == RBDSSELECT_A::DATA24
    }
    #[doc = "Checks if the value of the field is `DATA32`"]
    #[inline(always)]
    pub fn is_data32(&self) -> bool {
        *self == RBDSSELECT_A::DATA32
    }
    #[doc = "Checks if the value of the field is `DATA48`"]
    #[inline(always)]
    pub fn is_data48(&self) -> bool {
        *self == RBDSSELECT_A::DATA48
    }
    #[doc = "Checks if the value of the field is `DATA64`"]
    #[inline(always)]
    pub fn is_data64(&self) -> bool {
        *self == RBDSSELECT_A::DATA64
    }
}
#[doc = "Field `RBDS` writer - Rx Buffer Data Field Size"]
pub type RBDS_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, RXESC_SPEC, u8, RBDSSELECT_A, 3, O>;
impl<'a, const O: u8> RBDS_W<'a, O> {
    #[doc = "8 byte data field"]
    #[inline(always)]
    pub fn data8(self) -> &'a mut W {
        self.variant(RBDSSELECT_A::DATA8)
    }
    #[doc = "12 byte data field"]
    #[inline(always)]
    pub fn data12(self) -> &'a mut W {
        self.variant(RBDSSELECT_A::DATA12)
    }
    #[doc = "16 byte data field"]
    #[inline(always)]
    pub fn data16(self) -> &'a mut W {
        self.variant(RBDSSELECT_A::DATA16)
    }
    #[doc = "20 byte data field"]
    #[inline(always)]
    pub fn data20(self) -> &'a mut W {
        self.variant(RBDSSELECT_A::DATA20)
    }
    #[doc = "24 byte data field"]
    #[inline(always)]
    pub fn data24(self) -> &'a mut W {
        self.variant(RBDSSELECT_A::DATA24)
    }
    #[doc = "32 byte data field"]
    #[inline(always)]
    pub fn data32(self) -> &'a mut W {
        self.variant(RBDSSELECT_A::DATA32)
    }
    #[doc = "48 byte data field"]
    #[inline(always)]
    pub fn data48(self) -> &'a mut W {
        self.variant(RBDSSELECT_A::DATA48)
    }
    #[doc = "64 byte data field"]
    #[inline(always)]
    pub fn data64(self) -> &'a mut W {
        self.variant(RBDSSELECT_A::DATA64)
    }
}
impl R {
    #[doc = "Bits 0:2 - Rx FIFO 0 Data Field Size"]
    #[inline(always)]
    pub fn f0ds(&self) -> F0DS_R {
        F0DS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Rx FIFO 1 Data Field Size"]
    #[inline(always)]
    pub fn f1ds(&self) -> F1DS_R {
        F1DS_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Rx Buffer Data Field Size"]
    #[inline(always)]
    pub fn rbds(&self) -> RBDS_R {
        RBDS_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Rx FIFO 0 Data Field Size"]
    #[inline(always)]
    #[must_use]
    pub fn f0ds(&mut self) -> F0DS_W<0> {
        F0DS_W::new(self)
    }
    #[doc = "Bits 4:6 - Rx FIFO 1 Data Field Size"]
    #[inline(always)]
    #[must_use]
    pub fn f1ds(&mut self) -> F1DS_W<4> {
        F1DS_W::new(self)
    }
    #[doc = "Bits 8:10 - Rx Buffer Data Field Size"]
    #[inline(always)]
    #[must_use]
    pub fn rbds(&mut self) -> RBDS_W<8> {
        RBDS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Rx Buffer / FIFO Element Size Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxesc](index.html) module"]
pub struct RXESC_SPEC;
impl crate::RegisterSpec for RXESC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxesc::R](R) reader structure"]
impl crate::Readable for RXESC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxesc::W](W) writer structure"]
impl crate::Writable for RXESC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXESC to value 0"]
impl crate::Resettable for RXESC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
