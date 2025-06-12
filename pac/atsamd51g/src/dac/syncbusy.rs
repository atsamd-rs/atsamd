#[doc = "Register `SYNCBUSY` reader"]
pub type R = crate::R<SyncbusySpec>;
#[doc = "Field `SWRST` reader - Software Reset"]
pub type SwrstR = crate::BitReader;
#[doc = "Field `ENABLE` reader - DAC Enable Status"]
pub type EnableR = crate::BitReader;
#[doc = "Field `DATA0` reader - Data DAC 0"]
pub type Data0R = crate::BitReader;
#[doc = "Field `DATA1` reader - Data DAC 1"]
pub type Data1R = crate::BitReader;
#[doc = "Field `DATABUF0` reader - Data Buffer DAC 0"]
pub type Databuf0R = crate::BitReader;
#[doc = "Field `DATABUF1` reader - Data Buffer DAC 1"]
pub type Databuf1R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SwrstR {
        SwrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DAC Enable Status"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Data DAC 0"]
    #[inline(always)]
    pub fn data0(&self) -> Data0R {
        Data0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data DAC 1"]
    #[inline(always)]
    pub fn data1(&self) -> Data1R {
        Data1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data Buffer DAC 0"]
    #[inline(always)]
    pub fn databuf0(&self) -> Databuf0R {
        Databuf0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data Buffer DAC 1"]
    #[inline(always)]
    pub fn databuf1(&self) -> Databuf1R {
        Databuf1R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Synchronization Busy\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyncbusySpec;
impl crate::RegisterSpec for SyncbusySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syncbusy::R`](R) reader structure"]
impl crate::Readable for SyncbusySpec {}
#[doc = "`reset()` method sets SYNCBUSY to value 0"]
impl crate::Resettable for SyncbusySpec {}
