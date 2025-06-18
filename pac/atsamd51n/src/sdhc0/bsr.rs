#[doc = "Register `BSR` reader"]
pub type R = crate::R<BsrSpec>;
#[doc = "Register `BSR` writer"]
pub type W = crate::W<BsrSpec>;
#[doc = "Field `BLOCKSIZE` reader - Transfer Block Size"]
pub type BlocksizeR = crate::FieldReader<u16>;
#[doc = "Field `BLOCKSIZE` writer - Transfer Block Size"]
pub type BlocksizeW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "SDMA Buffer Boundary\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Boundaryselect {
    #[doc = "0: 4k bytes"]
    _4k = 0,
    #[doc = "1: 8k bytes"]
    _8k = 1,
    #[doc = "2: 16k bytes"]
    _16k = 2,
    #[doc = "3: 32k bytes"]
    _32k = 3,
    #[doc = "4: 64k bytes"]
    _64k = 4,
    #[doc = "5: 128k bytes"]
    _128k = 5,
    #[doc = "6: 256k bytes"]
    _256k = 6,
    #[doc = "7: 512k bytes"]
    _512k = 7,
}
impl From<Boundaryselect> for u8 {
    #[inline(always)]
    fn from(variant: Boundaryselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Boundaryselect {
    type Ux = u8;
}
impl crate::IsEnum for Boundaryselect {}
#[doc = "Field `BOUNDARY` reader - SDMA Buffer Boundary"]
pub type BoundaryR = crate::FieldReader<Boundaryselect>;
impl BoundaryR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Boundaryselect {
        match self.bits {
            0 => Boundaryselect::_4k,
            1 => Boundaryselect::_8k,
            2 => Boundaryselect::_16k,
            3 => Boundaryselect::_32k,
            4 => Boundaryselect::_64k,
            5 => Boundaryselect::_128k,
            6 => Boundaryselect::_256k,
            7 => Boundaryselect::_512k,
            _ => unreachable!(),
        }
    }
    #[doc = "4k bytes"]
    #[inline(always)]
    pub fn is_4k(&self) -> bool {
        *self == Boundaryselect::_4k
    }
    #[doc = "8k bytes"]
    #[inline(always)]
    pub fn is_8k(&self) -> bool {
        *self == Boundaryselect::_8k
    }
    #[doc = "16k bytes"]
    #[inline(always)]
    pub fn is_16k(&self) -> bool {
        *self == Boundaryselect::_16k
    }
    #[doc = "32k bytes"]
    #[inline(always)]
    pub fn is_32k(&self) -> bool {
        *self == Boundaryselect::_32k
    }
    #[doc = "64k bytes"]
    #[inline(always)]
    pub fn is_64k(&self) -> bool {
        *self == Boundaryselect::_64k
    }
    #[doc = "128k bytes"]
    #[inline(always)]
    pub fn is_128k(&self) -> bool {
        *self == Boundaryselect::_128k
    }
    #[doc = "256k bytes"]
    #[inline(always)]
    pub fn is_256k(&self) -> bool {
        *self == Boundaryselect::_256k
    }
    #[doc = "512k bytes"]
    #[inline(always)]
    pub fn is_512k(&self) -> bool {
        *self == Boundaryselect::_512k
    }
}
#[doc = "Field `BOUNDARY` writer - SDMA Buffer Boundary"]
pub type BoundaryW<'a, REG> = crate::FieldWriter<'a, REG, 3, Boundaryselect, crate::Safe>;
impl<'a, REG> BoundaryW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "4k bytes"]
    #[inline(always)]
    pub fn _4k(self) -> &'a mut crate::W<REG> {
        self.variant(Boundaryselect::_4k)
    }
    #[doc = "8k bytes"]
    #[inline(always)]
    pub fn _8k(self) -> &'a mut crate::W<REG> {
        self.variant(Boundaryselect::_8k)
    }
    #[doc = "16k bytes"]
    #[inline(always)]
    pub fn _16k(self) -> &'a mut crate::W<REG> {
        self.variant(Boundaryselect::_16k)
    }
    #[doc = "32k bytes"]
    #[inline(always)]
    pub fn _32k(self) -> &'a mut crate::W<REG> {
        self.variant(Boundaryselect::_32k)
    }
    #[doc = "64k bytes"]
    #[inline(always)]
    pub fn _64k(self) -> &'a mut crate::W<REG> {
        self.variant(Boundaryselect::_64k)
    }
    #[doc = "128k bytes"]
    #[inline(always)]
    pub fn _128k(self) -> &'a mut crate::W<REG> {
        self.variant(Boundaryselect::_128k)
    }
    #[doc = "256k bytes"]
    #[inline(always)]
    pub fn _256k(self) -> &'a mut crate::W<REG> {
        self.variant(Boundaryselect::_256k)
    }
    #[doc = "512k bytes"]
    #[inline(always)]
    pub fn _512k(self) -> &'a mut crate::W<REG> {
        self.variant(Boundaryselect::_512k)
    }
}
impl R {
    #[doc = "Bits 0:9 - Transfer Block Size"]
    #[inline(always)]
    pub fn blocksize(&self) -> BlocksizeR {
        BlocksizeR::new(self.bits & 0x03ff)
    }
    #[doc = "Bits 12:14 - SDMA Buffer Boundary"]
    #[inline(always)]
    pub fn boundary(&self) -> BoundaryR {
        BoundaryR::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - Transfer Block Size"]
    #[inline(always)]
    pub fn blocksize(&mut self) -> BlocksizeW<BsrSpec> {
        BlocksizeW::new(self, 0)
    }
    #[doc = "Bits 12:14 - SDMA Buffer Boundary"]
    #[inline(always)]
    pub fn boundary(&mut self) -> BoundaryW<BsrSpec> {
        BoundaryW::new(self, 12)
    }
}
#[doc = "Block Size\n\nYou can [`read`](crate::Reg::read) this register and get [`bsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BsrSpec;
impl crate::RegisterSpec for BsrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`bsr::R`](R) reader structure"]
impl crate::Readable for BsrSpec {}
#[doc = "`write(|w| ..)` method takes [`bsr::W`](W) writer structure"]
impl crate::Writable for BsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BSR to value 0"]
impl crate::Resettable for BsrSpec {}
