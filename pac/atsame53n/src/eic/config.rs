#[doc = "Register `CONFIG[%s]` reader"]
pub type R = crate::R<ConfigSpec>;
#[doc = "Register `CONFIG[%s]` writer"]
pub type W = crate::W<ConfigSpec>;
#[doc = "Input Sense Configuration 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sense0select {
    #[doc = "0: No detection"]
    None = 0,
    #[doc = "1: Rising edge detection"]
    Rise = 1,
    #[doc = "2: Falling edge detection"]
    Fall = 2,
    #[doc = "3: Both edges detection"]
    Both = 3,
    #[doc = "4: High level detection"]
    High = 4,
    #[doc = "5: Low level detection"]
    Low = 5,
}
impl From<Sense0select> for u8 {
    #[inline(always)]
    fn from(variant: Sense0select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sense0select {
    type Ux = u8;
}
impl crate::IsEnum for Sense0select {}
#[doc = "Field `SENSE0` reader - Input Sense Configuration 0"]
pub type Sense0R = crate::FieldReader<Sense0select>;
impl Sense0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sense0select> {
        match self.bits {
            0 => Some(Sense0select::None),
            1 => Some(Sense0select::Rise),
            2 => Some(Sense0select::Fall),
            3 => Some(Sense0select::Both),
            4 => Some(Sense0select::High),
            5 => Some(Sense0select::Low),
            _ => None,
        }
    }
    #[doc = "No detection"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Sense0select::None
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Sense0select::Rise
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Sense0select::Fall
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == Sense0select::Both
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Sense0select::High
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Sense0select::Low
    }
}
#[doc = "Field `SENSE0` writer - Input Sense Configuration 0"]
pub type Sense0W<'a, REG> = crate::FieldWriter<'a, REG, 3, Sense0select>;
impl<'a, REG> Sense0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No detection"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Sense0select::None)
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Sense0select::Rise)
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Sense0select::Fall)
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(Sense0select::Both)
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Sense0select::High)
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Sense0select::Low)
    }
}
#[doc = "Field `FILTEN0` reader - Filter Enable 0"]
pub type Filten0R = crate::BitReader;
#[doc = "Field `FILTEN0` writer - Filter Enable 0"]
pub type Filten0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Input Sense Configuration 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sense1select {
    #[doc = "0: No detection"]
    None = 0,
    #[doc = "1: Rising edge detection"]
    Rise = 1,
    #[doc = "2: Falling edge detection"]
    Fall = 2,
    #[doc = "3: Both edges detection"]
    Both = 3,
    #[doc = "4: High level detection"]
    High = 4,
    #[doc = "5: Low level detection"]
    Low = 5,
}
impl From<Sense1select> for u8 {
    #[inline(always)]
    fn from(variant: Sense1select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sense1select {
    type Ux = u8;
}
impl crate::IsEnum for Sense1select {}
#[doc = "Field `SENSE1` reader - Input Sense Configuration 1"]
pub type Sense1R = crate::FieldReader<Sense1select>;
impl Sense1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sense1select> {
        match self.bits {
            0 => Some(Sense1select::None),
            1 => Some(Sense1select::Rise),
            2 => Some(Sense1select::Fall),
            3 => Some(Sense1select::Both),
            4 => Some(Sense1select::High),
            5 => Some(Sense1select::Low),
            _ => None,
        }
    }
    #[doc = "No detection"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Sense1select::None
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Sense1select::Rise
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Sense1select::Fall
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == Sense1select::Both
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Sense1select::High
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Sense1select::Low
    }
}
#[doc = "Field `SENSE1` writer - Input Sense Configuration 1"]
pub type Sense1W<'a, REG> = crate::FieldWriter<'a, REG, 3, Sense1select>;
impl<'a, REG> Sense1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No detection"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Sense1select::None)
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Sense1select::Rise)
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Sense1select::Fall)
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(Sense1select::Both)
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Sense1select::High)
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Sense1select::Low)
    }
}
#[doc = "Field `FILTEN1` reader - Filter Enable 1"]
pub type Filten1R = crate::BitReader;
#[doc = "Field `FILTEN1` writer - Filter Enable 1"]
pub type Filten1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Input Sense Configuration 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sense2select {
    #[doc = "0: No detection"]
    None = 0,
    #[doc = "1: Rising edge detection"]
    Rise = 1,
    #[doc = "2: Falling edge detection"]
    Fall = 2,
    #[doc = "3: Both edges detection"]
    Both = 3,
    #[doc = "4: High level detection"]
    High = 4,
    #[doc = "5: Low level detection"]
    Low = 5,
}
impl From<Sense2select> for u8 {
    #[inline(always)]
    fn from(variant: Sense2select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sense2select {
    type Ux = u8;
}
impl crate::IsEnum for Sense2select {}
#[doc = "Field `SENSE2` reader - Input Sense Configuration 2"]
pub type Sense2R = crate::FieldReader<Sense2select>;
impl Sense2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sense2select> {
        match self.bits {
            0 => Some(Sense2select::None),
            1 => Some(Sense2select::Rise),
            2 => Some(Sense2select::Fall),
            3 => Some(Sense2select::Both),
            4 => Some(Sense2select::High),
            5 => Some(Sense2select::Low),
            _ => None,
        }
    }
    #[doc = "No detection"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Sense2select::None
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Sense2select::Rise
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Sense2select::Fall
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == Sense2select::Both
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Sense2select::High
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Sense2select::Low
    }
}
#[doc = "Field `SENSE2` writer - Input Sense Configuration 2"]
pub type Sense2W<'a, REG> = crate::FieldWriter<'a, REG, 3, Sense2select>;
impl<'a, REG> Sense2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No detection"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Sense2select::None)
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Sense2select::Rise)
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Sense2select::Fall)
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(Sense2select::Both)
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Sense2select::High)
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Sense2select::Low)
    }
}
#[doc = "Field `FILTEN2` reader - Filter Enable 2"]
pub type Filten2R = crate::BitReader;
#[doc = "Field `FILTEN2` writer - Filter Enable 2"]
pub type Filten2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Input Sense Configuration 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sense3select {
    #[doc = "0: No detection"]
    None = 0,
    #[doc = "1: Rising edge detection"]
    Rise = 1,
    #[doc = "2: Falling edge detection"]
    Fall = 2,
    #[doc = "3: Both edges detection"]
    Both = 3,
    #[doc = "4: High level detection"]
    High = 4,
    #[doc = "5: Low level detection"]
    Low = 5,
}
impl From<Sense3select> for u8 {
    #[inline(always)]
    fn from(variant: Sense3select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sense3select {
    type Ux = u8;
}
impl crate::IsEnum for Sense3select {}
#[doc = "Field `SENSE3` reader - Input Sense Configuration 3"]
pub type Sense3R = crate::FieldReader<Sense3select>;
impl Sense3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sense3select> {
        match self.bits {
            0 => Some(Sense3select::None),
            1 => Some(Sense3select::Rise),
            2 => Some(Sense3select::Fall),
            3 => Some(Sense3select::Both),
            4 => Some(Sense3select::High),
            5 => Some(Sense3select::Low),
            _ => None,
        }
    }
    #[doc = "No detection"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Sense3select::None
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Sense3select::Rise
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Sense3select::Fall
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == Sense3select::Both
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Sense3select::High
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Sense3select::Low
    }
}
#[doc = "Field `SENSE3` writer - Input Sense Configuration 3"]
pub type Sense3W<'a, REG> = crate::FieldWriter<'a, REG, 3, Sense3select>;
impl<'a, REG> Sense3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No detection"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Sense3select::None)
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Sense3select::Rise)
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Sense3select::Fall)
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(Sense3select::Both)
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Sense3select::High)
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Sense3select::Low)
    }
}
#[doc = "Field `FILTEN3` reader - Filter Enable 3"]
pub type Filten3R = crate::BitReader;
#[doc = "Field `FILTEN3` writer - Filter Enable 3"]
pub type Filten3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Input Sense Configuration 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sense4select {
    #[doc = "0: No detection"]
    None = 0,
    #[doc = "1: Rising edge detection"]
    Rise = 1,
    #[doc = "2: Falling edge detection"]
    Fall = 2,
    #[doc = "3: Both edges detection"]
    Both = 3,
    #[doc = "4: High level detection"]
    High = 4,
    #[doc = "5: Low level detection"]
    Low = 5,
}
impl From<Sense4select> for u8 {
    #[inline(always)]
    fn from(variant: Sense4select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sense4select {
    type Ux = u8;
}
impl crate::IsEnum for Sense4select {}
#[doc = "Field `SENSE4` reader - Input Sense Configuration 4"]
pub type Sense4R = crate::FieldReader<Sense4select>;
impl Sense4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sense4select> {
        match self.bits {
            0 => Some(Sense4select::None),
            1 => Some(Sense4select::Rise),
            2 => Some(Sense4select::Fall),
            3 => Some(Sense4select::Both),
            4 => Some(Sense4select::High),
            5 => Some(Sense4select::Low),
            _ => None,
        }
    }
    #[doc = "No detection"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Sense4select::None
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Sense4select::Rise
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Sense4select::Fall
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == Sense4select::Both
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Sense4select::High
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Sense4select::Low
    }
}
#[doc = "Field `SENSE4` writer - Input Sense Configuration 4"]
pub type Sense4W<'a, REG> = crate::FieldWriter<'a, REG, 3, Sense4select>;
impl<'a, REG> Sense4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No detection"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Sense4select::None)
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Sense4select::Rise)
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Sense4select::Fall)
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(Sense4select::Both)
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Sense4select::High)
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Sense4select::Low)
    }
}
#[doc = "Field `FILTEN4` reader - Filter Enable 4"]
pub type Filten4R = crate::BitReader;
#[doc = "Field `FILTEN4` writer - Filter Enable 4"]
pub type Filten4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Input Sense Configuration 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sense5select {
    #[doc = "0: No detection"]
    None = 0,
    #[doc = "1: Rising edge detection"]
    Rise = 1,
    #[doc = "2: Falling edge detection"]
    Fall = 2,
    #[doc = "3: Both edges detection"]
    Both = 3,
    #[doc = "4: High level detection"]
    High = 4,
    #[doc = "5: Low level detection"]
    Low = 5,
}
impl From<Sense5select> for u8 {
    #[inline(always)]
    fn from(variant: Sense5select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sense5select {
    type Ux = u8;
}
impl crate::IsEnum for Sense5select {}
#[doc = "Field `SENSE5` reader - Input Sense Configuration 5"]
pub type Sense5R = crate::FieldReader<Sense5select>;
impl Sense5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sense5select> {
        match self.bits {
            0 => Some(Sense5select::None),
            1 => Some(Sense5select::Rise),
            2 => Some(Sense5select::Fall),
            3 => Some(Sense5select::Both),
            4 => Some(Sense5select::High),
            5 => Some(Sense5select::Low),
            _ => None,
        }
    }
    #[doc = "No detection"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Sense5select::None
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Sense5select::Rise
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Sense5select::Fall
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == Sense5select::Both
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Sense5select::High
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Sense5select::Low
    }
}
#[doc = "Field `SENSE5` writer - Input Sense Configuration 5"]
pub type Sense5W<'a, REG> = crate::FieldWriter<'a, REG, 3, Sense5select>;
impl<'a, REG> Sense5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No detection"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Sense5select::None)
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Sense5select::Rise)
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Sense5select::Fall)
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(Sense5select::Both)
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Sense5select::High)
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Sense5select::Low)
    }
}
#[doc = "Field `FILTEN5` reader - Filter Enable 5"]
pub type Filten5R = crate::BitReader;
#[doc = "Field `FILTEN5` writer - Filter Enable 5"]
pub type Filten5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Input Sense Configuration 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sense6select {
    #[doc = "0: No detection"]
    None = 0,
    #[doc = "1: Rising edge detection"]
    Rise = 1,
    #[doc = "2: Falling edge detection"]
    Fall = 2,
    #[doc = "3: Both edges detection"]
    Both = 3,
    #[doc = "4: High level detection"]
    High = 4,
    #[doc = "5: Low level detection"]
    Low = 5,
}
impl From<Sense6select> for u8 {
    #[inline(always)]
    fn from(variant: Sense6select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sense6select {
    type Ux = u8;
}
impl crate::IsEnum for Sense6select {}
#[doc = "Field `SENSE6` reader - Input Sense Configuration 6"]
pub type Sense6R = crate::FieldReader<Sense6select>;
impl Sense6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sense6select> {
        match self.bits {
            0 => Some(Sense6select::None),
            1 => Some(Sense6select::Rise),
            2 => Some(Sense6select::Fall),
            3 => Some(Sense6select::Both),
            4 => Some(Sense6select::High),
            5 => Some(Sense6select::Low),
            _ => None,
        }
    }
    #[doc = "No detection"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Sense6select::None
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Sense6select::Rise
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Sense6select::Fall
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == Sense6select::Both
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Sense6select::High
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Sense6select::Low
    }
}
#[doc = "Field `SENSE6` writer - Input Sense Configuration 6"]
pub type Sense6W<'a, REG> = crate::FieldWriter<'a, REG, 3, Sense6select>;
impl<'a, REG> Sense6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No detection"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Sense6select::None)
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Sense6select::Rise)
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Sense6select::Fall)
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(Sense6select::Both)
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Sense6select::High)
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Sense6select::Low)
    }
}
#[doc = "Field `FILTEN6` reader - Filter Enable 6"]
pub type Filten6R = crate::BitReader;
#[doc = "Field `FILTEN6` writer - Filter Enable 6"]
pub type Filten6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Input Sense Configuration 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sense7select {
    #[doc = "0: No detection"]
    None = 0,
    #[doc = "1: Rising edge detection"]
    Rise = 1,
    #[doc = "2: Falling edge detection"]
    Fall = 2,
    #[doc = "3: Both edges detection"]
    Both = 3,
    #[doc = "4: High level detection"]
    High = 4,
    #[doc = "5: Low level detection"]
    Low = 5,
}
impl From<Sense7select> for u8 {
    #[inline(always)]
    fn from(variant: Sense7select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sense7select {
    type Ux = u8;
}
impl crate::IsEnum for Sense7select {}
#[doc = "Field `SENSE7` reader - Input Sense Configuration 7"]
pub type Sense7R = crate::FieldReader<Sense7select>;
impl Sense7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sense7select> {
        match self.bits {
            0 => Some(Sense7select::None),
            1 => Some(Sense7select::Rise),
            2 => Some(Sense7select::Fall),
            3 => Some(Sense7select::Both),
            4 => Some(Sense7select::High),
            5 => Some(Sense7select::Low),
            _ => None,
        }
    }
    #[doc = "No detection"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Sense7select::None
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Sense7select::Rise
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Sense7select::Fall
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == Sense7select::Both
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Sense7select::High
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Sense7select::Low
    }
}
#[doc = "Field `SENSE7` writer - Input Sense Configuration 7"]
pub type Sense7W<'a, REG> = crate::FieldWriter<'a, REG, 3, Sense7select>;
impl<'a, REG> Sense7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No detection"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Sense7select::None)
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Sense7select::Rise)
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Sense7select::Fall)
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(Sense7select::Both)
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Sense7select::High)
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Sense7select::Low)
    }
}
#[doc = "Field `FILTEN7` reader - Filter Enable 7"]
pub type Filten7R = crate::BitReader;
#[doc = "Field `FILTEN7` writer - Filter Enable 7"]
pub type Filten7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Input Sense Configuration 0"]
    #[inline(always)]
    pub fn sense0(&self) -> Sense0R {
        Sense0R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Filter Enable 0"]
    #[inline(always)]
    pub fn filten0(&self) -> Filten0R {
        Filten0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Input Sense Configuration 1"]
    #[inline(always)]
    pub fn sense1(&self) -> Sense1R {
        Sense1R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Filter Enable 1"]
    #[inline(always)]
    pub fn filten1(&self) -> Filten1R {
        Filten1R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Input Sense Configuration 2"]
    #[inline(always)]
    pub fn sense2(&self) -> Sense2R {
        Sense2R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Filter Enable 2"]
    #[inline(always)]
    pub fn filten2(&self) -> Filten2R {
        Filten2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Input Sense Configuration 3"]
    #[inline(always)]
    pub fn sense3(&self) -> Sense3R {
        Sense3R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Filter Enable 3"]
    #[inline(always)]
    pub fn filten3(&self) -> Filten3R {
        Filten3R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Input Sense Configuration 4"]
    #[inline(always)]
    pub fn sense4(&self) -> Sense4R {
        Sense4R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - Filter Enable 4"]
    #[inline(always)]
    pub fn filten4(&self) -> Filten4R {
        Filten4R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - Input Sense Configuration 5"]
    #[inline(always)]
    pub fn sense5(&self) -> Sense5R {
        Sense5R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - Filter Enable 5"]
    #[inline(always)]
    pub fn filten5(&self) -> Filten5R {
        Filten5R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Input Sense Configuration 6"]
    #[inline(always)]
    pub fn sense6(&self) -> Sense6R {
        Sense6R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - Filter Enable 6"]
    #[inline(always)]
    pub fn filten6(&self) -> Filten6R {
        Filten6R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - Input Sense Configuration 7"]
    #[inline(always)]
    pub fn sense7(&self) -> Sense7R {
        Sense7R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - Filter Enable 7"]
    #[inline(always)]
    pub fn filten7(&self) -> Filten7R {
        Filten7R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Input Sense Configuration 0"]
    #[inline(always)]
    #[must_use]
    pub fn sense0(&mut self) -> Sense0W<ConfigSpec> {
        Sense0W::new(self, 0)
    }
    #[doc = "Bit 3 - Filter Enable 0"]
    #[inline(always)]
    #[must_use]
    pub fn filten0(&mut self) -> Filten0W<ConfigSpec> {
        Filten0W::new(self, 3)
    }
    #[doc = "Bits 4:6 - Input Sense Configuration 1"]
    #[inline(always)]
    #[must_use]
    pub fn sense1(&mut self) -> Sense1W<ConfigSpec> {
        Sense1W::new(self, 4)
    }
    #[doc = "Bit 7 - Filter Enable 1"]
    #[inline(always)]
    #[must_use]
    pub fn filten1(&mut self) -> Filten1W<ConfigSpec> {
        Filten1W::new(self, 7)
    }
    #[doc = "Bits 8:10 - Input Sense Configuration 2"]
    #[inline(always)]
    #[must_use]
    pub fn sense2(&mut self) -> Sense2W<ConfigSpec> {
        Sense2W::new(self, 8)
    }
    #[doc = "Bit 11 - Filter Enable 2"]
    #[inline(always)]
    #[must_use]
    pub fn filten2(&mut self) -> Filten2W<ConfigSpec> {
        Filten2W::new(self, 11)
    }
    #[doc = "Bits 12:14 - Input Sense Configuration 3"]
    #[inline(always)]
    #[must_use]
    pub fn sense3(&mut self) -> Sense3W<ConfigSpec> {
        Sense3W::new(self, 12)
    }
    #[doc = "Bit 15 - Filter Enable 3"]
    #[inline(always)]
    #[must_use]
    pub fn filten3(&mut self) -> Filten3W<ConfigSpec> {
        Filten3W::new(self, 15)
    }
    #[doc = "Bits 16:18 - Input Sense Configuration 4"]
    #[inline(always)]
    #[must_use]
    pub fn sense4(&mut self) -> Sense4W<ConfigSpec> {
        Sense4W::new(self, 16)
    }
    #[doc = "Bit 19 - Filter Enable 4"]
    #[inline(always)]
    #[must_use]
    pub fn filten4(&mut self) -> Filten4W<ConfigSpec> {
        Filten4W::new(self, 19)
    }
    #[doc = "Bits 20:22 - Input Sense Configuration 5"]
    #[inline(always)]
    #[must_use]
    pub fn sense5(&mut self) -> Sense5W<ConfigSpec> {
        Sense5W::new(self, 20)
    }
    #[doc = "Bit 23 - Filter Enable 5"]
    #[inline(always)]
    #[must_use]
    pub fn filten5(&mut self) -> Filten5W<ConfigSpec> {
        Filten5W::new(self, 23)
    }
    #[doc = "Bits 24:26 - Input Sense Configuration 6"]
    #[inline(always)]
    #[must_use]
    pub fn sense6(&mut self) -> Sense6W<ConfigSpec> {
        Sense6W::new(self, 24)
    }
    #[doc = "Bit 27 - Filter Enable 6"]
    #[inline(always)]
    #[must_use]
    pub fn filten6(&mut self) -> Filten6W<ConfigSpec> {
        Filten6W::new(self, 27)
    }
    #[doc = "Bits 28:30 - Input Sense Configuration 7"]
    #[inline(always)]
    #[must_use]
    pub fn sense7(&mut self) -> Sense7W<ConfigSpec> {
        Sense7W::new(self, 28)
    }
    #[doc = "Bit 31 - Filter Enable 7"]
    #[inline(always)]
    #[must_use]
    pub fn filten7(&mut self) -> Filten7W<ConfigSpec> {
        Filten7W::new(self, 31)
    }
}
#[doc = "External Interrupt Sense Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigSpec;
impl crate::RegisterSpec for ConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config::R`](R) reader structure"]
impl crate::Readable for ConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for ConfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONFIG[%s]
to value 0"]
impl crate::Resettable for ConfigSpec {
    const RESET_VALUE: u32 = 0;
}
