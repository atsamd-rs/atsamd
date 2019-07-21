#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DID {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct DEVSELR {
    bits: u8,
}
impl DEVSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct REVISIONR {
    bits: u8,
}
impl REVISIONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DIER {
    bits: u8,
}
impl DIER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SERIESR {
    bits: u8,
}
impl SERIESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FAMILYR {
    bits: u8,
}
impl FAMILYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PROCESSORR {
    bits: u8,
}
impl PROCESSORR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - Device Select"]
    #[inline]
    pub fn devsel(&self) -> DEVSELR {
        let bits = ((self.bits >> 0) & 0xff) as u8;
        DEVSELR { bits }
    }
    #[doc = "Bits 8:11 - Revision"]
    #[inline]
    pub fn revision(&self) -> REVISIONR {
        let bits = ((self.bits >> 8) & 0x0f) as u8;
        REVISIONR { bits }
    }
    #[doc = "Bits 12:15 - Die Identification"]
    #[inline]
    pub fn die(&self) -> DIER {
        let bits = ((self.bits >> 12) & 0x0f) as u8;
        DIER { bits }
    }
    #[doc = "Bits 16:21 - Product Series"]
    #[inline]
    pub fn series(&self) -> SERIESR {
        let bits = ((self.bits >> 16) & 0x3f) as u8;
        SERIESR { bits }
    }
    #[doc = "Bits 23:27 - Product Family"]
    #[inline]
    pub fn family(&self) -> FAMILYR {
        let bits = ((self.bits >> 23) & 0x1f) as u8;
        FAMILYR { bits }
    }
    #[doc = "Bits 28:31 - Processor"]
    #[inline]
    pub fn processor(&self) -> PROCESSORR {
        let bits = ((self.bits >> 28) & 0x0f) as u8;
        PROCESSORR { bits }
    }
}
