#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::SLEEPCFG {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits };
        let mut w = W { bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `SLEEPMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEPMODER {
    #[doc = "CPU clock is OFF"]
    IDLE0,
    #[doc = "AHB clock is OFF"]
    IDLE1,
    #[doc = "APB clock are OFF"]
    IDLE2,
    #[doc = "All Clocks are OFF"]
    STANDBY,
    #[doc = "Backup domain is ON as well as some PDRAMs"]
    HIBERNATE,
    #[doc = "Only Backup domain is powered ON"]
    BACKUP,
    #[doc = "All power domains are powered OFF"]
    OFF,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SLEEPMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SLEEPMODER::IDLE0 => 0,
            SLEEPMODER::IDLE1 => 1,
            SLEEPMODER::IDLE2 => 2,
            SLEEPMODER::STANDBY => 4,
            SLEEPMODER::HIBERNATE => 5,
            SLEEPMODER::BACKUP => 6,
            SLEEPMODER::OFF => 7,
            SLEEPMODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SLEEPMODER {
        match value {
            0 => SLEEPMODER::IDLE0,
            1 => SLEEPMODER::IDLE1,
            2 => SLEEPMODER::IDLE2,
            4 => SLEEPMODER::STANDBY,
            5 => SLEEPMODER::HIBERNATE,
            6 => SLEEPMODER::BACKUP,
            7 => SLEEPMODER::OFF,
            i => SLEEPMODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `IDLE0`"]
    #[inline]
    pub fn is_idle0(&self) -> bool {
        *self == SLEEPMODER::IDLE0
    }
    #[doc = "Checks if the value of the field is `IDLE1`"]
    #[inline]
    pub fn is_idle1(&self) -> bool {
        *self == SLEEPMODER::IDLE1
    }
    #[doc = "Checks if the value of the field is `IDLE2`"]
    #[inline]
    pub fn is_idle2(&self) -> bool {
        *self == SLEEPMODER::IDLE2
    }
    #[doc = "Checks if the value of the field is `STANDBY`"]
    #[inline]
    pub fn is_standby(&self) -> bool {
        *self == SLEEPMODER::STANDBY
    }
    #[doc = "Checks if the value of the field is `HIBERNATE`"]
    #[inline]
    pub fn is_hibernate(&self) -> bool {
        *self == SLEEPMODER::HIBERNATE
    }
    #[doc = "Checks if the value of the field is `BACKUP`"]
    #[inline]
    pub fn is_backup(&self) -> bool {
        *self == SLEEPMODER::BACKUP
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == SLEEPMODER::OFF
    }
}
#[doc = "Values that can be written to the field `SLEEPMODE`"]
pub enum SLEEPMODEW {
    #[doc = "CPU clock is OFF"]
    IDLE0,
    #[doc = "AHB clock is OFF"]
    IDLE1,
    #[doc = "APB clock are OFF"]
    IDLE2,
    #[doc = "All Clocks are OFF"]
    STANDBY,
    #[doc = "Backup domain is ON as well as some PDRAMs"]
    HIBERNATE,
    #[doc = "Only Backup domain is powered ON"]
    BACKUP,
    #[doc = "All power domains are powered OFF"]
    OFF,
}
impl SLEEPMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SLEEPMODEW::IDLE0 => 0,
            SLEEPMODEW::IDLE1 => 1,
            SLEEPMODEW::IDLE2 => 2,
            SLEEPMODEW::STANDBY => 4,
            SLEEPMODEW::HIBERNATE => 5,
            SLEEPMODEW::BACKUP => 6,
            SLEEPMODEW::OFF => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLEEPMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _SLEEPMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLEEPMODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "CPU clock is OFF"]
    #[inline]
    pub fn idle0(self) -> &'a mut W {
        self.variant(SLEEPMODEW::IDLE0)
    }
    #[doc = "AHB clock is OFF"]
    #[inline]
    pub fn idle1(self) -> &'a mut W {
        self.variant(SLEEPMODEW::IDLE1)
    }
    #[doc = "APB clock are OFF"]
    #[inline]
    pub fn idle2(self) -> &'a mut W {
        self.variant(SLEEPMODEW::IDLE2)
    }
    #[doc = "All Clocks are OFF"]
    #[inline]
    pub fn standby(self) -> &'a mut W {
        self.variant(SLEEPMODEW::STANDBY)
    }
    #[doc = "Backup domain is ON as well as some PDRAMs"]
    #[inline]
    pub fn hibernate(self) -> &'a mut W {
        self.variant(SLEEPMODEW::HIBERNATE)
    }
    #[doc = "Only Backup domain is powered ON"]
    #[inline]
    pub fn backup(self) -> &'a mut W {
        self.variant(SLEEPMODEW::BACKUP)
    }
    #[doc = "All power domains are powered OFF"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(SLEEPMODEW::OFF)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:2 - Sleep Mode"]
    #[inline]
    pub fn sleepmode(&self) -> SLEEPMODER {
        SLEEPMODER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Sleep Mode"]
    #[inline]
    pub fn sleepmode(&mut self) -> _SLEEPMODEW {
        _SLEEPMODEW { w: self }
    }
}
