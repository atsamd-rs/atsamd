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
#[doc = "Field `SEREN0` reader - Serializer 0 Enable Synchronization Status"]
pub type Seren0R = crate::BitReader;
#[doc = "Field `SEREN1` reader - Serializer 1 Enable Synchronization Status"]
pub type Seren1R = crate::BitReader;
#[doc = "Field `DATA0` reader - Data 0 Synchronization Status"]
pub type Data0R = crate::BitReader;
#[doc = "Field `DATA1` reader - Data 1 Synchronization Status"]
pub type Data1R = crate::BitReader;
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
    #[doc = "Bit 4 - Serializer 0 Enable Synchronization Status"]
    #[inline(always)]
    pub fn seren0(&self) -> Seren0R {
        Seren0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Serializer 1 Enable Synchronization Status"]
    #[inline(always)]
    pub fn seren1(&self) -> Seren1R {
        Seren1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Data 0 Synchronization Status"]
    #[inline(always)]
    pub fn data0(&self) -> Data0R {
        Data0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Data 1 Synchronization Status"]
    #[inline(always)]
    pub fn data1(&self) -> Data1R {
        Data1R::new(((self.bits >> 9) & 1) != 0)
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
impl crate::Resettable for SyncbusySpec {
    const RESET_VALUE: u16 = 0;
}
