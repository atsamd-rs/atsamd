#[doc = "Register `SYNCBUSY` reader"]
pub type R = crate::R<SyncbusySpec>;
#[doc = "Field `SWRST` reader - Software Reset Synchronization Status"]
pub type SwrstR = crate::BitReader;
#[doc = "Field `ENABLE` reader - Enable Synchronization Status"]
pub type EnableR = crate::BitReader;
#[doc = "Field `CKEN0` reader - Clock Unit 0 Enable Synchronization Status"]
pub type Cken0R = crate::BitReader;
#[doc = "Field `CKEN1` reader - Clock Unit 1 Enable Synchronization Status"]
pub type Cken1R = crate::BitReader;
#[doc = "Field `TXEN` reader - Tx Serializer Enable Synchronization Status"]
pub type TxenR = crate::BitReader;
#[doc = "Field `RXEN` reader - Rx Serializer Enable Synchronization Status"]
pub type RxenR = crate::BitReader;
#[doc = "Field `TXDATA` reader - Tx Data Synchronization Status"]
pub type TxdataR = crate::BitReader;
#[doc = "Field `RXDATA` reader - Rx Data Synchronization Status"]
pub type RxdataR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Software Reset Synchronization Status"]
    #[inline(always)]
    pub fn swrst(&self) -> SwrstR {
        SwrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Synchronization Status"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clock Unit 0 Enable Synchronization Status"]
    #[inline(always)]
    pub fn cken0(&self) -> Cken0R {
        Cken0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clock Unit 1 Enable Synchronization Status"]
    #[inline(always)]
    pub fn cken1(&self) -> Cken1R {
        Cken1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Tx Serializer Enable Synchronization Status"]
    #[inline(always)]
    pub fn txen(&self) -> TxenR {
        TxenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rx Serializer Enable Synchronization Status"]
    #[inline(always)]
    pub fn rxen(&self) -> RxenR {
        RxenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Tx Data Synchronization Status"]
    #[inline(always)]
    pub fn txdata(&self) -> TxdataR {
        TxdataR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Rx Data Synchronization Status"]
    #[inline(always)]
    pub fn rxdata(&self) -> RxdataR {
        RxdataR::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "Synchronization Status\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyncbusySpec;
impl crate::RegisterSpec for SyncbusySpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`syncbusy::R`](R) reader structure"]
impl crate::Readable for SyncbusySpec {}
#[doc = "`reset()` method sets SYNCBUSY to value 0"]
impl crate::Resettable for SyncbusySpec {}
