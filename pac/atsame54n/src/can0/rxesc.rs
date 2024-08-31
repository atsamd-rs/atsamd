#[doc = "Register `RXESC` reader"]
pub type R = crate::R<RxescSpec>;
#[doc = "Register `RXESC` writer"]
pub type W = crate::W<RxescSpec>;
#[doc = "Rx FIFO 0 Data Field Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum F0dsselect {
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
impl From<F0dsselect> for u8 {
    #[inline(always)]
    fn from(variant: F0dsselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for F0dsselect {
    type Ux = u8;
}
impl crate::IsEnum for F0dsselect {}
#[doc = "Field `F0DS` reader - Rx FIFO 0 Data Field Size"]
pub type F0dsR = crate::FieldReader<F0dsselect>;
impl F0dsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> F0dsselect {
        match self.bits {
            0 => F0dsselect::Data8,
            1 => F0dsselect::Data12,
            2 => F0dsselect::Data16,
            3 => F0dsselect::Data20,
            4 => F0dsselect::Data24,
            5 => F0dsselect::Data32,
            6 => F0dsselect::Data48,
            7 => F0dsselect::Data64,
            _ => unreachable!(),
        }
    }
    #[doc = "8 byte data field"]
    #[inline(always)]
    pub fn is_data8(&self) -> bool {
        *self == F0dsselect::Data8
    }
    #[doc = "12 byte data field"]
    #[inline(always)]
    pub fn is_data12(&self) -> bool {
        *self == F0dsselect::Data12
    }
    #[doc = "16 byte data field"]
    #[inline(always)]
    pub fn is_data16(&self) -> bool {
        *self == F0dsselect::Data16
    }
    #[doc = "20 byte data field"]
    #[inline(always)]
    pub fn is_data20(&self) -> bool {
        *self == F0dsselect::Data20
    }
    #[doc = "24 byte data field"]
    #[inline(always)]
    pub fn is_data24(&self) -> bool {
        *self == F0dsselect::Data24
    }
    #[doc = "32 byte data field"]
    #[inline(always)]
    pub fn is_data32(&self) -> bool {
        *self == F0dsselect::Data32
    }
    #[doc = "48 byte data field"]
    #[inline(always)]
    pub fn is_data48(&self) -> bool {
        *self == F0dsselect::Data48
    }
    #[doc = "64 byte data field"]
    #[inline(always)]
    pub fn is_data64(&self) -> bool {
        *self == F0dsselect::Data64
    }
}
#[doc = "Field `F0DS` writer - Rx FIFO 0 Data Field Size"]
pub type F0dsW<'a, REG> = crate::FieldWriter<'a, REG, 3, F0dsselect, crate::Safe>;
impl<'a, REG> F0dsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 byte data field"]
    #[inline(always)]
    pub fn data8(self) -> &'a mut crate::W<REG> {
        self.variant(F0dsselect::Data8)
    }
    #[doc = "12 byte data field"]
    #[inline(always)]
    pub fn data12(self) -> &'a mut crate::W<REG> {
        self.variant(F0dsselect::Data12)
    }
    #[doc = "16 byte data field"]
    #[inline(always)]
    pub fn data16(self) -> &'a mut crate::W<REG> {
        self.variant(F0dsselect::Data16)
    }
    #[doc = "20 byte data field"]
    #[inline(always)]
    pub fn data20(self) -> &'a mut crate::W<REG> {
        self.variant(F0dsselect::Data20)
    }
    #[doc = "24 byte data field"]
    #[inline(always)]
    pub fn data24(self) -> &'a mut crate::W<REG> {
        self.variant(F0dsselect::Data24)
    }
    #[doc = "32 byte data field"]
    #[inline(always)]
    pub fn data32(self) -> &'a mut crate::W<REG> {
        self.variant(F0dsselect::Data32)
    }
    #[doc = "48 byte data field"]
    #[inline(always)]
    pub fn data48(self) -> &'a mut crate::W<REG> {
        self.variant(F0dsselect::Data48)
    }
    #[doc = "64 byte data field"]
    #[inline(always)]
    pub fn data64(self) -> &'a mut crate::W<REG> {
        self.variant(F0dsselect::Data64)
    }
}
#[doc = "Rx FIFO 1 Data Field Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum F1dsselect {
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
impl From<F1dsselect> for u8 {
    #[inline(always)]
    fn from(variant: F1dsselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for F1dsselect {
    type Ux = u8;
}
impl crate::IsEnum for F1dsselect {}
#[doc = "Field `F1DS` reader - Rx FIFO 1 Data Field Size"]
pub type F1dsR = crate::FieldReader<F1dsselect>;
impl F1dsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> F1dsselect {
        match self.bits {
            0 => F1dsselect::Data8,
            1 => F1dsselect::Data12,
            2 => F1dsselect::Data16,
            3 => F1dsselect::Data20,
            4 => F1dsselect::Data24,
            5 => F1dsselect::Data32,
            6 => F1dsselect::Data48,
            7 => F1dsselect::Data64,
            _ => unreachable!(),
        }
    }
    #[doc = "8 byte data field"]
    #[inline(always)]
    pub fn is_data8(&self) -> bool {
        *self == F1dsselect::Data8
    }
    #[doc = "12 byte data field"]
    #[inline(always)]
    pub fn is_data12(&self) -> bool {
        *self == F1dsselect::Data12
    }
    #[doc = "16 byte data field"]
    #[inline(always)]
    pub fn is_data16(&self) -> bool {
        *self == F1dsselect::Data16
    }
    #[doc = "20 byte data field"]
    #[inline(always)]
    pub fn is_data20(&self) -> bool {
        *self == F1dsselect::Data20
    }
    #[doc = "24 byte data field"]
    #[inline(always)]
    pub fn is_data24(&self) -> bool {
        *self == F1dsselect::Data24
    }
    #[doc = "32 byte data field"]
    #[inline(always)]
    pub fn is_data32(&self) -> bool {
        *self == F1dsselect::Data32
    }
    #[doc = "48 byte data field"]
    #[inline(always)]
    pub fn is_data48(&self) -> bool {
        *self == F1dsselect::Data48
    }
    #[doc = "64 byte data field"]
    #[inline(always)]
    pub fn is_data64(&self) -> bool {
        *self == F1dsselect::Data64
    }
}
#[doc = "Field `F1DS` writer - Rx FIFO 1 Data Field Size"]
pub type F1dsW<'a, REG> = crate::FieldWriter<'a, REG, 3, F1dsselect, crate::Safe>;
impl<'a, REG> F1dsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 byte data field"]
    #[inline(always)]
    pub fn data8(self) -> &'a mut crate::W<REG> {
        self.variant(F1dsselect::Data8)
    }
    #[doc = "12 byte data field"]
    #[inline(always)]
    pub fn data12(self) -> &'a mut crate::W<REG> {
        self.variant(F1dsselect::Data12)
    }
    #[doc = "16 byte data field"]
    #[inline(always)]
    pub fn data16(self) -> &'a mut crate::W<REG> {
        self.variant(F1dsselect::Data16)
    }
    #[doc = "20 byte data field"]
    #[inline(always)]
    pub fn data20(self) -> &'a mut crate::W<REG> {
        self.variant(F1dsselect::Data20)
    }
    #[doc = "24 byte data field"]
    #[inline(always)]
    pub fn data24(self) -> &'a mut crate::W<REG> {
        self.variant(F1dsselect::Data24)
    }
    #[doc = "32 byte data field"]
    #[inline(always)]
    pub fn data32(self) -> &'a mut crate::W<REG> {
        self.variant(F1dsselect::Data32)
    }
    #[doc = "48 byte data field"]
    #[inline(always)]
    pub fn data48(self) -> &'a mut crate::W<REG> {
        self.variant(F1dsselect::Data48)
    }
    #[doc = "64 byte data field"]
    #[inline(always)]
    pub fn data64(self) -> &'a mut crate::W<REG> {
        self.variant(F1dsselect::Data64)
    }
}
#[doc = "Rx Buffer Data Field Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rbdsselect {
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
impl From<Rbdsselect> for u8 {
    #[inline(always)]
    fn from(variant: Rbdsselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rbdsselect {
    type Ux = u8;
}
impl crate::IsEnum for Rbdsselect {}
#[doc = "Field `RBDS` reader - Rx Buffer Data Field Size"]
pub type RbdsR = crate::FieldReader<Rbdsselect>;
impl RbdsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rbdsselect {
        match self.bits {
            0 => Rbdsselect::Data8,
            1 => Rbdsselect::Data12,
            2 => Rbdsselect::Data16,
            3 => Rbdsselect::Data20,
            4 => Rbdsselect::Data24,
            5 => Rbdsselect::Data32,
            6 => Rbdsselect::Data48,
            7 => Rbdsselect::Data64,
            _ => unreachable!(),
        }
    }
    #[doc = "8 byte data field"]
    #[inline(always)]
    pub fn is_data8(&self) -> bool {
        *self == Rbdsselect::Data8
    }
    #[doc = "12 byte data field"]
    #[inline(always)]
    pub fn is_data12(&self) -> bool {
        *self == Rbdsselect::Data12
    }
    #[doc = "16 byte data field"]
    #[inline(always)]
    pub fn is_data16(&self) -> bool {
        *self == Rbdsselect::Data16
    }
    #[doc = "20 byte data field"]
    #[inline(always)]
    pub fn is_data20(&self) -> bool {
        *self == Rbdsselect::Data20
    }
    #[doc = "24 byte data field"]
    #[inline(always)]
    pub fn is_data24(&self) -> bool {
        *self == Rbdsselect::Data24
    }
    #[doc = "32 byte data field"]
    #[inline(always)]
    pub fn is_data32(&self) -> bool {
        *self == Rbdsselect::Data32
    }
    #[doc = "48 byte data field"]
    #[inline(always)]
    pub fn is_data48(&self) -> bool {
        *self == Rbdsselect::Data48
    }
    #[doc = "64 byte data field"]
    #[inline(always)]
    pub fn is_data64(&self) -> bool {
        *self == Rbdsselect::Data64
    }
}
#[doc = "Field `RBDS` writer - Rx Buffer Data Field Size"]
pub type RbdsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Rbdsselect, crate::Safe>;
impl<'a, REG> RbdsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 byte data field"]
    #[inline(always)]
    pub fn data8(self) -> &'a mut crate::W<REG> {
        self.variant(Rbdsselect::Data8)
    }
    #[doc = "12 byte data field"]
    #[inline(always)]
    pub fn data12(self) -> &'a mut crate::W<REG> {
        self.variant(Rbdsselect::Data12)
    }
    #[doc = "16 byte data field"]
    #[inline(always)]
    pub fn data16(self) -> &'a mut crate::W<REG> {
        self.variant(Rbdsselect::Data16)
    }
    #[doc = "20 byte data field"]
    #[inline(always)]
    pub fn data20(self) -> &'a mut crate::W<REG> {
        self.variant(Rbdsselect::Data20)
    }
    #[doc = "24 byte data field"]
    #[inline(always)]
    pub fn data24(self) -> &'a mut crate::W<REG> {
        self.variant(Rbdsselect::Data24)
    }
    #[doc = "32 byte data field"]
    #[inline(always)]
    pub fn data32(self) -> &'a mut crate::W<REG> {
        self.variant(Rbdsselect::Data32)
    }
    #[doc = "48 byte data field"]
    #[inline(always)]
    pub fn data48(self) -> &'a mut crate::W<REG> {
        self.variant(Rbdsselect::Data48)
    }
    #[doc = "64 byte data field"]
    #[inline(always)]
    pub fn data64(self) -> &'a mut crate::W<REG> {
        self.variant(Rbdsselect::Data64)
    }
}
impl R {
    #[doc = "Bits 0:2 - Rx FIFO 0 Data Field Size"]
    #[inline(always)]
    pub fn f0ds(&self) -> F0dsR {
        F0dsR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Rx FIFO 1 Data Field Size"]
    #[inline(always)]
    pub fn f1ds(&self) -> F1dsR {
        F1dsR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Rx Buffer Data Field Size"]
    #[inline(always)]
    pub fn rbds(&self) -> RbdsR {
        RbdsR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Rx FIFO 0 Data Field Size"]
    #[inline(always)]
    #[must_use]
    pub fn f0ds(&mut self) -> F0dsW<RxescSpec> {
        F0dsW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Rx FIFO 1 Data Field Size"]
    #[inline(always)]
    #[must_use]
    pub fn f1ds(&mut self) -> F1dsW<RxescSpec> {
        F1dsW::new(self, 4)
    }
    #[doc = "Bits 8:10 - Rx Buffer Data Field Size"]
    #[inline(always)]
    #[must_use]
    pub fn rbds(&mut self) -> RbdsW<RxescSpec> {
        RbdsW::new(self, 8)
    }
}
#[doc = "Rx Buffer / FIFO Element Size Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`rxesc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxesc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxescSpec;
impl crate::RegisterSpec for RxescSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxesc::R`](R) reader structure"]
impl crate::Readable for RxescSpec {}
#[doc = "`write(|w| ..)` method takes [`rxesc::W`](W) writer structure"]
impl crate::Writable for RxescSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXESC to value 0"]
impl crate::Resettable for RxescSpec {
    const RESET_VALUE: u32 = 0;
}
