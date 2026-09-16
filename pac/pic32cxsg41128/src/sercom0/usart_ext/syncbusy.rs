#[doc = "Register `SYNCBUSY` reader"]
pub type R = crate::R<SyncbusySpec>;
#[doc = "Field `SWRST` reader - Software Reset Synchronization Busy"]
pub type SwrstR = crate::BitReader;
#[doc = "Field `ENABLE` reader - SERCOM Enable Synchronization Busy"]
pub type EnableR = crate::BitReader;
#[doc = "Field `CTRLB` reader - CTRLB Synchronization Busy"]
pub type CtrlbR = crate::BitReader;
#[doc = "Field `RXERRCNT` reader - RXERRCNT Synchronization Busy"]
pub type RxerrcntR = crate::BitReader;
#[doc = "Field `LENGTH` reader - LENGTH Synchronization Busy"]
pub type LengthR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Software Reset Synchronization Busy"]
    #[inline(always)]
    pub fn swrst(&self) -> SwrstR {
        SwrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SERCOM Enable Synchronization Busy"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CTRLB Synchronization Busy"]
    #[inline(always)]
    pub fn ctrlb(&self) -> CtrlbR {
        CtrlbR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RXERRCNT Synchronization Busy"]
    #[inline(always)]
    pub fn rxerrcnt(&self) -> RxerrcntR {
        RxerrcntR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LENGTH Synchronization Busy"]
    #[inline(always)]
    pub fn length(&self) -> LengthR {
        LengthR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "USART_EXT Synchronization Busy\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyncbusySpec;
impl crate::RegisterSpec for SyncbusySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syncbusy::R`](R) reader structure"]
impl crate::Readable for SyncbusySpec {}
#[doc = "`reset()` method sets SYNCBUSY to value 0"]
impl crate::Resettable for SyncbusySpec {
    const RESET_VALUE: u32 = 0;
}
