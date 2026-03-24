#[doc = "Register `TXESC` reader"]
pub type R = crate::R<TxescSpec>;
#[doc = "Register `TXESC` writer"]
pub type W = crate::W<TxescSpec>;
#[doc = "Tx Buffer Data Field Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tbdsselect {
    #[doc = "0: 8 byte data field"]
    Data8 = 0,
    #[doc = "1: 12 byte data field"]
    Data12 = 1,
    #[doc = "2: 16 byte data field"]
    Data16 = 2,
    #[doc = "3: 20 byte data field"]
    Data20 = 3,
    #[doc = "4: 24 byte data field"]
    Data24 = 4,
    #[doc = "5: 32 byte data field"]
    Data32 = 5,
    #[doc = "6: 48 byte data field"]
    Data48 = 6,
    #[doc = "7: 64 byte data field"]
    Data64 = 7,
}
impl From<Tbdsselect> for u8 {
    #[inline(always)]
    fn from(variant: Tbdsselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tbdsselect {
    type Ux = u8;
}
impl crate::IsEnum for Tbdsselect {}
#[doc = "Field `TBDS` reader - Tx Buffer Data Field Size"]
pub type TbdsR = crate::FieldReader<Tbdsselect>;
impl TbdsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tbdsselect {
        match self.bits {
            0 => Tbdsselect::Data8,
            1 => Tbdsselect::Data12,
            2 => Tbdsselect::Data16,
            3 => Tbdsselect::Data20,
            4 => Tbdsselect::Data24,
            5 => Tbdsselect::Data32,
            6 => Tbdsselect::Data48,
            7 => Tbdsselect::Data64,
            _ => unreachable!(),
        }
    }
    #[doc = "8 byte data field"]
    #[inline(always)]
    pub fn is_data8(&self) -> bool {
        *self == Tbdsselect::Data8
    }
    #[doc = "12 byte data field"]
    #[inline(always)]
    pub fn is_data12(&self) -> bool {
        *self == Tbdsselect::Data12
    }
    #[doc = "16 byte data field"]
    #[inline(always)]
    pub fn is_data16(&self) -> bool {
        *self == Tbdsselect::Data16
    }
    #[doc = "20 byte data field"]
    #[inline(always)]
    pub fn is_data20(&self) -> bool {
        *self == Tbdsselect::Data20
    }
    #[doc = "24 byte data field"]
    #[inline(always)]
    pub fn is_data24(&self) -> bool {
        *self == Tbdsselect::Data24
    }
    #[doc = "32 byte data field"]
    #[inline(always)]
    pub fn is_data32(&self) -> bool {
        *self == Tbdsselect::Data32
    }
    #[doc = "48 byte data field"]
    #[inline(always)]
    pub fn is_data48(&self) -> bool {
        *self == Tbdsselect::Data48
    }
    #[doc = "64 byte data field"]
    #[inline(always)]
    pub fn is_data64(&self) -> bool {
        *self == Tbdsselect::Data64
    }
}
#[doc = "Field `TBDS` writer - Tx Buffer Data Field Size"]
pub type TbdsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Tbdsselect, crate::Safe>;
impl<'a, REG> TbdsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 byte data field"]
    #[inline(always)]
    pub fn data8(self) -> &'a mut crate::W<REG> {
        self.variant(Tbdsselect::Data8)
    }
    #[doc = "12 byte data field"]
    #[inline(always)]
    pub fn data12(self) -> &'a mut crate::W<REG> {
        self.variant(Tbdsselect::Data12)
    }
    #[doc = "16 byte data field"]
    #[inline(always)]
    pub fn data16(self) -> &'a mut crate::W<REG> {
        self.variant(Tbdsselect::Data16)
    }
    #[doc = "20 byte data field"]
    #[inline(always)]
    pub fn data20(self) -> &'a mut crate::W<REG> {
        self.variant(Tbdsselect::Data20)
    }
    #[doc = "24 byte data field"]
    #[inline(always)]
    pub fn data24(self) -> &'a mut crate::W<REG> {
        self.variant(Tbdsselect::Data24)
    }
    #[doc = "32 byte data field"]
    #[inline(always)]
    pub fn data32(self) -> &'a mut crate::W<REG> {
        self.variant(Tbdsselect::Data32)
    }
    #[doc = "48 byte data field"]
    #[inline(always)]
    pub fn data48(self) -> &'a mut crate::W<REG> {
        self.variant(Tbdsselect::Data48)
    }
    #[doc = "64 byte data field"]
    #[inline(always)]
    pub fn data64(self) -> &'a mut crate::W<REG> {
        self.variant(Tbdsselect::Data64)
    }
}
impl R {
    #[doc = "Bits 0:2 - Tx Buffer Data Field Size"]
    #[inline(always)]
    pub fn tbds(&self) -> TbdsR {
        TbdsR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Tx Buffer Data Field Size"]
    #[inline(always)]
    #[must_use]
    pub fn tbds(&mut self) -> TbdsW<TxescSpec> {
        TbdsW::new(self, 0)
    }
}
#[doc = "Tx Buffer Element Size Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`txesc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txesc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxescSpec;
impl crate::RegisterSpec for TxescSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txesc::R`](R) reader structure"]
impl crate::Readable for TxescSpec {}
#[doc = "`write(|w| ..)` method takes [`txesc::W`](W) writer structure"]
impl crate::Writable for TxescSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXESC to value 0"]
impl crate::Resettable for TxescSpec {
    const RESET_VALUE: u32 = 0;
}
