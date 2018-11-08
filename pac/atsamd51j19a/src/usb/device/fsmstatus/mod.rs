#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
impl super::FSMSTATUS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `FSMSTATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSMSTATER {
    #[doc = "OFF (L3). It corresponds to the powered-off, disconnected, and disabled state"]
    OFF,
    #[doc = "ON (L0). It corresponds to the Idle and Active states"]
    ON,
    #[doc = "SUSPEND (L2)"]
    SUSPEND,
    #[doc = "SLEEP (L1)"]
    SLEEP,
    #[doc = "DNRESUME. Down Stream Resume."]
    DNRESUME,
    #[doc = "UPRESUME. Up Stream Resume."]
    UPRESUME,
    #[doc = "RESET. USB lines Reset."]
    RESET,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FSMSTATER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FSMSTATER::OFF => 1,
            FSMSTATER::ON => 2,
            FSMSTATER::SUSPEND => 4,
            FSMSTATER::SLEEP => 8,
            FSMSTATER::DNRESUME => 16,
            FSMSTATER::UPRESUME => 32,
            FSMSTATER::RESET => 64,
            FSMSTATER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FSMSTATER {
        match value {
            1 => FSMSTATER::OFF,
            2 => FSMSTATER::ON,
            4 => FSMSTATER::SUSPEND,
            8 => FSMSTATER::SLEEP,
            16 => FSMSTATER::DNRESUME,
            32 => FSMSTATER::UPRESUME,
            64 => FSMSTATER::RESET,
            i => FSMSTATER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == FSMSTATER::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == FSMSTATER::ON
    }
    #[doc = "Checks if the value of the field is `SUSPEND`"]
    #[inline]
    pub fn is_suspend(&self) -> bool {
        *self == FSMSTATER::SUSPEND
    }
    #[doc = "Checks if the value of the field is `SLEEP`"]
    #[inline]
    pub fn is_sleep(&self) -> bool {
        *self == FSMSTATER::SLEEP
    }
    #[doc = "Checks if the value of the field is `DNRESUME`"]
    #[inline]
    pub fn is_dnresume(&self) -> bool {
        *self == FSMSTATER::DNRESUME
    }
    #[doc = "Checks if the value of the field is `UPRESUME`"]
    #[inline]
    pub fn is_upresume(&self) -> bool {
        *self == FSMSTATER::UPRESUME
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == FSMSTATER::RESET
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:6 - Fine State Machine Status"]
    #[inline]
    pub fn fsmstate(&self) -> FSMSTATER {
        FSMSTATER::_from({
            const MASK: u8 = 127;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
}
