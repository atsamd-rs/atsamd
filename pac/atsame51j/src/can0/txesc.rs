#[doc = "Register `TXESC` reader"]
pub type R = crate::R<TXESC_SPEC>;
#[doc = "Register `TXESC` writer"]
pub type W = crate::W<TXESC_SPEC>;
#[doc = "Field `TBDS` reader - Tx Buffer Data Field Size"]
pub type TBDS_R = crate::FieldReader<TBDSSELECT_A>;
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
impl crate::FieldSpec for TBDSSELECT_A {
    type Ux = u8;
}
impl TBDS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TBDSSELECT_A {
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
    #[doc = "8 byte data field"]
    #[inline(always)]
    pub fn is_data8(&self) -> bool {
        *self == TBDSSELECT_A::DATA8
    }
    #[doc = "12 byte data field"]
    #[inline(always)]
    pub fn is_data12(&self) -> bool {
        *self == TBDSSELECT_A::DATA12
    }
    #[doc = "16 byte data field"]
    #[inline(always)]
    pub fn is_data16(&self) -> bool {
        *self == TBDSSELECT_A::DATA16
    }
    #[doc = "20 byte data field"]
    #[inline(always)]
    pub fn is_data20(&self) -> bool {
        *self == TBDSSELECT_A::DATA20
    }
    #[doc = "24 byte data field"]
    #[inline(always)]
    pub fn is_data24(&self) -> bool {
        *self == TBDSSELECT_A::DATA24
    }
    #[doc = "32 byte data field"]
    #[inline(always)]
    pub fn is_data32(&self) -> bool {
        *self == TBDSSELECT_A::DATA32
    }
    #[doc = "48 byte data field"]
    #[inline(always)]
    pub fn is_data48(&self) -> bool {
        *self == TBDSSELECT_A::DATA48
    }
    #[doc = "64 byte data field"]
    #[inline(always)]
    pub fn is_data64(&self) -> bool {
        *self == TBDSSELECT_A::DATA64
    }
}
#[doc = "Field `TBDS` writer - Tx Buffer Data Field Size"]
pub type TBDS_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, TBDSSELECT_A>;
impl<'a, REG, const O: u8> TBDS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 byte data field"]
    #[inline(always)]
    pub fn data8(self) -> &'a mut crate::W<REG> {
        self.variant(TBDSSELECT_A::DATA8)
    }
    #[doc = "12 byte data field"]
    #[inline(always)]
    pub fn data12(self) -> &'a mut crate::W<REG> {
        self.variant(TBDSSELECT_A::DATA12)
    }
    #[doc = "16 byte data field"]
    #[inline(always)]
    pub fn data16(self) -> &'a mut crate::W<REG> {
        self.variant(TBDSSELECT_A::DATA16)
    }
    #[doc = "20 byte data field"]
    #[inline(always)]
    pub fn data20(self) -> &'a mut crate::W<REG> {
        self.variant(TBDSSELECT_A::DATA20)
    }
    #[doc = "24 byte data field"]
    #[inline(always)]
    pub fn data24(self) -> &'a mut crate::W<REG> {
        self.variant(TBDSSELECT_A::DATA24)
    }
    #[doc = "32 byte data field"]
    #[inline(always)]
    pub fn data32(self) -> &'a mut crate::W<REG> {
        self.variant(TBDSSELECT_A::DATA32)
    }
    #[doc = "48 byte data field"]
    #[inline(always)]
    pub fn data48(self) -> &'a mut crate::W<REG> {
        self.variant(TBDSSELECT_A::DATA48)
    }
    #[doc = "64 byte data field"]
    #[inline(always)]
    pub fn data64(self) -> &'a mut crate::W<REG> {
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
    pub fn tbds(&mut self) -> TBDS_W<TXESC_SPEC, 0> {
        TBDS_W::new(self)
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
#[doc = "Tx Buffer Element Size Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txesc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txesc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXESC_SPEC;
impl crate::RegisterSpec for TXESC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txesc::R`](R) reader structure"]
impl crate::Readable for TXESC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txesc::W`](W) writer structure"]
impl crate::Writable for TXESC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXESC to value 0"]
impl crate::Resettable for TXESC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
