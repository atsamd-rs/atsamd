#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::TIMESTAMP {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct SECONDR {
    bits: u8,
}
impl SECONDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MINUTER {
    bits: u8,
}
impl MINUTER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `HOUR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HOURR {
    #[doc = "AM when CLKREP in 12-hour"]
    AM,
    #[doc = "PM when CLKREP in 12-hour"]
    PM,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl HOURR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HOURR::AM => 0,
            HOURR::PM => 0x10,
            HOURR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HOURR {
        match value {
            0 => HOURR::AM,
            16 => HOURR::PM,
            i => HOURR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `AM`"]
    #[inline]
    pub fn is_am(&self) -> bool {
        *self == HOURR::AM
    }
    #[doc = "Checks if the value of the field is `PM`"]
    #[inline]
    pub fn is_pm(&self) -> bool {
        *self == HOURR::PM
    }
}
#[doc = r" Value of the field"]
pub struct DAYR {
    bits: u8,
}
impl DAYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MONTHR {
    bits: u8,
}
impl MONTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct YEARR {
    bits: u8,
}
impl YEARR {
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
    #[doc = "Bits 0:5 - Second Timestamp Value"]
    #[inline]
    pub fn second(&self) -> SECONDR {
        let bits = ((self.bits >> 0) & 0x3f) as u8;
        SECONDR { bits }
    }
    #[doc = "Bits 6:11 - Minute Timestamp Value"]
    #[inline]
    pub fn minute(&self) -> MINUTER {
        let bits = ((self.bits >> 6) & 0x3f) as u8;
        MINUTER { bits }
    }
    #[doc = "Bits 12:16 - Hour Timestamp Value"]
    #[inline]
    pub fn hour(&self) -> HOURR {
        HOURR::_from(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 17:21 - Day Timestamp Value"]
    #[inline]
    pub fn day(&self) -> DAYR {
        let bits = ((self.bits >> 17) & 0x1f) as u8;
        DAYR { bits }
    }
    #[doc = "Bits 22:25 - Month Timestamp Value"]
    #[inline]
    pub fn month(&self) -> MONTHR {
        let bits = ((self.bits >> 22) & 0x0f) as u8;
        MONTHR { bits }
    }
    #[doc = "Bits 26:31 - Year Timestamp Value"]
    #[inline]
    pub fn year(&self) -> YEARR {
        let bits = ((self.bits >> 26) & 0x3f) as u8;
        YEARR { bits }
    }
}
