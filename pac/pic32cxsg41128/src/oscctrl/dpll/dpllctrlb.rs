#[doc = "Register `DPLLCTRLB` reader"]
pub type R = crate::R<DpllctrlbSpec>;
#[doc = "Register `DPLLCTRLB` writer"]
pub type W = crate::W<DpllctrlbSpec>;
#[doc = "Proportional Integral Filter Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Filterselect {
    #[doc = "0: Bandwidth = 92.7Khz and Damping Factor = 0.76"]
    Filter1 = 0,
    #[doc = "1: Bandwidth = 131Khz and Damping Factor = 1.08"]
    Filter2 = 1,
    #[doc = "2: Bandwidth = 46.4Khz and Damping Factor = 0.38"]
    Filter3 = 2,
    #[doc = "3: Bandwidth = 65.6Khz and Damping Factor = 0.54"]
    Filter4 = 3,
    #[doc = "4: Bandwidth = 131Khz and Damping Factor = 0.56"]
    Filter5 = 4,
    #[doc = "5: Bandwidth = 185Khz and Damping Factor = 0.79"]
    Filter6 = 5,
    #[doc = "6: Bandwidth = 65.6Khz and Damping Factor = 0.28"]
    Filter7 = 6,
    #[doc = "7: Bandwidth = 92.7Khz and Damping Factor = 0.39"]
    Filter8 = 7,
    #[doc = "8: Bandwidth = 46.4Khz and Damping Factor = 1.49"]
    Filter9 = 8,
    #[doc = "9: Bandwidth = 65.6Khz and Damping Factor = 2.11"]
    Filter10 = 9,
    #[doc = "10: Bandwidth = 23.2Khz and Damping Factor = 0.75"]
    Filter11 = 10,
    #[doc = "11: Bandwidth = 32.8Khz and Damping Factor = 1.06"]
    Filter12 = 11,
    #[doc = "12: Bandwidth = 65.6Khz and Damping Factor = 1.07"]
    Filter13 = 12,
    #[doc = "13: Bandwidth = 92.7Khz and Damping Factor = 1.51"]
    Filter14 = 13,
    #[doc = "14: Bandwidth = 32.8Khz and Damping Factor = 0.53"]
    Filter15 = 14,
    #[doc = "15: Bandwidth = 46.4Khz and Damping Factor = 0.75"]
    Filter16 = 15,
}
impl From<Filterselect> for u8 {
    #[inline(always)]
    fn from(variant: Filterselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Filterselect {
    type Ux = u8;
}
impl crate::IsEnum for Filterselect {}
#[doc = "Field `FILTER` reader - Proportional Integral Filter Selection"]
pub type FilterR = crate::FieldReader<Filterselect>;
impl FilterR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Filterselect {
        match self.bits {
            0 => Filterselect::Filter1,
            1 => Filterselect::Filter2,
            2 => Filterselect::Filter3,
            3 => Filterselect::Filter4,
            4 => Filterselect::Filter5,
            5 => Filterselect::Filter6,
            6 => Filterselect::Filter7,
            7 => Filterselect::Filter8,
            8 => Filterselect::Filter9,
            9 => Filterselect::Filter10,
            10 => Filterselect::Filter11,
            11 => Filterselect::Filter12,
            12 => Filterselect::Filter13,
            13 => Filterselect::Filter14,
            14 => Filterselect::Filter15,
            15 => Filterselect::Filter16,
            _ => unreachable!(),
        }
    }
    #[doc = "Bandwidth = 92.7Khz and Damping Factor = 0.76"]
    #[inline(always)]
    pub fn is_filter1(&self) -> bool {
        *self == Filterselect::Filter1
    }
    #[doc = "Bandwidth = 131Khz and Damping Factor = 1.08"]
    #[inline(always)]
    pub fn is_filter2(&self) -> bool {
        *self == Filterselect::Filter2
    }
    #[doc = "Bandwidth = 46.4Khz and Damping Factor = 0.38"]
    #[inline(always)]
    pub fn is_filter3(&self) -> bool {
        *self == Filterselect::Filter3
    }
    #[doc = "Bandwidth = 65.6Khz and Damping Factor = 0.54"]
    #[inline(always)]
    pub fn is_filter4(&self) -> bool {
        *self == Filterselect::Filter4
    }
    #[doc = "Bandwidth = 131Khz and Damping Factor = 0.56"]
    #[inline(always)]
    pub fn is_filter5(&self) -> bool {
        *self == Filterselect::Filter5
    }
    #[doc = "Bandwidth = 185Khz and Damping Factor = 0.79"]
    #[inline(always)]
    pub fn is_filter6(&self) -> bool {
        *self == Filterselect::Filter6
    }
    #[doc = "Bandwidth = 65.6Khz and Damping Factor = 0.28"]
    #[inline(always)]
    pub fn is_filter7(&self) -> bool {
        *self == Filterselect::Filter7
    }
    #[doc = "Bandwidth = 92.7Khz and Damping Factor = 0.39"]
    #[inline(always)]
    pub fn is_filter8(&self) -> bool {
        *self == Filterselect::Filter8
    }
    #[doc = "Bandwidth = 46.4Khz and Damping Factor = 1.49"]
    #[inline(always)]
    pub fn is_filter9(&self) -> bool {
        *self == Filterselect::Filter9
    }
    #[doc = "Bandwidth = 65.6Khz and Damping Factor = 2.11"]
    #[inline(always)]
    pub fn is_filter10(&self) -> bool {
        *self == Filterselect::Filter10
    }
    #[doc = "Bandwidth = 23.2Khz and Damping Factor = 0.75"]
    #[inline(always)]
    pub fn is_filter11(&self) -> bool {
        *self == Filterselect::Filter11
    }
    #[doc = "Bandwidth = 32.8Khz and Damping Factor = 1.06"]
    #[inline(always)]
    pub fn is_filter12(&self) -> bool {
        *self == Filterselect::Filter12
    }
    #[doc = "Bandwidth = 65.6Khz and Damping Factor = 1.07"]
    #[inline(always)]
    pub fn is_filter13(&self) -> bool {
        *self == Filterselect::Filter13
    }
    #[doc = "Bandwidth = 92.7Khz and Damping Factor = 1.51"]
    #[inline(always)]
    pub fn is_filter14(&self) -> bool {
        *self == Filterselect::Filter14
    }
    #[doc = "Bandwidth = 32.8Khz and Damping Factor = 0.53"]
    #[inline(always)]
    pub fn is_filter15(&self) -> bool {
        *self == Filterselect::Filter15
    }
    #[doc = "Bandwidth = 46.4Khz and Damping Factor = 0.75"]
    #[inline(always)]
    pub fn is_filter16(&self) -> bool {
        *self == Filterselect::Filter16
    }
}
#[doc = "Field `FILTER` writer - Proportional Integral Filter Selection"]
pub type FilterW<'a, REG> = crate::FieldWriter<'a, REG, 4, Filterselect, crate::Safe>;
impl<'a, REG> FilterW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Bandwidth = 92.7Khz and Damping Factor = 0.76"]
    #[inline(always)]
    pub fn filter1(self) -> &'a mut crate::W<REG> {
        self.variant(Filterselect::Filter1)
    }
    #[doc = "Bandwidth = 131Khz and Damping Factor = 1.08"]
    #[inline(always)]
    pub fn filter2(self) -> &'a mut crate::W<REG> {
        self.variant(Filterselect::Filter2)
    }
    #[doc = "Bandwidth = 46.4Khz and Damping Factor = 0.38"]
    #[inline(always)]
    pub fn filter3(self) -> &'a mut crate::W<REG> {
        self.variant(Filterselect::Filter3)
    }
    #[doc = "Bandwidth = 65.6Khz and Damping Factor = 0.54"]
    #[inline(always)]
    pub fn filter4(self) -> &'a mut crate::W<REG> {
        self.variant(Filterselect::Filter4)
    }
    #[doc = "Bandwidth = 131Khz and Damping Factor = 0.56"]
    #[inline(always)]
    pub fn filter5(self) -> &'a mut crate::W<REG> {
        self.variant(Filterselect::Filter5)
    }
    #[doc = "Bandwidth = 185Khz and Damping Factor = 0.79"]
    #[inline(always)]
    pub fn filter6(self) -> &'a mut crate::W<REG> {
        self.variant(Filterselect::Filter6)
    }
    #[doc = "Bandwidth = 65.6Khz and Damping Factor = 0.28"]
    #[inline(always)]
    pub fn filter7(self) -> &'a mut crate::W<REG> {
        self.variant(Filterselect::Filter7)
    }
    #[doc = "Bandwidth = 92.7Khz and Damping Factor = 0.39"]
    #[inline(always)]
    pub fn filter8(self) -> &'a mut crate::W<REG> {
        self.variant(Filterselect::Filter8)
    }
    #[doc = "Bandwidth = 46.4Khz and Damping Factor = 1.49"]
    #[inline(always)]
    pub fn filter9(self) -> &'a mut crate::W<REG> {
        self.variant(Filterselect::Filter9)
    }
    #[doc = "Bandwidth = 65.6Khz and Damping Factor = 2.11"]
    #[inline(always)]
    pub fn filter10(self) -> &'a mut crate::W<REG> {
        self.variant(Filterselect::Filter10)
    }
    #[doc = "Bandwidth = 23.2Khz and Damping Factor = 0.75"]
    #[inline(always)]
    pub fn filter11(self) -> &'a mut crate::W<REG> {
        self.variant(Filterselect::Filter11)
    }
    #[doc = "Bandwidth = 32.8Khz and Damping Factor = 1.06"]
    #[inline(always)]
    pub fn filter12(self) -> &'a mut crate::W<REG> {
        self.variant(Filterselect::Filter12)
    }
    #[doc = "Bandwidth = 65.6Khz and Damping Factor = 1.07"]
    #[inline(always)]
    pub fn filter13(self) -> &'a mut crate::W<REG> {
        self.variant(Filterselect::Filter13)
    }
    #[doc = "Bandwidth = 92.7Khz and Damping Factor = 1.51"]
    #[inline(always)]
    pub fn filter14(self) -> &'a mut crate::W<REG> {
        self.variant(Filterselect::Filter14)
    }
    #[doc = "Bandwidth = 32.8Khz and Damping Factor = 0.53"]
    #[inline(always)]
    pub fn filter15(self) -> &'a mut crate::W<REG> {
        self.variant(Filterselect::Filter15)
    }
    #[doc = "Bandwidth = 46.4Khz and Damping Factor = 0.75"]
    #[inline(always)]
    pub fn filter16(self) -> &'a mut crate::W<REG> {
        self.variant(Filterselect::Filter16)
    }
}
#[doc = "Field `WUF` reader - Wake Up Fast"]
pub type WufR = crate::BitReader;
#[doc = "Field `WUF` writer - Wake Up Fast"]
pub type WufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Reference Clock Selection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Refclkselect {
    #[doc = "0: Dedicated GCLK clock reference"]
    Gclk = 0,
    #[doc = "1: XOSC32K clock reference"]
    Xosc32 = 1,
    #[doc = "2: XOSC0 clock reference"]
    Xosc0 = 2,
    #[doc = "3: XOSC1 clock reference"]
    Xosc1 = 3,
}
impl From<Refclkselect> for u8 {
    #[inline(always)]
    fn from(variant: Refclkselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Refclkselect {
    type Ux = u8;
}
impl crate::IsEnum for Refclkselect {}
#[doc = "Field `REFCLK` reader - Reference Clock Selection"]
pub type RefclkR = crate::FieldReader<Refclkselect>;
impl RefclkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Refclkselect> {
        match self.bits {
            0 => Some(Refclkselect::Gclk),
            1 => Some(Refclkselect::Xosc32),
            2 => Some(Refclkselect::Xosc0),
            3 => Some(Refclkselect::Xosc1),
            _ => None,
        }
    }
    #[doc = "Dedicated GCLK clock reference"]
    #[inline(always)]
    pub fn is_gclk(&self) -> bool {
        *self == Refclkselect::Gclk
    }
    #[doc = "XOSC32K clock reference"]
    #[inline(always)]
    pub fn is_xosc32(&self) -> bool {
        *self == Refclkselect::Xosc32
    }
    #[doc = "XOSC0 clock reference"]
    #[inline(always)]
    pub fn is_xosc0(&self) -> bool {
        *self == Refclkselect::Xosc0
    }
    #[doc = "XOSC1 clock reference"]
    #[inline(always)]
    pub fn is_xosc1(&self) -> bool {
        *self == Refclkselect::Xosc1
    }
}
#[doc = "Field `REFCLK` writer - Reference Clock Selection"]
pub type RefclkW<'a, REG> = crate::FieldWriter<'a, REG, 3, Refclkselect>;
impl<'a, REG> RefclkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Dedicated GCLK clock reference"]
    #[inline(always)]
    pub fn gclk(self) -> &'a mut crate::W<REG> {
        self.variant(Refclkselect::Gclk)
    }
    #[doc = "XOSC32K clock reference"]
    #[inline(always)]
    pub fn xosc32(self) -> &'a mut crate::W<REG> {
        self.variant(Refclkselect::Xosc32)
    }
    #[doc = "XOSC0 clock reference"]
    #[inline(always)]
    pub fn xosc0(self) -> &'a mut crate::W<REG> {
        self.variant(Refclkselect::Xosc0)
    }
    #[doc = "XOSC1 clock reference"]
    #[inline(always)]
    pub fn xosc1(self) -> &'a mut crate::W<REG> {
        self.variant(Refclkselect::Xosc1)
    }
}
#[doc = "Lock Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ltimeselect {
    #[doc = "0: No time-out. Automatic lock"]
    Default = 0,
    #[doc = "4: Time-out if no lock within 800us"]
    _800us = 4,
    #[doc = "5: Time-out if no lock within 900us"]
    _900us = 5,
    #[doc = "6: Time-out if no lock within 1ms"]
    _1ms = 6,
    #[doc = "7: Time-out if no lock within 1.1ms"]
    _1p1ms = 7,
}
impl From<Ltimeselect> for u8 {
    #[inline(always)]
    fn from(variant: Ltimeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ltimeselect {
    type Ux = u8;
}
impl crate::IsEnum for Ltimeselect {}
#[doc = "Field `LTIME` reader - Lock Time"]
pub type LtimeR = crate::FieldReader<Ltimeselect>;
impl LtimeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ltimeselect> {
        match self.bits {
            0 => Some(Ltimeselect::Default),
            4 => Some(Ltimeselect::_800us),
            5 => Some(Ltimeselect::_900us),
            6 => Some(Ltimeselect::_1ms),
            7 => Some(Ltimeselect::_1p1ms),
            _ => None,
        }
    }
    #[doc = "No time-out. Automatic lock"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Ltimeselect::Default
    }
    #[doc = "Time-out if no lock within 800us"]
    #[inline(always)]
    pub fn is_800us(&self) -> bool {
        *self == Ltimeselect::_800us
    }
    #[doc = "Time-out if no lock within 900us"]
    #[inline(always)]
    pub fn is_900us(&self) -> bool {
        *self == Ltimeselect::_900us
    }
    #[doc = "Time-out if no lock within 1ms"]
    #[inline(always)]
    pub fn is_1ms(&self) -> bool {
        *self == Ltimeselect::_1ms
    }
    #[doc = "Time-out if no lock within 1.1ms"]
    #[inline(always)]
    pub fn is_1p1ms(&self) -> bool {
        *self == Ltimeselect::_1p1ms
    }
}
#[doc = "Field `LTIME` writer - Lock Time"]
pub type LtimeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ltimeselect>;
impl<'a, REG> LtimeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No time-out. Automatic lock"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Ltimeselect::Default)
    }
    #[doc = "Time-out if no lock within 800us"]
    #[inline(always)]
    pub fn _800us(self) -> &'a mut crate::W<REG> {
        self.variant(Ltimeselect::_800us)
    }
    #[doc = "Time-out if no lock within 900us"]
    #[inline(always)]
    pub fn _900us(self) -> &'a mut crate::W<REG> {
        self.variant(Ltimeselect::_900us)
    }
    #[doc = "Time-out if no lock within 1ms"]
    #[inline(always)]
    pub fn _1ms(self) -> &'a mut crate::W<REG> {
        self.variant(Ltimeselect::_1ms)
    }
    #[doc = "Time-out if no lock within 1.1ms"]
    #[inline(always)]
    pub fn _1p1ms(self) -> &'a mut crate::W<REG> {
        self.variant(Ltimeselect::_1p1ms)
    }
}
#[doc = "Field `LBYPASS` reader - Lock Bypass"]
pub type LbypassR = crate::BitReader;
#[doc = "Field `LBYPASS` writer - Lock Bypass"]
pub type LbypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Sigma-Delta DCO Filter Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dcofilterselect {
    #[doc = "0: Capacitor(pF) = 0.5 and Bandwidth Fn (MHz) = 3.21"]
    Filter1 = 0,
    #[doc = "1: Capacitor(pF) = 1 and Bandwidth Fn (MHz) = 1.6"]
    Filter2 = 1,
    #[doc = "2: Capacitor(pF) = 1.5 and Bandwidth Fn (MHz) = 1.1"]
    Filter3 = 2,
    #[doc = "3: Capacitor(pF) = 2 and Bandwidth Fn (MHz) = 0.8"]
    Filter4 = 3,
    #[doc = "4: Capacitor(pF) = 2.5 and Bandwidth Fn (MHz) = 0.64"]
    Filter5 = 4,
    #[doc = "5: Capacitor(pF) = 3 and Bandwidth Fn (MHz) = 0.55"]
    Filter6 = 5,
    #[doc = "6: Capacitor(pF) = 3.5 and Bandwidth Fn (MHz) = 0.45"]
    Filter7 = 6,
    #[doc = "7: Capacitor(pF) = 4 and Bandwidth Fn (MHz) = 0.4"]
    Filter8 = 7,
}
impl From<Dcofilterselect> for u8 {
    #[inline(always)]
    fn from(variant: Dcofilterselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dcofilterselect {
    type Ux = u8;
}
impl crate::IsEnum for Dcofilterselect {}
#[doc = "Field `DCOFILTER` reader - Sigma-Delta DCO Filter Selection"]
pub type DcofilterR = crate::FieldReader<Dcofilterselect>;
impl DcofilterR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dcofilterselect {
        match self.bits {
            0 => Dcofilterselect::Filter1,
            1 => Dcofilterselect::Filter2,
            2 => Dcofilterselect::Filter3,
            3 => Dcofilterselect::Filter4,
            4 => Dcofilterselect::Filter5,
            5 => Dcofilterselect::Filter6,
            6 => Dcofilterselect::Filter7,
            7 => Dcofilterselect::Filter8,
            _ => unreachable!(),
        }
    }
    #[doc = "Capacitor(pF) = 0.5 and Bandwidth Fn (MHz) = 3.21"]
    #[inline(always)]
    pub fn is_filter1(&self) -> bool {
        *self == Dcofilterselect::Filter1
    }
    #[doc = "Capacitor(pF) = 1 and Bandwidth Fn (MHz) = 1.6"]
    #[inline(always)]
    pub fn is_filter2(&self) -> bool {
        *self == Dcofilterselect::Filter2
    }
    #[doc = "Capacitor(pF) = 1.5 and Bandwidth Fn (MHz) = 1.1"]
    #[inline(always)]
    pub fn is_filter3(&self) -> bool {
        *self == Dcofilterselect::Filter3
    }
    #[doc = "Capacitor(pF) = 2 and Bandwidth Fn (MHz) = 0.8"]
    #[inline(always)]
    pub fn is_filter4(&self) -> bool {
        *self == Dcofilterselect::Filter4
    }
    #[doc = "Capacitor(pF) = 2.5 and Bandwidth Fn (MHz) = 0.64"]
    #[inline(always)]
    pub fn is_filter5(&self) -> bool {
        *self == Dcofilterselect::Filter5
    }
    #[doc = "Capacitor(pF) = 3 and Bandwidth Fn (MHz) = 0.55"]
    #[inline(always)]
    pub fn is_filter6(&self) -> bool {
        *self == Dcofilterselect::Filter6
    }
    #[doc = "Capacitor(pF) = 3.5 and Bandwidth Fn (MHz) = 0.45"]
    #[inline(always)]
    pub fn is_filter7(&self) -> bool {
        *self == Dcofilterselect::Filter7
    }
    #[doc = "Capacitor(pF) = 4 and Bandwidth Fn (MHz) = 0.4"]
    #[inline(always)]
    pub fn is_filter8(&self) -> bool {
        *self == Dcofilterselect::Filter8
    }
}
#[doc = "Field `DCOFILTER` writer - Sigma-Delta DCO Filter Selection"]
pub type DcofilterW<'a, REG> = crate::FieldWriter<'a, REG, 3, Dcofilterselect, crate::Safe>;
impl<'a, REG> DcofilterW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Capacitor(pF) = 0.5 and Bandwidth Fn (MHz) = 3.21"]
    #[inline(always)]
    pub fn filter1(self) -> &'a mut crate::W<REG> {
        self.variant(Dcofilterselect::Filter1)
    }
    #[doc = "Capacitor(pF) = 1 and Bandwidth Fn (MHz) = 1.6"]
    #[inline(always)]
    pub fn filter2(self) -> &'a mut crate::W<REG> {
        self.variant(Dcofilterselect::Filter2)
    }
    #[doc = "Capacitor(pF) = 1.5 and Bandwidth Fn (MHz) = 1.1"]
    #[inline(always)]
    pub fn filter3(self) -> &'a mut crate::W<REG> {
        self.variant(Dcofilterselect::Filter3)
    }
    #[doc = "Capacitor(pF) = 2 and Bandwidth Fn (MHz) = 0.8"]
    #[inline(always)]
    pub fn filter4(self) -> &'a mut crate::W<REG> {
        self.variant(Dcofilterselect::Filter4)
    }
    #[doc = "Capacitor(pF) = 2.5 and Bandwidth Fn (MHz) = 0.64"]
    #[inline(always)]
    pub fn filter5(self) -> &'a mut crate::W<REG> {
        self.variant(Dcofilterselect::Filter5)
    }
    #[doc = "Capacitor(pF) = 3 and Bandwidth Fn (MHz) = 0.55"]
    #[inline(always)]
    pub fn filter6(self) -> &'a mut crate::W<REG> {
        self.variant(Dcofilterselect::Filter6)
    }
    #[doc = "Capacitor(pF) = 3.5 and Bandwidth Fn (MHz) = 0.45"]
    #[inline(always)]
    pub fn filter7(self) -> &'a mut crate::W<REG> {
        self.variant(Dcofilterselect::Filter7)
    }
    #[doc = "Capacitor(pF) = 4 and Bandwidth Fn (MHz) = 0.4"]
    #[inline(always)]
    pub fn filter8(self) -> &'a mut crate::W<REG> {
        self.variant(Dcofilterselect::Filter8)
    }
}
#[doc = "Field `DCOEN` reader - DCO Filter Enable"]
pub type DcoenR = crate::BitReader;
#[doc = "Field `DCOEN` writer - DCO Filter Enable"]
pub type DcoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIV` reader - Clock Divider"]
pub type DivR = crate::FieldReader<u16>;
#[doc = "Field `DIV` writer - Clock Divider"]
pub type DivW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:3 - Proportional Integral Filter Selection"]
    #[inline(always)]
    pub fn filter(&self) -> FilterR {
        FilterR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Wake Up Fast"]
    #[inline(always)]
    pub fn wuf(&self) -> WufR {
        WufR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Reference Clock Selection"]
    #[inline(always)]
    pub fn refclk(&self) -> RefclkR {
        RefclkR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Lock Time"]
    #[inline(always)]
    pub fn ltime(&self) -> LtimeR {
        LtimeR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Lock Bypass"]
    #[inline(always)]
    pub fn lbypass(&self) -> LbypassR {
        LbypassR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Sigma-Delta DCO Filter Selection"]
    #[inline(always)]
    pub fn dcofilter(&self) -> DcofilterR {
        DcofilterR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - DCO Filter Enable"]
    #[inline(always)]
    pub fn dcoen(&self) -> DcoenR {
        DcoenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:26 - Clock Divider"]
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - Proportional Integral Filter Selection"]
    #[inline(always)]
    #[must_use]
    pub fn filter(&mut self) -> FilterW<DpllctrlbSpec> {
        FilterW::new(self, 0)
    }
    #[doc = "Bit 4 - Wake Up Fast"]
    #[inline(always)]
    #[must_use]
    pub fn wuf(&mut self) -> WufW<DpllctrlbSpec> {
        WufW::new(self, 4)
    }
    #[doc = "Bits 5:7 - Reference Clock Selection"]
    #[inline(always)]
    #[must_use]
    pub fn refclk(&mut self) -> RefclkW<DpllctrlbSpec> {
        RefclkW::new(self, 5)
    }
    #[doc = "Bits 8:10 - Lock Time"]
    #[inline(always)]
    #[must_use]
    pub fn ltime(&mut self) -> LtimeW<DpllctrlbSpec> {
        LtimeW::new(self, 8)
    }
    #[doc = "Bit 11 - Lock Bypass"]
    #[inline(always)]
    #[must_use]
    pub fn lbypass(&mut self) -> LbypassW<DpllctrlbSpec> {
        LbypassW::new(self, 11)
    }
    #[doc = "Bits 12:14 - Sigma-Delta DCO Filter Selection"]
    #[inline(always)]
    #[must_use]
    pub fn dcofilter(&mut self) -> DcofilterW<DpllctrlbSpec> {
        DcofilterW::new(self, 12)
    }
    #[doc = "Bit 15 - DCO Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dcoen(&mut self) -> DcoenW<DpllctrlbSpec> {
        DcoenW::new(self, 15)
    }
    #[doc = "Bits 16:26 - Clock Divider"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DivW<DpllctrlbSpec> {
        DivW::new(self, 16)
    }
}
#[doc = "DPLL Control B\n\nYou can [`read`](crate::Reg::read) this register and get [`dpllctrlb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpllctrlb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpllctrlbSpec;
impl crate::RegisterSpec for DpllctrlbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpllctrlb::R`](R) reader structure"]
impl crate::Readable for DpllctrlbSpec {}
#[doc = "`write(|w| ..)` method takes [`dpllctrlb::W`](W) writer structure"]
impl crate::Writable for DpllctrlbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DPLLCTRLB to value 0x20"]
impl crate::Resettable for DpllctrlbSpec {
    const RESET_VALUE: u32 = 0x20;
}
