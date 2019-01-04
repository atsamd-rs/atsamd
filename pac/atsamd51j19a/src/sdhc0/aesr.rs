#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
impl super::AESR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `ERRST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRSTR {
    #[doc = "ST_STOP (Stop DMA)"]
    STOP,
    #[doc = "ST_FDS (Fetch Descriptor)"]
    FDS,
    #[doc = "ST_TFR (Transfer Data)"]
    TFR,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ERRSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ERRSTR::STOP => 0,
            ERRSTR::FDS => 1,
            ERRSTR::TFR => 3,
            ERRSTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ERRSTR {
        match value {
            0 => ERRSTR::STOP,
            1 => ERRSTR::FDS,
            3 => ERRSTR::TFR,
            i => ERRSTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline]
    pub fn is_stop(&self) -> bool {
        *self == ERRSTR::STOP
    }
    #[doc = "Checks if the value of the field is `FDS`"]
    #[inline]
    pub fn is_fds(&self) -> bool {
        *self == ERRSTR::FDS
    }
    #[doc = "Checks if the value of the field is `TFR`"]
    #[inline]
    pub fn is_tfr(&self) -> bool {
        *self == ERRSTR::TFR
    }
}
#[doc = "Possible values of the field `LMIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LMISR {
    #[doc = "No Error"]
    NO,
    #[doc = "Error"]
    YES,
}
impl LMISR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            LMISR::NO => false,
            LMISR::YES => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LMISR {
        match value {
            false => LMISR::NO,
            true => LMISR::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == LMISR::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline]
    pub fn is_yes(&self) -> bool {
        *self == LMISR::YES
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:1 - ADMA Error State"]
    #[inline]
    pub fn errst(&self) -> ERRSTR {
        ERRSTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bit 2 - ADMA Length Mismatch Error"]
    #[inline]
    pub fn lmis(&self) -> LMISR {
        LMISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
}
