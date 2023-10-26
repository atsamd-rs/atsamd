#[doc = "Register `DPLLCTRLB` reader"]
pub type R = crate::R<DPLLCTRLB_SPEC>;
#[doc = "Register `DPLLCTRLB` writer"]
pub type W = crate::W<DPLLCTRLB_SPEC>;
#[doc = "Field `FILTER` reader - Proportional Integral Filter Selection"]
pub type FILTER_R = crate::FieldReader<FILTERSELECT_A>;
#[doc = "Proportional Integral Filter Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FILTERSELECT_A {
    #[doc = "0: Bandwidth = 92.7Khz and Damping Factor = 0.76"]
    FILTER1 = 0,
    #[doc = "1: Bandwidth = 131Khz and Damping Factor = 1.08"]
    FILTER2 = 1,
    #[doc = "2: Bandwidth = 46.4Khz and Damping Factor = 0.38"]
    FILTER3 = 2,
    #[doc = "3: Bandwidth = 65.6Khz and Damping Factor = 0.54"]
    FILTER4 = 3,
    #[doc = "4: Bandwidth = 131Khz and Damping Factor = 0.56"]
    FILTER5 = 4,
    #[doc = "5: Bandwidth = 185Khz and Damping Factor = 0.79"]
    FILTER6 = 5,
    #[doc = "6: Bandwidth = 65.6Khz and Damping Factor = 0.28"]
    FILTER7 = 6,
    #[doc = "7: Bandwidth = 92.7Khz and Damping Factor = 0.39"]
    FILTER8 = 7,
    #[doc = "8: Bandwidth = 46.4Khz and Damping Factor = 1.49"]
    FILTER9 = 8,
    #[doc = "9: Bandwidth = 65.6Khz and Damping Factor = 2.11"]
    FILTER10 = 9,
    #[doc = "10: Bandwidth = 23.2Khz and Damping Factor = 0.75"]
    FILTER11 = 10,
    #[doc = "11: Bandwidth = 32.8Khz and Damping Factor = 1.06"]
    FILTER12 = 11,
    #[doc = "12: Bandwidth = 65.6Khz and Damping Factor = 1.07"]
    FILTER13 = 12,
    #[doc = "13: Bandwidth = 92.7Khz and Damping Factor = 1.51"]
    FILTER14 = 13,
    #[doc = "14: Bandwidth = 32.8Khz and Damping Factor = 0.53"]
    FILTER15 = 14,
    #[doc = "15: Bandwidth = 46.4Khz and Damping Factor = 0.75"]
    FILTER16 = 15,
}
impl From<FILTERSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: FILTERSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FILTERSELECT_A {
    type Ux = u8;
}
impl FILTER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FILTERSELECT_A {
        match self.bits {
            0 => FILTERSELECT_A::FILTER1,
            1 => FILTERSELECT_A::FILTER2,
            2 => FILTERSELECT_A::FILTER3,
            3 => FILTERSELECT_A::FILTER4,
            4 => FILTERSELECT_A::FILTER5,
            5 => FILTERSELECT_A::FILTER6,
            6 => FILTERSELECT_A::FILTER7,
            7 => FILTERSELECT_A::FILTER8,
            8 => FILTERSELECT_A::FILTER9,
            9 => FILTERSELECT_A::FILTER10,
            10 => FILTERSELECT_A::FILTER11,
            11 => FILTERSELECT_A::FILTER12,
            12 => FILTERSELECT_A::FILTER13,
            13 => FILTERSELECT_A::FILTER14,
            14 => FILTERSELECT_A::FILTER15,
            15 => FILTERSELECT_A::FILTER16,
            _ => unreachable!(),
        }
    }
    #[doc = "Bandwidth = 92.7Khz and Damping Factor = 0.76"]
    #[inline(always)]
    pub fn is_filter1(&self) -> bool {
        *self == FILTERSELECT_A::FILTER1
    }
    #[doc = "Bandwidth = 131Khz and Damping Factor = 1.08"]
    #[inline(always)]
    pub fn is_filter2(&self) -> bool {
        *self == FILTERSELECT_A::FILTER2
    }
    #[doc = "Bandwidth = 46.4Khz and Damping Factor = 0.38"]
    #[inline(always)]
    pub fn is_filter3(&self) -> bool {
        *self == FILTERSELECT_A::FILTER3
    }
    #[doc = "Bandwidth = 65.6Khz and Damping Factor = 0.54"]
    #[inline(always)]
    pub fn is_filter4(&self) -> bool {
        *self == FILTERSELECT_A::FILTER4
    }
    #[doc = "Bandwidth = 131Khz and Damping Factor = 0.56"]
    #[inline(always)]
    pub fn is_filter5(&self) -> bool {
        *self == FILTERSELECT_A::FILTER5
    }
    #[doc = "Bandwidth = 185Khz and Damping Factor = 0.79"]
    #[inline(always)]
    pub fn is_filter6(&self) -> bool {
        *self == FILTERSELECT_A::FILTER6
    }
    #[doc = "Bandwidth = 65.6Khz and Damping Factor = 0.28"]
    #[inline(always)]
    pub fn is_filter7(&self) -> bool {
        *self == FILTERSELECT_A::FILTER7
    }
    #[doc = "Bandwidth = 92.7Khz and Damping Factor = 0.39"]
    #[inline(always)]
    pub fn is_filter8(&self) -> bool {
        *self == FILTERSELECT_A::FILTER8
    }
    #[doc = "Bandwidth = 46.4Khz and Damping Factor = 1.49"]
    #[inline(always)]
    pub fn is_filter9(&self) -> bool {
        *self == FILTERSELECT_A::FILTER9
    }
    #[doc = "Bandwidth = 65.6Khz and Damping Factor = 2.11"]
    #[inline(always)]
    pub fn is_filter10(&self) -> bool {
        *self == FILTERSELECT_A::FILTER10
    }
    #[doc = "Bandwidth = 23.2Khz and Damping Factor = 0.75"]
    #[inline(always)]
    pub fn is_filter11(&self) -> bool {
        *self == FILTERSELECT_A::FILTER11
    }
    #[doc = "Bandwidth = 32.8Khz and Damping Factor = 1.06"]
    #[inline(always)]
    pub fn is_filter12(&self) -> bool {
        *self == FILTERSELECT_A::FILTER12
    }
    #[doc = "Bandwidth = 65.6Khz and Damping Factor = 1.07"]
    #[inline(always)]
    pub fn is_filter13(&self) -> bool {
        *self == FILTERSELECT_A::FILTER13
    }
    #[doc = "Bandwidth = 92.7Khz and Damping Factor = 1.51"]
    #[inline(always)]
    pub fn is_filter14(&self) -> bool {
        *self == FILTERSELECT_A::FILTER14
    }
    #[doc = "Bandwidth = 32.8Khz and Damping Factor = 0.53"]
    #[inline(always)]
    pub fn is_filter15(&self) -> bool {
        *self == FILTERSELECT_A::FILTER15
    }
    #[doc = "Bandwidth = 46.4Khz and Damping Factor = 0.75"]
    #[inline(always)]
    pub fn is_filter16(&self) -> bool {
        *self == FILTERSELECT_A::FILTER16
    }
}
#[doc = "Field `FILTER` writer - Proportional Integral Filter Selection"]
pub type FILTER_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 4, O, FILTERSELECT_A>;
impl<'a, REG, const O: u8> FILTER_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Bandwidth = 92.7Khz and Damping Factor = 0.76"]
    #[inline(always)]
    pub fn filter1(self) -> &'a mut crate::W<REG> {
        self.variant(FILTERSELECT_A::FILTER1)
    }
    #[doc = "Bandwidth = 131Khz and Damping Factor = 1.08"]
    #[inline(always)]
    pub fn filter2(self) -> &'a mut crate::W<REG> {
        self.variant(FILTERSELECT_A::FILTER2)
    }
    #[doc = "Bandwidth = 46.4Khz and Damping Factor = 0.38"]
    #[inline(always)]
    pub fn filter3(self) -> &'a mut crate::W<REG> {
        self.variant(FILTERSELECT_A::FILTER3)
    }
    #[doc = "Bandwidth = 65.6Khz and Damping Factor = 0.54"]
    #[inline(always)]
    pub fn filter4(self) -> &'a mut crate::W<REG> {
        self.variant(FILTERSELECT_A::FILTER4)
    }
    #[doc = "Bandwidth = 131Khz and Damping Factor = 0.56"]
    #[inline(always)]
    pub fn filter5(self) -> &'a mut crate::W<REG> {
        self.variant(FILTERSELECT_A::FILTER5)
    }
    #[doc = "Bandwidth = 185Khz and Damping Factor = 0.79"]
    #[inline(always)]
    pub fn filter6(self) -> &'a mut crate::W<REG> {
        self.variant(FILTERSELECT_A::FILTER6)
    }
    #[doc = "Bandwidth = 65.6Khz and Damping Factor = 0.28"]
    #[inline(always)]
    pub fn filter7(self) -> &'a mut crate::W<REG> {
        self.variant(FILTERSELECT_A::FILTER7)
    }
    #[doc = "Bandwidth = 92.7Khz and Damping Factor = 0.39"]
    #[inline(always)]
    pub fn filter8(self) -> &'a mut crate::W<REG> {
        self.variant(FILTERSELECT_A::FILTER8)
    }
    #[doc = "Bandwidth = 46.4Khz and Damping Factor = 1.49"]
    #[inline(always)]
    pub fn filter9(self) -> &'a mut crate::W<REG> {
        self.variant(FILTERSELECT_A::FILTER9)
    }
    #[doc = "Bandwidth = 65.6Khz and Damping Factor = 2.11"]
    #[inline(always)]
    pub fn filter10(self) -> &'a mut crate::W<REG> {
        self.variant(FILTERSELECT_A::FILTER10)
    }
    #[doc = "Bandwidth = 23.2Khz and Damping Factor = 0.75"]
    #[inline(always)]
    pub fn filter11(self) -> &'a mut crate::W<REG> {
        self.variant(FILTERSELECT_A::FILTER11)
    }
    #[doc = "Bandwidth = 32.8Khz and Damping Factor = 1.06"]
    #[inline(always)]
    pub fn filter12(self) -> &'a mut crate::W<REG> {
        self.variant(FILTERSELECT_A::FILTER12)
    }
    #[doc = "Bandwidth = 65.6Khz and Damping Factor = 1.07"]
    #[inline(always)]
    pub fn filter13(self) -> &'a mut crate::W<REG> {
        self.variant(FILTERSELECT_A::FILTER13)
    }
    #[doc = "Bandwidth = 92.7Khz and Damping Factor = 1.51"]
    #[inline(always)]
    pub fn filter14(self) -> &'a mut crate::W<REG> {
        self.variant(FILTERSELECT_A::FILTER14)
    }
    #[doc = "Bandwidth = 32.8Khz and Damping Factor = 0.53"]
    #[inline(always)]
    pub fn filter15(self) -> &'a mut crate::W<REG> {
        self.variant(FILTERSELECT_A::FILTER15)
    }
    #[doc = "Bandwidth = 46.4Khz and Damping Factor = 0.75"]
    #[inline(always)]
    pub fn filter16(self) -> &'a mut crate::W<REG> {
        self.variant(FILTERSELECT_A::FILTER16)
    }
}
#[doc = "Field `WUF` reader - Wake Up Fast"]
pub type WUF_R = crate::BitReader;
#[doc = "Field `WUF` writer - Wake Up Fast"]
pub type WUF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `REFCLK` reader - Reference Clock Selection"]
pub type REFCLK_R = crate::FieldReader<REFCLKSELECT_A>;
#[doc = "Reference Clock Selection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REFCLKSELECT_A {
    #[doc = "0: Dedicated GCLK clock reference"]
    GCLK = 0,
    #[doc = "1: XOSC32K clock reference"]
    XOSC32 = 1,
    #[doc = "2: XOSC0 clock reference"]
    XOSC0 = 2,
    #[doc = "3: XOSC1 clock reference"]
    XOSC1 = 3,
}
impl From<REFCLKSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: REFCLKSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for REFCLKSELECT_A {
    type Ux = u8;
}
impl REFCLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<REFCLKSELECT_A> {
        match self.bits {
            0 => Some(REFCLKSELECT_A::GCLK),
            1 => Some(REFCLKSELECT_A::XOSC32),
            2 => Some(REFCLKSELECT_A::XOSC0),
            3 => Some(REFCLKSELECT_A::XOSC1),
            _ => None,
        }
    }
    #[doc = "Dedicated GCLK clock reference"]
    #[inline(always)]
    pub fn is_gclk(&self) -> bool {
        *self == REFCLKSELECT_A::GCLK
    }
    #[doc = "XOSC32K clock reference"]
    #[inline(always)]
    pub fn is_xosc32(&self) -> bool {
        *self == REFCLKSELECT_A::XOSC32
    }
    #[doc = "XOSC0 clock reference"]
    #[inline(always)]
    pub fn is_xosc0(&self) -> bool {
        *self == REFCLKSELECT_A::XOSC0
    }
    #[doc = "XOSC1 clock reference"]
    #[inline(always)]
    pub fn is_xosc1(&self) -> bool {
        *self == REFCLKSELECT_A::XOSC1
    }
}
#[doc = "Field `REFCLK` writer - Reference Clock Selection"]
pub type REFCLK_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, REFCLKSELECT_A>;
impl<'a, REG, const O: u8> REFCLK_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Dedicated GCLK clock reference"]
    #[inline(always)]
    pub fn gclk(self) -> &'a mut crate::W<REG> {
        self.variant(REFCLKSELECT_A::GCLK)
    }
    #[doc = "XOSC32K clock reference"]
    #[inline(always)]
    pub fn xosc32(self) -> &'a mut crate::W<REG> {
        self.variant(REFCLKSELECT_A::XOSC32)
    }
    #[doc = "XOSC0 clock reference"]
    #[inline(always)]
    pub fn xosc0(self) -> &'a mut crate::W<REG> {
        self.variant(REFCLKSELECT_A::XOSC0)
    }
    #[doc = "XOSC1 clock reference"]
    #[inline(always)]
    pub fn xosc1(self) -> &'a mut crate::W<REG> {
        self.variant(REFCLKSELECT_A::XOSC1)
    }
}
#[doc = "Field `LTIME` reader - Lock Time"]
pub type LTIME_R = crate::FieldReader<LTIMESELECT_A>;
#[doc = "Lock Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LTIMESELECT_A {
    #[doc = "0: No time-out. Automatic lock"]
    DEFAULT = 0,
    #[doc = "4: Time-out if no lock within 800us"]
    _800US = 4,
    #[doc = "5: Time-out if no lock within 900us"]
    _900US = 5,
    #[doc = "6: Time-out if no lock within 1ms"]
    _1MS = 6,
    #[doc = "7: Time-out if no lock within 1.1ms"]
    _1P1MS = 7,
}
impl From<LTIMESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: LTIMESELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LTIMESELECT_A {
    type Ux = u8;
}
impl LTIME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LTIMESELECT_A> {
        match self.bits {
            0 => Some(LTIMESELECT_A::DEFAULT),
            4 => Some(LTIMESELECT_A::_800US),
            5 => Some(LTIMESELECT_A::_900US),
            6 => Some(LTIMESELECT_A::_1MS),
            7 => Some(LTIMESELECT_A::_1P1MS),
            _ => None,
        }
    }
    #[doc = "No time-out. Automatic lock"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == LTIMESELECT_A::DEFAULT
    }
    #[doc = "Time-out if no lock within 800us"]
    #[inline(always)]
    pub fn is_800us(&self) -> bool {
        *self == LTIMESELECT_A::_800US
    }
    #[doc = "Time-out if no lock within 900us"]
    #[inline(always)]
    pub fn is_900us(&self) -> bool {
        *self == LTIMESELECT_A::_900US
    }
    #[doc = "Time-out if no lock within 1ms"]
    #[inline(always)]
    pub fn is_1ms(&self) -> bool {
        *self == LTIMESELECT_A::_1MS
    }
    #[doc = "Time-out if no lock within 1.1ms"]
    #[inline(always)]
    pub fn is_1p1ms(&self) -> bool {
        *self == LTIMESELECT_A::_1P1MS
    }
}
#[doc = "Field `LTIME` writer - Lock Time"]
pub type LTIME_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, LTIMESELECT_A>;
impl<'a, REG, const O: u8> LTIME_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No time-out. Automatic lock"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(LTIMESELECT_A::DEFAULT)
    }
    #[doc = "Time-out if no lock within 800us"]
    #[inline(always)]
    pub fn _800us(self) -> &'a mut crate::W<REG> {
        self.variant(LTIMESELECT_A::_800US)
    }
    #[doc = "Time-out if no lock within 900us"]
    #[inline(always)]
    pub fn _900us(self) -> &'a mut crate::W<REG> {
        self.variant(LTIMESELECT_A::_900US)
    }
    #[doc = "Time-out if no lock within 1ms"]
    #[inline(always)]
    pub fn _1ms(self) -> &'a mut crate::W<REG> {
        self.variant(LTIMESELECT_A::_1MS)
    }
    #[doc = "Time-out if no lock within 1.1ms"]
    #[inline(always)]
    pub fn _1p1ms(self) -> &'a mut crate::W<REG> {
        self.variant(LTIMESELECT_A::_1P1MS)
    }
}
#[doc = "Field `LBYPASS` reader - Lock Bypass"]
pub type LBYPASS_R = crate::BitReader;
#[doc = "Field `LBYPASS` writer - Lock Bypass"]
pub type LBYPASS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DCOFILTER` reader - Sigma-Delta DCO Filter Selection"]
pub type DCOFILTER_R = crate::FieldReader<DCOFILTERSELECT_A>;
#[doc = "Sigma-Delta DCO Filter Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DCOFILTERSELECT_A {
    #[doc = "0: Capacitor(pF) = 0.5 and Bandwidth Fn (MHz) = 3.21"]
    FILTER1 = 0,
    #[doc = "1: Capacitor(pF) = 1 and Bandwidth Fn (MHz) = 1.6"]
    FILTER2 = 1,
    #[doc = "2: Capacitor(pF) = 1.5 and Bandwidth Fn (MHz) = 1.1"]
    FILTER3 = 2,
    #[doc = "3: Capacitor(pF) = 2 and Bandwidth Fn (MHz) = 0.8"]
    FILTER4 = 3,
    #[doc = "4: Capacitor(pF) = 2.5 and Bandwidth Fn (MHz) = 0.64"]
    FILTER5 = 4,
    #[doc = "5: Capacitor(pF) = 3 and Bandwidth Fn (MHz) = 0.55"]
    FILTER6 = 5,
    #[doc = "6: Capacitor(pF) = 3.5 and Bandwidth Fn (MHz) = 0.45"]
    FILTER7 = 6,
    #[doc = "7: Capacitor(pF) = 4 and Bandwidth Fn (MHz) = 0.4"]
    FILTER8 = 7,
}
impl From<DCOFILTERSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: DCOFILTERSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DCOFILTERSELECT_A {
    type Ux = u8;
}
impl DCOFILTER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DCOFILTERSELECT_A {
        match self.bits {
            0 => DCOFILTERSELECT_A::FILTER1,
            1 => DCOFILTERSELECT_A::FILTER2,
            2 => DCOFILTERSELECT_A::FILTER3,
            3 => DCOFILTERSELECT_A::FILTER4,
            4 => DCOFILTERSELECT_A::FILTER5,
            5 => DCOFILTERSELECT_A::FILTER6,
            6 => DCOFILTERSELECT_A::FILTER7,
            7 => DCOFILTERSELECT_A::FILTER8,
            _ => unreachable!(),
        }
    }
    #[doc = "Capacitor(pF) = 0.5 and Bandwidth Fn (MHz) = 3.21"]
    #[inline(always)]
    pub fn is_filter1(&self) -> bool {
        *self == DCOFILTERSELECT_A::FILTER1
    }
    #[doc = "Capacitor(pF) = 1 and Bandwidth Fn (MHz) = 1.6"]
    #[inline(always)]
    pub fn is_filter2(&self) -> bool {
        *self == DCOFILTERSELECT_A::FILTER2
    }
    #[doc = "Capacitor(pF) = 1.5 and Bandwidth Fn (MHz) = 1.1"]
    #[inline(always)]
    pub fn is_filter3(&self) -> bool {
        *self == DCOFILTERSELECT_A::FILTER3
    }
    #[doc = "Capacitor(pF) = 2 and Bandwidth Fn (MHz) = 0.8"]
    #[inline(always)]
    pub fn is_filter4(&self) -> bool {
        *self == DCOFILTERSELECT_A::FILTER4
    }
    #[doc = "Capacitor(pF) = 2.5 and Bandwidth Fn (MHz) = 0.64"]
    #[inline(always)]
    pub fn is_filter5(&self) -> bool {
        *self == DCOFILTERSELECT_A::FILTER5
    }
    #[doc = "Capacitor(pF) = 3 and Bandwidth Fn (MHz) = 0.55"]
    #[inline(always)]
    pub fn is_filter6(&self) -> bool {
        *self == DCOFILTERSELECT_A::FILTER6
    }
    #[doc = "Capacitor(pF) = 3.5 and Bandwidth Fn (MHz) = 0.45"]
    #[inline(always)]
    pub fn is_filter7(&self) -> bool {
        *self == DCOFILTERSELECT_A::FILTER7
    }
    #[doc = "Capacitor(pF) = 4 and Bandwidth Fn (MHz) = 0.4"]
    #[inline(always)]
    pub fn is_filter8(&self) -> bool {
        *self == DCOFILTERSELECT_A::FILTER8
    }
}
#[doc = "Field `DCOFILTER` writer - Sigma-Delta DCO Filter Selection"]
pub type DCOFILTER_W<'a, REG, const O: u8> =
    crate::FieldWriterSafe<'a, REG, 3, O, DCOFILTERSELECT_A>;
impl<'a, REG, const O: u8> DCOFILTER_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Capacitor(pF) = 0.5 and Bandwidth Fn (MHz) = 3.21"]
    #[inline(always)]
    pub fn filter1(self) -> &'a mut crate::W<REG> {
        self.variant(DCOFILTERSELECT_A::FILTER1)
    }
    #[doc = "Capacitor(pF) = 1 and Bandwidth Fn (MHz) = 1.6"]
    #[inline(always)]
    pub fn filter2(self) -> &'a mut crate::W<REG> {
        self.variant(DCOFILTERSELECT_A::FILTER2)
    }
    #[doc = "Capacitor(pF) = 1.5 and Bandwidth Fn (MHz) = 1.1"]
    #[inline(always)]
    pub fn filter3(self) -> &'a mut crate::W<REG> {
        self.variant(DCOFILTERSELECT_A::FILTER3)
    }
    #[doc = "Capacitor(pF) = 2 and Bandwidth Fn (MHz) = 0.8"]
    #[inline(always)]
    pub fn filter4(self) -> &'a mut crate::W<REG> {
        self.variant(DCOFILTERSELECT_A::FILTER4)
    }
    #[doc = "Capacitor(pF) = 2.5 and Bandwidth Fn (MHz) = 0.64"]
    #[inline(always)]
    pub fn filter5(self) -> &'a mut crate::W<REG> {
        self.variant(DCOFILTERSELECT_A::FILTER5)
    }
    #[doc = "Capacitor(pF) = 3 and Bandwidth Fn (MHz) = 0.55"]
    #[inline(always)]
    pub fn filter6(self) -> &'a mut crate::W<REG> {
        self.variant(DCOFILTERSELECT_A::FILTER6)
    }
    #[doc = "Capacitor(pF) = 3.5 and Bandwidth Fn (MHz) = 0.45"]
    #[inline(always)]
    pub fn filter7(self) -> &'a mut crate::W<REG> {
        self.variant(DCOFILTERSELECT_A::FILTER7)
    }
    #[doc = "Capacitor(pF) = 4 and Bandwidth Fn (MHz) = 0.4"]
    #[inline(always)]
    pub fn filter8(self) -> &'a mut crate::W<REG> {
        self.variant(DCOFILTERSELECT_A::FILTER8)
    }
}
#[doc = "Field `DCOEN` reader - DCO Filter Enable"]
pub type DCOEN_R = crate::BitReader;
#[doc = "Field `DCOEN` writer - DCO Filter Enable"]
pub type DCOEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DIV` reader - Clock Divider"]
pub type DIV_R = crate::FieldReader<u16>;
#[doc = "Field `DIV` writer - Clock Divider"]
pub type DIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 11, O, u16>;
impl R {
    #[doc = "Bits 0:3 - Proportional Integral Filter Selection"]
    #[inline(always)]
    pub fn filter(&self) -> FILTER_R {
        FILTER_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Wake Up Fast"]
    #[inline(always)]
    pub fn wuf(&self) -> WUF_R {
        WUF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Reference Clock Selection"]
    #[inline(always)]
    pub fn refclk(&self) -> REFCLK_R {
        REFCLK_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Lock Time"]
    #[inline(always)]
    pub fn ltime(&self) -> LTIME_R {
        LTIME_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Lock Bypass"]
    #[inline(always)]
    pub fn lbypass(&self) -> LBYPASS_R {
        LBYPASS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Sigma-Delta DCO Filter Selection"]
    #[inline(always)]
    pub fn dcofilter(&self) -> DCOFILTER_R {
        DCOFILTER_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - DCO Filter Enable"]
    #[inline(always)]
    pub fn dcoen(&self) -> DCOEN_R {
        DCOEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:26 - Clock Divider"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - Proportional Integral Filter Selection"]
    #[inline(always)]
    #[must_use]
    pub fn filter(&mut self) -> FILTER_W<DPLLCTRLB_SPEC, 0> {
        FILTER_W::new(self)
    }
    #[doc = "Bit 4 - Wake Up Fast"]
    #[inline(always)]
    #[must_use]
    pub fn wuf(&mut self) -> WUF_W<DPLLCTRLB_SPEC, 4> {
        WUF_W::new(self)
    }
    #[doc = "Bits 5:7 - Reference Clock Selection"]
    #[inline(always)]
    #[must_use]
    pub fn refclk(&mut self) -> REFCLK_W<DPLLCTRLB_SPEC, 5> {
        REFCLK_W::new(self)
    }
    #[doc = "Bits 8:10 - Lock Time"]
    #[inline(always)]
    #[must_use]
    pub fn ltime(&mut self) -> LTIME_W<DPLLCTRLB_SPEC, 8> {
        LTIME_W::new(self)
    }
    #[doc = "Bit 11 - Lock Bypass"]
    #[inline(always)]
    #[must_use]
    pub fn lbypass(&mut self) -> LBYPASS_W<DPLLCTRLB_SPEC, 11> {
        LBYPASS_W::new(self)
    }
    #[doc = "Bits 12:14 - Sigma-Delta DCO Filter Selection"]
    #[inline(always)]
    #[must_use]
    pub fn dcofilter(&mut self) -> DCOFILTER_W<DPLLCTRLB_SPEC, 12> {
        DCOFILTER_W::new(self)
    }
    #[doc = "Bit 15 - DCO Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dcoen(&mut self) -> DCOEN_W<DPLLCTRLB_SPEC, 15> {
        DCOEN_W::new(self)
    }
    #[doc = "Bits 16:26 - Clock Divider"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DIV_W<DPLLCTRLB_SPEC, 16> {
        DIV_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DPLL Control B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpllctrlb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpllctrlb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DPLLCTRLB_SPEC;
impl crate::RegisterSpec for DPLLCTRLB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpllctrlb::R`](R) reader structure"]
impl crate::Readable for DPLLCTRLB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dpllctrlb::W`](W) writer structure"]
impl crate::Writable for DPLLCTRLB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DPLLCTRLB to value 0x20"]
impl crate::Resettable for DPLLCTRLB_SPEC {
    const RESET_VALUE: Self::Ux = 0x20;
}
