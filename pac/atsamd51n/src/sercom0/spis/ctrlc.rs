#[doc = "Register `CTRLC` reader"]
pub type R = crate::R<CtrlcSpec>;
#[doc = "Register `CTRLC` writer"]
pub type W = crate::W<CtrlcSpec>;
#[doc = "Field `ICSPACE` reader - Inter-Character Spacing"]
pub type IcspaceR = crate::FieldReader;
#[doc = "Field `ICSPACE` writer - Inter-Character Spacing"]
pub type IcspaceW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Data 32 Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Data32bselect {
    #[doc = "0: Transaction from and to DATA register are 8-bit"]
    DataTrans8bit = 0,
    #[doc = "1: Transaction from and to DATA register are 32-bit"]
    DataTrans32bit = 1,
}
impl From<Data32bselect> for bool {
    #[inline(always)]
    fn from(variant: Data32bselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATA32B` reader - Data 32 Bit"]
pub type Data32bR = crate::BitReader<Data32bselect>;
impl Data32bR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Data32bselect {
        match self.bits {
            false => Data32bselect::DataTrans8bit,
            true => Data32bselect::DataTrans32bit,
        }
    }
    #[doc = "Transaction from and to DATA register are 8-bit"]
    #[inline(always)]
    pub fn is_data_trans_8bit(&self) -> bool {
        *self == Data32bselect::DataTrans8bit
    }
    #[doc = "Transaction from and to DATA register are 32-bit"]
    #[inline(always)]
    pub fn is_data_trans_32bit(&self) -> bool {
        *self == Data32bselect::DataTrans32bit
    }
}
#[doc = "Field `DATA32B` writer - Data 32 Bit"]
pub type Data32bW<'a, REG> = crate::BitWriter<'a, REG, Data32bselect>;
impl<'a, REG> Data32bW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transaction from and to DATA register are 8-bit"]
    #[inline(always)]
    pub fn data_trans_8bit(self) -> &'a mut crate::W<REG> {
        self.variant(Data32bselect::DataTrans8bit)
    }
    #[doc = "Transaction from and to DATA register are 32-bit"]
    #[inline(always)]
    pub fn data_trans_32bit(self) -> &'a mut crate::W<REG> {
        self.variant(Data32bselect::DataTrans32bit)
    }
}
impl R {
    #[doc = "Bits 0:5 - Inter-Character Spacing"]
    #[inline(always)]
    pub fn icspace(&self) -> IcspaceR {
        IcspaceR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 24 - Data 32 Bit"]
    #[inline(always)]
    pub fn data32b(&self) -> Data32bR {
        Data32bR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Inter-Character Spacing"]
    #[inline(always)]
    pub fn icspace(&mut self) -> IcspaceW<CtrlcSpec> {
        IcspaceW::new(self, 0)
    }
    #[doc = "Bit 24 - Data 32 Bit"]
    #[inline(always)]
    pub fn data32b(&mut self) -> Data32bW<CtrlcSpec> {
        Data32bW::new(self, 24)
    }
}
#[doc = "SPIS Control C\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlcSpec;
impl crate::RegisterSpec for CtrlcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlc::R`](R) reader structure"]
impl crate::Readable for CtrlcSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrlc::W`](W) writer structure"]
impl crate::Writable for CtrlcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLC to value 0"]
impl crate::Resettable for CtrlcSpec {}
