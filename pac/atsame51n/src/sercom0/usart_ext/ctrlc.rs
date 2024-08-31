#[doc = "Register `CTRLC` reader"]
pub type R = crate::R<CtrlcSpec>;
#[doc = "Register `CTRLC` writer"]
pub type W = crate::W<CtrlcSpec>;
#[doc = "Field `GTIME` reader - Guard Time"]
pub type GtimeR = crate::FieldReader;
#[doc = "Field `GTIME` writer - Guard Time"]
pub type GtimeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BRKLEN` reader - LIN Master Break Length"]
pub type BrklenR = crate::FieldReader;
#[doc = "Field `BRKLEN` writer - LIN Master Break Length"]
pub type BrklenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HDRDLY` reader - LIN Master Header Delay"]
pub type HdrdlyR = crate::FieldReader;
#[doc = "Field `HDRDLY` writer - LIN Master Header Delay"]
pub type HdrdlyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `INACK` reader - Inhibit Not Acknowledge"]
pub type InackR = crate::BitReader;
#[doc = "Field `INACK` writer - Inhibit Not Acknowledge"]
pub type InackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSNACK` reader - Disable Successive NACK"]
pub type DsnackR = crate::BitReader;
#[doc = "Field `DSNACK` writer - Disable Successive NACK"]
pub type DsnackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAXITER` reader - Maximum Iterations"]
pub type MaxiterR = crate::FieldReader;
#[doc = "Field `MAXITER` writer - Maximum Iterations"]
pub type MaxiterW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Data 32 Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Data32bselect {
    #[doc = "0: Data reads and writes according CTRLB.CHSIZE"]
    DataReadWriteChsize = 0,
    #[doc = "1: Data reads according CTRLB.CHSIZE and writes according 32-bit extension"]
    DataReadChsizeWrite32bit = 1,
    #[doc = "2: Data reads according 32-bit extension and writes according CTRLB.CHSIZE"]
    DataRead32bitWriteChsize = 2,
    #[doc = "3: Data reads and writes according 32-bit extension"]
    DataReadWrite32bit = 3,
}
impl From<Data32bselect> for u8 {
    #[inline(always)]
    fn from(variant: Data32bselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Data32bselect {
    type Ux = u8;
}
impl crate::IsEnum for Data32bselect {}
#[doc = "Field `DATA32B` reader - Data 32 Bit"]
pub type Data32bR = crate::FieldReader<Data32bselect>;
impl Data32bR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Data32bselect {
        match self.bits {
            0 => Data32bselect::DataReadWriteChsize,
            1 => Data32bselect::DataReadChsizeWrite32bit,
            2 => Data32bselect::DataRead32bitWriteChsize,
            3 => Data32bselect::DataReadWrite32bit,
            _ => unreachable!(),
        }
    }
    #[doc = "Data reads and writes according CTRLB.CHSIZE"]
    #[inline(always)]
    pub fn is_data_read_write_chsize(&self) -> bool {
        *self == Data32bselect::DataReadWriteChsize
    }
    #[doc = "Data reads according CTRLB.CHSIZE and writes according 32-bit extension"]
    #[inline(always)]
    pub fn is_data_read_chsize_write_32bit(&self) -> bool {
        *self == Data32bselect::DataReadChsizeWrite32bit
    }
    #[doc = "Data reads according 32-bit extension and writes according CTRLB.CHSIZE"]
    #[inline(always)]
    pub fn is_data_read_32bit_write_chsize(&self) -> bool {
        *self == Data32bselect::DataRead32bitWriteChsize
    }
    #[doc = "Data reads and writes according 32-bit extension"]
    #[inline(always)]
    pub fn is_data_read_write_32bit(&self) -> bool {
        *self == Data32bselect::DataReadWrite32bit
    }
}
#[doc = "Field `DATA32B` writer - Data 32 Bit"]
pub type Data32bW<'a, REG> = crate::FieldWriter<'a, REG, 2, Data32bselect, crate::Safe>;
impl<'a, REG> Data32bW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Data reads and writes according CTRLB.CHSIZE"]
    #[inline(always)]
    pub fn data_read_write_chsize(self) -> &'a mut crate::W<REG> {
        self.variant(Data32bselect::DataReadWriteChsize)
    }
    #[doc = "Data reads according CTRLB.CHSIZE and writes according 32-bit extension"]
    #[inline(always)]
    pub fn data_read_chsize_write_32bit(self) -> &'a mut crate::W<REG> {
        self.variant(Data32bselect::DataReadChsizeWrite32bit)
    }
    #[doc = "Data reads according 32-bit extension and writes according CTRLB.CHSIZE"]
    #[inline(always)]
    pub fn data_read_32bit_write_chsize(self) -> &'a mut crate::W<REG> {
        self.variant(Data32bselect::DataRead32bitWriteChsize)
    }
    #[doc = "Data reads and writes according 32-bit extension"]
    #[inline(always)]
    pub fn data_read_write_32bit(self) -> &'a mut crate::W<REG> {
        self.variant(Data32bselect::DataReadWrite32bit)
    }
}
impl R {
    #[doc = "Bits 0:2 - Guard Time"]
    #[inline(always)]
    pub fn gtime(&self) -> GtimeR {
        GtimeR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:9 - LIN Master Break Length"]
    #[inline(always)]
    pub fn brklen(&self) -> BrklenR {
        BrklenR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - LIN Master Header Delay"]
    #[inline(always)]
    pub fn hdrdly(&self) -> HdrdlyR {
        HdrdlyR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 16 - Inhibit Not Acknowledge"]
    #[inline(always)]
    pub fn inack(&self) -> InackR {
        InackR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Disable Successive NACK"]
    #[inline(always)]
    pub fn dsnack(&self) -> DsnackR {
        DsnackR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 20:22 - Maximum Iterations"]
    #[inline(always)]
    pub fn maxiter(&self) -> MaxiterR {
        MaxiterR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:25 - Data 32 Bit"]
    #[inline(always)]
    pub fn data32b(&self) -> Data32bR {
        Data32bR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Guard Time"]
    #[inline(always)]
    #[must_use]
    pub fn gtime(&mut self) -> GtimeW<CtrlcSpec> {
        GtimeW::new(self, 0)
    }
    #[doc = "Bits 8:9 - LIN Master Break Length"]
    #[inline(always)]
    #[must_use]
    pub fn brklen(&mut self) -> BrklenW<CtrlcSpec> {
        BrklenW::new(self, 8)
    }
    #[doc = "Bits 10:11 - LIN Master Header Delay"]
    #[inline(always)]
    #[must_use]
    pub fn hdrdly(&mut self) -> HdrdlyW<CtrlcSpec> {
        HdrdlyW::new(self, 10)
    }
    #[doc = "Bit 16 - Inhibit Not Acknowledge"]
    #[inline(always)]
    #[must_use]
    pub fn inack(&mut self) -> InackW<CtrlcSpec> {
        InackW::new(self, 16)
    }
    #[doc = "Bit 17 - Disable Successive NACK"]
    #[inline(always)]
    #[must_use]
    pub fn dsnack(&mut self) -> DsnackW<CtrlcSpec> {
        DsnackW::new(self, 17)
    }
    #[doc = "Bits 20:22 - Maximum Iterations"]
    #[inline(always)]
    #[must_use]
    pub fn maxiter(&mut self) -> MaxiterW<CtrlcSpec> {
        MaxiterW::new(self, 20)
    }
    #[doc = "Bits 24:25 - Data 32 Bit"]
    #[inline(always)]
    #[must_use]
    pub fn data32b(&mut self) -> Data32bW<CtrlcSpec> {
        Data32bW::new(self, 24)
    }
}
#[doc = "USART_EXT Control C\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlcSpec;
impl crate::RegisterSpec for CtrlcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlc::R`](R) reader structure"]
impl crate::Readable for CtrlcSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrlc::W`](W) writer structure"]
impl crate::Writable for CtrlcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRLC to value 0"]
impl crate::Resettable for CtrlcSpec {
    const RESET_VALUE: u32 = 0;
}
